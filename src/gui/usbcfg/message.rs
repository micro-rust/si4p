/// Messages and events of the USB configuration component.


#[derive(Clone, Debug)]
pub enum Message {
    /// A message for the `defmt` selector.
    Defmt( super::selector::ShowAction ),

    /// A message for the `probe-rs` selector.
    Probe( super::selector::ShowAction ),

    /// Changes the selected view.
    Selected( super::USBSelectorView ),

    /// The text input of the target selection changed.
    TargetTextChange( String ),
}

impl core::convert::Into<crate::gui::Message> for Message {
    fn into(self) -> crate::gui::Message {
        crate::gui::Message::USBConfiguration( self )
    }
}
