//! USB device selector.
//! Creates a menu to select a USB device up to a specific endpoint.
//! WARNING : Currently only allows selection of an endpoint, nothing higher.
//! WARNING: Currently leaks memory in the expand markers
//! TODO : Track which markers were not used


pub mod actions;
mod message;



use actions::show::ShowAction;

use crate::{
    usb::device::{
        USBConfig, USBDevice, USBEndpoint, USBInterface,
    }
};

use iced::widget::{
    Button, Column, Row, Text,
};

pub use self::message::Message;



pub struct USBSelector<M> {
    createmsg: fn(Message) -> M,
}

impl<M> USBSelector<M> {
    pub fn new(createmsg: fn(Message) -> M) -> Self {
        Self { createmsg, }
    }
}

impl<M: Into<super::Message>> /*Widget for*/ USBSelector<M> {
    pub fn update(&mut self, msg: Message) -> iced::Command<super::Message> {
        match msg {
            Message::Show( action ) => self.show(action),
        }

        iced::Command::none()
    }

    pub fn view(&self) -> iced::Element<super::Message> {
        use crate::usb::CONNECTED;
        use iced::widget::scrollable::{ Properties, Scrollable, };

        // Get a read lock on the devices.
        let list = CONNECTED.blocking_read();

        // Build the device list.
        let devices = list.iter()
            .map( |(key, device)| self.device(device, *key) )
            .collect();

        // Create the base column.
        let base = Column::with_children( devices );

        // Create the scrollable properties.
        let properties = Properties::new()
            .width(5)
            .margin(1)
            .scroller_width(15);

        Scrollable::new(base)
            .width(iced::Length::FillPortion(20))
            .vertical_scroll(properties)
            .into()
    }
}

/// Implementation of lower level display functions.
impl<M: Into<super::Message>> USBSelector<M> {
    fn collapse<A: ShowAction>(&self, expanded: bool, action: A) -> Button<super::Message> {
        use std::sync::Arc;

        // Create the super::Message.
        let msg = (self.createmsg)( Message::Show( Arc::new(action) ) ).into();

        match expanded {
            true => Button::new("+").on_press( msg ),
            _    => Button::new("-").on_press( msg ),
        }
    }

    /// Creates the title of a device.
    fn devtitle(&self, device: &USBDevice, key: usize) -> Column<super::Message> {
        use actions::show::DevShowAction;

        // Get the IDs of the device.
        let (vid, pid) = device.ids();

        // Build the label.
        let title = Text::new( format!("{} {:04X}:{:04X}", device.name(), vid, pid) );

        // Build the manufacturer string.
        let mnf = Text::new( device.manufacturer().clone() );

        // Get the bus and number of the device.
        let (b, n) = device.bus();

        // Build the bus information.
        let bus = Text::new( format!("Bus: {:03}:{:03}", b, n) );

        // Serial string information.
        let serial = Text::new( format!("Serial: {}", device.serial()) );

        // Build the configuration collapse.
        let configs = {
            // Build the label.
            let label = Text::new( format!("{} configurations", device.nconfigs()) );

            let collapse = self.collapse( !device.expanded, DevShowAction::new( !device.expanded, key ) );

            Row::new()
                .push(label)
                .push(collapse)
        };

        Column::new()
            .push(title)
            .push(mnf)
            .push(bus)
            .push(serial)
            .push(configs)
    }

    /// Creates the view of a device.
    fn device(&self, device: &USBDevice, key: usize) -> iced::Element<super::Message> {
        // Build the device title.
        let title = self.devtitle( device, key );

        // If the contents are not showed, return early.
        if !device.expanded {
            return title.into();
        }

        // Get the bus of the device.
        let bus = device.bus();

        // Create a list of all the configurations.
        let configurations = device.configs()
            .map(|config| self.config(config, key, bus))
            .collect();

        // Create the column of the configurations.
        let configurations = Column::with_children( configurations )
            .padding(5);

        Column::new()
            .push(title)
            .push(configurations)
            .into()
    }

    /// Creates the title of a configuration.
    fn cfgtitle(&self, config: &USBConfig, key: usize, idx: u8) -> Column<super::Message> {
        use actions::show::CfgShowAction;

        // Create the title.
        let title = Text::new( format!("Configuration {}", config.index()) );

        // Create the description.
        let description = Text::new( config.description().clone() );

        // Build the interfaces collapse.
        let configs = {
            // Build the label.
            let label = Text::new( format!("{} interfaces", config.ninterfaces()) );

            let collapse = self.collapse( !config.expanded, CfgShowAction::new( !config.expanded, key, idx ) );

            Row::new()
                .push(label)
                .push(collapse)
        };

        Column::new()
            .push(title)
            .push(description)
            .push(configs)
    }

