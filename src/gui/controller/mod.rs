//! Controller of the target.
//! Provides an interface to interact with the target, its cores, memory and peripherals.



mod cores;
mod event;
mod peripherals;



pub use event::Event;



use cores::Cores;

use crate::{
    gui::common::Widget,
    target::Peripheral,
};

use peripherals::Peripherals;

use std::sync::Arc;



pub struct Controller {
    /// A controller of the target's cores.
    cores: Cores,

    /// A controller of the target's peripherals.
    peripherals: Peripherals,
}

impl Widget for Controller {
    type Event = Event;

    fn view(&self) -> iced::Element<crate::gui::Message> {
        use iced::widget::{
            Column, scrollable::{
                Direction, Properties, Scrollable,
            },
        };

        // Build the content.
        let content = Column::new()
            .padding(10)
            .push(self.peripherals.view())
            .push(self.cores.view())
            .width(iced::Length::Fill);

        // Create the scrollable properties.
        let properties = Properties::new()
            .scroller_width(10)
            .width(5);

        // Collect into a scrollable.
        Scrollable::new(content)
            .direction( Direction::Both { vertical: properties, horizontal: properties } )
            .width(iced::Length::Fill)
            .into()
    }

    #[allow(unreachable_code)]
    fn update(&mut self, event: Self::Event) -> iced::Command<crate::gui::Message> {
        match event {
            Event::Peripheral( event ) => return self.peripherals.update( event ),
            Event::Core( event ) => return self.cores.update( event ),
        }

        iced::Command::none()
    }
}

impl Controller {
    /// Creates a new controller GUI.
    pub fn new() -> Self {
        Self {
            cores: Cores::new(),
            peripherals: Peripherals::new(),
        }
    }

    /// Sets a new target.
    pub fn target(&mut self, peripherals: Vec<Arc<Peripheral>>) {
        // Update the list of peripherals.
        self.peripherals = Peripherals::create( peripherals );
    }

    /// Rebuild the information.
    pub fn rebuild(&mut self) {
        self.cores.rebuild();
    }
}
