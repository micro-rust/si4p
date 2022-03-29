//! Database module.
//! Contains the different databases used in the app.



#![deny(warnings)]
#![allow(dead_code)]



pub mod chip;
pub mod project;
pub mod regex;
pub mod theme;



use database::{
    Root,
    common::{ DBInterface, Status },
};

use lazy_static::lazy_static;

use std::{
    path::PathBuf,
    sync::Mutex,
};

use tokio::{
    sync::{
        mpsc,
    },
};

use tracing::{
    error, info,
};



pub use self::chip::cmds::   { Command as ChipCommand,    Response as ChipResponse    };
pub use self::project::cmds::{ Command as ProjectCommand, Response as ProjectResponse };
pub use self::regex::cmds::  { Command as RegexCommand,   Response as RegexResponse   };
pub use self::theme::cmds::  { Command as ThemeCommand,   Response as ThemeResponse   };



lazy_static! {
    pub static ref BASEFOLDER: Mutex<Option<PathBuf>> = Mutex::new( None );
}



/// Root database that collects the interfaces to the databases.
pub struct Database {
    /// Root database.
    root: Root,

    /// Index of the chip database interface.
    chip: usize,

    /// Index of the theme database interface.
    theme: usize,

    /// Index of the regex database interface.
    regex: usize,

    /// Index of the project database interface.
    project: usize,
}

impl Database {
    /// Creates the database.
    pub fn new() -> Self {
        // Create the root database.
        let root = Root::new();

        Database {
            root,
            chip: 0,
            theme: 0,
            regex: 0,
            project: 0,
        }
    }

    /// Initializes the databases.
    pub fn init(&mut self) {
        // Add the basic databases.
        self.chip = self.root.add::<chip::Database>();
        self.theme = self.root.add::<theme::Database>();
        self.regex = self.root.add::<regex::Database>();
        self.project = self.root.add::<project::Database>();

        info!(origin="database", db="root", "Database initialized");
    }

    /// Returns an interface to the Chip Database.
    pub fn chip(&self) -> Option<Box<DBInterface<ChipCommand, ChipResponse>>> {
        match self.root.interface(self.chip) {
            Some(i) => match i.downcast() {
                Err(e) => {
                    error!(origin="database", db="root", "Could not downcast Chip Database from `dyn Any`: {:?}", e);
                    None
                },

                Ok(i) => Some(i),
            },
            _ => None,
        }
    }

    /// Returns an interface to the Theme Database.
    pub fn theme(&self) -> Option<Box<DBInterface<ThemeCommand, ThemeResponse>>> {
        match self.root.interface(self.theme) {
            Some(i) => match i.downcast() {
                Err(e) => {
                    error!(origin="database", db="root", "Could not downcast Theme Database from `dyn Any`: {:?}", e);
                    None
                },

                Ok(i) => Some(i),
            },
            _ => None,
        }
    }

    /// Returns an interface to the Regex Database.
    pub fn regex(&self) -> Option<Box<DBInterface<RegexCommand, RegexResponse>>> {
        match self.root.interface(self.regex) {
            Some(i) => match i.downcast() {
                Err(e) => {
                    error!(origin="database", db="root", "Could not downcast Regex Database from `dyn Any`: {:?}", e);
                    None
                },

                Ok(i) => Some(i),
            },
            _ => None,
        }
    }

    /// Returns an interface to the Project Database.
    pub fn project(&self) -> Option<Box<DBInterface<ProjectCommand, ProjectResponse>>> {
        match self.root.interface(self.project) {
            Some(i) => match i.downcast() {
                Err(e) => {
                    error!(origin="database", db="root", "Could not downcast Project Database from `dyn Any`: {:?}", e);
                    None
                },

                Ok(i) => Some(i),
            },
            _ => None,
        }
    }
}



pub(self) async fn basefolder(name: &str) -> Option<PathBuf> {
    // Check for the base folder.
    match BASEFOLDER.lock(){
        Ok(option) => match option.as_ref() {
            Some(basefolder) => {
                // Create the new folder's path.
                let new = basefolder.join(name);

                if new.exists() {
                    return Some(new);
                }

                error!(origin="database", db="root", "{} database folder does not exist", basefolder.display());
                None
            },

            _ => {
                error!(origin="database", db="root", "Base resource folder not loaded");
                None
            },
        },

        Err(e) => {
            error!(origin="database", db="root", "Base folder mutex is poisoned: {}", e);
            None
        },
    }
}


/// Sends a status update.
async fn statusupdate(channel: &mut mpsc::Sender<Status>, status: Status) {
    match channel.send(status).await {
        Err(e) => error!(origin="database", db="theme", "Could not send status update: {}", e),
        _ => (),
    }
}
