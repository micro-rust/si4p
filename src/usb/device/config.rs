//! USB Configuration descriptor
//! 



use rusb::{
    Context, DeviceHandle,
    ConfigDescriptor, Language,
    UsbContext,
};

use super::{
    USBDevice, USBInterface,
};


#[derive(Clone, Debug)]
pub struct USBConfig {
    /// The description of this configuration.
    description: String,

    /// The vendor and product IDs of the device.
    ids: (u16, u16),

    /// The configuratin index.
    index: u8,

    /// List of all interfaces in this configuration.
    ifaces: Vec<USBInterface>,

    #[cfg(feature = "application")]
    /// GUI flag that indicates if the display information is expanded.
    expanded: bool,
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

    /// Returns an iterator over all the interfaces of the device.
    pub fn interfaces<'a>(&'a self) -> impl Iterator<Item = &'a USBInterface> {
        self.ifaces.iter()
    }

    /// Builds the configuration descriptor.
    pub fn build<'a, C: UsbContext>(handle: &'a DeviceHandle<C>, descriptor: &'a ConfigDescriptor, language: Language, device: &'a USBDevice) -> Self {
        // Get the string description.
        let description = match handle.read_configuration_string(language, descriptor, super::TIMEOUT) {
            Err(_) => String::new(),
            Ok(s) => s,
        };

        // Create the configuration.
        let mut config = Self {
            description,
            ids: device.ids(),
            index: descriptor.number(),
            ifaces: Vec::with_capacity( descriptor.num_interfaces() as usize ),
            #[cfg(feature = "application")]
            expanded: true,
        };

        // Parse the interfaces.
        for interface in descriptor.interfaces() {
            for descriptor in interface.descriptors() {
                // Create the new interface.
                let interface = USBInterface::build(handle, &descriptor, language, &config);

                // Add the interface to the list.
                config.ifaces.push( interface );
            }
        }

        config
    }

    #[cfg(feature = "application")]
    pub fn view(&self) -> iced::Element<crate::gui::Message> {
        use iced::{
            widget::{
                Button, Column, Row, Text,
            },
        };

        // Create the topbar
        let topbar = {
            // Build the information section.
            let info = {
                // Buidl the configuration number
                let number = Text::new( format!("CFG {}", self.index) );

                // Build the description
                let description = Text::new( &self.description );

                Column::new()
                    .push(number)
                    .push(description)
            };

            // Builds the button
            let mut collapse = Button::new( "X" )
                .on_press( crate::gui::Message::USBConfigExpanded( self.ids, self.index, !self.expanded ) );

            Row::new()
                .push(info)
                .push(collapse)
        };

        // If not expanded, show no more information.
        if !self.expanded {
            return topbar.into();
        }

        // Create the information on the interfaces.
        let interfaces = self.ifaces.iter()
            .fold(Column::new(), |col, iface| col.push(iface.view()));

        Column::new()
            .push(topbar)
            .push(interfaces)
            .into()
    }

    #[cfg(feature = "application")]
    /// Expands the display information.
    pub fn expanded(&mut self, expanded: bool) {
        self.expanded = expanded;
    }
}
