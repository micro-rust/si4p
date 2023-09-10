//! Inspection view.



use crate::gui::Message;

use iced::{
    Element,

    widget::pane_grid::State as PaneGridState,
};

use micro_elf::elf::{
    ELFObject,

    data::{
        SectionHeader, Symbol, symbol::SymbolType,
    },
};

use std::{
    collections::HashMap,
    sync::Arc,
};




pub struct InspectionView {
    /// ID of the window.
    id: usize,

    /// The binary object.
    binary: Arc<ELFObject<Arc<[u8]>>>,

    /// All open section views.
    sections: HashMap<usize, Arc<SectionHeader>>,

    /// All open symbol views.
    symbols: HashMap<usize, Arc<Symbol>>,

    /// Internal panegrid state.
    pub(super) panegrid: PaneGridState<SelectView>,

    /// The current view.
    selected: Option<CurrentView>,
}

impl InspectionView {
    /// Creates a new `InspectionView`.
    pub(super) fn new(id: usize, binary: Arc<ELFObject<Arc<[u8]>>>) -> Self {
        use iced::widget::pane_grid::{
            Axis, Configuration,
        };

        // Build the panegrid.
        let config = Configuration::Split {
            axis: Axis::Vertical,
            ratio: 0.2,
            a: Box::new( Configuration::Split {
                axis: Axis::Horizontal,
                ratio: 0.5,
                a: Box::new( Configuration::Pane( SelectView::Sections ) ),
                b: Box::new( Configuration::Pane( SelectView::Symbols ) ),
            }),

            b: Box::new( Configuration::Pane( SelectView::Inspection ) ),
        };

        Self {
            id,
            binary,
            sections: HashMap::new(),
            symbols: HashMap::new(),
            panegrid: PaneGridState::with_configuration(config),
            selected: None,
        }
    }

    /// Builds the GUI view.
    pub(super) fn view(&self) -> Element<Message> {
        use iced::{
            Length,

            widget::{
                Column, PaneGrid, Row, Space, Text,

                pane_grid::Content,

                text::Shaping,
            },
        };

        use super::Event;

        // Create the tab selection.
        let tabs = {
            // Build the pane grid.
            let panegrid = PaneGrid::new(&self.panegrid, move |_, view, _| match view {
                    SelectView::Sections => {
                        // Open and close message closures.
                        let open = |index: usize| Event::ViewSection(index).global(self.id);
                        let close = |index: usize| Event::CloseSection(index).global(self.id);

                        // Collect the names.
                        let list = self.sections.iter()
                            .map(|(index, section)| (*index, section.name()));

                        // Build the content.
                        let content = self.select(list, &open, &close);

                        Content::new( content )
                    },

                    SelectView::Symbols => {
                        // Open and close message closures.
                        let open = |index: usize| Event::ViewSymbol(index).global(self.id);
                        let close = |index: usize| Event::CloseSymbol(index).global(self.id);

                        // Collect the names.
                        let list = self.symbols.iter()
                            .map(|(index, symbol)| (*index, symbol.name()));

                        // Build the content.
                        let content = self.select(list, &open, &close);

                        Content::new( content )
                    },

                    SelectView::Inspection => {
                        // Size format.
                        let sizeformat = |size: usize| match size {
                                  0..=1023       => format!("{} B", size),
                               1024..=1048576    => format!("{:.2} kiB", (size as f32 / 1024.0)),
                            1048576..=1073741824 => format!("{:.2} MiB", (size as f32 / 1048576.0)),
                            _                    => format!("{:.2} GiB", (size as f32 / 1073741824.0)),
                        };

                        // Check if there is a selected view.
                        match self.selected {
                            Some(view) => match view {
                                CurrentView::Section( i ) => match self.sections.get(&i) {
                                    Some( header ) => {
                                        // Create the section information.
                                        let information = {
                                            // Create the name.
                                            let name = Text::new( format!("Section \"{}\"", header.name()) );

                                            // Create the size information.
                                            let size = {
                                                // Create the file size.
                                                let file = Text::new( format!( "Size \u{2794} {}", sizeformat( usize::from( header.filesize() ) ) ) ).shaping( Shaping::Advanced );

                                                // Create the virtual address.
                                                let virt = Text::new( format!( "Virtual address \u{2794} {:X}", header.vaddr() ) ).shaping( Shaping::Advanced );

                                                Row::new()
                                                    .push(file)
                                                    .push(Space::new(Length::Fixed(30.0), Length::Shrink))
                                                    .push(virt)
                                            };

                                            Column::new()
                                                .push(name)
                                                .push(size)
                                        };

                                        // Create the data view.
                                        let data = Text::new( "Section data" );

                                        // Collect all.
                                        let all = Column::new()
                                            .push(information)
                                            .push(data);

                                        Content::new( all )
                                    },

                                    _ => Content::new( Text::new("Cannot find section header") ),
                                },

                                CurrentView::Symbol(i) => match self.symbols.get(&i) {
                                    Some( header ) => {
                                        // Create the section information.
                                        let information = {
                                            // Create the name.
                                            let name = Text::new( format!("Symbol \"{}\" [{}]", header.name(), header.kind() ) );

                                            let info = match header.kind() {
                                                SymbolType::Function => {
                                                    // Get the address.
                                                    let addr = Text::new( format!("Function address {:X}", header.value() ) );

                                                    // Get the size.
                                                    let size = Text::new( format!("Code size {}", sizeformat( usize::from( header.size() ) ) ) );

                                                    Row::new()
                                                        .push(addr)
                                                        .push(Space::new(Length::Fixed(30.0), Length::Shrink))
                                                        .push(size)
                                                },

                                                SymbolType::Object => {
                                                    // Get the address.
                                                    let addr = Text::new( format!("Object address {:X}", header.value() ) );

                                                    // Get the size.
                                                    let size = Text::new( format!("Object size {}", sizeformat( usize::from( header.size() ) ) ) );

                                                    Row::new()
                                                        .push(addr)
                                                        .push(Space::new(Length::Fixed(30.0), Length::Shrink))
                                                        .push(size)
                                                },

                                                _ => {
                                                    // Get the value.
                                                    let val = Text::new( format!("Symbol value {:X}", header.value() ) );

                                                    // Get the size.
                                                    let size = Text::new( format!("Symbol size {}", sizeformat( usize::from( header.size() ) ) ) );

                                                    // Get the binding.
                                                    let bind = Text::new( format!("Symbol binding {}", header.bind() ) );

                                                    Row::new()
                                                        .push(val)
                                                        .push(Space::new(Length::Fixed(30.0), Length::Shrink))
                                                        .push(size)
                                                        .push(Space::new(Length::Fixed(30.0), Length::Shrink))
                                                        .push(bind)
                                                },
                                            };

                                            Column::new()
                                                .push(name)
                                                .push(info)
                                        };

                                        // Create the data view.
                                        let data = Text::new( "Symbol data" );

                                        // Collect all.
                                        let all = Column::new()
                                            .push(information)
                                            .push(data);

                                        Content::new( all )
                                    },

                                    _ => Content::new( Text::new("Cannot find symbol") ),
                                },
                            },

                            _ => Content::new( Text::new("HERE BE DATA") ),
                        }
                    },
                })
                .height(Length::Fill)
                .width(Length::Fill)
                .on_resize(15, |r| Event::ResizeInspection(r).global(self.id));

            panegrid
        };

        tabs.into()
    }

