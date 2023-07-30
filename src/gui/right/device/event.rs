//! Internal events of the device selector.



use probe_rs::DebugProbeInfo;

use super::View;



#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Event {
    /// Opens the given probe.
    OpenProbe( DebugProbeInfo ),

    /// Closes the current probe.
    CloseProbe,

    /// Sets the selected view.
    SetView( View ),
}
