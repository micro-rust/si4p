//! USB Configuration descriptor
//! 



use rusb::{
    DeviceHandle, ConfigDescriptor,
    Language, UsbContext,
};

use std::sync::Arc;

use super::{
    USBDevice, USBInterface,
};


#[derive(Clone, Debug)]
pub struct USBConfig {
    /// The description of this configuration.
    description: String,

    /// The vendor and product IDs of the device.
    ids: (u16, u16),

    /// The USB bus and address.
    bus: (u8, u8),

    /// The configuratin index.
    index: u8,

    /// List of all interfaces in this configuration.
    ifaces: Vec<Arc<USBInterface>>,

    /// GUI flag that indicates if the display information is expanded.
    pub expanded: bool,
}

impl USBConfig {
    /// Returns the description of the configuration.
    pub fn description(&self) -> &String {
        &self.description
    }

    /// Returns the vendor and product IDs.
    pub fn ids(&self) -> (u16, u16) {
        self.ids
    }

    /// Returns the configuration index.
    pub fn index(&self) -> u8 {
        self.index
    }

    /// Returns the number of interfaces of this configuration.
    pub fn ninterfaces(&self) -> usize {
        self.ifaces.len()
    }

    /// Returns an iterator over all the interfaces of the device.
    pub fn interfaces<'a>(&'a self) -> impl Iterator<Item = &'a Arc<USBInterface>> {
        self.ifaces.iter()
    }

    /// Builds the configuration descriptor.
    pub fn build<'a, C: UsbContext>(handle: &'a DeviceHandle<C>, descriptor: &'a ConfigDescriptor, language: Language, device: &'a USBDevice, bus: (u8, u8)) -> Self {
        // Get the string description.
        let description = match handle.read_configuration_string(language, descriptor, super::TIMEOUT) {
            Err(_) => String::new(),
            Ok(s) => s,
        };

        // Create the configuration.
        let mut config = Self {
            description,
            ids: device.ids(),
            bus,
            index: descriptor.number(),
            ifaces: Vec::with_capacity( descriptor.num_interfaces() as usize ),
            expanded: false,
        };

        // Parse the interfaces.
        for interface in descriptor.interfaces() {
            for descriptor in interface.descriptors() {
                // Create the new interface.
                let interface = Arc::new( USBInterface::build(handle, &descriptor, language, &config, bus) );

                // Add the interface to the list.
                config.ifaces.push( interface );
            }
        }

        config
    }
}