    /// Creates the view of a configuration.
    fn config(&self, config: &USBConfig, key: usize, bus: (u8, u8)) -> iced::Element<super::Message> {
        // Get the index of the configuration.
        let idx = config.index();

        // Build the device title.
        let title = self.cfgtitle( config, key, idx );

        // If the contents are not showed, return early.
        if !config.expanded {
            return title.into();
        }

        // Create a list of all the interfaces.
        let interfaces = config.interfaces()
            .map(|interface| self.interface(interface, key, idx, bus))
            .collect();

        // Create the column of the interfaces.
        let interfaces = Column::with_children( interfaces )
            .padding(5);

        Column::new()
            .push(title)
            .push(interfaces)
            .into()
    }

    /// Creates the title of an interface.
    fn ifacetitle(&self, interface: &USBInterface, key: usize, idx: u8, num: u8) -> Column<super::Message>{
        use actions::show::IfaceShowAction;

        // Create the title.
        let title = Text::new( format!("Interface {}", interface.number()) );

        // Create the description.
        let description = Text::new( interface.description().clone() );

        // Create the interface class.
        let class = Text::new( format!("{:?}", interface.class()) );

        // Create the endpoints collapse.
        let endpoints = {
            // Build the label.
            let label = Text::new( format!("{} endpoints", interface.nendpoints()) );

            // Build the collapse button.
            let collapse = self.collapse( !interface.expanded, IfaceShowAction::new( !interface.expanded, key, idx, num ) );

            Row::new()
                .push(label)
                .push(collapse)
        };

        Column::new()
            .push(title)
            .push(description)
            .push(class)
            .push(endpoints)
    }

    fn interface(&self, interface: &USBInterface, key: usize, idx: u8, bus: (u8, u8)) -> iced::Element<super::Message> {
        // Get the interface number.
        let num = interface.number();

        // Build the device title.
        let title = self.ifacetitle( interface, key, idx, num );

        // If the contents are not showed, return early.
        if !interface.expanded {
            return title.into();
        }

        // Create a list of all the endpoints.
        let endpoints = interface.endpoints()
            .map(|endpoint| self.endpoint(endpoint, key, idx, num, bus))
            .collect();

        // Create the column of the configurations.
        let endpoints = Column::with_children( endpoints )
            .padding(5);

        Column::new()
            .push(title)
            .push(endpoints)
            .into()
    }

    /// Builds the view of an endpoint.
    fn endpoint(&self, endpoint: &USBEndpoint, _: usize, idx: u8, num: u8, bus: (u8, u8)) -> iced::Element<super::Message> {
        // Build the title.
        let title = {
            let label = Text::new(format!("Endpoint {}", endpoint.enumber()));

            let info = Text::new( format!("{:?} {:?} @ {:02X}h", endpoint.transfer(), endpoint.direction(), endpoint.address()) );

            Column::new()
                .push(label)
                .push(info)
        };

        match (endpoint.transfer(), endpoint.direction()) {
            (rusb::TransferType::Bulk, rusb::Direction::In) => {
                use crate::usb::{ Command as USBCommand, common::USBTarget, };

                // Create the USB command.
                let command = USBCommand::DefmtOpen( USBTarget::new( endpoint.ids(), bus, idx, num, endpoint.alternate(), endpoint.enumber() ) );

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

/// Implementation of super::Message action functions.
impl<M> USBSelector<M> {
    fn show(&mut self, action: std::sync::Arc<dyn ShowAction>) {
        use crate::usb::CONNECTED;

        // Get a read lock on the devices.
        let mut show = CONNECTED.blocking_write();

        // Get the device ID.
        let id = action.device();

        // Get the show state.
        let state = action.state();

        // Get the device it refers to.
        let dev = match show.get_mut( &id ) {
            Some(device) => device,
            _ => return,
        };

        // Check if the action is for a device of lower.
        let id = match action.configuration() {
            Some(id) => id,
            _ => {
                dev.expanded = state;
                return;
            },
        };

        // Get the configuration it refers to.
        let cfg = match dev.configs_mut().find(|c| c.index() == id) {
            Some(config) => config,
            _ => return,
        };

        // Check if the action is for a configuration or lower.
        let id = match action.interface() {
            Some(id) => id,
            _ => {
                cfg.expanded = state;
                return;
            },
        };
 
        // Get the interface it refers to.
        let iface = match cfg.interfaces_mut().find(|i| i.number() == id) {
            Some(interface) => interface,
            _ => return,
        };

        iface.expanded = state;
    }
}
