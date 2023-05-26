//! The messages of the console widget.



use super::{
    Entry, Level, Source,

    super::Message as Event,
};



#[derive(Clone, Debug)]
pub enum Message {
    /// A new entry.
    New( Entry ),

    /// The level filter changed.
    FilterLevel( Level ),

    /// The source filter changed.
    FilterSource( Source ),
}

impl Into<Event> for Message {
    fn into(self) -> Event {
        Event::Console( self )
    }
}
