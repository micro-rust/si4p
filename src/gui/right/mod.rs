//! Right side view.



mod device;
pub mod elf;
mod event;
pub mod target;



pub use event::Event;



use crate::{
    gui::Message,
    library::Library,
};

use device::USBSelector;

use elf::ELFSelector;

use iced::{
    Command, Element,
    widget::pane_grid::State as PaneGridState,
};

use probe_rs::DebugProbeInfo;

use std::{
    path::PathBuf,
    sync::Arc,
};

use target::TargetSelector;


pub struct RightSidebar {
    /// ELF selection component.
    file: ELFSelector,

    /// Device selector.
    device: USBSelector,

    /// Target selection component.
    target: TargetSelector,

    /// Pane structure.
    panes: PaneGridState<View>,
}

impl crate::gui::common::Widget for RightSidebar {
    type Event = Event;

    fn update(&mut self, event: Self::Event) -> Command<Message> {
        match event {
            // Process a resize event.
            Event::PanegridResize( resize ) => self.panes.resize(&resize.split, resize.ratio),
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        use iced::{
            Length,

            widget::{
                component,
                Button, Column, PaneGrid, Text,
                pane_grid::{
                    Content, TitleBar,
                },
            },
        };

        // Create the file selection.
        let file = component( self.file.clone() );

        // Build the pane grid.
        let panegrid = PaneGrid::new(&self.panes, |_, pane, _| match pane {
                View::Device => {
                    // Create the title bar.
                    let title_bar = TitleBar::new( Text::new("Debug configuration") );

                    // Create the body.
                    let body = component( self.device.clone() );

                    Content::new( body )
                        .title_bar(title_bar)
                },

                View::Target => {
                    // Create the title bar.
                    let title_bar = TitleBar::new( Text::new("Target selector") );

                    // Create the body.
                    let body = component( self.target.clone() );

                    Content::new( body )
                        .title_bar(title_bar)
                },
            })
            .height(Length::Fill)
            .width(Length::Fill)
            .spacing(2)
            .on_resize(10, |event| Message::Right( Event::PanegridResize(event) ));
        
        Column::new()
            .push( file )
            .push( panegrid )
            .into()
    }
}

impl RightSidebar {
    /// Instantiates the right sidebar.
    pub(super) fn new(library: Arc<Library>) -> Self {
        Self {
            file: ELFSelector::new(),
            device: USBSelector::new(),
            target: TargetSelector::new(library),
            panes: Self::panegrid(),
        }
    }

    /// Rebuilds the USB tree.
    #[inline]
    pub(super) fn rebuild(&mut self) {
        self.device.rebuild();
    }

    /// Sets a valid ELF path.
    #[inline]
    pub(super) fn setpath(&mut self, path: PathBuf) {
        self.file.setpath( path );
    }

    /// Sets the current debug probe.
    #[inline]
    pub(super) fn setprobe(&mut self, info: DebugProbeInfo) {
        self.device.setprobe(info);
    }

    /// Clears the current debug probe.
    #[inline]
    pub(super) fn clearprobe(&mut self) {
        self.device.clearprobe();
    }

    /// Selects the target.
    #[inline]
    pub(super) fn select(&mut self, name: String) {
        self.target.select(name);
    }

    /// Deselects the target.
    #[inline]
    pub(super) fn deselect(&mut self) {
        self.target.deselect();
    }

    /// Builds the pane grid structure.
    fn panegrid() -> PaneGridState<View> {
        use iced::widget::pane_grid::{ Axis, Configuration, };

        // Build the configuration.
        let configuration = Configuration::Split {
            axis: Axis::Horizontal,
            ratio: 0.6667,
            a: Box::new( Configuration::Pane( View::Target ) ),
            b: Box::new( Configuration::Pane( View::Device ) ),
        };

        PaneGridState::with_configuration( configuration )
    }
}



#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum View {
    /// Device selection view.
    Device,

    /// Target selection view.
    Target,
}
