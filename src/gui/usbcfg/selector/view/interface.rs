//! USB Interface view.



use crate::usb::device::USBInterface;

use std::sync::Arc;

use super::{
    Message, ShowAction, USBEndpointView,
};



pub struct USBInterfaceView {
    /// Function that creates the message variant for this view.
    message: fn(ShowAction) -> crate::gui::Message,

    /// Reference to the USB device information.
    reference: Arc<USBInterface>,

    /// Interfaces of this device.
    endpoints: Vec<USBEndpointView>,

    /// The key of the device.
    key: usize,

    /// The configuration has its list expanded or not.
    expanded: bool,
}

impl crate::gui::common::Widget for USBInterfaceView {
    type Event = ();

    fn view(&self) -> iced::Element<crate::gui::Message> {
        use iced::widget::Column;

        // Build the device title.
        let title = self.title();

        // If the contents are not showed, return early.
        if !self.expanded {
            return title;
        }

        // Build the list of endpoints.
        let endpoints = self.endpoints.iter()
            .map(|endpoint| endpoint.view())
            .collect();

        let base = Column::with_children( endpoints );

        Column::new()
            .push( title )
            .push( base )
            .into()
    }

    fn update(&mut self, event: Self::Event) -> iced::Command<crate::gui::Message> {
        iced::Command::none()
    }
}

impl USBInterfaceView {
    /// Creates a new `USBDeviceView`.
    pub fn create(reference: Arc<USBInterface>, key: usize, message: fn(ShowAction) -> crate::gui::Message) -> Self {
        // Build the configurations.
        let endpoints = reference.endpoints()
            .map(|endpoint| USBEndpointView::create( endpoint.clone(), key ))
            .collect();

        Self { message, reference, key, endpoints, expanded: false }
    }

    /// Processes a show action.
    pub fn show(&mut self, action: &ShowAction) -> bool {
        if action.num == self.reference.number() {
            self.expanded = action.state;

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
        let title = Text::new( format!("Interface {}", self.reference.number()) );

        // Create the description.
        let description = Text::new( self.reference.description().clone() );

        // Build the interfaces collapse.
        let configs = {
            // Build the label.
            let label = Text::new( format!("{} endpoints", self.reference.nendpoints()) );

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
        let msg = (self.message)( ShowAction::interface( !self.expanded, self.key, self.reference.index(), self.reference.number() ) );

        match self.expanded {
            true => Button::new("-").on_press( msg ),
            _    => Button::new("+").on_press( msg ),
        }.into()
    }
}
