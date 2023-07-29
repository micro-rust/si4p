//! ELF selector.
//! Creates an UI element that allows the selection of the ELF being debugged.



mod event;



use crate::gui::Message;

use event::Event;

use iced::{
    Element, Renderer, widget::Component,
};

use std::path::PathBuf;



#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ELFSelector;

impl ELFSelector {
    /// Static intializer.
    pub const fn new() -> Self {
        Self
    }
}

impl Component<Message, Renderer> for ELFSelector {
    type Event = Event;
    type State = Option<PathBuf>;

    fn update(&mut self, state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            Event::Select => match &state {
                Some(path) => {
                    // Create the parent path.
                    let mut parent = path.clone();
                    parent.pop();

                    Some( Message::SelectELF( Some( parent ) ) )
                },

                None => Some( Message::SelectELF( None ) )
            },

            Event::Reload => match &state {
                Some( path ) => Some( Message::LoadELF( path.clone() ) ),

                None => None,
            },
        }
    }

    fn view(&self, state: &Self::State,) -> Element<Self::Event, Renderer> {
        use iced::{
            Length,
            widget::{
                Button, Row,
            },
        };

        // Build the select file button.
        let mut select = Button::new( "Select ELF" )
            .width( Length::FillPortion(65) )
            .on_press( Event::Select );

        // Build the reload button.
        let mut reload = Button::new( "Reload" )
            .width( Length::FillPortion(35) );

        // Enable reload button only if there is a path open.
        if let Some( path ) = &state {
            reload = reload.on_press( Event::Reload );
        }

        Row::new()
            .push( select )
            .push( reload )
            .into()
    }
}
