//! Elf View messages.


#[derive(Debug, Clone, Copy)]
pub enum ElfMessage {
	// The button to view the file header info was clicked.
	ViewFileHeaderInfo,

	// The button to view the program header info was clicked.
	ViewAllProgramHeaderInfo,

	// The button to view the section header info was clicked.
	ViewAllSectionHeaderInfo,

	/// Displays the information of the given Program header.
	ViewProgramHeaderInfo(usize),

	/// Displays the information of the given Section header.
	ViewSectionHeaderInfo(usize),

	/// Show contents of the given section.
	ShowSectionContent(usize),
}