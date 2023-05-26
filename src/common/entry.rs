//! Console entry.



use super::{
    Level, Source,
};


#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Entry {
    /// The information level of the entry.
    pub(super) level: Level,

    /// The source of the entry.
    pub(super) source: Source,

    /// The text of the entry.
    pub(super) text: String,
}

impl Entry {
    /// Creates a new entry.
    pub const fn new(level: Level, source: Source, text: String) -> Self {
        Self { level, source, text, }
    }

    /// Creates an error entry.
    pub const fn error(source: Source, text: String) -> Self {
        Self { level: Level::Error, source, text, }
    }

    /// Creates a warn entry.
    pub const fn warn(source: Source, text: String) -> Self {
        Self { level: Level::Warn, source, text, }
    }

    /// Creates an info entry.
    pub const fn info(source: Source, text: String) -> Self {
        Self { level: Level::Info, source, text, }
    }

    /// Creates an debug entry.
    pub const fn debug(source: Source, text: String) -> Self {
        Self { level: Level::Debug, source, text, }
    }

    /// Creates an trace entry.
    pub const fn trace(source: Source, text: String) -> Self {
        Self { level: Level::Trace, source, text, }
    }

    /// Returns `true` if the entry matches the filter.
    pub fn matches(&self, level: Level, source: Source) -> bool {
        (self.level <= level) && ((source == Source::All) || (self.source == source))
    }

    /// Returns the `Level` of this entry.
    pub const fn level(&self) -> Level {
        self.level
    }

    /// Returns the `Source` of this entry.
    pub const fn source(&self) -> Source {
        self.source
    }

    /// Returns the text of this entry.
    pub const fn text(&self) -> &String {
        &self.text
    }

    /// Default string for USB crashes.
    pub fn usbcrash() -> Self {
        Entry::new( Level::Error, Source::Host, String::from("USB Thread crashed or dropped the channel") )
    }
}
