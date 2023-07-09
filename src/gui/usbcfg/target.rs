//! GUI component to select the target.



use crate::library::Library;

use std::sync::Arc;



pub(super) struct TargetSelection {
    /// The current text input.
    input: String,

    /// The currently selected target.
    selected: Option<String>,

    /// The indices of the targets that currently match the search.
    matches: Option<Vec<usize>>,

    /// A reference to the resource library.
    library: Arc<Library>,
}

impl crate::gui::common::Widget for TargetSelection {
    type Event = ();

    fn view(&self) -> iced::Element<crate::gui::Message> {
        match &self.selected {
            Some(name) => self.show(name),
            _ =>self.select(),
        }
    }

    fn update(&mut self, _: Self::Event) -> iced::Command<crate::gui::Message> {
        iced::Command::none()
    }
}

impl TargetSelection {
    /// Creates a new target selector.
    pub(super) fn new(library: Arc<Library>) -> Self {
        Self {
            input: String::new(),
            selected: None,
            matches: None,
            library,
        }
    }

    /// Updates the text input.
    pub(super) fn textinput(&mut self, new: String) {
        // Insert the new input.
        self.input = new;

        match self.input.len() {
            // Under 2 characters show all the targets.
            0..=2 => self.matches = None,

            // Everything else, perform a search.
            _ => {
                // Get the matches.
                let matches = self.library.svd.blocking_read().matches( &self.input );

                self.matches = matches;
            },
        }
    }

    /// Marks the given target as selected.
    pub(super) fn mark(&mut self, name: String) {
        // Set the input.
        self.input = name.clone();

        // Set the selected.
        self.selected = Some(name);
    }

    /// Unmarks the target.
    pub(super) fn unmark(&mut self) {
        // Unset the selected.
        self.selected = None;
    }

    /// Creates the GUI when there is no selected target.
    fn select(&self) -> iced::Element<crate::gui::Message> {
        use iced::{
            widget::{
                Button, Column, Row, TextInput,
            },
        };

        // Get a read on the SVD library.
        let svd = self.library.svd.blocking_read();

        // Create the text input.
        let input = TextInput::new("Select target...", &self.input,)
            .on_input(|new| super::Message::TargetTextChange(new).into());

        // Create the select button.
        let mut select = Button::new( "Select" );

        if svd.exists(&self.input) {
            select = select.on_press(crate::gui::Message::SelectTarget(self.input.clone()));
        }

        // Create the top row.
        let top = Row::new()
            .push(input)
            .push(select);

        // Create the possible target list.
        let list = self.possible();

        Column::new()
            .height(iced::Length::FillPortion(40))
            .push(top)
            .push(list)
            .into()
    }

    /// Creates the GUI when a target is selected.
    fn show(&self, name: &String) -> iced::Element<crate::gui::Message> {
        use iced::widget::{
            Button, Row, Text,
        };

        // Create the selected text.
        let selected = Text::new( name.clone() )
            .width( iced::Length::FillPortion(80) );

        // Create the reset button.
        let reset = Button::new( "Reset" )
            .width( iced::Length::FillPortion(20) )
            .on_press( crate::gui::Message::DeselectTarget );

        Row::new()
            .push(selected)
            .push(reset)
            .into()
    }

    /// Creates the list of possible targets that match the input.
    fn possible(&self) -> iced::Element<crate::gui::Message> {
        use iced::widget::{
            Button, Column, Row, Scrollable, Text, scrollable::Properties,
        };

        // Get the list of all targets that match the input.
        let names = {
            // Get a read on the SVD library.
            // Do this inside to force a drop at the end of the scope.
            let svd = self.library.svd.blocking_read();
        
            match &self.matches {
                Some(matches) => matches.iter()
                    .map(|index| svd.target(*index))
                    .filter(|maybe| maybe.is_some())
                    .map(|some| some.unwrap().clone())
                    .collect(),

                _ => svd.all().clone(),
            }
        };

        // Create the buttons.
        let buttons = names.chunks(2)
            .fold(Column::new().spacing(2), |col, names| {
                // Create the row.
                let mut row = Row::new()
                    .width(iced::Length::Fill);

                for name in names {
                    // Create the button.
                    let btn = Button::new( Text::new( name.clone() ) )
                        .width( iced::Length::FillPortion(1) )
                        .on_press( super::Message::TargetTextChange( name.clone() ).into() );

                    // Add the button.
                    row = row.push( btn );
                }

                // Add the row.
                col.push(row)
            });

        // Create the scrollable properties.
        let properties = Properties::new()
            .scroller_width(15)
            .width(5)
            .margin(2);

        Scrollable::new( buttons )
            .width(iced::Length::Fill)
            .into()

    }
}
