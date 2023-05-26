//! GUI application messages.



#[derive(Clone, Debug)]
pub enum Message {
    /// An internal message for the console.
    Console( super::console::Message ),

    Selector( super::selector::Message ),

    /// Indicates a request for a USB defmt connection.
    DefmtConnect( (u16, u16), u8, u8, u8 ),

    /// Indicates a change in the expansion status of a USB config display.
    USBConfigExpanded( (u16, u16), u8, bool ),

    /// The USB thread crashed.
    USBThreadCrashed,

    /// No message emitted.
    /// Placeholder for functions with mandatory return message.
    None,
}
