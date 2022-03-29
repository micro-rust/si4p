//! Project Database GUI view.


mod state;
mod theme;


use crate::{
    database::{
        project::{ ProjectCommand, ProjectResponse },
    },
    gui::{
        msg::{
            Message, DatabaseViewMessage,
            database::ProjectViewMessage,
        },
    },
    project::ProjectSerial,
};

use database::{
    common::{
        DBCommand, DBInterface,
    },
};

use iced::{
    Command, Column, Container, Element, Row,

    Align, Length,

    Text, PickList, TextInput, Scrollable,

    button::{ self, Button },
    tooltip::{ Position, Tooltip },
};

use state::{ ProjectState, State };

use std::{
    collections::HashMap,
    mem::MaybeUninit,
    sync::Arc,
};

use tokio::{
    sync::RwLock,
};

use tracing::{
    debug, error, info, warn,

    instrument::WithSubscriber,
};



pub struct ProjectDatabaseView {
    /// Internal state of the components.
    state: State,

    /// Collection of projects.
    projects: Arc<RwLock<Vec<ProjectSerial>>>,

    /// Suffix search of the projects.
    suffix: Arc<RwLock<HashMap<String, Vec<usize>>>>,

    /// Indices of the projects that match the search input.
    matched: Vec<usize>,

    /// Internal state of new or editing projects.
    project: ProjectState,

    /// Current action.
    action: Action,

    /// Current list of projects to be displayed and their states.
    searchlist: Vec<(usize, String, String, button::State, button::State)>,

    /// Indicates wether there is an active search.
    searching: bool,

    /// Interface to the `project` database.
    interface: Option<DBInterface<ProjectCommand, ProjectResponse>>,

    /// Internal theme for this view.
    theme: theme::Theme,
}

impl ProjectDatabaseView {
    /// Sets the database interface.
    pub(super) fn interface(&mut self, interface: DBInterface<ProjectCommand, ProjectResponse>) -> Command<Message> {
        self.interface = Some(interface.clone());

        info!(origin="app", view="database/project", "Database interface aquired");

        // Create the async command.
        return Command::perform(
            getdb(interface).with_current_subscriber(),
            |m| { Message::Database( DatabaseViewMessage::Project( m ) ) }
        );
    }
}

impl ProjectDatabaseView {
    /// Creates the view.
    pub fn new() -> Self {

        ProjectDatabaseView {
            state: State::new(),
            projects: Arc::new( RwLock::new( Vec::new() ) ),
            suffix: Arc::new( RwLock::new( HashMap::new() ) ),
            matched: Vec::new(),
            project: ProjectState::new(),
            action: Action::Searching,
            searchlist: Vec::new(),
            searching: false,
            interface: None,

            theme: Default::default(),
        }
    }

