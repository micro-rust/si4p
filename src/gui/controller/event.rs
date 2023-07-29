//! Controller events.



#[derive(Clone, Debug)]
pub enum Event {
    /// Events of the peripherals view.
    Peripheral( super::peripherals::Event ),

    /// Events of the core view.
    Core( super::cores::Event ),
}
