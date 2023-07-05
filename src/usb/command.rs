//! Commands that the USB logger can process.



use std::path::PathBuf;

use super::device::USBDevice;



#[derive(Clone, Debug)]
pub enum Command {
    /// Open the device with the given VID, PID and (optional) serial number.
    Open( usize, u8, u8, u8, u8, (u8, u8) ),

    /// Sets the defmt file.
    SetDefmtFile( std::sync::Arc<[u8]> ),

    /// A warning that the hotplug handler was dropped.
    HotplugDropped,

    /// Contains the information of a new connected USB device.
    NewConnection( USBDevice ),

    /// Signal to close the thread.
    Quit,
}
