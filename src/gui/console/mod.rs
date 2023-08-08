//! GUI console widget.



mod entries;
mod event;
mod theme;



pub use event::Event;

use iced::{
    BorderRadius, Command, Length,

    widget::{
        Column, Container, PickList,
        Row, Scrollable,

        scrollable::{
            Id, Properties,
        },
    },
};

use std::sync::Arc;

use theme::Theme;



pub use crate::{
    common::{
        Entry, Level, Source,
    },

    gui::Message,
};



pub struct Console {
    /// Unique scrollable ID.
    scrollid: Id,

    /// Theme of the console.
    theme: Theme,

    /// Internal entries widget.
    inner: entries::Entries,
}

impl Console {
    pub(super) fn new(theme: Arc<marcel::Theme>) -> Self {
        // Create the theme of the console.
        let theme = Self::theme(theme);

        Self {
            scrollid: Id::new("console"),
            theme,
            inner: entries::Entries::new()
        }
    }

    /// Updates the console.
    pub(super) fn update(&mut self, event: Event) -> Command<Message> {
        match event {
            Event::FilterLevel(level) => {
                // Set the new level filter.
                self.inner.level = level;

                // Rebuild the list.
                self.inner.rebuild();
            },

            Event::FilterSource(source) => {
                // Set the new source filter.
                self.inner.source = source;

                // Rebuild the list.
                self.inner.rebuild();
            },
        }

        Command::none()
    }

    /// Builds the view of the `Console`.
    pub(super) fn view(&self) -> iced::Element<Message> {
        // Build the topbar.
        let topbar = self.topbar();

        // Build the content.
        let content = self.content();

        // Set in a container for format.
        let container = Container::new(content)
            .height(Length::Fill)
            .width(Length::Fill);

        // Container to style it.
        let col = Column::new()
            .padding(2)
            .width(Length::Fill)
            .push(topbar)
            .push(container);

        Container::new(col)
            .height(Length::Fill)
            .width(Length::FillPortion(80))
            .style( (*self.theme.topbar).clone() )
            .into()
    }
}

impl Console {
    /// Adds a new entry.
    #[inline]
    pub(super) fn push(&mut self, entry: Entry) {
        self.inner.push(entry);
    }

    /// Creates the theme of the console
    fn theme(theme: Arc<marcel::Theme>) -> Theme {
        use marcel::{ Border, Color, Container, Picklist, picklist::{ Menu, State } };

        let background = match theme.container.get("console-background") {
            Some(data) => data.clone(),
            _ => Arc::new( Container {
                color: Arc::new( Color::new(255, 255, 255, 1.0) ),
                border: Arc::new( Border {
                    color: Arc::new( Color::new(0, 0, 0, 0.0) ), 
                    radius: BorderRadius::from(0.0),
                    width: 0.0,
                })
            }),
        };

        let topbar = match theme.container.get("console-topbar") {
            Some(data) => data.clone(),
            _ => Arc::new( Container {
                color: Arc::new( Color::new(255, 255, 255, 1.0) ),
                border: Arc::new( Border {
                    color: Arc::new( Color::new(0, 0, 0, 0.0) ),
                    radius: BorderRadius::from(0.0),
                    width: 0.0
                }),
            }),
        };

        let picklist = match theme.picklist.get("console-list") {
            Some(data) => data.clone(),
            _ => Arc::new( Picklist {
                state: [
                    Arc::new( State {
                        background: Arc::new( Color::new(96, 96, 96, 1.0) ),
                        text: Arc::new( Color::new(196, 196, 196, 1.0) ),
                        placeholder: Arc::new( Color::new(196, 196, 196, 1.0) ),
                        border: Arc::new( Border {
                            color: Arc::new( Color::new(0, 0, 0, 0.0) ),
                            radius: BorderRadius::from(0.0),
                            width: 0.0
                        }),
                        handle: Arc::new( Color::new(128, 128, 128, 1.0) ),
                    }),
                    Arc::new( State {
                        background: Arc::new( Color::new(96, 96, 96, 1.0) ),
                        text: Arc::new( Color::new(196, 196, 196, 1.0) ),
                        placeholder: Arc::new( Color::new(196, 196, 196, 1.0) ),
                        border: Arc::new( Border {
                            color: Arc::new( Color::new(0, 0, 0, 0.0) ),
                            radius: BorderRadius::from(0.0),
                            width: 0.0
                        }),
                        handle: Arc::new( Color::new(128, 128, 128, 1.0) ),
                    }),
                ],
                menu: Arc::new( Menu {
                    background: [Arc::new( Color::new(96, 96, 96, 1.0) ), Arc::new( Color::new(96, 96, 96, 1.0) )],
                    text: [Arc::new( Color::new(196, 196, 196, 1.0) ), Arc::new( Color::new(196, 196, 196, 1.0) )],
                    border: Arc::new( Border {
                        color: Arc::new( Color::new(0, 0, 0, 0.0) ),
                        radius: BorderRadius::from(0.0),
                        width: 0.0
                    }),
                }),
            }),
        };

        let text = match theme.color.get("console-text") {
            Some(data) => data.clone(),
            _ => Arc::new( Color::new(255, 255, 255, 1.0) ),
        };

        let level = [
            Arc::new( Color::new(255, 255, 255, 1.0) ),
            Arc::new( Color::new(  0,   0, 255, 1.0) ),
            Arc::new( Color::new(  0, 255,   0, 1.0) ),
            Arc::new( Color::new(128, 128,   0, 1.0) ),
            Arc::new( Color::new(255,   0,   0, 1.0) ),
        ];

        Theme { background, topbar, picklist, text, level, }
    }

    /// Builds the topbar.
    /// Creates a picklist for the level filter and a picklist for the source
    /// filter, displaying them in a row.
    fn topbar(&self) -> Container<Message> {
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
            Some( self.inner.level.clone() ),
            |l| Event::FilterLevel(l).into(),
        )
        .style( (*self.theme.picklist).clone() );

        // Create the dropdown filter for the source.
        let source = PickList::new(
            &SOURCE[..],
            Some( self.inner.source.clone() ),
            |s| Event::FilterSource(s).into(),
        )
        .style( (*self.theme.picklist).clone() );

        // Create the row and contain it for style.
        let row = Row::new()
            .spacing(20)
            .padding(2)
            .push(level)
            .push(source);

        Container::new(row)
            .style( (*self.theme.topbar).clone() )
    }

    /// Builds the entries' content.
    /// Takes all the entries selected by the current filter and displays their
    /// information in a single row within a scrollable section.
    fn content(&self) -> Scrollable<Message> {
        use iced::widget::scrollable::Direction;

        // Build the scrollable properties.
        let properties = Properties::new()
            .margin(4)
            .scroller_width(10)
            .width(5);

        // Container to style it.
        let container = Container::new(self.inner.clone())
            .style( (*self.theme.topbar).clone() )
            .width(Length::Fill);

        Scrollable::new(container)
            .direction( Direction::Vertical( properties ) )
            .height(Length::Fill)
            .width(Length::Fill)
            .id(self.scrollid.clone())
    }
}
