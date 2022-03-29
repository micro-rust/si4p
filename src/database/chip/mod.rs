//! Chip database.
//! The chip database contains a registry of all supported chips and user configured chips.



pub mod cmds;
mod fmls;
mod mnfs;
mod vars;



use architecture::Chip;

use crate::log::TimeReport;

use database::{ Database as DatabaseTrait, Interface, common::DBInterface };

use futures::future::join_all;

use std::{
    collections::HashMap,
    path::PathBuf,
};

use tokio::{
    join,

    fs::{ read_dir, File },
    io::{ AsyncReadExt },
    sync::{ mpsc },
};

use tracing::{
    debug, error, info,

    instrument::WithSubscriber,
};



use self::cmds::DBCommand as Command;
use self::fmls::FamilyDatabase;
use self::mnfs::ManufacturerDatabase;
use self::vars::VariantDatabase;



pub struct Database {
    /// Base list of chips loaded into the database.
    chip: Vec<Chip>,

    /// Manufacturer database.
    mnfs: ManufacturerDatabase,

    /// Family database.
    fmls: FamilyDatabase,

    /// Variant database.
    vars: VariantDatabase,

    /// Suffix search engine.
    suff: HashMap<String, Vec<usize>>,

    /// Command channel receiver.
    cmds: mpsc::Receiver<Command>,
}

impl Database {
    /// Creates a new `Database`.
    pub fn new(cmds: mpsc::Receiver<Command>) -> Self {
        Database {
            chip: Vec::new(),
            mnfs: ManufacturerDatabase::new(),
            fmls: FamilyDatabase::new(),
            vars: VariantDatabase::new(),
            suff: HashMap::new(),
            cmds,
        }
    }

    /// Initializes the `Database` and runs in a separate thread.
    pub async fn run(&mut self) {
        // Create the time reports.
        let mut report = TimeReport::new(String::from("chipdb-init"));
        let filereport = TimeReport::new(String::from("chipdb-init-files"));
        let mnfsreport = TimeReport::new(String::from("chipdb-init-engine-manufacturer"));
        let fmlsreport = TimeReport::new(String::from("chipdb-init-engine-family"));
        let varsreport = TimeReport::new(String::from("chipdb-init-engine-variant"));
        let suffreport = TimeReport::new(String::from("chipdb-init-engine-suffix"));

        let start = report.start();

        // Get the base folder.
        let base = match super::basefolder("chip").await {
            Some(path) => {
                debug!(origin="database", db="chip", "Found chip database folder at {}", path.display());
                path
            },

            _ => {
                error!(origin="database", db="chip", "Could not find ChipDatabase folder.");
                return;
            },
        };

        // Load all the chips.
        let file = load(base, &mut self.chip, filereport).with_current_subscriber().await;

        // Spawn the subtasks and wait on them.
        let (mnfs, fmls, vars, suff) = join!{
            tokio::spawn( mnfs::mnfs(self.chip.clone(), mnfsreport).with_current_subscriber() ),
            tokio::spawn( fmls::fmls(self.chip.clone(), fmlsreport).with_current_subscriber() ),
            tokio::spawn( vars::vars(self.chip.clone(), varsreport).with_current_subscriber() ),
            tokio::spawn( suff(self.chip.clone(), suffreport).with_current_subscriber() ),
        };

        // End the report.
        report.end(start);

        // Add the secondary reports to the report.
        report.add(file);

        match mnfs {
            Err(e) => error!(origin="database", db="chip", "Could not generate manufacturer relational database: {}", e),
            Ok((map, time)) => {
                report.add(time);
                self.mnfs = map;
            },
        }

        match fmls {
            Err(e) => error!(origin="database", db="chip", "Could not generate family relational database: {}", e),
            Ok((map, time)) => {
                report.add(time);
                self.fmls = map;
            },
        }

        match vars {
            Err(e) => error!(origin="database", db="chip", "Could not generate variant relational database: {}", e),
            Ok((map, time)) => {
                report.add(time);
                self.vars = map;
            },
        }

        match suff {
            Err(e) => error!(origin="database", db="chip", "Could not generate suffix search engine: {}", e),
            Ok((map, time)) => {
                report.add(time);
                self.suff = map;
            },
        }

        info!(origin="database", db="chip", "Chip Database load time:\n{}", report);

        // Start executing database events.
        self.eventloop().await;
    }

