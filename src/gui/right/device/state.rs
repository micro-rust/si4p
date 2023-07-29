//! Internal state of the USB selector component.



use super::View;



pub struct State {
    /// The currently selected view.
    pub(super) selected: View,
}

impl Default for State {
    fn default() -> Self {
        Self {
            selected: View::Probe,
        }
    }
}
