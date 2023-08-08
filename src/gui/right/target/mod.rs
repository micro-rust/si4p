//! UI component that selects the target microcontroller.



mod event;
mod state;



use crate::{
    gui::Message,
    library::Library,
    usb::Command,
};

use event::Event;

use iced::{
    Element, Renderer, widget::Component,
};

use state::State;

use std::sync::Arc;



#[derive(Clone)]
pub struct TargetSelector {
    /// Reference to the library.
    library: Arc<Library>,

    /// The currently selected target.
    selected: Option<String>,
}

impl Component<Message, Renderer> for TargetSelector {
    type Event = Event;
    type State = State;

    fn update(&mut self, state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            Event::TextChange( new ) => {
                // Insert the new input.
                state.input = new;

                // If the string is empty do not perform a search.
                if state.input.len() == 0 {
                    // Set no matches.
                    state.matches = None;

                    // Early return.
                    return None;
                }

                // Perform a search.
                let matches = self.library.svd.blocking_read().matches( &state.input );

                // Update the matches.
                state.matches = matches;

                // Do not emit a message
                None
            },

            Event::SelectTarget( target ) => {
                // Update the string.
                state.input = target.clone();

                // Emit an message to select the target.
                Some( Command::SetDebugTarget( target ).into() )
            },

            Event::DeselectTarget => {
                // Unlock the target.
                self.selected = None;

                // Emit a message to deselect the target.
                Some( Command::ClearDebugTarget.into() )
            },
        }
    }

    fn view(&self, state: &Self::State) -> Element<Self::Event, Renderer> {
        use iced::{
            Length,

            widget::{
                Button, Column, Row, Scrollable, Text, TextInput,
            }
        };

        match &self.selected {
            Some( name ) => {
                // Create the selected text.
                let selected = Text::new( name.clone() )
                    .width( Length::FillPortion(80) );

                // Create the reset button.
                let reset = Button::new( Text::new("Reset") )
                    .width( Length::FillPortion(20) )
                    .on_press( Event::DeselectTarget );

                Row::new()
                    .push( selected )
                    .push( reset )
                    .into()
            },

            None => {
                // Get a read lock on the SVD library.
                let svd = self.library.svd.blocking_read();

                // Create the top row.
                let top = {
                    // Create the text input.
                    let input = TextInput::new( "Select target...", &state.input )
                        .on_input( |new| Event::TextChange( new ) );

                    // Create the select button.
                    let mut select = Button::new( "Select" );

                    if svd.exists( &state.input ) {
                        select = select.on_press( Event::SelectTarget( state.input.clone() ) );
                    }

                    Row::new()
                        .push( input )
                        .push( select )
                };

                // Create the possible target list.
                let list = {
                    // Get the names of the selectable targets.
                    let names = match &state.matches {
                        Some( matches ) => matches.iter()
                            .map( |index| svd.target(*index) )
                            .filter( |maybe| maybe.is_some() )
                            .map( |some| some.unwrap().clone() )
                            .collect(),

                        None => svd.all().clone(),
                    };

                    // Create the buttons for all targets.
                    let buttons = names.chunks(2)
                        .fold( Column::new(), |column, names| {
                            // Create the row.
                            let mut row = Row::new()
                                .width( Length::Fill );

                            for name in names {
                                // Create the button.
                                let btn = Button::new( Text::new( name.clone() ) )
                                    .on_press( Event::TextChange( name.clone() ) )
                                    .width( Length::FillPortion(1) );

                                // Add the button.
                                row = row.push( btn );
                            }

                            // Add the row.
                            column.push( row )
                        });

                    Scrollable::new( buttons )
                        .width( Length::Fill )
                };

                Column::new()
                    .padding( 5 )
                    .push( top )
                    .push( list )
                    .into()
            },
        }
    }
}

impl TargetSelector {
    /// Static initializer.
    pub(super) const fn new(library: Arc<Library>) -> Self {
        Self { library, selected: None }
    }

    /// Selects the given target.
    #[inline]
    pub(super) fn select(&mut self, name: String) {
        self.selected = Some(name);
    }

    /// Deselects the current target.
    #[inline]
    pub(super) fn deselect(&mut self) {
        self.selected = None;
    }
}
