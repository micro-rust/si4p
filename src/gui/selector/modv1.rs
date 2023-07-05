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

use std::{
    collections::HashMap,
    sync::Mutex,
};

use tokio::sync::mpsc::Sender;



pub struct USBSelector {
    /// Channel to send USB commands.
    usb: Sender<USBCommand>,

    /// Currently selected device.
    selected: Option<usize>,
}

impl USBSelector {
    pub fn new(usb: Sender<USBCommand>) -> Self {
        Self { usb, selected: None, }
    }

    pub fn update(&mut self, msg: Message) -> iced::Command<super::Message> {
        match msg {
            Message::Show( action ) => self.showaction(action),

            _ => (),
        }

        iced::Command::none()
    }

    pub fn view(&self) -> iced::Element<super::Message> {
        use iced::{
            Length,

            widget::{
                Button, Column, Row,
                Scrollable, Text,

                scrollable::{
                    Properties,
                },
            },
        };

        // Get a read lock on the devices.
        let list = CONNECTED.blocking_read();

        // Build the base configuration.
        let mut base = Column::new();

        // List of all currently connected devices.
        let mut connected = Vec::new();

        // Add all the connected devices to the list.
        for (key, dev) in list.iter() {
            // Add the key to the list of connected devices.
            connected.push(key);

            // Build the device title.
            let title = {
                // Get the IDs of the device.
                let ids = dev.ids();

                // Build the label.
                let label = Text::new(format!("{} {:04X}:{:04X}", dev.name(), ids.0, ids.1));

                // Build the collapse button.
                let collapse = if dev.expanded {
                    Button::new("-").on_press( ShowAction::Device(false, *key).into() )
                } else {
                    Button::new("+").on_press( ShowAction::Device(true, *key).into() )
                };

                Row::new()
                    .push(label)
                    .push(collapse)
            };

            // If the contents are not showed, return early.
            if !dev.expanded {
                // Add the title to the GUI.
                base = base.push( title );

                continue;
            }

            // Build the information section.
            let information = {
                // Build the manufacturer string.
                let mnf = Text::new(dev.manufacturer().clone());

                // Build the bus information.
                let bus = Text::new(format!("Bus: {:03}:{:03}", dev.bus().0, dev.bus().1));

                // Serial string information.
                let serial = Text::new( format!("Serial: {}", dev.serial()) );

                Column::new()
                    .push(mnf)
                    .push(bus)
                    .push(serial)
            };

            // Build the configuration collapse.
            let cfgbar = {
                // Build the label.
                let label = Text::new(format!("{} configurations", dev.nconfigs()));

                // Build the collapse button.
                let collapse = if dev.showlist {
                    Button::new("-").on_press( ShowAction::ConfigList(false, *key).into() )
                } else {
                    Button::new("+").on_press( ShowAction::ConfigList(true, *key).into() )
                };

                Row::new()
                    .push(label)
                    .push(collapse)
            };

            // Create the column of the section.
            let devcol = Column::new()
                .padding(5)
                .push(title)
                .push(information)
                .push(cfgbar);

            // If the contents are not showed, return early.
            if !dev.showlist {
                // Add the title to the GUI.
                base = base.push( devcol );

                continue;
            }

            // Create the configurations list.
            let mut configs = Column::new()
                .padding(10);

            for (i, cfg) in dev.configs().enumerate() {
                // Create the title.
                let title = Text::new(format!("Configuration {}", cfg.index()));

                // Create the description.
                let description = Text::new( cfg.description().clone() );

                // Create the interfaces collapse.
                let collapse = {
                    // Build the label.
                    let label = Text::new(format!("{} interfaces", cfg.ninterfaces()));

                    // Build the collapse button.
                    let collapse = if cfg.expanded {
                        Button::new("-").on_press( ShowAction::Config(false, *key, cfg.index()).into() )
                    } else {
                        Button::new("+").on_press( ShowAction::Config(true, *key, cfg.index()).into() )
                    };

                    Row::new()
                        .padding(2)
                        .push(label)
                        .push(collapse)
                };

                // Create this section.
                let cfgcol = Column::new()
                    .push(title)
                    .push(description)
                    .push(collapse);

                // If not expanded, early return.
                if !cfg.expanded {
                    // Add to the list.
                    configs = configs.push(cfgcol);

                    continue;
                }

                // Create the interfaces list.
                let mut interfaces = Column::new()
                    .padding(10);

                for (j, iface) in cfg.interfaces().enumerate() {
                    // Create the title.
                    let title = Text::new(format!("Interface {}", iface.number()));

                    // Create the description.
                    let description = Text::new( iface.description().clone() );

                    // Create the interface class.
                    let class = Text::new(format!("{:?}", iface.class()));

                    // Create the endpoints collapse.
                    let collapse = {
                        // Build the label.
                        let label = Text::new(format!("{} endpoints", iface.nendpoints()));

                        // Build the collapse button.
                        let collapse = if iface.expanded {
                            Button::new("-").on_press( ShowAction::Interface(false, *key, cfg.index(), iface.number(), iface.alternate()).into() )
                        } else {
                            Button::new("+").on_press( ShowAction::Interface(true, *key, cfg.index(), iface.number(), iface.alternate()).into() )
                        };

                        Row::new()
                            .padding(2)
                            .push(label)
                            .push(collapse)
                    };

                    // Create this section.
                    let ifacecol = Column::new()
                        .push(title)
                        .push(description)
                        .push(class)
                        .push(collapse);

                    // If not expanded, return early.
                    if !iface.expanded {
                        // Add the interface.
                        interfaces = interfaces.push(ifacecol);

                        continue;
                    }

                    // Create the list of endpoints.
                    let mut endpoints = Column::new()
                        .padding(5);

                    for endpoint in iface.endpoints() {
                        // Build the title.
                        let title = Text::new(format!("Endpoint {}", endpoint.enumber()));

                        // Build the information.
                        let info = Text::new(format!("{:?} {:?} @ {:02X}h", endpoint.transfer(), endpoint.direction(), endpoint.address()));

                        let epcol = Column::new()
                            .push(title)
                            .push(info);

                        match (endpoint.transfer(), endpoint.direction()) {
                            (rusb::TransferType::Bulk, rusb::Direction::In) => {
                                // Create the selection button.
                                let select = Button::new( Text::new( "Select" ) );

                                // Create the row.
                                let row = Row::new()
                                    .push(epcol)
                                    .push(select);

                                endpoints = endpoints.push(row);
                            },
                            _ => endpoints = endpoints.push(epcol),
                        }

                        ;
                    }

                    interfaces = interfaces.push( ifacecol.push(endpoints) );
                }

                configs = configs.push( cfgcol.push(interfaces) )
            }

            // Push the new configuration.
            base = base.push( devcol.push(configs) );
        }

        // Create the scrollable properties.
        let properties = Properties::new()
            .width(5)
            .margin(1)
            .scroller_width(15);

        Scrollable::new(base)
            .width(Length::FillPortion(20))
            .vertical_scroll(properties)
            .into()
    }

