//! GUI console widget.



mod message;



use iced::{
    Command, Length,

    widget::{
        Column, Container, PickList,
        Row, Scrollable, Text,

        scrollable::{
            Id, Properties,
        },
    },
};



pub use crate::common::Entry;
pub use crate::common::Level;
pub use crate::common::Source;
pub use message::Message;



pub struct Console {
    /// All entries currently logged.
    entries: Vec<Entry>,

    /// List of entries to display after applying filters.
    selected: Vec<usize>,

    /// Level filter.
    level: Level,

    /// Source filter.
    source: Source,

    /// Unique scrollable ID.
    scrollid: Id,
}

impl Console {
    pub(super) fn new() -> Self {
        Self {
            entries: Vec::new(),
            selected: Vec::new(),
            level: Level::Info,
            source: Source::All,
            scrollid: Id::new("console"),
        }
    }

    /// Updates the console.
    pub(super) fn update(&mut self, message: Message) -> Command<super::Message> {
        match message {
            Message::New(entry) => {
                // Check if the entry matches the current filter.
                let matches = entry.matches(self.level, self.source);

                // Push the entry.
                self.entries.push(entry);

                // Select the entry for display if it matched the filters.
                if matches {
                    self.selected.push(self.entries.len() - 1);
                }
            },

            Message::FilterLevel(level) => {
                // Set the new level filter.
                self.level = level;

                // Rebuild the list.
                self.rebuild();
            },

            Message::FilterSource(source) => {
                // Set the new source filter.
                self.source = source;

                // Rebuild the list.
                self.rebuild();
            },
        }

        Command::none()
    }

    /// Builds the view of the `Console`.
    pub(super) fn view(&self) -> Column<super::Message> {
        // Build the topbar.
        let topbar = self.topbar();

        // Build the content.
        let content = self.content();

        // Set in a container for format.
        let container = Container::new(content)
            .height(Length::Fill)
            .width(Length::Fill);

        Column::new()
            .padding(2)
            .width(iced::Length::FillPortion(80))
            .push(topbar)
            .push(container)
            .into()
    }
}

impl Console {
    /// Builds the topbar.
    /// Creates a picklist for the level filter and a picklist for the source
    /// filter, displaying them in a row.
    fn topbar(&self) -> Row<super::Message> {
        // List of all entry levels.
        const LEVELS: [Level; 5] = [
            Level::Error, Level::Warn, Level::Info, Level::Debug, Level::Trace,
        ];

        // List of all sources.
        const SOURCE: [Source; 3] = [
            Source::All, Source::Host, Source::Target,
        ];

        // Create the dropdown filter for the level.
        let level = PickList::new(
            &LEVELS[..],
            Some( self.level.clone() ),
            |l| Message::FilterLevel(l).into(),
        );

        // Create the dropdown filter for the source.
        let source = PickList::new(
            &SOURCE[..],
            Some( self.source.clone() ),
            |s| Message::FilterSource(s).into(),
        );

        Row::new()
            .spacing(20)
            .padding(2)
            .push(level)
            .push(source)
    }

    /// Builds the entries' content.
    /// Takes all the entries selected by the current filter and displays their
    /// information in a single row within a scrollable section.
    fn content(&self) -> Scrollable<super::Message> {
        // Create the entries.
        let entries = self.selected.iter()
            .map(|i| {
                // Get the entry.
                let entry = &self.entries[*i];

                // Get the source.
                let source = Text::new( format!("[{}]", entry.source()) );

                // Get the level.
                let level = Text::new( format!("{}", entry.level()) );

                // Get the message.
                let text = Text::new( entry.text() );

                Row::new()
                    .width(Length::Fill)
                    .spacing(5)
                    .push(source)
                    .push(level)
                    .push(text)
            })
            .fold(Column::new().width(Length::Fill), |column, entry| {
                column.push(entry)
            });

        // Build the scrollable properties.
        let properties = Properties::new()
            .margin(4)
            .scroller_width(10)
            .width(5);

        Scrollable::new(entries)
            .vertical_scroll(properties)
            .height(Length::Fill)
            .width(Length::Fill)
            .id(self.scrollid.clone())
    }

    /// Rebuilds the list of entries to display.
    /// Filters the index of all the entries that match the currently applied
    /// filters. This list will be used to select which entries to display.
    fn rebuild(&mut self) {
        self.selected = self.entries.iter()
            .enumerate()
            .filter(|(_, e)| e.matches(self.level, self.source))
            .map(|(i, _)| i)
            .collect();
    }
}
