//! USB Endopoints



use rusb::{
    DeviceHandle, UsbContext, Language,
    EndpointDescriptor, TransferType, Direction,
};

use super::{
    USBInterface,
};



#[derive(Clone, Debug)]
pub struct USBEndpoint {
    /// The vendor and product IDs of the device.
    ids: (u16, u16),

    /// The configuration index.
    index: u8,

    /// Interface number.
    number: u8,

    /// Alternate settings.
    alternate: u8,

    /// The endpoint address.
    address: u8,

    /// The endpoint number.
    enumber: u8,

    /// The direction of the endpoint.
    direction: Direction,

    /// The transfer type.
    transfer: TransferType,
}

impl USBEndpoint {
    /// Returns the vendor and product IDs.
    pub fn ids(&self) -> (u16, u16) {
        self.ids
    }

    /// Returns the configuration index.
    pub fn index(&self) -> u8 {
        self.index
    }

    /// Returns the endpoint interface number.
    pub fn number(&self) -> u8 {
        self.number
    }

    /// Returns the endpoint alternate setting.
    pub fn alternate(&self) -> u8 {
        self.alternate
    }

    /// Returns the endpoint number.
    pub fn enumber(&self) -> u8 {
        self.enumber
    }

    /// Returns the endpoint address.
    pub fn address(&self) -> u8 {
        self.address
    }

    /// Returns the endpoint transfer type.
    pub fn transfer(&self) -> TransferType {
        self.transfer
    }

    /// Returns the endpoint direction.
    pub fn direction(&self) -> Direction {
        self.direction
    }

    /// Builds a new endpoint descriptor.
    pub fn build<'a, C: UsbContext>(handle: &'a DeviceHandle<C>, descriptor: &'a EndpointDescriptor, language: Language, iface: &'a USBInterface) -> Self {
        // Create the endpoint.
        let mut endpoint = Self {
            ids: iface.ids(),
            index: iface.index(),
            number: iface.number(),
            alternate: iface.alternate(),

            address: descriptor.address(),
            enumber: descriptor.number(),
            direction: descriptor.direction(),
            transfer: descriptor.transfer_type(),
        };

        endpoint
    }
}
