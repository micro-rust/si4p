//! GUI Database view.
//! This GUI view handles the interface to the databases.



mod project;



use crate::{
    database::{
        project::{ ProjectCommand, ProjectResponse },
    },
    gui::{
        theme::Theme,
        msg::{ Message, DatabaseViewMessage },
        widgets::Topbar,
    },
};

use database::{
    common::{
        DBInterface,
    },
};

use iced::{
    Command, Column, Element, Length
};

use tracing::{ debug };


use self::{
    project::ProjectDatabaseView
};



pub struct DatabaseView {
    /// Topbar to select database.
    topbar: Topbar,

    /// Current selected view.
    current: Option<View>,

    /// The database view.
    project: ProjectDatabaseView,

    // Default themes for the buttons.
    //btndefault: ButtonTheme,
}

impl DatabaseView {
    // Builds the Database view. 
    pub fn new() -> Self {
        // Create the topbar.
        let mut topbar = Topbar::new();

        topbar.add(Message::Database( DatabaseViewMessage::ChangeToChip    ), String::from( "Chip" )   );
        topbar.add(Message::Database( DatabaseViewMessage::ChangeToRegex   ), String::from( "Regex" )  );
        topbar.add(Message::Database( DatabaseViewMessage::ChangeToTheme   ), String::from( "Theme" )  );
        topbar.add(Message::Database( DatabaseViewMessage::ChangeToProject ), String::from( "Project") );

        DatabaseView {
            topbar,
            current: None,
            project: ProjectDatabaseView::new(),
        }
    }

    /// Updates the view.
    pub fn update(&mut self, msg: DatabaseViewMessage) -> Command<Message> {
        match msg {
            DatabaseViewMessage::ChangeToChip => {
                self.current = Some( View::Chip );
                debug!(origin="app", view="database", "Changed database view to Chip Database");
            },

            DatabaseViewMessage::ChangeToRegex => {
                self.current = Some( View::Regex );
                debug!(origin="app", view="database", "Changed database view to Regex Database");
            },

            DatabaseViewMessage::ChangeToTheme => {
                self.current = Some( View::Theme );
                debug!(origin="app", view="database", "Changed database view to Theme Database");
            },

            DatabaseViewMessage::ChangeToProject => {
                self.current = Some( View::Project );
                debug!(origin="app", view="database", "Changed database view to Project Database");
            },

            // Sets the interfaces for the sub views.
            DatabaseViewMessage::InterfacesCreated(_, _, _, project) => {
                let commands = [
                    self.project.interface(*project),
                ];

                return Command::batch( commands );
            },

            DatabaseViewMessage::Project(m) => return self.project.update(m),
        }

        Command::none()
    }

    /// Builds the GUI view.
    pub fn view(&mut self) -> Element<Message> {
        // Build topbar.
        let topbar = self.topbar.view(true);

        // Build the selected view.
        let content = match self.current {
            Some(View::Project) => self.project.view(),
            _ => Column::new().into(),
        };

        Column::new()
            .height(Length::Fill)
            .width(Length::Fill)
            .push(topbar)
            .push(content)
            .into()
    }
}


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum View {
    Chip,
    Regex,
    Theme,
    Project,
}
