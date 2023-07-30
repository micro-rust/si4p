//! GUI application messages.



use crate::target::Peripheral;

use probe_rs::DebugProbeInfo;

use std::{
    path::PathBuf,
    sync::Arc,
};


#[derive(Clone, Debug)]
pub enum Message {
    /// An internal message for the console.
    Console( super::console::Message ),

    /// A new console entry.
    ConsoleEntry( crate::common::Entry ),

    /// Events of the left sidebar.
    Left( super::left::Event ),

    /// Events of the right sidebar.
    Right( super::right::Event ),

    
    Controller( super::controller::Event ),

    /// A new USB command was emitted.
    USB( crate::usb::Command ),

    /// Indicates a request for a USB defmt connection.
    DefmtConnect( usize, u8, u8, u8, u8, (u8, u8) ),

    /// Select defmt file.
    SelectELF( Option<PathBuf> ),

    /// Flashes the current ELF file.
    FlashELF,

    /// Loads the given defmt file.
    LoadELF( PathBuf ),

    /// A new defmt file was picked.
    NewELF( Arc<[u8]>, PathBuf ),

    /// A new SVD file was picked.
    NewSVD( Vec<Arc<Peripheral>>, PathBuf, ),

    /// A library rebuild is needed.
    LibraryRebuild,

    NewDebugSession,

    /// Selects the given target.
    SetDebugTarget( String ),

    /// Deselects the current target.
    ClearDebugTarget,

    /// Indicates a change in the expansion status of a USB config display.
    USBConfigExpanded( (u16, u16), u8, bool ),

    /// A main view pane grid was resized.
    PaneGridResize( iced::widget::pane_grid::ResizeEvent ),

    /// The USB thread crashed.
    USBThreadCrashed,

    /// Rebuild the list of USBs.
    USBTreeRebuild,

    /// A new debug probe is open.
    SetDebugProbe( DebugProbeInfo ),

    /// The debug probe was removed.
    ClearDebugProbe,

    /// A message of the USB configuration component.
    //USBConfiguration( super::usbcfg::Message ),

    /// No message emitted.
    /// Placeholder for functions with mandatory return message.
    None,
}


impl core::convert::Into<Message> for () {
    fn into(self) -> Message {
        Message::None
    }
}