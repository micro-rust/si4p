//! `Tooltip` widget theme.



use iced::{
    Color,
    container::{
        Style, StyleSheet
    },
};

use serde::{ Deserialize, Serialize };



#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Theme {
    /// Text color.
    textcolor: (u8, u8, u8, f32),

    /// Background color.
    background: (u8, u8, u8, f32),

    /// Theme of the tooltip border.
    border: super::border::Theme,
}

impl StyleSheet for Theme {
    fn style(&self) -> Style {
        // Create the background color.
        let (r, g, b, a) = self.background;
        let background = Color::from_rgba8(r, g, b, a).into();

        // Create border color.
        let (r, g, b, a) = self.border.color;
        let border_color = Color::from_rgba8(r, g, b, a);

        // Create text color.
        let (r, g, b, a) = self.textcolor;
        let text_color = Color::from_rgba8(r, g, b, a);

        Style {
            text_color: Some( text_color ),
            border_color,
            background: Some( background ),
            border_radius: self.border.radius,
            border_width: self.border.width,
        }
    }
}
