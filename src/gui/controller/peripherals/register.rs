//! GUI container of a register.



use crate::target::Register;

use std::sync::Arc;

use tokio::sync::RwLock;



pub(super) struct Container {
    /// The register this container refers to.
    register: Arc<RwLock<Register>>,

    /// Index of this register.
    index: usize,

    /// Indicates wether the register contents are shown.
    show: bool,
}

impl crate::gui::common::Widget for Container {
    type Event = ();

    fn view(&self) -> iced::Element<crate::gui::Message> {
        // Build the title.
        let title = self.title();

        // If not showing, return only the title.
        if !self.show {
            return title.into();
        }

        title.into()
    }
}

impl Container {
    /// Creates a container from a register.
    pub(super) fn create(register: Arc<RwLock<Register>>, index: usize) -> Self {
        Self { register, index, show: false, }
    }

    /// Change the show state.
    pub(super) fn show(&mut self, show: bool) {
        self.show = show;
    }

    /// Creates the title of the container.
    fn title(&self) -> iced::Element<crate::gui::Message> {
        use iced::widget::{
            Button, Column, Row, Text,
        };

        // Get access to the register information.
        let register = self.register.blocking_read();

        // Name of the register.
        let name = Text::new( register.name().clone() );

        Column::new()
            .push(name)
            .into()
    }
}
