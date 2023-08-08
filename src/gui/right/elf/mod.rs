//! ELF selector.
//! Creates an UI element that allows the selection of the ELF being debugged.



mod event;



use crate::gui::Message;

use event::Event;

use iced::{
    Element, Renderer, widget::Component,
};

use std::path::PathBuf;



#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ELFSelector {
    /// Currently selected path.
    path: Option<PathBuf>,
}

impl Component<Message, Renderer> for ELFSelector {
    type Event = Event;
    type State = ();

    fn update(&mut self, _: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            Event::Select => match &self.path {
                Some(path) => {
                    // Create the parent path.
                    let mut parent = path.clone();
                    parent.pop();

                    Some( Message::SelectELF( Some( parent ) ) )
                },

                None => Some( Message::SelectELF( None ) )
            },

            Event::Reload => match &self.path {
                Some( path ) => Some( Message::LoadELF( path.clone() ) ),

                None => None,
            },

            Event::Flash => Some( Message::FlashELF ),
        }
    }

    fn view(&self, _: &Self::State) -> Element<Self::Event, Renderer> {
        use iced::{
            Length,
            widget::{
                Button, Column, Row, Tooltip,

                tooltip::Position,
            },
        };

        // Build the select file button.
        let select = Button::new( "Select ELF" )
            .width( Length::FillPortion(65) )
            .on_press( Event::Select );

        // Build the reload button.
        let mut reload = Button::new( "Reload" )
            .width( Length::FillPortion(35) );

        // Build the flash button.
        let mut flash = Button::new( "Flash executable" )
            .width( Length::Fill );

        // Check if there is a path chosen.
        match &self.path {
            Some(path) => {
                // Enable the buttons.
                reload = reload.on_press( Event::Reload );
                flash  = flash.on_press( Event::Flash );

                // Build the top row.
                let top = Row::new()
                    .push( select )
                    .push( reload );

                // Convert into tooltips showing the path.
                let ttflash  = Tooltip::new( flash,  path.display().to_string(), Position::Left )
                    .gap(5);

                // Build the full view.
                Column::new()
                    .push(   top   )
                    .push( ttflash )
                    .into()
            },

            None => {
                // Build the top row.
                let top = Row::new()
                    .push( select )
                    .push( reload );

                // Build the full view.
                Column::new()
                    .push(  top  )
                    .push( flash )
                    .into()
            },
        }
    }
}

impl ELFSelector {
    /// Static intializer.
    pub const fn new() -> Self {
        Self { path: None, }
    }

    /// Sets the file's path.
    #[inline]
    pub(super) fn setpath(&mut self, path: PathBuf) {
        self.path = Some(path);
    }
}
