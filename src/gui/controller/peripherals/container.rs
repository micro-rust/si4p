//! GUI container of a peripheral.



use crate::gui::controller::Peripheral;
use super::{ Event, register, };

use std::sync::Arc;


pub struct Container {
    /// The peripheral this container refers to.
    peripheral: Arc<Peripheral>,

    /// Index of this peripheral.
    index: usize,

    /// Indicates wether the peripheral contents are shown.
    show: bool,

    /// List of registers in this peripheral.
    registers: Vec<register::Container>,
}

impl crate::gui::common::Widget for Container {
    type Event = ();

    fn view(&self) -> iced::Element<crate::gui::Message> {
        use iced::widget::Column;

        // Build the title.
        let title = self.title();

        // If not showing, return only the title.
        if !self.show {
            return title.into();
        }

        // Build the list of registers.
        let registers = self.registers.iter()
            .fold(Column::new().padding(10), |col, reg| col.push(reg.view()));

        Column::new()
            .push(title)
            .push(registers)
            .into()
    }
}

impl Container {
    /// Creates a container from a peripheral.
    pub(super) fn create(peripheral: Arc<Peripheral>, index: usize,) -> Self {
        // Build the list of registers.
        let registers = peripheral.registers()
            .iter()
            .enumerate()
            .map(|(index, register)| register::Container::create(register.clone(), index))
            .collect();

        Self { peripheral, index, show: false, registers, }
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

        // Name of the peripheral.
        let name = Text::new( self.peripheral.name().clone() );

        // Collapse button.
        let collapse = match self.show {
            true  => Button::new( "-" ).on_press( Event::ShowPeripheral(self.index, false).into() ),
            false => Button::new( "+" ).on_press( Event::ShowPeripheral(self.index,  true).into() ),
        };

        // Join in a row.
        let row = Row::new()
            .push(name)
            .push(collapse);

        Column::new()
            .push(row)
            .into()
    }
}
