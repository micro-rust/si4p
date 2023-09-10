//! Internal events of binary inspection windows.



use crate::gui::Message;

use iced::widget::pane_grid::ResizeEvent;



#[derive(Clone, Debug)]
pub enum Event {
    /// Open the given section for inspection.
    OpenSection( usize ),

    /// Open the given symbol for inspection.
    OpenSymbol( usize ),

    /// A resize event of the main view panegrid.
    Resize( ResizeEvent ),

    /// A resize of the inspection pane grid.
    ResizeInspection( ResizeEvent ),

    /// The search input was modified.
    SearchInput( String ),

    /// Set the symbol function filter.
    SetSymFunctions( bool ),

    /// Set the symbol object filter.
    SetSymObjects( bool ),

    /// Set all symbols that are not objects or functions.
    SetSymOther( bool ),

    /// Opens the inspection view of the given section.
    ViewSection( usize ),

    /// Opens the inspection view of the given symbol.
    ViewSymbol( usize ),

    /// Closes the inspection view of the given section.
    CloseSection( usize ),

    /// Closes the inspection view of the given symbol.
    CloseSymbol( usize ),
}

impl Event {
    /// Converts an `Event` into a `Message` for the given window ID.
    pub const fn global(self, id: usize) -> Message {
        Message::BinaryWindow( id, self )
    }
}
