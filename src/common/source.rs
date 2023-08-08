//! Indicates the source of the entry.



#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Source {
    /// All message sources. Use only in filters.
    All,

    /// The entry was emitted by the host.
    Host,

    /// The entry was emitted by the target.
    Target,
}

impl Source {
    /// How to display the source in the console.
    pub fn display(&self) -> &'static str {
        match self {
            Source::All    => " ALL  ",
            Source::Host   => " HOST ",
            Source::Target => "TARGET",
        }
    }
}

impl core::fmt::Display for Source {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str(self.display())
    }
}
