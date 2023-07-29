//! Internal events of the 



use crate::gui::Message;

use iced::widget::pane_grid::ResizeEvent;



#[derive(Clone, Debug)]
pub enum Event {
    /// The panegrid was resized.
    PanegridResize( ResizeEvent ),
}

impl Into<Message> for Event {
    fn into(self) -> Message {
        Message::Right( self )
    }
}
