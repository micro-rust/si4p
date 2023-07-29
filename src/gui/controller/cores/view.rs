//! Core view.



pub use super::event::Event;

use crate::target::{
    CoreInformation, CoreRegister as Register, core::RegisterType,
};

use std::{sync::Arc, fmt::format};

use tokio::sync::RwLock;



pub(super) struct CoreView {
    /// Reference to the data of this view.
    pub(super) core: Arc<RwLock<CoreInformation>>,

    /// Show the list of core registers.
    pub(super) cregs: bool,

    /// Show the list of FPU registers.
    pub(super) fregs: bool,

    /// Index of the core view.
    pub(super) index: usize,
}

impl crate::gui::common::Widget for CoreView {
    type Event = ();

    fn view(&self) -> iced::Element<crate::gui::Message> {
        use iced::widget::Column;

        // Get read access.
        let core = self.core.blocking_read();

        // Create the title.
        let title = self.title(&core);

        // Create the buttons.
        let buttons = self.buttons(&core);

        // Create the views of the registers.
        let cregs = self.cregs(&core);
        let fregs = self.fregs(&core);

        Column::new()
            .push(title)
            .push(buttons)
            .push(cregs)
            .push(fregs)
            .into()
    }
}

impl CoreView {
    /// Creates a new core view.
    pub(super) fn create(core: Arc<RwLock<CoreInformation>>, index: usize) -> Self {
        Self {
            core,
            cregs: false,
            fregs: false,
            index,
        }
    }

    /// Creates the title of the core view.
    fn title(&self, core: &CoreInformation) -> iced::Element<crate::gui::Message> {
        use iced::widget::{
            Column, Text,
        };

        // Core number.
        let label = Text::new( format!("Core {}", self.index) );

        // Architecture of the core.
        let arch = Text::new( format!("{:?} - {:?}", core.architecture, core.coretype) );

        // Status of the core.
        let status = Text::new( format!("{:?}", core.status) );

        Column::new()
            .push( label )
            .push( arch )
            .push( status )
            .into()
    }

    /// Builds the row of buttons.
    fn buttons(&self, core: &CoreInformation) -> iced::Element<crate::gui::Message> {
        use crate::usb::Command;

        use iced::widget::{
            Button, Row,
        };

        // Create the run button.
        let run = Button::new( "Run" )
            .on_press( Command::CoreRun(core.index).into() );

        // Create the halt button.
        let halt = Button::new( "Halt" )
            .on_press( Command::CoreHalt(core.index).into() );

        // Create the reset button.
        let reset = Button::new( "Reset" )
            .on_press( Command::CoreReset(core.index).into() );

        Row::new()
            .push( run )
            .push( halt )
            .push( reset )
            .into()
    }

    /// Builds the core registers view.
    fn cregs(&self, core: &CoreInformation) -> iced::Element<crate::gui::Message> {
        self.regs(&core.cregs, "Core", self.cregs, false)
    }

    /// Builds the FPU registers view.
    fn fregs(&self, core: &CoreInformation) -> iced::Element<crate::gui::Message> {
        self.regs(&core.fregs, "FPU ", self.fregs, true)
    }

    /// Builds the view of a register list.
    fn regs(&self, registers: &Vec<Register>, sub: &str, show: bool, fpu: bool) -> iced::Element<crate::gui::Message> {
        use iced::widget::{
            Button, Column, Row, Text,
        };

        // If it's empty, return an empty column.
        if registers.len() == 0 {
            return Column::new().into();
        }

        // Create the title of the register view.
        let title = {
            // Create the label.
            let label = Text::new( format!("{} registers", sub) );

            // Create the event.
            let event = if fpu { Event::ShowFPURegisters(self.index, !show) }
                else { Event::ShowCoreRegisters(self.index, !show) };

            // Create the collapse button.
            let button = Button::new( if show { "-" } else { "+" } )
                .on_press( event.into() );

            Row::new()
                .push(label)
                .push(button)
        };

        // Create the column.
        let col = Column::new()
            .push(title);

        // Early return.
        if !show { return col.into() }

        // Create the list of registers.
        registers.iter()
            .map(|register| {
                // Creates the register name.
                let name = Text::new( register.name.clone() );

                // Creates the formatting width.
                let width = register.bytes * 2;

                // Creates the value text.
                let value = match register.data {
                    RegisterType::FloatingPoint(raw) => Text::new( format!("{}", raw) ),
                    RegisterType::Unsigned(raw) => Text::new( format!("0x{:0width$X}", raw) ),
                };

                Column::new()
                    .push(name)
                    .push(value)
            })
            .fold(col, |col, reg| col.push(reg))
            .into()
    }
}
