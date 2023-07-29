//! Internal events of the ELF selector.




#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Event {
    /// Opens a selection dialog.
    Select,

    /// Reloads the current file.
    Reload,
}
