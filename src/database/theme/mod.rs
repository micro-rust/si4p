//! Chip database.
//! The chip database contains a registry of all supported chips and user configured chips.



pub mod cmds;



use einstein::TimeReport;

use marcel::{
    Theme,
};



use database::{
    Database as DatabaseTrait, Interface,
    common::{
        DBInterface, Status,
    },
};

use futures::future::join_all;

use std::{
    collections::HashMap,
    path::PathBuf,
};

use super::statusupdate;

use std::sync::Arc;

use tokio::{
    fs::{ read_dir, File },
    io::{ AsyncReadExt },
    sync::{ mpsc, RwLock },
};

use tracing::{
    debug, error, info, warn,

    instrument::WithSubscriber,
};



pub use self::cmds::{
    DBCommand as Command,
    Command as ThemeCommand,
    Response as ThemeResponse,
};



pub struct Database {
    /// Base list of themes loaded in the database.
    themes: HashMap<String, Arc<RwLock<Theme>>>,

    /// Current active theme.
    current: Arc<RwLock<Theme>>,

    /// Command channel receiver.
    cmds: mpsc::Receiver<Command>,
}

impl Database {
    /// Creates a new `Database`.
    pub fn new(cmds: mpsc::Receiver<Command>) -> Self {
        Database {
            themes: HashMap::new(),
            current: Arc::new( RwLock::new( Default::default() ) ),
            cmds,
        }
    }

    /// Initializes the `Database` and runs in a separate thread.
    pub async fn run(&mut self) {
        // Create the time reports.
        let mut report = TimeReport::new(String::from("themedb-init"));

        let start = report.start();

        // Get the base folder.
        let base = match super::basefolder("theme").await {
            Some(path) => {
                debug!(origin="database", db="theme", "Found theme database folder at {}", path.display());
                path
            },

            _ => {
                error!(origin="database", db="theme", "Could not find ThemeDatabase folder.");
                return;
            },
        };

        // Load all the themes.
        let filereport = load(base, &mut self.themes, report.subtask("load")).with_current_subscriber().await;

        // End the report.
        report.end(start);
        report.add(filereport);

        info!(origin="database", db="theme", "Theme Database load time:\n{}", report);

        // Load default current theme until a new one is set.
        let name = match self.themes.values().next() {
            Some(th) => {
                self.current = th.clone();
                th.read().await.name.clone()
            },
            _ => {
                warn!(origin="database/project", "No default theme could be loaded");
                String::from("None")
            },
        };

        info!(origin="database", db="theme", "Theme Database defaulted to theme '{}'", name);

        // Start executing database events.
        self.eventloop().await;
    }