    /// Performs the given show action
    fn showaction(&mut self, action: ShowAction) {
        // Get a read lock on the devices.
        let mut show = CONNECTED.blocking_write();

        match action {
            ShowAction::Device(state, key) => match show.get_mut(&key) {
                Some(device) => device.expanded = state,
                _ => (),
            },

            ShowAction::ConfigList(state, key) => match show.get_mut(&key) {
                Some(device) => device.showlist = state,
                _ => (),
            },

            ShowAction::Config(state, key, index) => match show.get_mut(&key) {
                Some(device) => match device.configs_mut().find(|cfg| cfg.index() == index) {
                    Some(config) => config.expanded = state,
                    _ => (),
                },

                _ => (),
            },

            ShowAction::Interface(state, key, index, number, alternate) => match show.get_mut(&key) {
                Some(device) => match device.configs_mut().find(|cfg| cfg.index() == index) {
                    Some(config) => match config.interfaces_mut().find(|iface| iface.number() == number && iface.alternate() == alternate) {
                        Some(iface) => iface.expanded = state,
                        _ => (),
                    },
                    _ => (),
                },

                _ => (),
            },

            _ => (),
        }
    }

    /// Checks that the given show state exists. If it does not, it creates a new showstate.
    fn showstate(&self, show: &mut HashMap<usize, ShowState>, key: &usize) {
        // Check for the show state.
        match show.get(key) {
            Some(_) => return,
            _ => (),
        }

        // Ignore the error, already checked.
        show.insert(*key, ShowState::new());
    }
}


pub struct ShowState {
    /// Indicates if the device is expanded.
    device: bool,

    /// Indicates if the config list is expanded.
    configlist: bool,
}

impl ShowState {
    pub fn new() -> Self {
        Self { device: true, configlist: false, }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ShowAction {
    /// Indicates a show action for a device.
    Device(bool, usize),

    /// Indicates a show action for a configuration list.
    ConfigList(bool, usize),

    /// Indicates a show action for a configuration.
    Config(bool, usize, u8),

    /// Indicates a show action for an interface.
    Interface(bool, usize, u8, u8, u8),
}

impl core::convert::Into<super::Message> for ShowAction {
    fn into(self) -> super::Message {
        super::Message::Selector( Message::Show( self ) )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Message {
    /// A new show action occured.
    Show( ShowAction ),
}
