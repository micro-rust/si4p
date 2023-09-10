//! Binary inspection window.



mod event;
mod inspection;
mod search;



pub use event::Event;

use crate::gui::Message;

use std::{
    path::PathBuf,
    sync::Arc,
};

use iced::{
    Element,

    widget::pane_grid::State,
};

use micro_elf::elf::ELFObject;



pub struct BinaryWindow {
    /// Window ID.
    /// Differentiates this window from other windows of the same type.
    id: usize,

    /// This window's binary file.
    file: Arc<ELFObject<Arc<[u8]>>>,

    /// The path to this window's binary file.
    path: PathBuf,

    /// The internal state of the panegrid.
    panegrid: State<BinaryWindowComponent>,

    /// The search engine.
    search: search::SearchEngine,

    /// The inspection view.
    inspection: inspection::InspectionView,
}

impl BinaryWindow {
    /// Creates a new `BinaryWindow` instance with the given ID and file.
    pub fn new(id: usize, file: Arc<ELFObject<Arc<[u8]>>>, path: PathBuf) -> Self {
        use iced::widget::pane_grid::{
            Axis, Configuration,
        };

        // Create the panegrid configuration.
        let config = Configuration::Split {
            axis: Axis::Vertical,
            ratio: 0.2,
            a: Box::new( Configuration::Split {
                axis: Axis::Horizontal,
                ratio: 0.2,
                a: Box::new( Configuration::Pane( BinaryWindowComponent::Info   ) ),
                b: Box::new( Configuration::Pane( BinaryWindowComponent::Search ) ),
            }),
            b: Box::new( Configuration::Pane( BinaryWindowComponent::Inspection ) ),
        };

        // Create the panegrid state.
        let panegrid = State::with_configuration(config);

        // Build the search engine.
        let search = search::SearchEngine::new(id, file.clone());

        // Build the inspection view.
        let inspection = inspection::InspectionView::new(id, file.clone());

        Self {
            id,
            file,
            path,
            panegrid,
            search,
            inspection,
        }
    }

    /// Creates the view of the binary inspection window.
    pub fn view(&self) -> Element<Message> {
        use iced::{
            Length,

            widget::{
                Column, PaneGrid, Text,

                pane_grid::Content,
            },
        };

        // Create the binary path text.
        let path = Text::new( format!("Binary path: {}", self.path.display()) );

        // Create the search engine.
        //let search = self.search.view();

        // Create the tabs section.
        //let tabs = self.tabs.view();

        // Create the panegrid.
        let panegrid = PaneGrid::new(&self.panegrid, move |_, view, _| match view {
                BinaryWindowComponent::Info => {
                    // Architecture and target OS.
                    let target = Text::new( format!("{} - {}", self.file.architecture(), self.file.os() ) );
 
                    // Endianness.
                    let endian = Text::new( format!("{:?} endianness", self.file.endianness() ) );

                    // Number of programs, sections and symbols.
                    let nprogram = Text::new( format!("{} program", self.file.programs().len()) );
                    let nsection = Text::new( format!("{} sections", self.file.sections().len()) );
                    let nsymbol  = Text::new( format!("{} symbols",  self.file.symbols().len()) );

                    // Create the column.
                    let column = Column::new()
                        .height( Length::Fill )
                        .width( Length::Fill )
                        .padding( 10 )
                        .push(target)
                        .push(endian)
                        .push(nprogram)
                        .push(nsection)
                        .push(nsymbol);

                    Content::new( column )
                },

                &BinaryWindowComponent::Inspection => Content::new( self.inspection.view() ),

                &BinaryWindowComponent::Search => Content::new( self.search.view() ),
            })
            .on_resize( 12, |resize| Event::Resize(resize).global(self.id) );

        Column::new()
            .height( Length::Fill )
            .width( Length::Fill )
            .push(path)
            .push(panegrid)
            .into()
    }

    /// Updates the binary inspection window.
    pub fn update(&mut self, event: Event) {
        match event {
            // Open a section.
            Event::OpenSection( index ) => self.inspection.addsection( index ),

            // Open a symbol.
            Event::OpenSymbol( index ) => self.inspection.addsymbol( index ),

            // Resize the panegrid.
            Event::Resize( resize ) => self.panegrid.resize( &resize.split, resize.ratio.clamp(0.15, 0.85) ),

            // Resize the inspection panegrid.
            Event::ResizeInspection( resize ) => self.inspection.panegrid.resize( &resize.split, resize.ratio.clamp(0.15, 0.85) ),

            // Search input was modified.
            Event::SearchInput( string ) => self.search.input = string,

            // Toggle the symbol functions filter.
            Event::SetSymFunctions(flag) => self.search.symfunctions = flag,

            // Toggle the symbol objects filter.
            Event::SetSymObjects(flag) => self.search.symobjects = flag,

            // Toggle the symbol other filter.
            Event::SetSymOther(flag) => self.search.symother = flag,

            // Opens the given section for inspection.
            Event::ViewSection( index ) => self.inspection.opensection( index ),

            // Opens the given symbol for inspection.
            Event::ViewSymbol( index ) => self.inspection.opensymbol( index ),

            // Closes the given section.
            Event::CloseSection( index ) => self.inspection.closesection( index ),

            // Closes the given symbol.
            Event::CloseSymbol( index ) => self.inspection.closesymbol( index ),
        }
    }
}



/// All components of a binary inspection window.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BinaryWindowComponent {
    /// Information view.
    Info,

    /// Search view.
    Search,

    /// Inspection view.
    Inspection,
}