    /// Updates the GUI.
    pub fn update(&mut self, msg: ProjectViewMessage) -> Command<Message> {
        match msg {
            ProjectViewMessage::Search(s) => {
                // Search for the indices with this suffix.
                match s.len() {
                    0 | 1 => {
                        self.matched = Vec::new();
                        self.searching = false;
                    },
                    _ => {
                        // Get the read permission.
                        let suffix = self.suffix.blocking_read();

                        self.matched = match suffix.get(&s) {
                            Some(v) => v.clone(),
                            _ => Vec::new(),
                        };

                        self.searching = true;
                    },
                }

                // Update the input state.
                self.state.currentsearch = s;

                // Change the action.
                self.action = Action::Searching;
            },

            ProjectViewMessage::NewEntry => {
                // Update the input state.
                self.state.currentsearch = String::new();

                // Change the action.
                self.action = Action::New;

                // Build a new `ProjectState`.
                self.project = ProjectState::from(ProjectSerial::new());

                debug!(origin="app", view="database/project", "Entering new project creation view");
            },

            ProjectViewMessage::Cancel => {
                match self.action {
                    Action::New => debug!(origin="app", view="database/project", "New project entry creation cancelled"),
                    _ => debug!(origin="app", view="database/project", "Project entry update cancelled"),
                }
                // Change the action.
                self.action = Action::Searching;
            },

            ProjectViewMessage::Create => {
                match self.project.rebuild() {
                    None => error!(origin="app", view="database/project", "Cannot build project"),
                    Some(p) => {
                        // Clone the interface.
                        let interface = match &self.interface {
                            Some(i) => i.clone(),
                            _ => {
                                error!(origin="app", view="database/project", "Cannot create project without database interface");
                                return Command::none();
                            },
                        };

                        // Switch to editing.
                        self.action = Action::Editing;

                        debug!(origin="app", view="database/project", "Creating new entry in project database");

                        // Create the async command.
                        return Command::perform(
                            createproject(interface, p).with_current_subscriber(),
                            |m| { m }
                        );
                    },
                }
            },

            ProjectViewMessage::AddTarget => {
                self.project.newtarget();

                debug!(origin="app", view="database/project", "Adding a new target to the new project");
            },

            ProjectViewMessage::RemoveTarget(i) => {
                self.project.remove(i);

                debug!(origin="app", view="database/project", "Removing target {} from the new project", i);
            },

            ProjectViewMessage::ChangeName(s) => {
                (self.project.name).1 = s;
            },

            ProjectViewMessage::Description(s) => {
                (self.project.description).1 = s;
            },

            ProjectViewMessage::TargetName(i, s) => {
                (self.project.targets[i].0).1 = s;
            },

            ProjectViewMessage::TargetChip(i, s) => {
                (self.project.targets[i].1).1 = s;
            },

            ProjectViewMessage::TargetBinary(i, s) => {
                (self.project.targets[i].2).1 = s;
            },

            ProjectViewMessage::UpdateDatabase => {
                // Get read permission.
                let projects = self.projects.blocking_read();

                // Update the search engine.
                self.searchlist = projects.iter()
                    .enumerate()
                    .map(|(i, p)| {
                        (i, p.name().clone(), p.description().clone(), button::State::new(), button::State::new())
                    })
                    .collect();

                {
                    let names: Vec<_> = self.searchlist.iter().map(|(_, n, _, _, _)| n.clone()).collect();
                    debug!(origin="app", view="database/project", "Updated element names: {:?}", &names);
                }
                
                debug!(origin="app", view="database/project", "GUI Element of project database has updated the database: Number of projects = {}", projects.len());
            },

            ProjectViewMessage::DatabaseUpdateFailed => error!(origin="app", view="database/project", "Database update failed"),

            ProjectViewMessage::CreationFailed => {
                if self.action == Action::Editing {
                    self.action = Action::New;
                }

                error!(origin="app", view="database/project", "Database entry creation failed");
            },

            ProjectViewMessage::DeletionFailed => error!(origin="app", view="database/project", "Database entry deletion failed"),

            ProjectViewMessage::EditProject(idx) => {
                // Get read permission.
                let projects = self.projects.blocking_read();

                self.project = ProjectState::from( &projects[idx] );
                self.action = Action::Editing;

                debug!(origin="app", view="database/project", "Editing project {}", idx);
            },

            ProjectViewMessage::Update => {
                error!(origin="app", view="database/project", "Update not yet implemented");
            },

            ProjectViewMessage::DeleteProject(name) => {
                // Clone the interface.
                let interface = match &self.interface {
                    Some(i) => i.clone(),
                    _ => {
                        error!(origin="app", view="database/project", "Cannot delete project without database interface");
                        return Command::none();
                    },
                };

                return Command::perform(
                    deleteproject(interface, name).with_current_subscriber(),
                    |m| { m }
                );
            },

            ProjectViewMessage::DatabaseReference(projects, suffix) => {
                self.projects = projects;
                self.suffix = suffix;

                info!(origin="app", view="database/project", "Received reference to project database:\n  self.projects = {:?}", self.projects);
            },
        }

        Command::none()
    }

