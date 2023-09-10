//! Search engine component.



use crate::gui::Message;

use iced::Element;

use micro_elf::elf::ELFObject;

use std::{
    collections::HashMap,
    sync::Arc,
};



pub(super) struct SearchEngine {
    /// The ID of the window.
    id: usize,

    /// Current search input.
    pub(super) input: String,

    /// Reference to the binary object.
    binary: Arc<ELFObject<Arc<[u8]>>>,

    /// Substring indexing of sections.
    sectionmap: HashMap<String, Vec<usize>>,

    /// Substring indexing of symbols.
    symbolmap: HashMap<String, Vec<usize>>,

    /// Flag to indicate if object symbols are shown.
    pub(super) symobjects: bool,

    /// Flag to indicate if function symbols are shown.
    pub(super) symfunctions: bool,

    /// Flag to indicate if other symbols are shown.
    pub(super) symother: bool,
}

impl SearchEngine {
    /// Builds the new search engine.
    pub(super) fn new(id: usize, binary: Arc<ELFObject<Arc<[u8]>>>) -> Self {
        // Build the section substring map.
        let sectionmap = Self::substrings( binary.sections().iter().map(|section| section.name()) );
        let symbolmap = Self::substrings( binary.symbols().iter().map(|symbol| symbol.name()) );

        Self {
            id,
            input: String::new(),
            binary,
            sectionmap,
            symbolmap,
            symobjects: true,
            symfunctions: true,
            symother: true,
        }
    }

    /// Creates the GUI view of the `SearchEngine`.
    pub(super) fn view(&self) -> Element<Message> {
        use iced::{
            Length,

            widget::{
                Checkbox, Column, Scrollable, TextInput,

                scrollable::{
                    Direction, Properties,
                },
            },
        };

        use super::Event;

        // Create the search input.
        let input = TextInput::new("Search...", &self.input)
            .on_input(|string| Event::SearchInput(string).global(self.id));

        // Create the symbol filter section.
        let selection = {
            // Create the object checkbox.
            let object = Checkbox::new("Objects", self.symobjects, |b| Event::SetSymObjects(b).global(self.id));

            // Create the function checkbox.
            let function = Checkbox::new("Functions", self.symfunctions, |b| Event::SetSymFunctions(b).global(self.id));

            // Create the object checkbox.
            let other = Checkbox::new("Other", self.symother, |b| Event::SetSymOther(b).global(self.id));

            Column::new()
                .height(Length::Shrink)
                .width(Length::Fill)
                .padding(5)
                .push(object)
                .push(function)
                .push(other)
        };

        // Create the message creation closures.
        let sectionmsg = |index: usize| Event::OpenSection(index).global(self.id);
        let symbolmsg  = |index: usize| Event::OpenSymbol(index).global(self.id);

        // Create the symbol filter function.
        let filter = |(index, symbol): & (usize, &std::sync::Arc<micro_elf::elf::data::Symbol>)| if symbol.is_object() {
            self.symobjects
        } else if symbol.is_function() {
            self.symfunctions
        } else {
            self.symother
        };

        // Get all sections and symbols.
        let allsections = self.binary.sections();
        let allsymbols = self.binary.symbols();

        // Create the search results.
        let results = match self.input.len() {
            // No input, show all sections and symbols.
            0 => {
                // Enumerate all sections.
                let mut sectionlist: Vec<(usize, &String)> = allsections
                    .iter()
                    .enumerate()
                    .skip(1)
                    .map(|(index, section)| (index, section.name()))
                    .collect();

                sectionlist.sort_by(|(_, a), (_, b)| a.cmp(b));

                // Enumerate all symbols.
                let mut symlist: Vec<(usize, &String)> = allsymbols
                    .iter()
                    .enumerate()
                    .skip(1)
                    .filter( filter )
                    .map(|(index, symbol)| (index, symbol.name()))
                    .collect();

                symlist.sort_by(|(_, a), (_, b)| a.cmp(b));

                // Create the results for all sections.
                let sections = self.results( String::from("Sections"), sectionlist.into_iter(), &sectionmsg );

                // Create the results for all symbols.
                let symbols = self.results(String::from("Symbols"), symlist.into_iter(), &symbolmsg );

                Column::new()
                    .height(Length::Shrink)
                    .width(Length::Shrink)
                    .push(sections)
                    .push(symbols)
            },

            // Show all matching sections and symbols.
            _ => {
                // This is a dummy empty list for a no match case.
                const EMPTY: [(usize, &String); 0] = [];

                // Get the list of matching sections.
                let sections = match self.sectionmap.get( &self.input ) {
                    None => self.results(String::from("Sections"), EMPTY.iter().map(|x| *x), &sectionmsg),

                    Some(list) => {
                        // Get the list of sections.
                        let mut sectionlist: Vec<(usize, &String)> = list.iter()
                            .map( |index| (*index, allsections[*index].name()) )
                            .collect();

                        sectionlist.sort_by(|(_, a), (_, b)| a.cmp(b));

                        // Build the result list.
                        self.results(String::from("Sections"), sectionlist.into_iter(), &sectionmsg )
                    },
                };

                // Get the list of matching symbols.
                let symbols = match self.symbolmap.get( &self.input ) {
                    None => self.results(String::from("Symbols"), EMPTY.iter().map(|x| *x), &symbolmsg),

                    Some(list) => {
                        // Get the list of sections.
                        let mut symlist: Vec<(usize, &String)> = list.iter()
                            .map(|index| (*index, &allsymbols[*index]))
                            .filter(|(index, symbol)| filter(&(*index, symbol)))
                            .map(|(index, symbol)| (index, symbol.name()))
                            .collect();

                        symlist.sort_by(|(_, a), (_, b)| a.cmp(b));

                        // Build the result list.
                        self.results(String::from("Symbols"), symlist.into_iter(), &symbolmsg )
                    },
                };

                Column::new()
                    .height(Length::Shrink)
                    .width(Length::Shrink)
                    .push(sections)
                    .push(symbols)
            },
        };

        // Configure the scrolling properties.
        let properties = Properties::new()
            .margin(0)
            .scroller_width(8)
            .width(5);

        // Configure the scrolling direction.
        let direction = Direction::Both {
            vertical: properties.clone(),
            horizontal: properties.clone(),
        };

        // Put the results into a scrollable.
        let scrollable = Scrollable::new( results )
            .height(Length::Fill)
            .width(Length::Shrink)
            .direction( direction );

        // Create the column.
        Column::new()
            .height( Length::Fill )
            .width( Length::Fill )
            .padding( 5 )
            .push(input)
            .push(selection)
            .push(scrollable)
            .into()
    }

