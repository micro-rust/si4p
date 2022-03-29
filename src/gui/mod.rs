//! Graphical User Interface module.
//! Handles the rendering of the App state and user interaction.


//mod database;
mod msg;
mod views;
pub mod theme;
mod widgets;



use crate::database::*;

use database::common::DBInterface;

use iced::{
    Clipboard, Command, Subscription,
    Element, Column,

    Length,

    Text,
};

use self::msg::Message;

use tokio::sync::{ mpsc };

use tracing::{
    debug, error, info,
};


pub struct Application {
    /// Current view of the App.
    view: AppView,

    /// Current working project.
    current: Option<usize>,

    /// Root of the database.
    rootdb: crate::database::Database,

    /// The views of the application.
    views: (views::ProbeView, views::SettingsView, views::DatabaseView),

    /// The interfaces to the databases.
    interfaces: Option<(
        Box<DBInterface<ChipCommand,    ChipResponse   >>,
        Box<DBInterface<ThemeCommand,   ThemeResponse  >>,
        Box<DBInterface<RegexCommand,   RegexResponse  >>,
        Box<DBInterface<ProjectCommand, ProjectResponse>>,
    )>,

    /// Internal topbar state.
    topbar: widgets::Topbar,

    /// Indicates if the initialization finished.
    ready: bool,
}

impl Application {
    /// Runs the application.
    pub fn start(flags: ()) {
        use iced::{ Application, Settings, window::Settings as Window };

        let settings: Settings<()> = Settings {
            window: Window {
                size: (900, 620),
                resizable: true,
                decorations: true,
                icon: None,
                min_size: Some((640, 480)),
                max_size: None,
                always_on_top: false,
                transparent: false,
            },

            default_text_size: 17,
            exit_on_close_request: true,
            antialiasing: true,
            default_font: None,
            flags,
        };

        Self::run(settings).expect("Could not create main application")
    }

    /// Initialization method.
    pub async fn initialize() {
        // Launch the app folder initialization.
        crate::init::data::init().await;
    }
}

impl iced::Application for Application {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_: Self::Flags) -> (Self, Command<Message>) {
        // Create the root database.
        let rootdb = crate::database::Database::new();

        // Create the topbar.
        let mut topbar = widgets::Topbar::new();
        topbar.add(Message::ChangeToProbeView,    String::from("Probe")    );
        topbar.add(Message::ChangeToSettingsView, String::from("Settings") );
        topbar.add(Message::ChangeToDatabaseView, String::from("Database") );

        // Create the application data struct.
        let app = Application {
            view: AppView::Loading,
            current: None,

            rootdb,

            views: (
                views::ProbeView::new(),
                views::SettingsView::new(),
                views::DatabaseView::new(),
            ),

            interfaces: None,

            topbar,

            ready: false,
        };

        // Create the initialization command.
        let command = Command::perform( Self::initialize(), |_| Message::Initialized );

        (app, command)
    }

    fn title(&self) -> String {
        match self.view {
            AppView::Database => String::from("Si4+ - Database"),
            AppView::Loading => String::from("Si4+ - Loading..."),
            AppView::Probe => String::from("Si4+ - Probe"),
            AppView::Settings => String::from("Si4+ - Settings"),
        }
    }

    fn update(&mut self, message: Message, _: &mut Clipboard) -> Command<Message> {
        match message {
            // Initialization finished.
            Message::Initialized => {
                debug!(origin="app", "Base folder initialized");

                match crate::database::BASEFOLDER.lock() {
                    Ok(folder) => match folder.as_ref() {
                        Some(path) => debug!(origin="app", "Base folder = {}", path.display()),
                        None => error!(origin="app", "No base folder exists"),
                    },

                    Err(e) => {
                        error!(origin="app", "Base folder mutex poisoned: {}", e);
                    },
                }

                // Initialize root database.
                self.rootdb.init();

                // Initialize the common database interfaces.
                let chip = self.rootdb.chip().expect("Chip Database interfaces aquisition failed");
                let theme = self.rootdb.theme().expect("Theme Database interfaces aquisition failed");
                let regex = self.rootdb.regex().expect("Regex Database interfaces aquisition failed");
                let project = self.rootdb.project().expect("Project Database interfaces aquisition failed");

                // Get the project database interface and update the views that need it.
                let commands = self.views.2.update(
                    msg::DatabaseViewMessage::InterfacesCreated(
                        chip.clone(),
                        theme.clone(),
                        regex.clone(),
                        project.clone()
                    )
                );

                // Store a copy of the database interfaces for the future.
                self.interfaces = Some( ( chip, theme, regex, project, ) );

                // Change to ready state.
                self.ready = true;

                info!(origin="app", "Loading databases.");

                return commands;
            },

            // Switch to the probe view.
            Message::ChangeToProbeView => {
                self.view = AppView::Probe;

                debug!(origin="app", "Switch to probe view");
            },

            // Switch to the settings view.
            Message::ChangeToSettingsView => {
                self.view = AppView::Settings;

                debug!(origin="app", "Switch to settings view");
            },

            // Switch to the database view.
            Message::ChangeToDatabaseView => {
                self.view = AppView::Database;

                debug!(origin="app", "Switch to database view");
            },

            // Propagate the message to the settings view.
            Message::Settings(msg) => return self.views.1.update(msg),

            // Propagate the message to the probe view.
            Message::Probe(msg) => return self.views.0.update(msg),

            // Propagate the message to the database view.
            Message::Database(msg) => return self.views.2.update(msg),

            // Update the project database.
            Message::UpdateProjectDatabase => {
                // Get the commands to perform for the update.
                let commands = [
                    self.views.0.update( msg::ProbeMessage::UpdateProjectDatabase ),
                    self.views.2.update( msg::DatabaseViewMessage::Project( msg::database::ProjectViewMessage::UpdateDatabase ) ),
                ];

                debug!(origin="app", "Updating APP project database");

                return Command::batch(commands);
            },

            _ => (),
        }

        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        // Create the top bar.
        let topbar = self.topbar.view(self.ready);

        let content = match self.view {
            // Display the content of the probe view.
            AppView::Database => self.views.2.view(),

            // Display the content of the probe view.
            AppView::Probe => self.views.0.view(),

            // Display the content of the settings view.
            AppView::Settings => self.views.1.view(),

            // Loading screen.
            AppView::Loading => {
                Column::new()
                    .push(Text::new("Loading resources").size(40))
                    .into()
            },
        };

        Column::new()
            .height(Length::Fill)
            .width(Length::Fill)
            .push(topbar)
            .push(content)
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        //msg::global::subscription()
        Subscription::none()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppView {
    Database,
    Loading,
    Probe,
    Settings,
}
