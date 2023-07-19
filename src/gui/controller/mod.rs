//! Controller of the target.
//! Provides an interface to interact with the target, its cores, memory and peripherals.



mod event;
mod peripherals;



pub use event::Event;



use crate::{
    gui::common::Widget,
    target::Peripheral,
};

use peripherals::Peripherals;

use std::sync::Arc;



pub struct Controller {
    /// A controller of the target's peripherals.
    peripherals: Peripherals,
}

impl Widget for Controller {
    type Event = Event;

    fn view(&self) -> iced::Element<crate::gui::Message> {
        use iced::widget::Column;

        Column::new()
            .padding(10)
            .push(self.peripherals.view())
            .width(iced::Length::Fill)
            .into()
    }

    fn update(&mut self, event: Self::Event) -> iced::Command<crate::gui::Message> {
        match event {
            Event::Peripheral( event ) => return self.peripherals.update( event ),
        }

        iced::Command::none()
    }
}

impl Controller {
    /// Creates a new controller GUI.
    pub fn new() -> Self {
        Self {
            peripherals: Peripherals::new(),
        }
    }

    /// Sets a new target.
    pub fn target(&mut self, peripherals: Vec<Arc<Peripheral>>) {
        // Update the list of peripherals.
        self.peripherals = Peripherals::create( peripherals );
    }
}
