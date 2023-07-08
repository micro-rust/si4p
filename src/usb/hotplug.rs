//! USB hotplug component.
//! Checks for arriving and leaving devices and updates the list.



use seqid::impls::SeqHashMap;

use std::sync::Arc;

use super::device::USBDevice;

use tokio::sync::RwLock;



pub(super) struct Hotplug {
    /// Global list of currently connected devices.
    global: Arc<RwLock<SeqHashMap<usize, USBDevice>>>,
}

impl Hotplug {
    /// Creates a new hotplug component.
    pub(super) fn new(global: Arc<RwLock<SeqHashMap<usize, USBDevice>>>) -> Self {
        Self { global }
    }

    /// Updates the list of all devices currently connected.
    pub(super) fn update(&self) -> Result<(Vec<(u16, u16)>, Vec<(u16, u16)>), rusb::Error> {
        use rusb::DeviceList;

        // Get the list of devices.
        let list = match DeviceList::new() {
            Err(e) => panic!(),

            Ok(list) => list,
        };

        // Arriving devices.
        let arriving = self.arriving(&list);

        // Leaving devices.
        let leaving = self.leaving(&list);

        // The information of the arriving devices.
        let mut ainfo = Vec::new();

        // The information of the leaving devices.
        let mut linfo = Vec::new();

        // Insert the devices into the global list.
        let mut global = self.global.blocking_write();

        // Insert the new devices.
        for new in arriving.into_iter() {
            let ids = new.ids();

            match global.insert(new) {
                Some(_) => ainfo.push( ids ),

                //None => self.error(format!("Failed to insert device {:04X}:{:04X} in the map: Ran out of indexable space. Requires an application restart.", ids.0, ids.1)),
                _ => (),
            }
        }

        // Remove the old devices.
        for old in leaving.iter() {
            match global.remove(old) {
                Some(device) => linfo.push( device.ids() ),
                _ => continue,
            }
        }

        Ok( (ainfo, linfo) )
    }

    /// Checks for arriving devices.
    fn arriving<C: rusb::UsbContext>(&self, list: &rusb::DeviceList<C>) -> Vec<USBDevice> {
        // Get a read lock on the device list.
        // It's okay to block because this thread is the only one to write.
        let global = self.global.blocking_read();

        // Hotplugged devices.
        let mut arriving = Vec::new();

        // Check if all the devices are already listed.
        for device in list.iter() {
            // Get the descriptor.
            let descriptor = match device.device_descriptor() {
                Ok(d) => d,
                _ => continue,
            };

            // Get the VID and PID.
            let ids = (descriptor.vendor_id(), descriptor.product_id());

            // If the device is in the list, skip.
            // TODO: Match serial numbers to allow for multiple debuggers to be connected at the same time.
            if global.values().any(|device| device.ids() == ids) {
                continue;
            }

            // Attempt to build a device info for the device.
            match USBDevice::build(device) {
                Some(d) => arriving.push(d),
                _ => (),
            }
        }

        arriving
    }

    /// Checks for leaving devices.
    fn leaving<C: rusb::UsbContext>(&self, list: &rusb::DeviceList<C>) -> Vec<usize> {
        // Get a read lock on the device list.
        // It's okay to block because this thread is the only one to write.
        let global = self.global.blocking_read();

        // Get all the descriptors of the devices.
        let descriptors: Vec<rusb::DeviceDescriptor> = list.iter()
            .map(|device| device.device_descriptor())
            .filter(|maybe| maybe.is_ok())
            .map(|ok| ok.unwrap())
            .collect();

        // List of leaving devices' indices.
        let mut leaving = Vec::new();

        for (key, device) in global.iter() {
            // Check if the device is in the currently connected (and accessible) list.
            if descriptors.iter().any(|descriptor| device.ids() == (descriptor.vendor_id(), descriptor.product_id())) {
                continue;
            }

            // The device is not on the list, mark it to be removed.
            leaving.push( key.clone() );
        }

        leaving
    }
}
