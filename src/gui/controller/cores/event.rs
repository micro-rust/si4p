//! Events of the peripherals view.



use crate::gui::Message;



#[derive(Clone, Debug)]
pub enum Event {
    /// Sets the show state of the peripheral list.
    ShowCoreList(bool),

    /// Shows the given core's core registers.
    ShowCoreRegisters(usize, bool),

    /// Shows the given core's FPU registers.
    ShowFPURegisters(usize, bool),
}

impl Into<Message> for Event {
    fn into(self) -> Message {
        Message::Controller( super::super::Event::Core( self ) )
    }
}
