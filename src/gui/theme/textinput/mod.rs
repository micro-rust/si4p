//! Serializable theme for a text input.



use iced::{
    Color,
    text_input::{ Style, StyleSheet },
};

use serde::{ Deserialize, Serialize };



#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct Theme {
    /// Style of an active theme.
    /// This is also the default style.
    pub active: StateTheme,

    /// Style of a focused theme.
    pub focused: Option<StateTheme>,

    /// Style of the hovered theme.
    pub hovered: Option<StateTheme>,

    /// A set of colors for the placeholder, value and selection.
    pub colors: [(u8, u8, u8, f32); 3],
}

impl StyleSheet for Theme {
    fn active(&self) -> Style {
        self.active.into()
    }

    fn focused(&self) -> Style {
        match self.focused {
            Some(style) => style.into(),
            _ => self.active.into(),
        }
    }

    fn hovered(&self) -> Style {
        match self.hovered {
            Some(style) => style.into(),
            _ => self.active.into(),
        }
    }

    fn placeholder_color(&self) -> Color {
        let (r, g, b, a) = self.colors[0];

        Color::from_rgba8(r, g, b, a)
    }

    fn value_color(&self) -> Color {
        let (r, g, b, a) = self.colors[1];

        Color::from_rgba8(r, g, b, a)
    }

    fn selection_color(&self) -> Color {
        let (r, g, b, a) = self.colors[2];

        Color::from_rgba8(r, g, b, a)
    }
}



/// The theme for an individual state of the text input.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct StateTheme {
    /// Background color of the text input.
    pub background: (u8, u8, u8, f32),

    /// Theme of the text input border.
    pub border: super::border::Theme,
}

impl core::convert::Into<Style> for StateTheme {
    fn into(self) -> Style {
        let (r, g, b, a) = self.background;
        let background = Color::from_rgba8(r, g, b, a).into();

        let (r, g, b, a) = self.border.color;
        let border_color = Color::from_rgba8(r, g, b, a).into();

        Style {
            background,
            border_radius: self.border.radius,
            border_width: self.border.width,
            border_color,
        }
    }
}
