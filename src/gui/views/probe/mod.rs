//! GUI Probe view.
//! This GUI view handles connection with the enumerated probes, along with
//! debugging, memory inspection and other utilities.





pub mod common;
mod state;



use crate::{
    gui::{
        msg::{
            Message, ProbeMessage,
        },

        theme::{
            Theme,
            button, tooltip,
        },
    },
};

use iced::{
    Command, Column, Container, Element, Row,

    Align, Length,

    Text, PickList, TextInput,

    button::{ Button },
    tooltip::{ Position, Tooltip },
};

use probe_rs::{
    DebugProbeInfo, Probe,
};

use regex::Regex;

use self::common::{
    Datatype, DATATYPES,
};



pub struct ProbeView {
    /// Internal widget state.
    state: state::State,

    /// List of all projects.
    projects: Vec<String>,

    /// Selected project.
    selproject: Option<String>,

    /// List of all probes.
    probes: Vec<DebugProbeInfo>,

    /// List of all probe names.
    probenames: Vec<String>,

    /// Selected probe.
    selprobe: Option<String>,

    /// Internal regex to validate input.
    regex: [Regex; 4],
}

impl ProbeView {
    /// Creates a new Loading view.
    pub fn new() -> Self {
        // Create the regex.
        let hex = Regex::new(r"^0[xX][0-9A-Fa-f]{0,8}$").expect("Pre validated regex creation failed.");
        let bin = Regex::new(r"^0[bB][0-1]{0,32}$").expect("Pre validated regex creation failed.");
        let oct = Regex::new(r"^0[oO][0-7]{0,14}$").expect("Pre validated regex creation failed.");
        let dec = Regex::new(r"^[\d]{1,12}$").expect("Pre validated regex creation failed.");

        let probes = Probe::list_all();

        let probenames = probes.iter().map(|p| p.identifier.clone()).collect();

        ProbeView {
            state: state::State::new(),
            projects: Vec::new(),
            selproject: None,
            probes,
            probenames,
            selprobe: None,
            regex: [hex, bin, oct, dec],
        }
    }

    /// Updates the view.
    pub fn update(&mut self, msg: ProbeMessage) -> Command<Message> {
        match msg {
            ProbeMessage::Datatype(d) => {
                self.state.seldatatype = Some(d);
                Command::none()
            },

            ProbeMessage::ReadAddressChanged(s) => {
                if self.validaddr(&s) {
                    self.state.textinput.readaddrval = s;
                }

                Command::none()
            },

            ProbeMessage::ReadRangeSAddressChanged(s) => {
                if self.validaddr(&s) {
                    self.state.textinput.saddrval = s;
                }

                Command::none()
            },

            ProbeMessage::ReadRangeEAddressChanged(s) => {
                if self.validaddr(&s) {
                    self.state.textinput.eaddrval = s;
                }

                Command::none()
            },

            _ => Command::none(),
        }
    }

    /// Validates input in the address fields.
    fn validaddr(&self, s: &String) -> bool {
        if s.len() == 0 { return true; }

        self.regex.iter()
            .any(|regex| regex.is_match(s))
    }

