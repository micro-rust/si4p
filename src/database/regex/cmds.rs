//! Regex Database commands.



use regex::Regex;



pub type DBCommand = database::common::DBCommand<Command, Response>;



#[derive(Debug, Clone)]
pub enum Command {
    /// Gets the `Regex` with the given name.
    GetRegex(String),
}

impl database::Command for Command {}



#[derive(Debug, Clone)]
pub enum Response {
    /// Returns the requested regex.
    Regex(Regex),
}

impl database::Response for Response {}