    /// Internal function to create the selection list.
    fn select<'a>(&'a self, list: impl Iterator<Item = (usize, &'a String)>, open: &dyn Fn(usize) -> Message, close: &dyn Fn(usize) -> Message) -> Element<Message> {
        use iced::{
            Length,

            widget::{
                Button, Column, Row, Scrollable, Text,

                scrollable::{
                    Direction, Properties,
                },

                text::Shaping,
            },
        };

        // Configure the column.
        let column = Column::new()
            .height(Length::Shrink)
            .width(Length::Shrink)
            .padding(5);

        // Create the list of elements.
        let elements = list.map(|(index, name)| {
                // Unicode characters text.
                let osym = Text::new( "\u{2794}" ).shaping( Shaping::Advanced );
                let csym = Text::new( "\u{274C}" ).shaping( Shaping::Advanced );

                // Create the label.
                let label = Text::new( format!("{}", name) )
                    .width(Length::FillPortion(6));

                // Create the view button.
                let view = Button::new( osym.clone() ).on_press( open(index) )
                    .width(Length::FillPortion(1));

                // Create the close button.
                let close = Button::new( csym.clone() ).on_press( close(index) )
                    .width(Length::FillPortion(1));

                Row::new()
                    .push(label)
                    .push(view)
                    .push(close)
            })
            .fold(column, |column, element| column.push(element));

        // Build the scrollable properties.
        let properties = Properties::new()
            .margin(0)
            .scroller_width(8)
            .width(5);

        // Build the scrollable direction.
        let direction = Direction::Vertical( properties );

        // Build the scrollable.
        let scrollable = Scrollable::new( elements )
            .direction( direction )
            .height(Length::Fill)
            .width(Length::Fill);

        // Create the title.
        let title = Text::new( "Elements" );

        Column::new()
            .height(Length::Fill)
            .width(Length::Fill)
            .push(title)
            .push(scrollable)
            .into()
    }

    /// Adds the given section.
    pub(super) fn addsection(&mut self, index: usize) {
        self.sections.insert(index, self.binary.sections()[index].clone());
    }

    /// Adds the given symbol.
    pub(super) fn addsymbol(&mut self, index: usize) {
        self.symbols.insert(index, self.binary.symbols()[index].clone());
    }

    /// Opens the given section.
    pub(super) fn opensection(&mut self, index: usize) {
        self.selected = Some( CurrentView::Section( index ) );
    }

    /// Opens the given symbol.
    pub(super) fn opensymbol(&mut self, index: usize) {
        self.selected = Some( CurrentView::Symbol( index ) );
    }

    /// Closes the given section.
    pub(super) fn closesection(&mut self, index: usize) {
        self.sections.remove(&index);
    }

    /// Closes the given symbol.
    pub(super) fn closesymbol(&mut self, index: usize) {
        self.symbols.remove(&index);
    }
}



#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum SelectView {
    //// Section selection.
    Sections,

    //// Symbol selection.
    Symbols,

    /// Inspection view.
    Inspection,
}



#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum CurrentView {
    /// The current view is the given section.
    Section( usize ),

    /// The current view is the given symbol.
    Symbol( usize ),
}
