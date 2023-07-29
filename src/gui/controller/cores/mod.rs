//! Controller of the target's cores.



mod event;
mod view;


pub use event::Event;

use crate::target::{
    CoreInformation, CoreRegister as Register, core::RegisterType,
};

use std::sync::Arc;

use tokio::sync::RwLock;

use view::CoreView;



pub struct Cores {
    /// List of cores of the target.
    cores: Vec<CoreView>,

    /// The show state of the list of cores.
    show: bool,
}

impl crate::gui::common::Widget for Cores {
    type Event = Event;

    fn view(&self) -> iced::Element<crate::gui::Message> {
        use iced::widget::{
            Button, Column, Row, Text,
        };

        // Get a read lock into the cores.
        let corelist = crate::usb::CORES.blocking_read();

        // Create the title.
        let title: iced::Element<_> = {
            // Text.
            let label = Text::new("Cores");

            if corelist.len() == 0 {
                return label.into();
            }

            // Create the button.
            let button = match self.show {
                false => Button::new("+").on_press( Event::ShowCoreList(true).into() ),
                true => Button::new("-").on_press( Event::ShowCoreList(false).into() ),
            };

            Row::new()
                .push(label)
                .push(button)
                .into()
        };

        // Push into a column.
        let col = Column::new()
            .push(title);

        // Early return.
        if !self.show { return col.into(); }

        // Dispaly the cores.
        self.cores.iter()
            .map(|core| core.view())
            .fold(col, |col, view| col.push(view))
            .into()
    }

    fn update(&mut self, event: Self::Event) -> iced::Command<crate::gui::Message> {
        match event {
            Event::ShowCoreList(show) => self.show = show,

            Event::ShowCoreRegisters(index, show) => match self.cores.get_mut(index) {
                Some(core) => core.cregs = show,
                _ => (),
            },

            Event::ShowFPURegisters(index, show) => match self.cores.get_mut(index) {
                Some(core) => core.fregs = show,
                _ => (),
            },
        }

        iced::Command::none()
    }
}

impl Cores {
    /// Creates a new cores controller.
    pub(super) fn new() -> Self {
        Self {
            cores: Vec::new(),
            show: false,
        }
    }

    /// Rebuild the core views.
    pub(super) fn rebuild(&mut self) {
        // Get a read on the cores.
        let cores = crate::usb::CORES.blocking_read();

        // Creates the core views.
        self.cores = cores.iter()
            .enumerate()
            .map(|(index, core)| CoreView::create(core.clone(), index))
            .collect();
    }
}
