//! Commands that the USB logger can process.



use super::device::USBDevice;



#[derive(Clone, Debug)]
pub enum Command {
    /// Open the device with the given VID, PID and (optional) serial number.
    Open( u16, u16, Option<String> ),

    /// A warning that the hotplug handler was dropped.
    HotplugDropped,

    /// Contains the information of a new connected USB device.
    NewConnection( USBDevice ),

    /// Signal to close the thread.
    Quit,
}
