//! Commands that the USB logger can process.



use probe_rs::DebugProbeInfo;

use super::{
    common::USBTarget,
    device::USBDevice,
};



#[derive(Clone, Debug)]
pub enum Command {

    // Commands to configure the connections and target.
    // ************************************************************************

    /// Open the device with the given VID, PID and (optional) serial number.
    DefmtOpen( USBTarget ),

    /// Closes the connection to the defmt device.
    DefmtClose,

    /// Open the device with the given VID, PID and (optional) serial number.
    ProbeOpen( DebugProbeInfo ),

    /// Closes the connection to the debug probe.
    ProbeClose,

    /// Sets the debug target.
    SetDebugTarget( String ),

    /// Resets the debug target.
    ClearDebugTarget,

    // ************************************************************************



    // File commands.
    // ************************************************************************

    /// Sets the defmt file.
    SetExecutableFile( std::sync::Arc<[u8]> ),

    /// Flashes the file.
    Flash,

    // ************************************************************************



    // Core control and manipulation.
    // ************************************************************************

    /// Halts the given core.
    CoreHalt( usize ),

    /// Resets the given core.
    CoreReset( usize ),

    /// Runs the given core.
    CoreRun( usize ),

    // ************************************************************************



    // Miscellaneous commands.
    // ************************************************************************

    /// Signal to close the thread.
    Quit,

    // ************************************************************************
}

impl Into<crate::gui::Message> for Command {
    fn into(self) -> crate::gui::Message {
        crate::gui::Message::USB( self )
    }
}
