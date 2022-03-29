//! Chip database.
//! The chip database contains a registry of all supported chips and user configured chips.


#![allow(warnings)]


pub mod cmds;



use crate::{
    log::TimeReport,
    project::ProjectSerial,
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
    sync::Arc,
};

use super::statusupdate;

use tokio::{
    join,

    fs::{ read_dir, File },
    io::{ AsyncReadExt },
    sync::{ mpsc, RwLock },
};

use tracing::{
    debug, error, info, warn,

    instrument::WithSubscriber,
};



use self::cmds::DBCommand as Command;

pub use self::cmds::{ Command as ProjectCommand, Response as ProjectResponse };



pub struct Database {
    /// Collection of all existing (and found) projects.
    projects: Arc<RwLock<Vec<ProjectSerial>>>,

    /// A name collection of the projects.
    name: Arc<RwLock<HashMap<String, usize>>>,

    /// Suffix search engine.
    suff: Arc<RwLock<HashMap<String, Vec<usize>>>>,

    /// Command channel receiver.
    cmds: mpsc::Receiver<Command>,

    /// Flag to check for database updates.
    updated: bool,
}

impl Database {
    /// Creates a new `Database`.
    pub fn new(cmds: mpsc::Receiver<Command>) -> Self {
        Database {
            projects: Arc::new( RwLock::new( Vec::new() ) ),
            name: Arc::new( RwLock::new( HashMap::new() ) ),
            suff: Arc::new( RwLock::new( HashMap::new() ) ),
            cmds,
            updated: false,
        }
    }

    /// Initializes the `Database` and runs in a separate thread.
    pub async fn run(&mut self) {
        let mut report = TimeReport::new( String::from("projectdb-init") );
        let start = report.start();

        // Load the database and generate the search engine.
        match load().await {
            (Some(mut projects), rl) => {
                // Sort the projects by alphabetical name and store it in the database.
                projects.sort_by_key(|p| p.info.name.clone());
                *self.projects.write().await = projects;

                report.add(rl);

                // Build the search engine.
                let (name, suff) = join!{
                    tokio::spawn( name(self.projects.clone()).with_current_subscriber() ),
                    tokio::spawn( suff(self.projects.clone()).with_current_subscriber() ),
                };

                // Check the result of the name and suffix datasets.
                match name {
                    Err(e) => {
                        error!(origin="database", db="project","Could not generate project name dataset");
                    },

                    Ok((name, rn)) => {
                        *self.name.write().await = name;
                        report.add(rn);
                    },
                }

                match suff {
                    Err(e) => {
                        error!(origin="database", db="project","Could not generate project suffix dataset");
                    },

                    Ok((suff, rs)) => {
                        *self.suff.write().await = suff;
                        report.add(rs);
                    },
                }
            },

            (None, rl) => {
                report.end(start);
                report.add(rl);

                error!(origin="database", db="project","Did not load project database during initialization");
            },
        }

        info!(origin="database", db="project","Project database initialization time:\n{}", report);

        // Start executing database events.
        self.eventloop().await;
    }

