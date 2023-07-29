//! USB device selector component.



mod event;
mod state;



use crate::gui::Message;

use event::Event;

use iced::{
    Element, Renderer,

    widget::Component,
};

use state::State;



#[derive(Clone, Copy)]
pub struct USBSelector;

impl USBSelector {
    /// Static initializer.
    pub const fn new() -> Self {
        Self
    }
}

impl Component<Message, Renderer> for USBSelector {
    type Event = Event;
    type State = State;

    fn update(&mut self, state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            Event::SetView( view ) => state.selected = view,
        }

        None
    }

    fn view(&self, state: &Self::State) -> Element<Self::Event, Renderer> {
        use iced::{
            Length,

            widget::{
                Button, Row,
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

        buttons.into()
    }
}



#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum View {
    /// Defmt device selection.
    Defmt,

    /// Probe device selection.
    Probe,
}