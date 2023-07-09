//! USB device view component.



use crate::usb::device::USBDevice;

use std::sync::Arc;

use super::{
    ShowAction, USBConfigView,
};



pub struct USBDeviceView {
    /// Function that creates the message variant for this view.
    message: fn(ShowAction) -> crate::gui::Message,

    /// Reference to the USB device information.
    reference: Arc<USBDevice>,

    /// Configurations of this device.
    configurations: Vec<USBConfigView>,

    /// The key of the device.
    key: usize,

    /// The device has its list expanded or not.
    expanded: bool,
}

impl crate::gui::common::Widget for USBDeviceView {
    type Event = ();

    fn view(&self) -> iced::Element<crate::gui::Message> {
        use iced::widget::Column;

        // Build the device title.
        let title = self.title();

        // If the contents are not showed, return early.
        if !self.expanded {
            return title;
        }

        // Build the list of configurations.
        let configurations = self.configurations.iter()
            .map(|config| config.view())
            .collect();

        let base = Column::with_children( configurations );

        Column::new()
            .push( title )
            .push( base )
            .into()
    }

    fn update(&mut self, event: Self::Event) -> iced::Command<crate::gui::Message> {
        iced::Command::none()
    }
}

impl USBDeviceView {
    /// Returns the key of this device.
    pub fn key(&self) -> usize {
        self.key
    }

    /// Creates a new `USBDeviceView`.
    pub fn create(reference: Arc<USBDevice>, key: usize, message: fn(ShowAction) -> crate::gui::Message) -> Self {
        // Build the configurations.
        let configurations = reference.configs()
            .map(|config| USBConfigView::create( config.clone(), key, message ))
            .collect();

        Self { message, reference, key, configurations, expanded: false }
    }

    /// Processes a show action.
    pub fn show(&mut self, action: &ShowAction) -> bool {
        if action.key == self.key {
            // Check the level of the action.
            match action.level {
                0 => self.expanded = action.state,
                _ => for config in &mut self.configurations {
                    config.show( action );
                },
            }

            return true;
        }

        false
    }

    /// Creates the title of the view.
    fn title(&self) -> iced::Element<crate::gui::Message> {
        use iced::widget::{
            Column, Row, Text,
        };
        
        // Get the IDs of the device.
        let (vid, pid) = self.reference.ids();

        // Build the label.
        let title = Text::new( format!("{} {:04X}:{:04X}", self.reference.name(), vid, pid) );

        // Build the manufacturer string.
        let mnf = Text::new( self.reference.manufacturer().clone() );

        // Get the bus and number of the device.
        let (b, n) = self.reference.bus();

        // Build the bus information.
        let bus = Text::new( format!("Bus: {:03}:{:03}", b, n) );

        // Serial string information.
        let serial = Text::new( format!("Serial: {}", self.reference.serial()) );

        // Build the configuration collapse.
        let configs = {
            // Build the label.
            let label = Text::new( format!("{} configurations", self.reference.nconfigs()) );

            let collapse = self.collapse();

            Row::new()
                .push(label)
                .push(collapse)
        };

        Column::new()
            .push( title )
            .push( mnf )
            .push( bus )
            .push( serial )
            .push( configs )
            .into()
    }

    /// Creates the collapse button.
    fn collapse(&self) -> iced::Element<crate::gui::Message> {
        use iced::widget::Button;

        // Create the message.
        let msg = (self.message)( ShowAction::device( !self.expanded, self.key ) );

        match self.expanded {
            true => Button::new("-").on_press( msg ),
            _    => Button::new("+").on_press( msg ),
        }.into()
    }
}
