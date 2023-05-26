//! USB device selector.
//! WARNING: Currently leaks memory in the expand markers
//! TODO : Track which markers were not used




use crate::{
    gui::{
        USBCommand,
    },

    usb::{
        CONNECTED,
    },
};

use iced::{
    Element,

    widget::{
        Column, Text,
    },
};

use super::Message;

use tokio::sync::mpsc::Sender;




pub struct USBSelector {
    /// Channel to send USB commands.
    usb: Sender<USBCommand>,

    /// Currently selected device.
    selected: Option<usize>,

    /// Expand tracker.
    expand: HashMap<usize, ExpandMarker>,
}

impl USBSelector {
    pub fn new(usb: Sender<USBCommand>) -> Self {
        Self { usb, selected: None, collapse: HashMap::new() }
    }

    pub fn update(&mut self, event: Event) -> Command<Message> {

    }

    pub fn view(&self) -> Element<Message> {
        // Get a read lock on the devices.
        let list = CONNECTED.blocking_read();

        for (key, dev) in list.iter() {
            println!("Found device {}-{}", key, dev.name());
        }

        // Build the base configuration.
        let base = Column::new()
            .width(iced::Length::FillPortion(20));

        // Add all the connected devices to the list.
        for (key, dev) in list.iter() {
            // Build the device title.
            let title = {
                // Get the IDs of the device.
                let ids = dev.ids();

                // Build the label.
                let label = Text::new(format!("{} {:04X}:{:04X}", dev.name(), ids.0, ids.1));

                // Build the collapse button.
            };
        }

        devices.into()
    }
}


pub struct ExpandMarker {
    /// Indicates if the device is expanded.
    device: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ShowAction {
    /// Indicates a show action for a device.
    Device(bool, usize),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Event {
    /// Collapses the given marker.
    Collapse(usize),
}