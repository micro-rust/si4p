//! Chip Database commands.



use architecture::Chip;



pub type DBCommand = database::common::DBCommand<Command, Response>;



#[derive(Debug, Clone)]
pub enum Command {
	/// Requests the `Chip` with the given name.
	GetChip(String),
}

impl database::Command for Command {}



#[derive(Debug, Clone)]
pub enum Response {
	/// Returns a `Chip`.
	Chip(Chip),
}

impl database::Response for Response {}
