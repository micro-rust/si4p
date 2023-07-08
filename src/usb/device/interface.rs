//! USB Interfaces



use rusb::{
    DeviceHandle, InterfaceDescriptor,
    Language, UsbContext,
};

use super::{
    USBConfig, USBEndpoint,
};



#[derive(Clone, Debug)]
pub struct USBInterface {
    /// Description of this interface.
    description: String,

    /// The vendor and product IDs of the device.
    ids: (u16, u16),

    /// The configuratin index.
    index: u8,

    /// Interface number.
    number: u8,

    /// Alternate settings.
    alternate: u8,

    /// Interface class, subclass and protocol.
    class: (u8, u8, u8),

    /// List of all endpoints in this interface.
    endpoints: Vec<USBEndpoint>,

    /// GUI flag that indicates if the display information is expanded.
    pub expanded: bool,
}

impl USBInterface {
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

    /// Returns the interface number.
    pub fn number(&self) -> u8 {
        self.number
    }

    /// Returns the interface alternate setting.
    pub fn alternate(&self) -> u8 {
        self.alternate
    }

    /// Returns the class of the interface.
    pub fn class(&self) -> (u8, u8, u8) {
        self.class
    }

    /// Returns the number of endpoints of this configuration.
    pub fn nendpoints(&self) -> usize {
        self.endpoints.len()
    }

    /// Returns an iterator over all the endpoints of the device.
    pub fn endpoints<'a>(&'a self) -> impl Iterator<Item = &'a USBEndpoint> {
        self.endpoints.iter()
    }

    /// Returns an iterator over all the endpoints of the device.
    pub fn endpoints_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut USBEndpoint> {
        self.endpoints.iter_mut()
    }

    /// Builds a new interface descriptor.
    pub fn build<'a, C: UsbContext>(handle: &'a DeviceHandle<C>, descriptor: &'a InterfaceDescriptor, language: Language, config: &'a USBConfig) -> Self {
        // Get the string description.
        let description = match handle.read_interface_string(language, descriptor, super::TIMEOUT) {
            Err(_) => String::new(),
            Ok(s) => s,
        };

        let mut interface = Self {
            description,
            ids: config.ids(),
            index: config.index(),
            number: descriptor.interface_number(),
            alternate: descriptor.setting_number(),
            class: (
                descriptor.class_code(),
                descriptor.sub_class_code(),
                descriptor.protocol_code(),
            ),
            endpoints: Vec::new(),
            expanded: false,
        };

        // Parse the endpoints.
        for endpoint in descriptor.endpoint_descriptors() {
            // Create the new enpoint.
            let endpoint = USBEndpoint::build(handle, &endpoint, language, &interface);

            // Add the endpoint to the list.
            interface.endpoints.push(endpoint);
        }

        interface
    }
}
