//! Project database commands.

use crate::{
    project::ProjectSerial,
};

use std::{
    collections::HashMap,
    sync::Arc,
};

use tokio::{
    sync::{
        mpsc,
        RwLock,
    },
};


pub type DBCommand = database::common::DBCommand<Command, Response>;



#[derive(Debug, Clone)]
pub enum Command {
    /// Adds a new project to the database.
    CreateProject( ProjectSerial ),

    /// Deletes a new project from the database.
    DeleteProject( String ),

    /// Requests the current search engine.
    GetSearchEngine,
}

impl database::Command for Command {}



#[derive(Debug, Clone)]
pub enum Response {
    /// Returns the requested project.
    Project,

    /// Returns the current search engine state.
    SearchEngine(Arc<RwLock<Vec<ProjectSerial>>>, Arc<RwLock<HashMap<String, Vec<usize>>>>),

    /// The project does not exist.
    DoesNotExist,

    /// The request was done.
    Done,
}

impl database::Response for Response {}
