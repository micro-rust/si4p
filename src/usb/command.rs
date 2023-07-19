//! Commands that the USB logger can process.



use super::{
    common::USBTarget,
    device::USBDevice,
};



#[derive(Clone, Debug)]
pub enum Command {
    /// Open the device with the given VID, PID and (optional) serial number.
    DefmtOpen( USBTarget ),

    /// Open the device with the given VID, PID and (optional) serial number.
    ProbeOpen( USBTarget ),

    /// Sets the defmt file.
    SetDefmtFile( std::sync::Arc<[u8]> ),

    /// A warning that the hotplug handler was dropped.
    HotplugDropped,

    /// Contains the information of a new connected USB device.
    NewConnection( USBDevice ),

    /// Signal to close the thread.
    Quit,
}
