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
        use iced::widget::{
            Button, Column, Row, Text,
        };

        // Get access to the register information.
        let register = self.register.blocking_read();

        // Name of the register.
        let name = Text::new( register.name().clone() );

        // Create the edit button.
        let edit = Button::new("Edit");

        // Check if the register can be read without side effects.
        let row = match register.readaction() {
            Some(action) => {
                // Create the row.
                Row::new()
                    .push( name )
                    .push( edit )
            },

            _ => {
                // Create the read action.
                let read = Button::new( "Read" );

                // Create the row.
                Row::new()
                    .push( name )
                    .push( read )
                    .push( edit )
            },
        };

        // Create the value.
        let value = Text::new( format!("0x{:08X}", register.raw()) );

        Column::new()
            .push(row)
            .push(value)
            .into()
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
}
