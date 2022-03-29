//! GUI Message module.
//! The messages sent by all the different events in the GUI.



pub mod database;
pub mod global;
pub mod settings;

pub use self::database::DatabaseViewMessage;
pub use self::global::MessageCenter;
pub use self::settings::SettingsMessage;


#[derive(Debug, Clone)]
pub enum Message {
    /// The topbar Database Button was pressed.
    ChangeToDatabaseView,

    /// The topbar Probe Button was pressed.
    ChangeToProbeView,

    /// The topbar Settings Button was pressed.
    ChangeToSettingsView,

    /// The message dispatcher has been created.
    DispatcherCreated(tokio::sync::mpsc::Sender<Message>),

    /// The databases are available.
    DatabasesAvailable,

    /// Indicates that initialization has ended.
    Initialized,

    /// A project has been selected.
    ProjectSelected(String),

    /// A message of the probe view.
    Probe(ProbeMessage),

    /// A message for the settings view.
    Settings(SettingsMessage),

    /// A message for the database view.
    Database(DatabaseViewMessage),

    /// The project database was updated.
    UpdateProjectDatabase,

    /// The theme database was updated.
    UpdateThemeDatabase,

    None,
}

#[derive(Debug, Clone)]
pub enum ProbeMessage {
    Datatype(crate::gui::views::probe::common::Datatype),

    Load,
    Stop,
    Reset,
    Run,
    Step,
    Dump,

    Read,
    ReadRange,
    ReadSymbol,

    ReadAddressChanged(String),
    NewReadAddress,

    ReadRangeSAddressChanged(String),
    NewReadRangeSAddress,

    ReadRangeEAddressChanged(String),
    NewReadRangeEAddress,

    /// An indication that the themes can be loaded now.
    ThemesAvailable(usize),

    /// A message to update the project database.
    UpdateProjectDatabase,
}
