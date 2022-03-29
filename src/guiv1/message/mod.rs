//! Module containing all messages of the App.


mod elf;
mod probe;


use crate::gui::AppState;

pub use self::elf::ElfMessage;
pub use self::probe::ProbeMessage;


#[derive(Debug, Clone, Copy)]
pub enum AppMessage {
	/// The app has been initialized.
	Initialized(AppState),

	/// The project with the given project ID has been selected.
	ProjectSelected(String),

	/// The user requested the creation of a new project.
	NewProjectRequest,

	/// The user requested to load the probe with code.
	LoadProbe,

	/// ELF VIew messages.
	ElfView(ElfMessage),

	Probe(ProbeMessage),
}