//! Module for all GUI view modes.


pub mod database;
pub mod load;
pub mod probe;
pub mod settings;



pub use self::database::DatabaseView;
pub use self::load::LoadingView;
pub use self::probe::ProbeView;
pub use self::settings::SettingsView;
