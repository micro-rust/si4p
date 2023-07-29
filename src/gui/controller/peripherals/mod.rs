//! Controller of the target's peripherals.



mod container;
mod event;
mod register;



pub use event::Event;



use crate::target::Peripheral;

use container::Container;

use std::sync::Arc;



pub struct Peripherals {
    /// List of all the peripherals of the target.
    peripherals: Vec<Container>,

    /// Collapse flag.
    show: bool,
}

impl crate::gui::common::Widget for Peripherals {
    type Event = Event;

    fn view(&self) -> iced::Element<crate::gui::Message> {
        use iced::widget::{
            Button, Column, Row, Text,
        };

        // Create the title.
        let title: iced::Element<_> = {
            // Text.
            let label = Text::new("Peripherals");

            // Collapse button.
            match self.peripherals.len() {
                0 => label.into(),
                _ => {
                    // Create the button.
                    let button = match self.show {
                        false => Button::new("+").on_press( Event::ShowPeripheralList(true).into() ),
                        true => Button::new("-").on_press( Event::ShowPeripheralList(false).into() ),
                    };

                    Row::new()
                        .push(label)
                        .push(button)
                        .into()
                }
            }
        };

        // Push into a column.
        let col = Column::new()
            .push(title);

        // Early return.
        if !self.show { return col.into(); }

        // Get all the peripherals in a column.
        let all = self.peripherals.iter()
            .fold(col, |col, per| col.push(per.view()));

        all.into()

        /*
        // Create the scrollable properties.
        let properties = Properties::new()
            .scroller_width(10)
            .width(5);

        // Collect into a scrollable.
        Scrollable::new(all)
            .vertical_scroll(properties)
            .height(iced::Length::FillPortion(80))
            .width(iced::Length::Fill)
            .into()
        */
    }

    fn update(&mut self, event: Self::Event) -> iced::Command<crate::gui::Message> {
        match event {
            // Change the visibility state of the peripheral list.
            Event::ShowPeripheralList(show) => self.show = show,

            // Change the visibility state of a peripheral.
            Event::ShowPeripheral(idx, show) => if self.peripherals.len() > idx {
                self.peripherals[idx].show(show);
            },
        }

        iced::Command::none()
    }
}

impl Peripherals {
    /// Creates a new empty peripherals GUI.
    pub fn new() -> Self {
        Self { peripherals: Vec::new(), show: false, }
    }

    /// Creates a new peripherals GUI from the given list.
    pub fn create(list: Vec<Arc<Peripheral>>) -> Self {
        // Parse all the peripherals.
        let peripherals = list.into_iter()
            .enumerate()
            .map(|(i, peripheral)| Container::create(peripheral, i))
            .collect();

        Self { peripherals, show: false, }
    }
}
