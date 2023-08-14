//! Internal widget to view the entries.



use crate::{
    common::{
        Entry, Level, Source,
    },
    gui::Message,
};

use iced::{
    Element, Length, Rectangle, Renderer, Size, Theme,

    advanced::{
        Clipboard, Shell,

        layout::{
            Layout, Limits, Node,
        },

        renderer::Style,

        widget::{
            Tree, Widget,
        },
    },

    event::{
        Event, Status,
    },

    mouse::Cursor,
};

use std::sync::Arc;

use tokio::sync::RwLock;



#[derive(Clone, Debug)]
pub struct Entries {
    /// List of all the entries of the console and the selected indices.
    pub(super) entries: Arc<RwLock<(Vec<Entry>, Vec<usize>)>>,

    /// Size of the text inside the console.
    pub(super) fontsize: f32,

    /// Level filter.
    pub(super) level: Level,

    /// Source filter.
    pub(super) source: Source,
}

impl Entries {
    /// Initializer for the entries widget.
    pub(super) fn new() -> Self {
        Self {
            entries : Arc::new( RwLock::new( ( Vec::new(), Vec::new() ) ) ),
            fontsize: 14.0,
            level: Level::Info,
            source: Source::All,
        }
    }

    /// Rebuilds the list of entries to display.
    /// Filters the index of all the entries that match the currently applied
    /// filters. This list will be used to select which entries to display.
    pub(super) fn rebuild(&mut self) {
        use std::ops::DerefMut;

        // Get a lock on the entries.
        let mut lock = self.entries.blocking_write();

        // Destructure the entries.
        let (entries, selected) = lock.deref_mut();

        // List of selected indices.
        let mut new = Vec::new();

        for (i, entry) in entries.iter().enumerate() {
            if entry.matches( self.level, self.source ) {
                // Add the index to the list of selected indices.
                new.push(i);
            }
        }

        // Set the selected entries.
        *selected = new;
    }

    /// Adds a new entry.
    pub(super) fn push(&mut self, entry: Entry) {
        use std::ops::DerefMut;

        // Check if the entry matches the current filter.
        let matches = entry.matches(self.level, self.source);

        // Get write access to both the entries and the selected to avoid data races.
        let mut lock = self.entries.blocking_write();

        // Destrcuture the entries and the selected.
        let (entries, selected) = lock.deref_mut();

        // Push the entry.
        entries.push( entry );

        // Select the entry for display if it matched the filters.
        if matches {
            selected.push( entries.len() - 1 );
        }
    }

    /// Calculates the bounds for the given entry.
    fn textheight(&self, renderer: &Renderer, limits: Size, entry: &Entry) -> f32 {
        use iced::{
            Font,
            advanced::text::{
                Renderer as TextRenderer,
                LineHeight, Shaping,
            },
        };

        // Measure the expected size of the text.
        let measured = renderer.measure(
            entry.text(),
            self.fontsize,
            LineHeight::Relative(1.2),
            Font::MONOSPACE,
            limits,
            Shaping::Advanced
        );

        // Calculate the bounds's height.
        measured.height + 10.0
    }

    /// Calculates the bounds on the viewport and layout as used by the widget.
    fn viewbounds(&self, layout: &Layout<'_>, viewport: &Rectangle) -> ([[f32; 2]; 2], Rectangle, Size) {
        // Get the viewport coordinates.
        let vxcoord = [ viewport.x, viewport.x + viewport.width  ];
        let vycoord = [ viewport.y, viewport.y + viewport.height ];

        // Create the bounds of each element.
        let bounds = layout.bounds();

        // Get the layout limits.
        let limits = {
            // Get the size.
            let mut size = bounds.size();

            // Reduce the width of the text section.
            size.width -= (11.0 * self.fontsize) + 75.0;

            size
        };

        ([vxcoord, vycoord], bounds, limits)
    }
}

impl Widget<Message, Renderer> for Entries {
    #[inline]
    fn height(&self) -> Length {
        Length::Fill
    }

    #[inline]
    fn width(&self) -> Length {
        Length::Fill
    }

    fn on_event(&mut self, _: &mut Tree, event: Event, layout: Layout<'_>, cursor: Cursor, renderer: &Renderer, _: &mut dyn Clipboard, shell: &mut Shell<'_, Message>, viewport: &Rectangle) -> Status {
        use iced::mouse::Event as MouseEvent;

        use std::ops::Deref;

        match event {
            Event::Mouse( MouseEvent::ButtonPressed( _ ) ) if cursor.is_over(*viewport) => {
                // Get the necessary view bounds.
                let ([_, vycoord], mut bounds, limits) = self.viewbounds(&layout, viewport);

                // Get a read lock on the entries.
                let lock = self.entries.blocking_read();

                // Destructure the lists.
                let (entries, selected) = lock.deref();

                for index in selected {
                    // Get the selected entry.
                    let entry = entries.get( *index ).expect( "Console entries and selected entries list are desynchronized" );

                    // Modify the bounds with the text size.
                    bounds.height = self.textheight(renderer, limits, entry);

                    // If the element bottom does not reach the viewport, skip it.
                    if (bounds.y + bounds.height) <= vycoord[0] {
                        // Increment the element Y position.
                        bounds.y += bounds.height;

                        // Skip this element.
                        continue;
                    }

                    // If the element base exceeds the viewport, break the loop.
                    if bounds.y >= vycoord[1] { break; }

                    // Check if the cursor is over the element.
                    if cursor.is_over(bounds) {
                        // Emit the show message if the message is a target DEFMT message.
                        //if let Some(line) = entry.line() {
                            shell.publish( Message::None );
                            //shell.publish( Message::ShowCodeLine( line ) );
                        //}

                        return Status::Captured;
                    }

                    // Increment the element Y position.
                    bounds.y += bounds.height;
                }

                Status::Ignored
            },

            _ => Status::Ignored
        }
    }

