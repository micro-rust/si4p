//! Description and UI element of a bit range.



use svd_rs::bitrange::BitRange;



pub struct Field {
    /// The display name of the peripheral.
    name: String,

    /// Description of the peripheral.
    description: Option<String>,

    /// Bit range of the field.
    bitrange: BitRange,

    /// Access permissions of the field.
    access: Access,
}

impl Widget for Field {
    fn view(&self) -> iced::Element {
        
    }
}
