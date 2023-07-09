//! USB element view show action.



#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ShowAction {
    /// Level of the collapse.
    pub(super) level: usize,

    /// Key of the device.
    pub(super) key: usize,

    /// Configuration index.
    pub(super) idx: u8,

    /// Interface number.
    pub(super) num: u8,

    /// Desired state of the collapsable region.s
    pub(super) state: bool,
}

impl ShowAction {
    /// Creates the `ShowAction` for a device view.
    pub const fn device(state: bool, key: usize) -> Self {
        ShowAction { level: 0, key, idx: 0, num: 0, state }
    }

    /// Creates the `ShowAction` for a configuration view.
    pub const fn config(state: bool, key: usize, idx: u8) -> Self {
        ShowAction { level: 1, key, idx, num: 0, state }
    }

    /// Creates the `ShowAction` for a configuration view.
    pub const fn interface(state: bool, key: usize, idx: u8, num: u8) -> Self {
        ShowAction { level: 2, key, idx, num, state }
    }
}