    /// Runs the event loop of the database.
    async fn eventloop(&mut self) {
        loop {
            match self.cmds.recv().await {
                Some(_) => (),
                _ => break,
            }
        }
    }
}


impl DatabaseTrait for Database {
    fn spawn() -> Box<dyn Interface> {
        // Create a new channel.
        let (tx, rx) = mpsc::channel(64);

        // Create a new `Database`, spawn it in a new `tokio` task and return the interface.
        let mut database = Database::new(rx);

        tokio::spawn( async move { database.run().with_current_subscriber().await } );

        Box::new( DBInterface::new(tx) )
    }
}



/// Generates the suffix search engine.
async fn suff(chips: Vec<Chip>, mut report: TimeReport) -> (HashMap<String, Vec<usize>>, TimeReport) {
    let start = report.start();

    let mut suff = HashMap::new();

    for (i, chip) in chips.iter().enumerate() {
        let name = chip.name();

        for j in 0..name.len() {
            for k in (j+1)..name.len() {
                let substring = String::from( &name[j..=k] );

                match suff.get_mut(&substring) {
                    None => { suff.insert(substring, vec![i]); },
                    Some(list) => list.push(i),
                }
            }
        }
    }

    report.end(start);
    (suff, report)
}

/// Loads all files below the current directory and parses them.
async fn load(folder: PathBuf, chips: &mut Vec<Chip>, mut report: TimeReport) -> TimeReport {
    /// Search depth.
    const DEPTH: usize = 10;

    let start = report.start();

    // List of files to parse.
    let mut files = Vec::new();

    // List of folders to search.
    let mut dirs = vec![folder];

    for i in 0..DEPTH {
        let mut newdirs = Vec::new();

        for dir in dirs.iter() {
            // Get all entries within the directory.
            let mut entries = match read_dir(dir).await {
                Err(e) => {
                    error!(origin = "database", db="chip", "Could not read entries in directory {}: {}", dir.display(), e);
                    continue;
                },

                Ok(e) => e,
            };

            // Classify all entries into folders and files, discard symlinks.
            while let Ok(Some(entry)) = entries.next_entry().await {
                let path = entry.path();

                if path.is_file() {
                    files.push(path);
                    continue;
                }

                if path.is_dir() {
                    newdirs.push(path);
                    continue;
                }
            }
        }

        if i < (DEPTH - 1) {
            dirs = newdirs;
        }
    }

    // Small info on number of files.
    debug!(origin = "database", db="chip", "Preparing to parse {} files", files.len());


    // Preallocate the chip vector.
    let tasks: Vec<_> = files.iter()
        .map(|file| {
            tokio::spawn( parse(file.clone()).with_current_subscriber() )
        })
        .collect();

    // Await all futures.
    let newchips = join_all(tasks).await;

    for chip in newchips.iter() {
        match chip {
            Ok(Some(c)) => chips.push(c.clone()),
            _ => (),
        }
    }

    report.end(start);
    report
}

/// Parses a Chip file.
async fn parse(path: PathBuf) -> Option<Chip> {
    // Open the file.
    let mut file = match File::open(path.clone()).await {
        Err(e) => {
            error!(origin = "database", db="chip", "Could not open file {}: {}", path.display(), e);

            return None;
        },
        Ok(f) => f,
    };

    // Create a buffer and read the file into it.
    let mut buffer = Vec::new();

    match file.read_to_end(&mut buffer).await {
        Err(e) => {
            error!(origin = "database", db="chip", "Could not read contents of file {}: {}", path.display(), e);
            return None;
        },

        _ => (),
    }

    // Parse the file.
    match Chip::parse(buffer) {
        Err(e) => {
            error!(origin = "database", db="chip", "Failed to parse chip data in file {}: {}", path.display(), e);
            None
        },

        Ok(c) => {
            debug!(origin = "database", db="chip", "Successfully loaded chip file {}", path.display());

            return Some(c);
        },
    }
}