    /// Runs the event loop of the database.
    async fn eventloop(&mut self) {
        loop {
            match self.cmds.recv().await {
                Some(cmd) => {
                    // Destructure the command.
                    let (cmd, channel, mut status) = cmd.destructure();

                    statusupdate(&mut status, Status::Acknowledged).await;

                    match cmd {
                        // Return the current global theme.
                        ThemeCommand::GetTheme => {
                            match channel.send( ThemeResponse::Theme( self.current.clone() ) ) {
                                Err(_) => {
                                    error!(origin="database/theme", "Coiuld not send currently active theme: Channel closed unexpectedly");
                                    statusupdate(&mut status, Status::Failed(None)).await;
                                },
                                _ => info!(origin="database/theme", "Sent a copy of the Global Theme"),
                            }

                            statusupdate(&mut status, Status::Completed).await;
                        },

                        ThemeCommand::ChangeTheme(name) => {
                            match self.themes.get(&name) {
                                None => {
                                    error!(origin="database/theme", "No theme named {} in the database", name);
                                    statusupdate(&mut status, Status::Failed(None)).await;
                                },
                                Some(theme) => {
                                    // Change current theme.
                                    self.current = theme.clone();

                                    // Send back the updated theme.
                                    match channel.send( ThemeResponse::Theme( self.current.clone() ) ) {
                                        Err(_) => {
                                            error!(origin="database/theme", "Coiuld not send newly changed active theme: Channel closed unexpectedly");
                                            statusupdate(&mut status, Status::Failed(None)).await;
                                        },
                                        _ => info!(origin="database/theme", "Sent a copy of the newly changed Global Theme"),
                                    }

                                    statusupdate(&mut status, Status::Completed).await;
                                },
                            }
                        },
                    }
                },
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



/// Loads all files below the current directory and parses them.
async fn load(folder: PathBuf, themes: &mut HashMap<String, Arc<RwLock<Theme>>>, mut report: TimeReport) -> TimeReport {
    let start = report.start();

    // Read the entries in the base folder.
    let mut entries = match read_dir(folder.clone()).await {
        Err(e) => {
            error!(origin = "database", db="theme", "Could not read theme entries in directory {}: {}", folder.display(), e);
            report.end(start);
            return report;
        },
        Ok(e) => e,
    };

    // Collect only the entries that are folders.
    let mut files = Vec::new();

    while let Ok(Some(entry)) = entries.next_entry().await {
        // Get the path of the entry.
        let path = entry.path();

        if path.is_file() {
            // Get the name.
            let name = match entry.file_name().into_string() {
                Err(e) => {
                    error!(origin = "database", db="theme", "Could not format name {:?} as UTF-8", e);
                    continue;
                },
                Ok(s) => s,
            };

            files.push( (name, path) );
        }
    }

    // Small info on number of themes.
    debug!(origin = "database", db="theme", "Preparing to parse {} themes", files.len());


    // Preallocate the theme vector.
    let tasks: Vec<_> = files.into_iter()
        .map(|(name, folder)| {
            tokio::spawn( parse(folder, report.subtask(name)).with_current_subscriber() )
        })
        .collect();

    // Await all futures.
    let new = join_all(tasks).await;

    for theme in new.into_iter() {
        match theme {
            Err(e) => error!(origin="database/project", "A theme parse task failed: {}", e),

            Ok((t, r)) => match t {
                Some(th) => {
                    themes.insert(th.name.clone(), Arc::new( RwLock::new( th ) ));
                    report.add(r);
                },
                _ => {
                    warn!(origin="database/project", "A theme parse failed");
                    report.add(r);
                },
            },
        }
    }

    report.end(start);
    report
}

/// Parses a `Theme` file.
async fn parse(path: PathBuf, mut report: TimeReport) -> (Option<Theme>, TimeReport) {
    // Start the report.
    let start = report.start();

    // Open the file.
    let mut file = match File::open(path.clone()).await {
        Err(e) => {
            error!(origin = "database", db="theme", "Could not open file {}: {}", path.display(), e);
            report.end(start);
            return (None, report);
        },
        Ok(f) => f,
    };

    // Create a buffer and read the file into it.
    let mut buffer = Vec::new();

    match file.read_to_end(&mut buffer).await {
        Err(e) => {
            error!(origin = "database", db="theme", "Could not read contents of file {}: {}", path.display(), e);
            report.end(start);
            return (None, report);
        },

        _ => (),
    }

    // Convert the raw file into a String and parse it.
    match String::from_utf8(buffer) {
        Err(e) => {
            error!(origin = "database", db="theme", "Failed to parse UTF-8 string in file {}: {}", path.display(), e);
            report.end(start);
            return (None, report);
        },

        Ok(s) => match ron::from_str(&s) {
            Err(e) => {
                error!(origin = "database", db="theme", "Could not parse RON file {}: {}", path.display(), e);
                report.end(start);
                return (None, report);
            },

            Ok(t) => {
                debug!(origin = "database", db="theme", "Successfully loaded Theme file {}", path.display());
                report.end(start);
                return (Some(t), report);
            },
        },
    }
}
