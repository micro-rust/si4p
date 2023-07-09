//! GUI application messages.



#[derive(Clone, Debug)]
pub enum Message {
    /// An internal message for the console.
    Console( super::console::Message ),

    /// A new USB command was emitted.
    USB( crate::usb::Command ),

    /// Indicates a request for a USB defmt connection.
    DefmtConnect( usize, u8, u8, u8, u8, (u8, u8) ),

    /// Select defmt file.
    SelectELF( Option<std::path::PathBuf> ),

    /// Loads the given defmt file.
    LoadELF( std::path::PathBuf ),

    /// A new defmt file was picked.
    NewELF( std::sync::Arc<[u8]>, std::path::PathBuf ),

    /// Indicates a change in the expansion status of a USB config display.
    USBConfigExpanded( (u16, u16), u8, bool ),

    /// A main view pane grid was resized.
    PaneGridResize( iced::widget::pane_grid::ResizeEvent ),

    /// The USB thread crashed.
    USBThreadCrashed,

    /// Rebuild the list of USBs.
    USBTreeRebuild,

    /// A message of the USB configuration component.
    USBConfiguration( super::usbcfg::Message ),

    /// No message emitted.
    /// Placeholder for functions with mandatory return message.
    None,
}


impl core::convert::Into<Message> for () {
    fn into(self) -> Message {
        Message::None
    }
}