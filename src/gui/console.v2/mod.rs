//! GUI console widget.



mod event;
mod message;
mod theme;



use iced::{
    BorderRadius, Command, Element, Length, Renderer,

    alignment::Horizontal,

    widget::{
        Column, Component, Container, PickList,
        Row, Scrollable, Text,

        scrollable::{
            Direction, Id, Properties,
        },
    },
};

use std::sync::Arc;

use theme::Theme;



pub use crate::common::Entry;
pub use crate::common::Level;
pub use crate::common::Source;
use event::Event;
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

    /// Theme of the console.
    theme: Theme,
}

impl<'a> From<Console> for Element<'a, crate::gui::Message, Renderer> {
    fn from(console: Console) -> Element<'a, crate::gui::Message> {
        iced::widget::component( console )
    }
}

impl Component<crate::gui::Message, Renderer> for Console {
    // No state, it's all in the struct.
    type State = ();

    // The internal event type.
    type Event = Event;

    fn view(&self, _: &Self::State) -> Element<Self::Event, Renderer> {
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

    fn update(&mut self, _: &mut Self::State, event: Self::Event) -> Option<crate::gui::Message> {
        match event {
            Event::FilterSource( source ) => self.source = source,
            Event::FilterLevel( level ) => self.level = level,
        }

        None
    }
}

impl Console {
    /// Creates a new console component.
    pub(super) fn new(theme: Arc<marcel::Theme>) -> Self {
        // Create the theme of the console.
        let theme = Self::theme(theme);

        Self {
            entries: Vec::new(),
            selected: Vec::new(),
            level: Level::Info,
            source: Source::All,
            scrollid: Id::new("console"),
            theme,
        }
    }

    /// Pushes a new entry to the console.
    pub(super) fn push(&mut self, entry: Entry) {
        self.entries.push(entry)
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
                    radius: BorderRadius::from( 0.0 ),
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
                    radius: BorderRadius::from( 0.0 ),
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
                            radius: BorderRadius::from( 0.0 ),
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
                            radius: BorderRadius::from( 0.0 ),
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
                        radius: BorderRadius::from( 0.0 ),
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
    fn topbar(&self) -> Container<Event> {
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
            |l| Event::FilterLevel(l),
        )
        .style( (*self.theme.picklist).clone() );

        // Create the dropdown filter for the source.
        let source = PickList::new(
            &SOURCE[..],
            Some( self.source.clone() ),
            |s| Event::FilterSource(s),
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
    fn content(&self) -> Scrollable<Event> {
        // Create the entries.
        let entries = self.selected.iter()
            .map(|i| {
                // Get the entry.
                let entry = &self.entries[*i];

                // Get the source.
                let source = Container::new( Text::new( format!("[{}]", entry.source()) ).style( (*self.theme.text).clone() ) )
                    .align_x( Horizontal::Center )
                    .width(Length::FillPortion(8));

                // Get the level.
                let mut lvl = Text::new( format!("{}", entry.level()));

                lvl = match entry.level() {
                    Level::Trace => lvl.style( *self.theme.level[0] ),
                    Level::Debug => lvl.style( *self.theme.level[1] ),
                    Level::Info  => lvl.style( *self.theme.level[2] ),
                    Level::Warn  => lvl.style( *self.theme.level[3] ),
                    Level::Error => lvl.style( *self.theme.level[4] ),
                };

                let level = Container::new( lvl )
                    .align_x( Horizontal::Center )
                    .width(Length::FillPortion(8));

                // Get the message.
                let text = Container::new( Text::new( entry.text() ).style( (*self.theme.text).clone() ) )
                    .width(Length::FillPortion(90));

                // Container to style it.
                let row = Row::new()
                    .width(Length::Fill)
                    .spacing(5)
                    .push(source)
                    .push(level)
                    .push(text);

                Container::new(row) 
                    .style( (*self.theme.background).clone() )
                    .width(Length::Fill)
            })
            .fold(Column::new().spacing(2).width(Length::Fill), |column, entry| {
                column.push(entry)
            });

        // Build the scrollable properties.
        let properties = Properties::new()
            .margin(4)
            .scroller_width(10)
            .width(5);

        // Container to style it.
        let container = Container::new(entries)
            .style( (*self.theme.topbar).clone() )
            .width(Length::Fill);

        Scrollable::new(container)
            .direction( Direction::Vertical(properties) )
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
