//! Internal level representation.


#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Level {
    /// Error level.
    Error = 0,

    /// Warn level.
    Warn = 1,

    /// Info level.
    Info = 2,

    /// Debug level.
    Debug = 3,

    /// Trace level.
    Trace = 4,
}

impl Level {
    /// How to display the level in the console.
    pub(super) fn display(&self) -> &str {
        match self {
            Level::Error => " ERROR ",
            Level::Warn  => " WARN  ",
            Level::Info  => " INFO  ",
            Level::Debug => " DEBUG ",
            Level::Trace => " TRACE ",
        }
    }
}

impl core::convert::From<defmt_parser::Level> for Level {
    fn from(l: defmt_parser::Level) -> Self {
        match l {
            defmt_parser::Level::Error => Self::Error,
            defmt_parser::Level::Warn  => Self::Warn ,
            defmt_parser::Level::Info  => Self::Info ,
            defmt_parser::Level::Debug => Self::Debug,
            defmt_parser::Level::Trace => Self::Trace,
        }
    }
}

impl core::fmt::Display for Level {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str(self.display())
    }
}
