//! Controller of the target's peripherals.



mod event;
mod register;



pub use event::Event;



use crate::target::Peripheral;

use std::sync::Arc;



pub struct Peripherals {
    /// List of all the peripherals of the target.
    peripherals: Vec<Container>,
}

impl crate::gui::common::Widget for Peripherals {
    type Event = Event;

    fn view(&self) -> iced::Element<crate::gui::Message> {
        use iced::widget::{
            Column, scrollable::{
                Properties, Scrollable,
            },
        };

        // Get all the peripherals in a column.
        let all = self.peripherals.iter()
            .fold(Column::new(), |col, per| col.push(per.view()));

        // Create the scrollable properties.
        let properties = Properties::new()
            .scroller_width(10)
            .width(5);

        // Collect into a scrollable.
        Scrollable::new(all)
            .vertical_scroll(properties)
            .width(iced::Length::Fill)
            .into()
    }

    fn update(&mut self, event: Self::Event) -> iced::Command<crate::gui::Message> {
        match event {
            // Change the visibility state.
            Event::PeripheralShow(idx, show) => if self.peripherals.len() > idx {
                self.peripherals[idx].show(show);
            },
        }

        iced::Command::none()
    }
}

impl Peripherals {
    /// Creates a new empty peripherals GUI.
    pub fn new() -> Self {
        Self { peripherals: Vec::new(), }
    }

    /// Creates a new peripherals GUI from the given list.
    pub fn create(list: Vec<Arc<Peripheral>>) -> Self {
        // Parse all the peripherals.
        let peripherals = list.into_iter()
            .enumerate()
            .map(|(i, peripheral)| Container::create(peripheral, i))
            .collect();

        Self { peripherals, }
    }
}



/// GUI container of a peripheral.
struct Container {
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
    pub(self) fn create(peripheral: Arc<Peripheral>, index: usize,) -> Self {
        // Build the list of registers.
        let registers = peripheral.registers()
            .iter()
            .enumerate()
            .map(|(index, register)| register::Container::create(register.clone(), index))
            .collect();

        Self { peripheral, index, show: false, registers, }
    }

    /// Change the show state.
    pub(self) fn show(&mut self, show: bool) {
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
            true  => Button::new( "-" ).on_press( Event::PeripheralShow(self.index, false).into() ),
            false => Button::new( "+" ).on_press( Event::PeripheralShow(self.index,  true).into() ),
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
