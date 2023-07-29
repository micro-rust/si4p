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

    /// The bus and address of the USB.
    bus: (u8, u8),

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

    /// Returns the bus and address.
    pub fn bus(&self) -> (u8, u8) {
        self.bus
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
    pub fn build<'a, C: UsbContext>(_: &'a DeviceHandle<C>, descriptor: &'a EndpointDescriptor, _: Language, iface: &'a USBInterface, bus: (u8, u8)) -> Self {
        // Create the endpoint.
        let endpoint = Self {
            ids: iface.ids(),
            bus,
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
