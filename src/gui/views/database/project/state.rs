//! State of the Project Database View.



use crate::{
    project::{ ProjectSerial, ProjectInfo, TargetInfo },
};

use iced::{
    button, text_input, scrollable,
};

use tracing::{ error, warn };



type TextInputPair = (text_input::State, String);



pub struct State {
    /// Internal `State` of the add button.
    pub(super) newbtn: button::State,

    /// Internal `State` of the search bar.
    pub(super) search: text_input::State,

    /// Internal `State` of the search scroll section.
    pub(super) searchscroll: scrollable::State,

    /// Current search input.
    pub(super) currentsearch: String,

    /// Internal `State` of the cancel project button.
    pub(super) cancel: button::State,

    /// Internal `State` of the create project button.
    pub(super) create: button::State,
}

impl State {
    /// Builds a new state.
    pub fn new() -> Self {
        State {
            newbtn: button::State::new(),

            search: text_input::State::new(),
            searchscroll: scrollable::State::new(),
            currentsearch: String::new(),

            cancel: button::State::new(),
            create: button::State::new(),
        }
    }
}


pub struct ProjectState {
    /// Internal `State` for the project name.
    pub(super) name: TextInputPair,

    /// Internal `State` for the project description.
    pub(super) description: TextInputPair,

    /// Internal `State` for the 'Add' (target) button.
    pub(super) addtarget: button::State,

    /// Internal `State` for the scrollable targets section.
    pub(super) scroll: scrollable::State,

    /// List of internal states of the targets.
    pub(super) targets: Vec<(TextInputPair, TextInputPair, TextInputPair, button::State)>,
}

impl ProjectState {
    pub fn new() -> Self {
        ProjectState {
            name: (text_input::State::new(), String::new()),
            description: (text_input::State::new(), String::new()),
            addtarget: button::State::new(),
            scroll: scrollable::State::new(),
            targets: Vec::new(),
        }
    }

    /// Pushes a new target.
    pub fn newtarget(&mut self) {
        self.targets.push(
            (
                (text_input::State::new(), String::new()),
                (text_input::State::new(), String::new()),
                (text_input::State::new(), String::new()),
                button::State::new(),
            )
        );
    }

    /// Remove a target.
    pub fn remove(&mut self, i: usize) {
        self.targets.remove(i);
    }

    /// Rebuilds the `ProjectSerial` from the `ProjectState`.
    pub fn rebuild(&self) -> Option<ProjectSerial> {
        // Assert that the name is present.
        if self.name.1.len() == 0 {
            error!(origin="app", view="database/projects", "Cannot build project without name");
            return None;
        }

        // Assert that there is at least one target.
        if self.targets.len() == 0 {
            error!(origin="app", view="database/projects", "Cannot build project without targets");
            return None;
        }

        // Build the name.
        let name = self.name.1.clone();

        // Build the description.
        let description = self.description.1.clone();

        // Build the targets.
        let targets = self.targets.iter()
            .fold(Vec::new(), |mut vec, (name, target, binary, _)| {
                // Assert the minimum information is present.
                if !((name.1.len() == 0) || (target.1.len() == 0) || (target.1.len() == 0)) {
                    // Build the target information.
                    let info = TargetInfo {
                        name: name.1.clone(),
                        target: target.1.clone(),
                        binary: binary.1.clone(),
                    };

                    // Append it to the array.
                    vec.push(info);
                } else {
                    warn!(origin="app", view="database/projects", "Skipped a target due to insufficient information");
                }

                vec
            });

        Some(ProjectSerial {
            info: ProjectInfo {
                name,
                description
            },

            targets,
        })
    }
}

impl core::convert::From<ProjectSerial> for ProjectState {
    fn from(project: ProjectSerial) -> ProjectState {

        ProjectState {
            name: (text_input::State::new(), project.info.name),
            description: (text_input::State::new(), project.info.description),
            addtarget: button::State::new(),
            scroll: scrollable::State::new(),
            targets: project.targets.iter().map(|target| {
                    let TargetInfo {
                        name,
                        target,
                        binary,
                    } = target;

                    (
                        (text_input::State::new(), name.clone()),
                        (text_input::State::new(), target.clone()),
                        (text_input::State::new(), binary.clone()),
                        button::State::new(),
                    )
                }).collect()
        }
    }
}

impl core::convert::From<&ProjectSerial> for ProjectState {
    fn from(project: &ProjectSerial) -> ProjectState {

        ProjectState {
            name: (text_input::State::new(), project.info.name.clone()),
            description: (text_input::State::new(), project.info.description.clone()),
            addtarget: button::State::new(),
            scroll: scrollable::State::new(),
            targets: project.targets.iter().map(|target| {
                    let TargetInfo {
                        name,
                        target,
                        binary,
                    } = target;

                    (
                        (text_input::State::new(), name.clone()),
                        (text_input::State::new(), target.clone()),
                        (text_input::State::new(), binary.clone()),
                        button::State::new(),
                    )
                }).collect()
        }
    }
}
