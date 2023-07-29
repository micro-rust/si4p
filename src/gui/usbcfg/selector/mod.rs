//! USB selector component.



pub(super) mod view;

mod show;



pub use show::ShowAction;

use crate::usb::{
    common::USBTarget,
    device::USBDevice,
};

use seqid::impls::SeqHashMap;

use std::sync::Arc;

use view::USBDeviceView;



pub struct USBSelector<C> {
    /// Function that creates the action message variant for this view.
    message: fn(ShowAction) -> crate::gui::Message,

    /// Function that creates the selection message variant for this view.
    select: fn(USBTarget) -> crate::gui::Message,

    /// Configuration of this USB selector.
    pub configuration: C,
    
    /// Tree of USB devices.
    tree: Vec<USBDeviceView>,
}

impl<C: crate::gui::common::Widget<Event = ()>> crate::gui::common::Widget for USBSelector<C> {
    type Event = ();

    fn view(&self) -> iced::Element<crate::gui::Message> {
        use iced::widget::{
            Column, Scrollable,

            scrollable::{
                Direction, Properties,
            },
        };

        // Get the configuration view.
        let config = self.configuration.view();

        // Collect all the devices.
        let tree: Vec<iced::Element<crate::gui::Message>> = self.tree.iter()
            .map( |device| device.view() )
            .collect();

        // Set the devices into a scrollable.
        let devices = {
            // Collect into a column.
            let base = Column::with_children( tree );

            // Create the scrollable properties.
            let properties = Properties::new()
                .width(5)
                .margin(1)
                .scroller_width(15);

            // Create the scrollable.
            Scrollable::new(base)
                .direction( Direction::Both { vertical: properties, horizontal: properties } )
                .width(iced::Length::Fill)
        };

        Column::new()
            .height(iced::Length::FillPortion(50))
            .push( config )
            .push( devices )
            .into()
    }

    fn update(&mut self, event: Self::Event) -> iced::Command<crate::gui::Message> {
        iced::Command::none()
    }
}

impl<C> USBSelector<C> {
    /// Creates a new `USBSelector`.
    pub(super) fn new(message: fn(ShowAction) -> crate::gui::Message, select: fn(USBTarget) -> crate::gui::Message, configuration: C) -> Self {
        Self {
            message,
            select,
            configuration,
            tree: Vec::new(),
        }
    }

    /// Rebuilds the USB tree.
    pub(super) fn rebuild(&mut self, connected: &SeqHashMap<usize, Arc<USBDevice>>) {
        // Create all new devices.
        for (key, device) in connected.iter() {
            if !self.tree.iter().any(|dev| dev.key() == *key) {
                self.tree.push( USBDeviceView::create( device.clone(), *key, self.message, self.select ) )
            }
        }

        // Remove all old devices.
        let mut remove = Vec::new();

        for (i, element) in self.tree.iter().enumerate() {
            if !connected.keys().any(|key| element.key() == *key) {
                remove.push( i );
            }
        }

        remove.sort();
        remove.reverse();

        for index in remove.iter() {
            self.tree.remove( *index );
        }
    }

    /// Process a show action.
    pub(super) fn show(&mut self, action: &ShowAction) {
        for device in &mut self.tree {
            if device.show( action ) {
                break;
            }
        }
    }
}
