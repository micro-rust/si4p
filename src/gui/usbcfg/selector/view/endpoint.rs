//! USB Endpoint view.



use crate::usb::device::USBEndpoint;

use std::sync::Arc;



pub struct USBEndpointView {
    /// Reference to the USB device information.
    reference: Arc<USBEndpoint>,

    /// The key of the device.
    key: usize,
}

impl crate::gui::common::Widget for USBEndpointView {
    type Event = ();

    fn view(&self) -> iced::Element<crate::gui::Message> {
        // Build the device title.
        let title = self.title();

        title
    }

    fn update(&mut self, event: Self::Event) -> iced::Command<crate::gui::Message> {
        iced::Command::none()
    }
}

impl USBEndpointView {
    /// Creates a new `USBDeviceView`.
    pub fn create(reference: Arc<USBEndpoint>, key: usize) -> Self {
        Self { reference, key, }
    }

    /// Creates the title of the view.
    fn title(&self) -> iced::Element<crate::gui::Message> {
        use iced::widget::{
            Button, Column, Row, Text,
        };
        
        // Build the title.
        let title = {
            let label = Text::new(format!("Endpoint {}", self.reference.enumber()));

            let info = Text::new( format!("{:?} {:?} @ {:02X}h", self.reference.transfer(), self.reference.direction(), self.reference.address()) );

            Column::new()
                .push(label)
                .push(info)
        };

        match (self.reference.transfer(), self.reference.direction()) {
            (rusb::TransferType::Bulk, rusb::Direction::In) => {
                use crate::usb::{ Command as USBCommand, common::USBTarget, };

                // Create the USB command.
                let command = USBCommand::DefmtOpen(
                    USBTarget::new(
                        self.reference.ids(),
                        self.reference.bus(),
                        self.reference.index(),
                        self.reference.number(),
                        self.reference.alternate(),
                        self.reference.enumber()
                    )
                );

                // Create the selection button.
                let select = Button::new( Text::new( "Select" ) )
                    .on_press( crate::gui::Message::USB( command ) );

                Row::new()
                    .push(title)
                    .push(select)
                    .into()
            },
            _ => title.into(),
        }
    }
}