    /// Builds the GUI view.
    pub fn view(&mut self) -> Element<Message> {
        match self.action {
            Action::New | Action::Editing => {

                let header = {
                    // Build the text of the header.
                    let text = Text::new("Target:")
                        .size(20)
                        .color(self.theme.projectheader.textcolor);

                    // Build the input.
                    let input = TextInput::new(
                            &mut self.project.name.0,
                            "Name of the project...",
                            &self.project.name.1,
                            |s| { Message::Database( DatabaseViewMessage::Project( ProjectViewMessage::ChangeName( s ) ) ) },
                        )
                    .max_width(400)
                    .width(Length::Fill)
                    .padding(5)
                    .size(14)
                    .style(self.theme.projectheader.textinput);

                    let cancel = Button::new(&mut self.state.cancel, Text::new("Cancel").size(20))
                        .on_press( Message::Database( DatabaseViewMessage::Project( ProjectViewMessage::Cancel ) ) )
                        .style(self.theme.projectheader.button);

                    let otherbtn = match self.action {
                        Action::New => Button::new(&mut self.state.create, Text::new("Create").size(20))
                            .on_press( Message::Database( DatabaseViewMessage::Project( ProjectViewMessage::Create ) ) )
                            .style(self.theme.projectheader.button),

                        Action::Editing => Button::new(&mut self.state.create, Text::new("Update").size(20))
                            .on_press( Message::Database( DatabaseViewMessage::Project( ProjectViewMessage::Update ) ) )
                            .style(self.theme.projectheader.button),

                        _ => unreachable!(),
                    };

                    let left = Row::new()
                        .spacing(5)
                        .height(Length::Shrink)
                        .width(Length::Fill)
                        .push(text)
                        .push(input);

                    let right = Row::new()
                        .height(Length::Shrink)
                        .width(Length::Shrink)
                        .push(otherbtn)
                        .push(cancel);

                    let row = Row::new()
                        .push(left)
                        .push(right);

                    Container::new( row )
                        .style(self.theme.background.clone())
                };

                let description = {
                    // Build the text of the header.
                    let text = Text::new("Description:")
                        .size(20)
                        .color(self.theme.projectheader.textcolor);

                    // Build the input.
                    let input = TextInput::new(
                            &mut self.project.description.0,
                            "Short description of the project...",
                            &self.project.description.1,
                            |s| { Message::Database( DatabaseViewMessage::Project( ProjectViewMessage::Description(s) ) ) },
                        )
                    .padding(5)
                    .size(14)
                    .style(self.theme.projectheader.textinput);

                    let row = Row::new()
                        .spacing(5)
                        .height(Length::Shrink)
                        .width(Length::Fill)
                        .push(text)
                        .push(input);

                    Container::new( row )
                        .style(self.theme.background.clone())
                };


                let targetheader = {
                    // Build the text of the header.
                    let text = Text::new("Targets")
                        .size(20)
                        .color(self.theme.projectheader.textcolor);

                    // Build the button.
                    let button = Button::new(&mut self.project.addtarget, Text::new("Add").size(14))
                        .on_press( Message::Database( DatabaseViewMessage::Project( ProjectViewMessage::AddTarget ) ) )
                        .style(self.theme.projectheader.button);

                    Row::new()
                        .spacing(5)
                        .push(text)
                        .push(button)
                };

                // Build the scrollable section for the targets.
                let scrollable = Scrollable::new(&mut self.project.scroll)
                    .spacing(5)
                    .padding(5)
                    .height(Length::Fill)
                    .width(Length::Fill)
                    .scrollbar_margin(2)
                    .scrollbar_width(5)
                    .scroller_width(10);

                let bgstyle = self.theme.targetinfo.background.clone();
                let inputstyle = self.theme.targetinfo.textinput.clone();
                let buttonstyle = self.theme.targetinfo.button.clone();


                let targets = self.project.targets.iter_mut()
                    .enumerate()
                    .fold(scrollable, |col, (i, (name, target, binary, button))| {
                        let nameinput = TextInput::new(
                                &mut name.0,
                                "Target name...",
                                &name.1,
                                move |s| { Message::Database( DatabaseViewMessage::Project( ProjectViewMessage::TargetName(i, s) ) ) }
                            )
                            .padding(5)
                            .size(16)
                            .style(inputstyle.clone());

                        let chipinput = TextInput::new(
                                &mut target.0,
                                "Target chip...",
                                &target.1,
                                move |s| { Message::Database( DatabaseViewMessage::Project( ProjectViewMessage::TargetChip(i, s) ) ) }
                            )
                            .padding(5)
                            .size(16)
                            .style(inputstyle.clone());

                        let binaryinput = TextInput::new(
                                &mut binary.0,
                                "Target binary...",
                                &binary.1,
                                move |s| { Message::Database( DatabaseViewMessage::Project( ProjectViewMessage::TargetBinary(i, s) ) ) }
                            )
                            .padding(5)
                            .size(16)
                            .style(inputstyle.clone());

                        let remove = Button::new(button, Text::new("Remove").size(18))
                            .on_press( Message::Database( DatabaseViewMessage::Project( ProjectViewMessage::RemoveTarget(i) ) ) )
                            .style(buttonstyle);

                        let inner = Column::new()
                            .spacing(2)
                            .height(Length::Shrink)
                            .width(Length::Fill)
                            .padding(10)
                            .push(nameinput)
                            .push(chipinput)
                            .push(binaryinput)
                            .push(remove);

                        col.push( Container::new(inner).style(bgstyle.clone()) )
                    });

                let column = Column::new()
                    .push(header)
                    .push(description)
                    .push(targetheader)
                    .push(targets)
                    .height(Length::Fill)
                    .width(Length::Fill)
                    .padding(10);

                Container::new( column )
                    .style(self.theme.background.clone())
                    .into()
            },

            Action::Searching => {
                // Build the search bar.
                let searchbar = {
                    // Build the text input.
                    let input = TextInput::new(
                            &mut self.state.search,
                            "Project name...",
                            &self.state.currentsearch,
                            |s| { Message::Database( DatabaseViewMessage::Project( ProjectViewMessage::Search(s) ) ) }
                        )
                        .padding(5)
                        .size(14)
                        .width(Length::Fill);

                    // Build the add button.
                    let button = Button::new(&mut self.state.newbtn, Text::new("New").size(14))
                        .on_press( Message::Database( DatabaseViewMessage::Project( ProjectViewMessage::NewEntry ) ) )
                        .height(Length::Shrink)
                        .width(Length::Fill);

                    Row::new()
                        .push(input)
                        .push(button)
                        .height(Length::Shrink)
                        .width(Length::Fill)
                };

                // Build the currently matched projects.
                let projects = {
                    // Build the scrollable section for the targets.
                    let scrollable = Scrollable::new(&mut self.state.searchscroll)
                        .spacing(0)
                        .padding(5)
                        .height(Length::Fill)
                        .width(Length::Fill)
                        .scrollbar_margin(2)
                        .scrollbar_width(5)
                        .scroller_width(10);

                    let matches = self.matched.clone();
                    let searching = self.searching.clone();

                    self.searchlist.iter_mut()
                        .filter(|(idx, _, _, _, _)| {
                            match matches.len() {
                                0 => !searching,
                                _ => matches.contains(idx)
                            }
                        })
                        .fold(scrollable, |col, (idx, name, desc, edit, del)| {
                            // Build the edit button.
                            let editbtn = Button::new(edit, Text::new("Edit"))
                                .on_press( Message::Database( DatabaseViewMessage::Project( ProjectViewMessage::EditProject(*idx) ) ) );

                            // Build the delete button.
                            let delbtn = Button::new(del, Text::new("Delete"))
                                .on_press( Message::Database( DatabaseViewMessage::Project( ProjectViewMessage::DeleteProject( name.clone() ) ) ) );

                            // Collect the buttons into the container.
                            let btnrow = Row::new()
                                .height(Length::Shrink)
                                .width(Length::Shrink)
                                .align_items(Align::End)
                                .spacing(5)
                                .push(editbtn)
                                .push(delbtn);

                            let buttons = Container::new(btnrow)
                                .height(Length::Shrink)
                                .width(Length::Fill)
                                .align_x(Align::End);

                            // Build the text.
                            let text = {
                                let name = Text::new( name.clone() )
                                    .size(24);

                                let desc = Text::new( desc.clone() )
                                    .size(16);

                                Column::new()
                                    .height(Length::Shrink)
                                    .width(Length::Fill)
                                    .push(name)
                                    .push(desc)
                            };

                            let inner = Row::new()
                                .height(Length::Shrink)
                                .width(Length::Fill)
                                .padding(10)
                                .push(text)
                                .push(buttons);

                            col.push(inner)
                        })
                };

                Column::new()
                    .push(searchbar)
                    .push(projects)
                    .height(Length::Fill)
                    .width(Length::Fill)
                    .padding(10)
                    .into()
            }
        }
    }
}



