//! Internal events of the device selector.



use super::View;



#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Event {
    /// Sets the selected view.
    SetView( View ),
}