    /// Builds the GUI view for probe interaction.
    pub fn view(&mut self) -> Element<Message> {
        // Create top bar.
        let topbar = {
            // Create project selector.
            let project = {
                let text = Text::new("Project").size(20);
                let select = PickList::new(
                    &mut self.state.projectlist,
                    &self.projects,
                    self.selproject.clone(),
                    |s| { Message::ProjectSelected( String::from(s) ) }
                );
                //.placeholder("Select project...");

                Row::new()
                    .spacing(7)
                    .push(text)
                    .push(select)
            };

            // Create probe selector.
            let probe = {
                let text = Text::new("Probe").size(20);
                let select = PickList::new(
                    &mut self.state.probelist,
                    &self.probenames,
                    self.selprobe.clone(),
                    |s| { Message::ProjectSelected( String::from(s) ) }
                );
                //.placeholder("Select project...");

                Row::new()
                    .spacing(7)
                    .push(text)
                    .push(select)
            };

            Row::new()
                .padding(5)
                .spacing(5)
                .height(Length::Shrink)
                .width(Length::Fill)
                .push(project)
                .push(probe)
        };

        // Create command section.
        let cmdview = {
            // Create the core selector.
            let coreselect = {
                let text = Text::new("Core: ").size(14);

                Row::new()
                    .push(text)
            };

            // Create left side button commands.
            let left = {
                let position = Position::Right;

                // Create the Load button.
                let load = {
                    let key = String::from("load");

                    let button = Button::new(&mut self.state.button.load, Text::new("Load").size(14))
                        .on_press(Message::Probe( ProbeMessage::Load ))
                        .height(Length::Shrink)
                        .width(Length::Fill);

                    let tip = "Loads the current ELF or binary into the target";

                    Tooltip::new(button, tip, position)
                        .padding(5)
                        .gap(2)
                };

                // Create the Stop button.
                let stopinner = Button::new(&mut self.state.button.stop, Text::new("Stop").size(14))
                    .on_press(Message::Probe( ProbeMessage::Stop ))
                    .height(Length::Shrink)
                    .width(Length::Fill);

                let stoptip = "Stops the execution of the current target core";

                let stop = Tooltip::new(stopinner, stoptip, position)
                    .padding(5)
                    .gap(2);

                // Create the Reset button.
                let resetinner = Button::new(&mut self.state.button.reset, Text::new("Reset").size(14))
                    .on_press(Message::Probe( ProbeMessage::Reset ))
                    .height(Length::Shrink)
                    .width(Length::Fill);

                let resettip = "Resets the current target core";

                let reset = Tooltip::new(resetinner, resettip, position)
                    .padding(5)
                    .gap(2);

                // Create the Run button.
                let runinner = Button::new(&mut self.state.button.run, Text::new("Run").size(14))
                    .on_press(Message::Probe( ProbeMessage::Run ))
                    .height(Length::Shrink)
                    .width(Length::Fill);

                let runtip = "Continues execution of the current target core";

                let run = Tooltip::new(runinner, runtip, position)
                    .padding(5)
                    .gap(2);

                // Create the Step button.
                let stepinner = Button::new(&mut self.state.button.step, Text::new("Step").size(14))
                    .on_press(Message::Probe( ProbeMessage::Step ))
                    .height(Length::Shrink)
                    .width(Length::Fill);

                let steptip = "Steps the current target core one instruction";

                let step = Tooltip::new(stepinner, steptip, position)
                    .padding(5)
                    .gap(2);

                // Create the Dump register button.
                let dumpinner = Button::new(&mut self.state.button.dump, Text::new("Dump registers").size(14))
                    .on_press(Message::Probe( ProbeMessage::Dump ))
                    .height(Length::Shrink)
                    .width(Length::Fill);

                let dumptip = "Dumps the current target core's registers";

                let dump = Tooltip::new(dumpinner, dumptip, position)
                    .padding(5)
                    .gap(2);

                Column::new()
                    .padding(5)
                    .spacing(5)
                    .height(Length::Fill)
                    .width(Length::Shrink)
                    .max_width(125)
                    .align_items(Align::Center)
                    .push(load)
                    .push(stop)
                    .push(reset)
                    .push(run)
                    .push(step)
                    .push(dump)
            };

            // Create the center commands.
            let right = {
                let position = Position::Right;

                // Create the Read button.
                let read = {
                    let inner = Button::new(&mut self.state.button.read, Text::new("Read").size(14))
                        .on_press( Message::Probe( ProbeMessage::Read ) )
                        .height(Length::Shrink)
                        .width(Length::Fill);

                    let tip = "Reads the given type from the address";

                    let button = Tooltip::new(inner, tip, position)
                        //.style(self.theme.tooltip.clone());
                        .padding(5)
                        .gap(2);

                    let col = Column::new()
                        .push(button)
                        .max_width(125)
                        .height(Length::Shrink);

                    // The picklist for the datatype.
                    let datatype = PickList::new(
                        &mut self.state.rddatatype,
                        &DATATYPES[..],
                        self.state.seldatatype.clone(),
                        |d| { Message::Probe( ProbeMessage::Datatype(d) ) }
                    )
                    .padding(4)
                    .width(Length::Shrink);

                    // The address input.
                    let address = TextInput::new(
                        &mut self.state.textinput.readaddr,
                        "Read address",
                        &self.state.textinput.readaddrval,
                        |a| { Message::Probe( ProbeMessage::ReadAddressChanged(a) ) }
                    )
                    .padding(5)
                    .size(14)
                    .width(Length::Fill)
                    .on_submit(Message::Probe( ProbeMessage::NewReadAddress ));

                    Row::new()
                        .push(col)
                        .push(datatype)
                        .push(address)
                        .height(Length::Shrink)
                        .width(Length::Fill)
                };

                // Create the Read Range button.
                let range = {
                    let inner = Button::new(&mut self.state.button.range, Text::new("Read range").size(14))
                        .on_press( Message::Probe( ProbeMessage::ReadRange ) )
                        .height(Length::Shrink)
                        .width(Length::Fill);

                    let tip = "Reads the data as bytes from the given address range";

                    let button = Tooltip::new(inner, tip, position)
                        //.style(self.theme.tooltip.clone());
                        .padding(5)
                        .gap(2);

                    let col = Column::new()
                        .push(button)
                        .max_width(125)
                        .height(Length::Shrink);

                    // The address input.
                    let saddress = TextInput::new(
                        &mut self.state.textinput.saddr,
                        "Start address",
                        &self.state.textinput.saddrval,
                        |a| { Message::Probe( ProbeMessage::ReadRangeSAddressChanged(a) ) }
                    )
                    .padding(5)
                    .size(14)
                    .width(Length::Fill)
                    .on_submit(Message::Probe( ProbeMessage::NewReadRangeSAddress ));

                    // The address input.
                    let eaddress = TextInput::new(
                        &mut self.state.textinput.eaddr,
                        "End address",
                        &self.state.textinput.eaddrval,
                        |a| { Message::Probe( ProbeMessage::ReadRangeEAddressChanged(a) ) }
                    )
                    .padding(5)
                    .size(14)
                    .width(Length::Fill)
                    .on_submit(Message::Probe( ProbeMessage::NewReadRangeEAddress ));

                    Row::new()
                        .push(col)
                        .push(saddress)
                        .push(eaddress)
                        .height(Length::Shrink)
                        .width(Length::Fill)
                };

                // Create the Read Symbol button.
                let symbol = {
                    let inner = Button::new(&mut self.state.button.symbol, Text::new("Read symbol").size(14))
                        .on_press( Message::Probe( ProbeMessage::ReadSymbol ) )
                        .height(Length::Shrink)
                        .width(Length::Fill);

                    let tip = "Reads the current value of the given symbol";

                    let button = Tooltip::new(inner, tip, position)
                        //.style(self.theme.tooltip.clone());
                        .padding(5)
                        .gap(2);

                    let col = Column::new()
                        .push(button)
                        .max_width(125)
                        .height(Length::Shrink);

                    Row::new()
                        .push(col)
                        .height(Length::Shrink)
                        .width(Length::Fill)
                };

                Column::new()
                    .padding(5)
                    .spacing(5)
                    .height(Length::Fill)
                    .width(Length::Shrink)
                    .max_width(350)
                    .align_items(Align::Center)
                    .push(read)
                    .push(range)
                    .push(symbol)
            };


            Column::new()
                .padding(5)
                .spacing(5)
                .height(Length::Shrink)
                .width(Length::Shrink)
                .push(coreselect)
                .push(
                    Row::new()
                        .padding(5)
                        .spacing(5)
                        .height(Length::Shrink)
                        .width(Length::Shrink)
                        .push(left)
                        .push(right)
                )
        };

        // Create display.
        let display = {
            // Text of the display.
            let text = Text::new("Command display").size(14);

            Container::new(text)
                .height(Length::Fill)
                .width(Length::Fill)
        };

        // Create Console / Log / Events.
        let console = {
            // Text of the display.
            let text = Text::new("Console display").size(14);

            Container::new(text)
                .height(Length::FillPortion(15))
                .width(Length::Fill)
        };


        // Create the command / display row.
        let row = Row::new()
            .padding(5)
            .spacing(5)
            .height(Length::FillPortion(80))
            .width(Length::Fill)
            .push(cmdview)
            .push(display);


        // Create the final column.
        let column = Column::new()
            .padding(5)
            .push(topbar)
            .push(row)
            .push(console);

        Container::new(column)
            .into()
    }
}