    fn draw(&self, _: &Tree, renderer: &mut Renderer, _: &Theme, _: &Style, layout: Layout<'_>, cursor: Cursor, viewport: &Rectangle) {
        use iced::{
            Background, Color, Font,

            advanced::{
                renderer::{
                    Renderer, Quad,
                },

                text::{
                    LineHeight, Renderer as TextRenderer,
                    Shaping, Text,
                },
            },

            alignment::{
                Horizontal, Vertical,
            },
        };

        use std::ops::Deref;

        // Get the necessary view bounds.
        let ([_, vycoord], mut ebounds, limits) = self.viewbounds(&layout, viewport);

        // Get a read lock on the entries.
        let lock = self.entries.blocking_read();

        // Destructure the lists.
        let (entries, selected) = lock.deref();

        for index in selected {
            // Get the selected entry.
            // Safe to unwrap because the lock to both selected and entries is
            // taken at the same time.
            let entry = entries.get( *index ).unwrap();

            // Modify the bounds with the text size.
            ebounds.height = self.textheight(renderer, limits, entry);

            // If the element bottom does not reach the viewport, skip it.
            if (ebounds.y + ebounds.height) <= vycoord[0] {
                // Increment the element Y position.
                ebounds.y += ebounds.height;

                // Skip this element.
                continue;
            }

            // If the element base exceeds the viewport, break the loop.
            if ebounds.y >= vycoord[1] { break; }

            // Fill the rectangle to mark it if it's under the cursor.
            if cursor.is_over(ebounds) {
                renderer.fill_quad(
                    Quad {
                        bounds: ebounds,
                        border_radius: Default::default(),
                        border_width: 1.0,
                        border_color: Color::from_rgb(0.6, 0.2, 0.6),
                    },
                    Background::Color( Color::from_rgb(0.2, 0.6, 0.2) ),
                );
            }

            // Create the text bounds with padding.
            let mut bounds = Rectangle {
                x: ebounds.x + 20.0,
                y: ebounds.y + 5.0,
                width: 6.0 * self.fontsize,
                height: ebounds.height - 10.0,
            };

            renderer.fill_text(Text {
                content: entry.source().display(),
                bounds,
                size: self.fontsize,
                color: Color::from_rgb(1.0, 1.0, 1.0),
                line_height: LineHeight::Relative(1.2),
                font: Font::MONOSPACE,
                horizontal_alignment: Horizontal::Left,
                vertical_alignment: Vertical::Top,
                shaping: Shaping::Basic,
            });

            // Render the level text.
            bounds.x += bounds.width + 20.0;
            bounds.width = 5.0 * self.fontsize;

            renderer.fill_text(Text {
                content: entry.level().display(),
                bounds,
                size: self.fontsize,
                color: Color::from_rgb(0.9, 0.0, 0.9),
                line_height: LineHeight::Relative(1.2),
                font: Font::MONOSPACE,
                horizontal_alignment: Horizontal::Left,
                vertical_alignment: Vertical::Top,
                shaping: Shaping::Basic,
            });

            // Render the message text.
            bounds.x += bounds.width + 20.0;
            bounds.width = ebounds.width - (11.0 * self.fontsize) - 75.0;

            renderer.fill_text(Text {
                content: entry.text(),
                bounds,
                size: self.fontsize,
                color: Color::from_rgb(0.0, 0.9, 0.9),
                line_height: LineHeight::Relative(1.2),
                font: Font::MONOSPACE,
                horizontal_alignment: Horizontal::Left,
                vertical_alignment: Vertical::Top,
                shaping: Shaping::Advanced,
            });

            // Increment the element Y position.
            ebounds.y += ebounds.height;
        }
    }

    #[inline]
    fn layout(&self, renderer: &Renderer, limits: &Limits) -> Node {
        use std::ops::Deref;

        // Get a reading lock to the entries.
        let lock = self.entries.blocking_read();

        // Destructure both lists.
        let (entries, selected) = lock.deref();

        // Start adding height.
        let mut height = 0.0;

        for index in selected {
            // Get the selected entry.
            let entry = entries.get(*index).expect( "Console entries and selected entries list are desynchronized" );

            // Add the height of this line.
            height += self.textheight(renderer, limits.max(), entry) + 1.0;
        }

        Node::new( Size { height, width: limits.max().width, } )
    }
}

impl<'a> From<Entries> for Element<'a, Message, Renderer> {
    fn from(widget: Entries) -> Self {
        Element::new( widget )
    }
}
