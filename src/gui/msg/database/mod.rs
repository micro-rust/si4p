//! Collection of messages emitted by the database view.



use crate::{
    database::*,
    project::ProjectSerial,
};

use database::common::DBInterface;

use tokio::{
    sync::RwLock,
};

use std::{
    collections::HashMap,
    sync::Arc,
};




#[derive(Debug, Clone)]
pub enum DatabaseViewMessage {
    /// Change to view the `Chip` database.
    ChangeToChip,

    /// Change to view the `Regex` database.
    ChangeToRegex,

    /// Change to view the `Theme` database.
    ChangeToTheme,

    /// Change to view the `Project` database.
    ChangeToProject,

    /// The interfaces to the databases were created.
    InterfacesCreated(
        Box<DBInterface<ChipCommand,    ChipResponse   >>,
        Box<DBInterface<ThemeCommand,   ThemeResponse  >>,
        Box<DBInterface<RegexCommand,   RegexResponse  >>,
        Box<DBInterface<ProjectCommand, ProjectResponse>>,
    ),

    /// A message related to the `Project` database.
    Project( ProjectViewMessage )
}



#[derive(Debug, Clone)]
pub enum ProjectViewMessage {
    /// Add a new target to the currently editing project.
    AddTarget,

    /// Cancelled the creation of a new entry.
    Cancel,

    /// Changes the name of the currently editing project.
    ChangeName( String ),

    /// Creation of a new entry.
    Create,

    /// The database did not acknowledge or failed the creation of a new entry.
    CreationFailed,

    /// Contains the `Arc` to the database data.
    DatabaseReference(Arc<RwLock<Vec<ProjectSerial>>>, Arc<RwLock<HashMap<String, Vec<usize>>>>),

    /// A project database update failed.
    DatabaseUpdateFailed,

    /// The 'Delete' button was pressed for the given project.
    DeleteProject(String),

    /// A deletion attempt failed.
    DeletionFailed,

    /// The description of the currently editing project was updated.
    Description(String),

    /// The 'Edit' button was pressed for the given project.
    EditProject(usize),

    /// Creates a new entry.
    NewEntry,

    /// Removes a target from the project.
    RemoveTarget(usize),

    /// Initiated a search for a new item.
    Search(String),

    /// The name of one of the currently editing project's target was updated.
    TargetName(usize, String),

    /// The chip of one of the currently editing project's target was updated.
    TargetChip(usize, String),

    /// The chip of one of the currently editing project's binary was updated.
    TargetBinary(usize, String),

    /// Update of an existing entry.
    Update,

    /// A message to indicate an update to the project database.
    UpdateDatabase,
}