#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
    /// Editing a new entry.
    Editing,

    /// Creating a new entry.
    New,

    /// Searching for an entry.
    Searching,
}



/// Async function to create a new `ProjectSerial`.
async fn createproject(mut interface: DBInterface<ProjectCommand, ProjectResponse>, project: ProjectSerial) -> Message {
    // Create a command response pair.
    let (cmd, res) = DBCommand::create( ProjectCommand::CreateProject(project) );

    match interface.send(cmd).await {
        Err(e) => {
            error!(origin="app", view="database/project", "Could not send a 'CreateProject' command: {}", e);
            return Message::Database( DatabaseViewMessage::Project( ProjectViewMessage::CreationFailed ) );
        },
        _ => (),
    }

    match res.response().await {
        Some(r) => match r {
            ProjectResponse::Done => Message::UpdateProjectDatabase,
            _ => {
                error!(origin="app", view="database/project", "Unknown response to 'CreateProject' command");
                Message::Database( DatabaseViewMessage::Project( ProjectViewMessage::CreationFailed ) )
            },
        },
        _ => {
            error!(origin="app", view="database/project", "Channel closed before a response to ''CreateProject' command was received");
            Message::Database( DatabaseViewMessage::Project( ProjectViewMessage::CreationFailed ) )
        },
    }
}

