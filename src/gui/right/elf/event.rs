//! Internal events of the ELF selector.




#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Event {
    /// Flashes the current file.
    Flash,

    /// Reloads the current file.
    Reload,

    /// Opens a selection dialog.
    Select,
}
