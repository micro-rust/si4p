//! Left sidebar UI component.



//mod core;
//mod peripherals;
mod event;



pub use event::Event;

use iced::{
    Command, Element,
    widget::pane_grid::State as PaneGridState,
};



pub struct LeftSidebar {
    /// Pane grid structure of the left sidebar.
    panes: PaneGridState<View>,
}

impl crate::gui::common::Widget for LeftSidebar {
    type Event = Event;

    fn update(&mut self, event: Self::Event) -> Command<super::Message> {
        match event {
            // Process a resize event.
            Event::PaneGridResize( resize ) => self.panes.resize(&resize.split, resize.ratio.clamp(0.25, 0.75)),
        }

        Command::none()
    }

    fn view(&self) -> Element<super::Message> {
        use iced::{
            Length,

            widget::{
                PaneGrid, Text,

                pane_grid::{
                    Content, TitleBar,
                },
            },
        };

        // Create the panes
        let panes = PaneGrid::new(&self.panes, |_, view, _| {
                match view {
                    View::Cores => {
                        // Create the title bar.
                        let title_bar = TitleBar::new( Text::new("Cores") );

                        let body = Text::new( "SOMETHING" );

                        Content::new(body)
                            .title_bar(title_bar)
                    },

                    View::Peripherals => {
                        // Create the title bar.
                        let title_bar = TitleBar::new( Text::new("Peripherals") );

                        let body = Text::new( "SOMETHING" );

                        Content::new(body)
                            .title_bar(title_bar)
                    },
                }
            })
            .on_resize(10, |event| Event::PaneGridResize(event).into());

        panes
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
    }
}

impl LeftSidebar {
    /// Creates the left sidebar.
    pub fn new() -> Self {
        use iced::widget::pane_grid::{
            Axis, Configuration,
        };

        // Create the pane structure.
        let config = Configuration::Split {
            axis: Axis::Horizontal,
            ratio: 0.5,
            a: Box::new( Configuration::Pane( View::Peripherals ) ),
            b: Box::new( Configuration::Pane( View::Cores       ) ),
        };

        let panes = PaneGridState::with_configuration(config);

        Self {
            panes,
        }
    }
}




#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum View {
    /// UI view of the cores of the target.
    Cores,

    /// UI view of the peripherals of the target.
    Peripherals,
}