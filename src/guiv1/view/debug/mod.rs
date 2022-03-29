//! Debugger View.
//! GUI Element that allows the usage of debug probes.


mod state;
mod theme;


use self::state::DebugViewState;
use self::theme::DebugViewTheme;

use crate::gui::message::{ AppMessage, DebugMessage };

use iced::{
    Column, Container, Element, Row,
    Command,
    button::{ self, Button }, scrollable::{ self, Scrollable }, Text,
    Align, Length,
};

use probe::{ Command, Data, Sender, Receiver, DebugProbeInfo, Probe };


/// Container for the information and structures needed for functionality.
pub struct DebugView {
    /// Internal GUI state.
    state: DebugViewState,

    /// Theme of the Debug View.
    theme: DebugViewTheme,

    /// List of current probes.
    probelist: Vec<DebugProbeInfo>,

    /// The probe this view is currently attached to.
    probe: Option<(Sender<Command>, Receiver<Data>)>,
}

impl DebugView {
    /// Debug View initializer.
    pub fn create() -> Self {
        Self {
            state: DebugViewState::new(),
            theme: DebugViewTheme::default(),
            probelist: Probe::list_all(),
            probe: None,
        }
    }

    /// Build the GUI view.
    pub fn view(&mut self) -> Element<AppMessage> {
        // Build the view title.
        let title = Text::new("Debug").size(40);

        // Create the Probe List section.
        let probe = {
            // Generate a list of probes.
            let list = Picklist::new(
                &mut self.state.probelist,
                &self.probelist,
                self.state.probe,

            )
            .placeholder("Choose a Probe");

            // Create the label.
            let label = Text::new("Probe:").size(10);

            // Create the refresh button.
            let refresh = Button::new(&mut self.state.probereload, Text::new("Reload").size(16))
                .on_press(AppMessage::Probe(ProbeMessage::ReloadProbeList));

            Row::new()
                .width( Length::Fill )
                .height( Length::Shrink )
                .push( label )
                .push( list )
                .push( refresh )
        };


        let column = Column::new()
            .push(probe)
            .width(Length::Fill)
            .height(Length::Fill);

        Container::new(column)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