    /// Runs the event loop of the database.
    async fn eventloop(&mut self) {
        loop {
            match self.cmds.recv().await {
                Some(cmd) => {
                    let (cmd, channel, mut status) = cmd.destructure();

                    statusupdate(&mut status, Status::Acknowledged).await;

                    match cmd {
                        ProjectCommand::CreateProject(project) => {
                            // Get write access to the projects and name database.
                            let (mut projects, mut name) = join!{
                                self.projects.write(),
                                self.name.write(),
                            };

                            // Index this project would be inserted into.
                            let i = projects.len();

                            // The name of the project.
                            let namestr = project.name().clone();

                            // Insert in the name map.
                            match name.get(&namestr) {
                                None => {
                                    name.insert(namestr.clone(), i);
                                    projects.push(project);

                                    debug!(origin="database", db="project", "Current number of projects: {}", projects.len());

                                    self.updated = true;

                                    // Get write access to the suffix database.
                                    let mut suff = self.suff.write().await;

                                    // Create the suffixes.
                                    for j in 0..namestr.len() {
                                        for k in (j+1)..namestr.len() {
                                            let substring = String::from( &namestr[j..=k] );

                                            match suff.get_mut(&substring) {
                                                None => { suff.insert(substring, vec![i]); },
                                                Some(list) => list.push(i),
                                            }
                                        }
                                    }

                                    {
                                        drop(suff);
                                        drop(name);
                                        drop(projects);
                                    }

                                    debug!(origin="database", db="project", "Added a new project named {}", namestr);

                                    channel.send(ProjectResponse::Done);

                                    match status.send(Status::Completed).await {
                                        Err(e) => error!(origin="database", db="project", "Could not send 'Completed' status update for 'CreateProject' command: {}", e),
                                        _ => (),
                                    }
                                },

                                _ => match status.send(Status::Denied).await {
                                    Err(e) => error!(origin="database", db="project", "Could not send 'Denied' status update for failed 'CreateProject' command: {}", e),
                                    _ => (),
                                },
                            }
                        },

                        ProjectCommand::GetSearchEngine => {
                            channel.send( ProjectResponse::SearchEngine(self.projects.clone(), self.suff.clone()) );

                            match status.send(Status::Completed).await {
                                Err(e) => error!(origin="database", db="project", "Could not send 'Completed' status update for 'GetSearchEngine' command: {}", e),
                                _ => (),
                            }
                        },

                        ProjectCommand::DeleteProject(namestr) => {
                            // Get write access to the name database.
                            let mut name = self.name.write().await;

                            match name.remove(&namestr) {
                                Some(idx) => {
                                    debug!(origin="database", db="project", "Deleting project named {}", namestr);

                                    // Get write access to the suffix and projects database.
                                    let (mut projects, mut suffix) = join!{
                                        self.projects.write(),
                                        self.suff.write(),
                                    };

                                    // Check if the project actually exists.
                                    if idx >= projects.len() {
                                        error!(origin="database", db="project", "Attempted to delete an index that does not exist");

                                        // Send completed status.
                                        channel.send( ProjectResponse::DoesNotExist );

                                        match status.send(Status::Completed).await {
                                            Err(e) => error!(origin="database", db="project", "Could not send 'Completed' status update for 'DeleteProject' command: {}", e),
                                            _ => (),
                                        }

                                        continue;
                                    }

                                    // Remove the entries in the suffix engine.
                                    for array in suffix.values_mut() {
                                        *array = (*array).iter().map(|x| *x).filter(|x| *x != idx).collect();

                                        for value in array {
                                            if *value > idx {
                                                *value -= 1;
                                            }
                                        }
                                    }

                                    // Remove the entries in the name engine.
                                    for value in name.values_mut() {
                                        if *value > idx {
                                            *value -= 1;
                                        }
                                    }

                                    // Remove from the array.
                                    projects.remove(idx);

                                    debug!(origin="database", db="project", "Project named {} removed: Number of projects: {}", namestr, projects.len());

                                    {
                                        drop(suffix);
                                        drop(name);
                                        drop(projects);
                                    }

                                    // Send completed status.
                                    channel.send( ProjectResponse::Done );

                                    match status.send(Status::Completed).await {
                                        Err(e) => error!(origin="database", db="project", "Could not send 'Completed' status update for 'DeleteProject' command: {}", e),
                                        _ => (),
                                    }
                                },
                                None => {
                                    error!(origin="database", db="project", "Attempted to delete non existent project {}", namestr);
                                    channel.send( ProjectResponse::DoesNotExist );

                                    match status.send(Status::Completed).await {
                                        Err(e) => error!(origin="database", db="project", "Could not send 'Completed' status update for 'DeleteProject' command: {}", e),
                                        _ => (),
                                    }
                                },
                            }
                        },

                        _ => (),
                    }
                },
                _ => break,
            }
        }

        // TODO : Implement saving
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



/// Loads a project.
async fn load() -> (Option<Vec<ProjectSerial>>, TimeReport) {
    let mut report = TimeReport::new(String::from("projectdb-load"));
    let start = report.start();

    // Get the base folder.
    let base = match super::basefolder("projects").await {
        Some(path) => {
            debug!(origin="database", db="project", "Found project database folder at {}", path.display());
            path
        },

        _ => {
            error!(origin="database", db="project", "Could not find ProjectDatabase folder");
            report.end(start);
            return (None, report);
        },
    };

    // Read the entries in the base folder.
    let mut entries = match read_dir(base.clone()).await {
        Err(e) => {
            error!(origin = "database", db="project", "Could not read project files in directory {}: {}", base.display(), e);
            report.end(start);
            return (None, report);
        },
        Ok(e) => e,
    };

    // Create the arrays to store the results.
    let mut files = Vec::new();

    while let Ok(Some(entry)) = entries.next_entry().await {
        // Get the path of the entry.
        let path = entry.path();

        if path.is_file() {
            // Get the name.
            let name = match entry.file_name().into_string() {
                Err(e) => {
                    error!(origin = "database", db="theme", "Could not format name {:?} as UTF-8", e);
                    String::from("unknown")
                },
                Ok(s) => s,
            };

            files.push((name, path));
        }
    }

    // Small info on number of themes.
    debug!(origin = "database", db="project", "Preparing to parse {} projects", files.len());


    // Preallocate the theme vector.
    let tasks: Vec<_> = files.into_iter()
        .map(|(name, filepath)| {
            tokio::spawn( projectparse(filepath, name).with_current_subscriber() )
        })
        .collect();

    // Await all futures.
    let new = join_all(tasks).await;

    // Create the output vector.
    let mut projects = Vec::with_capacity(new.len());

    for project in new.into_iter() {
        match project {
            Ok(Some((p, r))) => {
                projects.push( p );
                report.add(r);
            },
            _ => (),
        }
    }

    report.end(start);

    ( Some( projects ), report )
}


/// Parses a project's folder.
async fn projectparse(path: PathBuf, filename: String) -> Option<(ProjectSerial, TimeReport)> {
    let mut report = TimeReport::new(format!("projectdb-load-[{}]", filename));
    let start = report.start();

    // Open the file.
    let mut file = match File::open(path.clone()).await {
        Err(e) => {
            error!(origin = "database", db="project", "Could not open file {}: {}", path.display(), e);

            return None;
        },
        Ok(f) => f,
    };

    // Create a buffer and read the file into it.
    let mut buffer = Vec::new();

    match file.read_to_end(&mut buffer).await {
        Err(e) => {
            error!(origin = "database", db="project", "Could not read contents of file {}: {}", path.display(), e);
            return None;
        },

        _ => (),
    }

    // Parse the file.
    let project = match ProjectSerial::parse(buffer) {
        Err(e) => {
            error!(origin = "database", db="project", "Failed to parse project data in file {}: {}", path.display(), e);
            return None;
        },

        Ok(p) => {
            debug!(origin = "database", db="project", "Successfully loaded project file {}", path.display());

            p
        },
    };

    report.end(start);

    Some( (project, report) )
}

/// Generates the suffix search engine.
async fn suff(projects: Arc<RwLock<Vec<ProjectSerial>>>) -> (HashMap<String, Vec<usize>>, TimeReport) {
    let mut report = TimeReport::new( String::from("projectdb-build-searchengine") );
    let start = report.start();

    let mut suff = HashMap::new();

    let projects = projects.read().await;

    for (i, chip) in projects.iter().enumerate() {
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

/// Generates the name hashmap collection.
async fn name(projects: Arc<RwLock<Vec<ProjectSerial>>>) -> (HashMap<String, usize>, TimeReport) {
    let mut report = TimeReport::new( String::from("projectdb-build-namemap") );
    let start = report.start();

    let mut name = HashMap::new();

    let projects = projects.read().await;

    for (i, project) in projects.iter().enumerate() {
        let n = project.name();

        name.insert(n.clone(), i);
    }

    report.end(start);
    (name, report)
}
