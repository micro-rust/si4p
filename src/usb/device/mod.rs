//! USB device descriptors.



mod config;
mod endpoint;
mod interface;



pub use config::USBConfig;
pub use endpoint::USBEndpoint;
pub use interface::USBInterface;

use rusb::{
    Device, UsbContext,
};



/// Common timeout for USB operations.
pub(self) const TIMEOUT: std::time::Duration = std::time::Duration::from_millis(250);


#[derive(Clone, Debug)]
pub struct USBDevice {
    /// Manufacturer string.
    manufacturer: String,

    /// Name string.
    name: String,

    /// The bus to which this device is connected.
    bus: (u8, u8),

    /// The vendor and product IDs of the device.
    ids: (u16, u16),

    /// Serial number string.
    serial: String,

    /// List of all the configurations of the device.
    configs: Vec<USBConfig>,

    #[cfg(feature = "application")]
    /// GUI flag that indicates if the display information is expanded.
    pub expanded: bool,

    #[cfg(feature = "application")]
    /// GUI flag that indicates if the config list is expanded.
    pub showlist: bool,
}

impl USBDevice {
    /// Returns a reference to the manufacturer string.
    pub fn manufacturer(&self) -> &String {
        &self.manufacturer
    }

    /// Returns a reference to the name string.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Returns a reference to the serial string.
    pub fn serial(&self) -> &String {
        &self.serial
    }

    /// Returns the bus to which this device is connected.
    pub fn bus(&self) -> (u8, u8) {
        self.bus
    }

    /// Returns the vendor and product IDs.
    pub fn ids(&self) -> (u16, u16) {
        self.ids
    }

    /// Returns the number of configurations.
    pub fn nconfigs(&self) -> usize {
        self.configs.len()
    }

    /// Returns an iterator over all the configurations of the device.
    pub fn configs<'a>(&'a self) -> impl Iterator<Item = &'a USBConfig> {
        self.configs.iter()
    }

    /// Returns an iterator over all the configurations of the device.
    pub fn configs_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut USBConfig> {
        self.configs.iter_mut()
    }

    /// Builds the device descriptor.
    pub fn build<'a, C: UsbContext>(device: Device<C>) -> Option<Self> {
        // Try to read the languages.
        let(handle, language) = match device.open() {
            Ok(handle) => match handle.read_languages(TIMEOUT) {
                Ok(languages) if languages.len() > 0 => (handle, languages[0]),
                _ => return None,
            },
            _ => return None,
        };

        // Get the device descriptor.
        let descriptor = match device.device_descriptor() {
            Err(_) => return None,
            Ok(d) => d,
        };

        // Get the IDs and the name and manufacturer.
        let (manufacturer, name, ids, serial) = {
            // Get the Vendor and Product IDs.
            let vid = descriptor.vendor_id();
            let pid = descriptor.product_id();

            // Get the manufacturer name.
            let manufacturer = match handle.read_manufacturer_string(language, &descriptor, TIMEOUT) {
                Err(_) => String::new(),
                Ok(s) => s,
            };

            // Get the manufacturer name.
            let product = match handle.read_product_string(language, &descriptor, TIMEOUT) {
                Err(_) => String::new(),
                Ok(s) => s,
            };

            // Read the serial string if possible.
            let serial = match handle.read_serial_number_string(language, &descriptor, TIMEOUT) {
                Err(_) => String::new(),
                Ok(s) => s,
            };

            (manufacturer, product, (vid, pid), serial)
        };

        // Get the device bus.
        let bus = (device.bus_number(), device.address());

        // Build the descriptor.
        let mut out = Self {
            manufacturer,
            name,
            bus,
            ids,
            serial,
            configs: Vec::with_capacity( descriptor.num_configurations() as usize ),
            #[cfg(feature = "application")]
            expanded: false,
            #[cfg(feature = "application")]
            showlist: false,
        };

        for c in 0..descriptor.num_configurations() {
            match device.config_descriptor(c) {
                Ok(descriptor) => out.configs.push( USBConfig::build(&handle, &descriptor, language, &out) ),
                Err(_) => continue,
            }
        }


        Some(out)
    }
}