/// Async function to delete a `ProjectSerial`.
async fn deleteproject(mut interface: DBInterface<ProjectCommand, ProjectResponse>, project: String) -> Message {
    // Create a command response pair.
    let (cmd, res) = DBCommand::create( ProjectCommand::DeleteProject(project.clone()) );

    match interface.send(cmd).await {
        Err(e) => {
            error!(origin="app", view="database/project", "Could not send a 'DeleteProject' command: {}", e);
            return Message::Database( DatabaseViewMessage::Project( ProjectViewMessage::DeletionFailed ) );
        },
        _ => (),
    }

    match res.response().await {
        Some(r) => match r {
            ProjectResponse::Done => Message::UpdateProjectDatabase,

            ProjectResponse::DoesNotExist => Message::Database( DatabaseViewMessage::Project( ProjectViewMessage::UpdateDatabase ) ),

            _ => {
                error!(origin="app", view="database/project", "Unknown response to 'DeleteProject' command");
                Message::Database( DatabaseViewMessage::Project( ProjectViewMessage::DeletionFailed ) )
            },
        },
        _ => {
            error!(origin="app", view="database/project", "Channel closed before a response to ''DeleteProject' command was received");
            Message::Database( DatabaseViewMessage::Project( ProjectViewMessage::DeletionFailed ) )
        },
    }
}

/// Async function to update the project database.
async fn getdb(mut interface: DBInterface<ProjectCommand, ProjectResponse>) -> ProjectViewMessage {
    // Create a command response pair.
    let (cmd, res) = DBCommand::create( ProjectCommand::GetSearchEngine );

    match interface.send(cmd).await {
        Err(e) => {
            error!(origin="app", view="database/project", "Could not send a 'GetSearchEngine' command: {}", e);
            return ProjectViewMessage::DatabaseUpdateFailed;
        },
        _ => (),
    }

    match res.response().await {
        Some(r) => match r {
            ProjectResponse::SearchEngine(projects, suffix) => ProjectViewMessage::DatabaseReference(projects, suffix),
            _ => {
                error!(origin="app", view="database/project", "Unknown response to 'GetSearchEngine' command");
                ProjectViewMessage::DatabaseUpdateFailed
            },
        },
        _ => {
            error!(origin="app", view="database/project", "Channel closed before a response to ''GetSearchEngine' command was received");
            ProjectViewMessage::DatabaseUpdateFailed
        },
    }
}
