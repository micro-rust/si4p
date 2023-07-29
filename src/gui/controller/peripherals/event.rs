//! Events of the peripherals view.



use crate::gui::Message;



#[derive(Clone, Debug)]
pub enum Event {
    /// Sets the show state of the given peripheral.
    ShowPeripheral(usize, bool),

    /// Sets the show state of the peripheral list.
    ShowPeripheralList(bool),
}

impl Into<Message> for Event {
    fn into(self) -> Message {
        Message::Controller( super::super::Event::Peripheral( self ) )
    }
}
