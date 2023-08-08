//! The messages of the console widget.



use super::{
    Level, Source,

    super::Message,
};



#[derive(Clone, Debug)]
pub enum Event {
    /// The level filter changed.
    FilterLevel( Level ),

    /// The source filter changed.
    FilterSource( Source ),
}

impl Into<Message> for Event {
    fn into(self) -> Message {
        Message::Console( self )
    }
}
