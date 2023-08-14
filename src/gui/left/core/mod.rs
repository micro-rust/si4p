//! View of a single core.



mod event;
mod state;



use crate::{
    gui::Message,

    target::{
        CoreInformation, CoreRegister, core::RegisterType,
    },

    usb::Command,
};

use event::Event;

use iced::{
    Element, Renderer, widget::Component,
};

use state::State;

use std::sync::Arc;

use tokio::sync::RwLock;



#[derive(Clone, Debug)]
pub(super) struct CoreView {
    /// Reference to the data of this view.
    pub(super) core: Arc<RwLock<CoreInformation>>,

    /// Index of the core view.
    pub(super) index: usize,
}

impl CoreView {
    /// Initializer.
    pub(super) fn new(core: Arc<RwLock<CoreInformation>>, index: usize) -> Self {
        Self { core, index, }
    }

    /// Builds the view of a register list.
    fn registers(&self, registers: &Vec<CoreRegister>, sub: &str, show: bool, fpu: bool) -> iced::Element<Event> {
        use iced::{
            Length,

            alignment::Horizontal,

            widget::{
                Button, Column, Container, Row, Text,
            },
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
            let event = if fpu { Event::ShowFPURegisters(!show) }
                else { Event::ShowCoreRegisters(!show) };

            // Create the collapse button.
            let button = {
                // Create the button.
                let collapse = Button::new( if show { "-" } else { "+" } )
                    .on_press( event.into() );

                // Push into a container for style.
                Container::new( collapse )
                    .align_x( Horizontal::Right )
                    .width( Length::Fill )
            };

            Row::new()
                .width( Length::Fill )
                .push(label)
                .push(button)
        };

        // Create the column for padding.
        let col = Column::new()
            .padding( 5 )
            .push( title );

        // If it doesn't show anything, return early.
        if !show { return col.into(); }

        // Create the list of registers.
        registers.iter()
            .map(|register| {
                // Creates the register name.
                let label = Text::new( register.name.clone() );

                // Create the formatting width.
                let width = register.bytes * 2;

                // Creates the value text.
                let value = match register.data {
                    RegisterType::FloatingPoint( f ) => Text::new( format!("{}", f) ),
                    RegisterType::Unsigned( u ) => Text::new( format!("0x{:0width$X}", u) ),
                };

                Column::new()
                    .push( label )
                    .push( value )
            })
            .fold( col, |column, register| column.push( register ))
            .into()
    }
}

impl Component<Message, Renderer> for CoreView {
    // No internal state (for now).
    type State = State;

    // No internal events (for now).
    type Event = Event;

    fn update(&mut self, state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            // Change the show state.
            Event::ShowCoreRegisters( show ) => state.core = show,
            Event::ShowFPURegisters( show )  => state.fpu  = show,

            // Emit a USB command.
            Event::Halt  => return Some( Command::CoreHalt( self.index ).into()  ),
            Event::Run   => return Some( Command::CoreRun( self.index ).into()   ),
            Event::Reset => return Some( Command::CoreReset( self.index ).into() ),
        }

        None
    }

    fn view(&self, state: &Self::State) -> Element<'_, Self::Event, Renderer> {
        use iced::widget::{
            Button, Column, Row, Text,
        };

        // Get read access to the core information.
        let core = self.core.blocking_read();

        // Create the title.
        let title = {
            // Create the core number.
            let label = Text::new( format!("Core {}", self.index) );

            // Architecture of the core.
            let arch = Text::new( format!("{:?} - {:?}", core.architecture, core.coretype) );

            // Status of the core.
            let status = Text::new( format!("{:?}", core.status) );

            Column::new()
                .push( label )
                .push( arch )
                .push( status )
        };

        // Create the buttons.
        let buttons = {
            // Create the run button.
            let run = Button::new( "Run" )
                .on_press( Event::Run );

            // Create the halt button.
            let halt = Button::new( "Halt" )
                .on_press( Event::Halt );

            Row::new()
                .push( run )
                .push( halt )
        };

        // Create the list of registers.
        let cregs = self.registers( &core.cregs, "Core", state.core, false );
        let fregs = self.registers( &core.fregs, "FPU" , state.fpu , true  );

        Column::new()
            .push( title )
            .push( buttons )
            .push( cregs )
            .push( fregs )
            .into()
    }
}
