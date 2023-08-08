//! USB device selector component.



mod event;
mod state;



use crate::{
    gui::Message,
    usb::Command,
};

use event::Event;

use iced::{
    Element, Renderer,

    widget::Component,
};

use probe_rs::{ DebugProbeInfo, Probe, };

use state::State;



#[derive(Clone, Debug)]
pub struct USBSelector {
    /// List of all probes available.
    probes: Vec<DebugProbeInfo>,

    /// Currently selected probe.
    probe: Option<DebugProbeInfo>,
}

impl Component<Message, Renderer> for USBSelector {
    type Event = Event;
    type State = State;

    fn update(&mut self, state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            Event::OpenProbe( info ) => return Some( Command::ProbeOpen(info).into() ),

            Event::CloseProbe => return Some( Command::ProbeClose.into() ),

            Event::SetView( view ) => state.selected = view,
        }

        None
    }

    fn view(&self, state: &Self::State) -> Element<Self::Event, Renderer> {
        use iced::{
            Length,

            alignment::Horizontal,

            widget::{
                Button, Column, Container, Row, Scrollable, Tooltip,
                scrollable::{ Direction, Properties }, tooltip::Position,
            },
        };

        // Build the two selection buttons.
        let buttons = {
            // Build the defmt button.
            let mut defmt = Button::new( "defmt" )
                .width(Length::Fill);

            // Build the probe button.
            let mut probe = Button::new( "probe" )
                .width(Length::Fill);

            // Enable the unselected button.
            match state.selected {
                View::Defmt => probe = probe.on_press( Event::SetView( View::Probe ) ),
                View::Probe => defmt = defmt.on_press( Event::SetView( View::Defmt ) ),
            }

            Row::new()
                .push( defmt )
                .push( probe )
        };

        // Build the currently selected view.
        let mut view = match state.selected {
            View::Defmt => {

                Column::new()
                    .push(  buttons  )
            },

            // Check if there is a probe selected.
            View::Probe => match &self.probe {
                Some(probe) => {
                    // Create the probe title.
                    let title = Self::probetitle( probe );

                    // Create the close connection button.
                    let content = Button::new( "Close" )
                        .on_press( Event::CloseProbe );

                    let close = Tooltip::new( content, "Close the connection to the debug probe", Position::Left )
                        .gap(5);

                    // Create the container for alignment.
                    let container = Container::new(close)
                        .height( Length::Shrink )
                        .width( Length::Fill )
                        .align_x( Horizontal::Right );

                    // Build the information.
                    let probeinfo = Row::new()
                        .push(title)
                        .push(container);

                    Column::new()
                        .push(  buttons  )
                        .push( probeinfo )
                },

                None => {
                    // Create the list of probes.
                    let probelist = self.probes.iter()
                        .map(|probe| {
                            // Create the title.
                            let title = Self::probetitle( probe );

                            // Create the open button.
                            let open = {
                                // Create the button.
                                let button = Button::new("Open")
                                    .on_press( Event::OpenProbe( probe.clone() ) );

                                // Create the container for alignment.
                                let container = Container::new(button)
                                    .height( Length::Shrink )
                                    .width( Length::Fill )
                                    .align_x( Horizontal::Right );

                                container
                            };

                            Row::new()
                                .push(title)
                                .push(open)
                        })
                        .fold(Column::new(), |column, row| column.push(row));

                    // Create the scrollable properties.
                    let properties = Properties::new()
                        .margin(5)
                        .scroller_width(15)
                        .width(10);

                    // Fit into a scrollable.
                    let scrollable = Scrollable::new( probelist )
                        .direction( Direction::Vertical( properties ) )
                        .height( Length::Fill )
                        .width( Length::Fill );

                    Column::new()
                        .push( buttons )
                        .push( scrollable )
                },                
            },
        };

        // Configure the view.
        view = view.padding(5);

        // TODO : Container and style.

        view.into()
    }
}

impl USBSelector {
    /// Static initializer.
    pub(super) fn new() -> Self {
        Self {
            probes: Probe::list_all(),
            probe: None,
        }
    }

    /// Rebuilds the device tree.
    pub(super) fn rebuild(&mut self) {
        // Updates the probes.
        self.probes = Probe::list_all();
    }

    /// Sets the active debug probe.
    pub(super) fn setprobe(&mut self, info: DebugProbeInfo) {
        self.probe = Some( info );
    }

    /// Clears the active debug probe.
    pub(super) fn clearprobe(&mut self) {
        self.probe = None;
    }

    /// Creates the probe title.
    fn probetitle(probe: &DebugProbeInfo) -> Element<Event> {
        use iced::widget::{
            Column, Text,
        };

        // Get the IDS of the device.
        let ids = Text::new( format!("Debug Probe {:04X}:{:04X}", probe.vendor_id, probe.product_id) );

        // Get the serial number.
        let serial = Text::new( format!("S/N : {}", probe.serial_number.as_ref().unwrap_or(&String::new())) )
            .size(10);

        Column::new()
            .push(ids)
            .push(serial)
            .into()
    }
}



#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum View {
    /// Defmt device selection.
    Defmt,

    /// Probe device selection.
    Probe,
}