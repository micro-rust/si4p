//! Internal events of the left sidebar.



use crate::gui::Message;

use iced::widget::pane_grid::ResizeEvent;



#[derive(Clone, Debug)]
pub enum Event {
    /// The panegrid was resized.
    PaneGridResize( ResizeEvent ),
}

impl Into<Message> for Event {
    fn into(self) -> Message {
        Message::Left( self )
    }
}
