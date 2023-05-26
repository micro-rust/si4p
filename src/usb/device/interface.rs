//! USB Interfaces



use rusb::{
    Context, DeviceHandle,
    InterfaceDescriptor, Language,
    UsbContext,
};

use super::USBConfig;



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

    /// Builds a new interface descriptor.
    pub fn build<'a, C: UsbContext>(handle: &'a DeviceHandle<C>, descriptor: &'a InterfaceDescriptor, language: Language, config: &'a USBConfig) -> Self {
        // Get the string description.
        let description = match handle.read_interface_string(language, descriptor, super::TIMEOUT) {
            Err(_) => String::new(),
            Ok(s) => s,
        };

        Self {
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
        }
    }

    #[cfg(feature = "application")]
    pub fn view(&self) -> iced::Element<crate::gui::Message> {
        use iced::{
            widget::{
                Button, Column, Row, Text,
            },
        };

        // Build the information section.
        let information = {
            // Build the interface name.
            let name = Text::new( &self.description );

            // Build the number and alternate setting.
            let info = {
                // Build the interface number.
                let number = Text::new( format!("ID: {:02X}h", self.number) );

                // Build the alternate setting.
                let setting = Text::new( format!("Setting: {:02X}h", self.alternate) );

                Row::new()
                    .push(number)
                    .push(setting)
            };

            // Build the class.
            let class = Text::new( format!("Class: {:02X}:{:02X}:{:02X}", self.class.0, self.class.1, self.class.2) );

            Column::new()
                .push(name)
                .push(info)
                .push(class)
        };

        // Build the connect button.
        let button = Button::new( "Connect" )
            .on_press( crate::gui::Message::DefmtConnect( self.ids, self.index, self.number, self.alternate ) );

        Row::new()
            .push(information)
            .push(button)
            .into()
    }
}
