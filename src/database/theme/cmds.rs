//! Theme Database commands.



use marcel::Theme;
use std::sync::Arc;
use tokio::sync::RwLock;



pub type DBCommand = database::common::DBCommand<Command, Response>;



#[derive(Debug, Clone)]
pub enum Command {
    /// Switches the currently active (global) theme.
    ChangeTheme(String),

    /// Gets a reference to the currently active (global) theme.
    GetTheme,
}

impl database::Command for Command {}



#[derive(Debug, Clone)]
pub enum Response {
    /// Returns the requested theme.
    Theme( Arc<RwLock<Theme>> ),
}

impl database::Response for Response {}
