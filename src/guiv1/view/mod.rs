//! All possible views.


//mod debug;
mod elf;
mod project;


use std::fs::File;
use std::io::{ BufReader, Read };
use std::path::Path;


pub use self::elf::ELFView;


pub struct AppViews {
	/// ELF Viewer.
	pub(super) elf: ELFView,
}

impl AppViews {
	/// Creates the App views.
	pub fn create() -> AppViews {
		Self {
			elf: ELFView::create(),
		}
	}

	/// Loads and ELF file from the given path.
	pub fn loadelf(&mut self, path: &Path) {
		let mut data = Vec::new();

		// Attempt to open file.
		let elf = match File::open(path) {
			Ok(f) => match BufReader::new(f).read_to_end(&mut data) {
				Ok(_) => micro_elf::parse(&data),
				_ => panic!("Could not read ELF file"),
			},

			_ => panic!("Could not open ELF file"),
		};

		// Load the ELF viewer data.
		self.elf.load(&elf);
	}
}