    /// Creates a result list with the given names.
    fn results<'a>(&self, title: String, names: impl Iterator<Item = (usize, &'a String)>, message: &'a dyn Fn(usize) -> Message) -> Element<Message> {
        use iced::{
            Length,

            widget::{
                Button, Column, Text,
            },
        };

        // Base results column.
        let base = Column::new()
            .height(Length::Shrink)
            .width(Length::Shrink)
            .padding(10);

        // Button creation closure.
        let button = |(index, name): (usize, &String)| {
            // Create the message.
            let msg = message(index);

            Button::new( Text::new( name.clone() ) )
                .on_press( msg )
        };

        // Collect all results.
        let results = names.map( button )
            .fold( base, |column, button| column.push(button) );

        // Create the title.
        let title = Text::new( title );

        Column::new()
            .height(Length::Shrink)
            .width(Length::Shrink)
            .padding(5)
            .push(title)
            .push(results)
            .into()
    }

    /// Internal function to build the substring indexing of a list of strings.
    fn substrings<'a>(iter: impl Iterator<Item = &'a String>) -> HashMap<String, Vec<usize>> {
        // Create the empty substring map.
        let mut map = HashMap::new();

        for (index, string) in iter.enumerate() {
            // Skip empty strings.
            if string.len() == 0 {
                continue;
            }

            // Get the string as chars.
            let chars: Vec<char> = string.chars().collect();

            for j in 0..chars.len() {
                for k in (j+1)..chars.len() {
                    // Create the substring.
                    let substring = (&chars[j..=k]).iter().collect::<String>().to_lowercase();

                    // Check if the substring has been added already.
                    match map.get_mut(&substring) {
                        None => match map.insert( substring, vec![index] ) {
                            Some(_) => panic!("Binary object search engine hashmap overwrite"),
                            _ => ()
                        },

                        Some(list) => list.push( index ),
                    }
                }
            }
        }

        map
    }
}
