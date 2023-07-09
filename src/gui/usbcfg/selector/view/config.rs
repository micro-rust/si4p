//! USB configuration view component.



use crate::usb::device::USBConfig;

use std::sync::Arc;

use super::{
    Message, ShowAction, USBInterfaceView,
};



pub struct USBConfigView {
    /// Function that creates the message variant for this view.
    message: fn(ShowAction) -> crate::gui::Message,

    /// Reference to the USB device information.
    reference: Arc<USBConfig>,

    /// Interfaces of this device.
    interfaces: Vec<USBInterfaceView>,

    /// The key of the device.
    key: usize,

    /// The configuration has its list expanded or not.
    expanded: bool,
}

impl crate::gui::common::Widget for USBConfigView {
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
        let interfaces = self.interfaces.iter()
            .map(|interface| interface.view())
            .collect();

        let base = Column::with_children( interfaces );

        Column::new()
            .push( title )
            .push( base )
            .into()
    }

    fn update(&mut self, event: Self::Event) -> iced::Command<crate::gui::Message> {
        iced::Command::none()
    }
}

impl USBConfigView {
    /// Creates a new `USBDeviceView`.
    pub fn create(reference: Arc<USBConfig>, key: usize, message: fn(ShowAction) -> crate::gui::Message) -> Self {
        // Build the configurations.
        let interfaces = reference.interfaces()
            .map(|interface| USBInterfaceView::create( interface.clone(), key, message ))
            .collect();

        Self { message, reference, key, interfaces, expanded: false }
    }

    /// Processes a show action.
    pub fn show(&mut self, action: &ShowAction) -> bool {
        if action.idx == self.reference.index() {
            // Check the level of the action.
            match action.level {
                1 => self.expanded = action.state,
                _ => for interface in &mut self.interfaces {
                    interface.show( action );
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
        
        // Create the title.
        let title = Text::new( format!("Configuration {}", self.reference.index()) );

        // Create the description.
        let description = Text::new( self.reference.description().clone() );

        // Build the interfaces collapse.
        let configs = {
            // Build the label.
            let label = Text::new( format!("{} interfaces", self.reference.ninterfaces()) );

            let collapse = self.collapse();

            Row::new()
                .push(label)
                .push(collapse)
        };

        Column::new()
            .push(title)
            .push(description)
            .push(configs)
            .into()
    }

    /// Creates the collapse button.
    fn collapse(&self) -> iced::Element<crate::gui::Message> {
        use iced::widget::Button;

        // Create the message.
        let msg = (self.message)( ShowAction::config( !self.expanded, self.key, self.reference.index() ) );

        match self.expanded {
            true => Button::new("-").on_press( msg ),
            _    => Button::new("+").on_press( msg ),
        }.into()
    }
}
