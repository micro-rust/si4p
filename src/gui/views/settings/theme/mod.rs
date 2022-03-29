//! Theme settings view.
//! GUI View that allows changing the settings of the application.


#![allow(dead_code)]
#![allow(unused_imports)]



use crate::gui::msg::{
    Message,
};

use iced::{
    Element, Column,

    button::{ self },
    text_input::{ self },
};



pub struct ThemeSettings {

}

impl ThemeSettings {
    /// Creates a new `ThemeSettings`.
    pub fn new() -> Self {
        ThemeSettings {

        }
    }

    /// Updates the `ThemeSettings` with the given message.
    pub fn update(&mut self, msg: ()) {
        match msg {
            _ => (),
        }
    }

    /// Build the GUI of the `ThemeSettings`.
    pub fn view(&mut self) -> Element<Message> {
        Column::new()
            .into()
    }
}
