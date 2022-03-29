//! Button theme.



use iced::{
    Color, Vector,
    button::{ StyleSheet, Style },
};

use serde::{
    Deserialize, Serialize,
};



#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct Theme {
    /// Theme for active buttons.
    pub active: StateTheme,

    /// Theme for hovered buttons.
    pub hovered: Option<StateTheme>,

    /// Theme for pressed buttons.
    pub pressed: Option<StateTheme>,

    /// Theme for disabled buttons.
    pub disabled: Option<StateTheme>,
}

impl StyleSheet for Theme {
    fn active(&self) -> Style {
        self.active.into()
    }

    fn hovered(&self) -> Style {
        match self.hovered {
            Some(s) => s.into(),
            _ => self.active.into(),
        }
    }

    fn pressed(&self) -> Style {
        match self.pressed {
            Some(s) => s.into(),
            _ => self.active.into(),
        }
    }

    fn disabled(&self) -> Style {
        match self.disabled {
            Some(s) => s.into(),
            _ => self.active.into(),
        }
    }
}



/// The theme for an individual state of the button.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct StateTheme {
    /// Background color of the button in this state.
    pub background: (u8, u8, u8, f32),

    /// Border theme of the button in this state.
    pub border: super::border::Theme,

    /// Text color in this state.
    pub textcolor: (u8, u8, u8, f32),
}

impl core::convert::Into<Style> for StateTheme {
    fn into(self) -> Style {
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
            background: Some(background),
            border_radius: self.border.radius,
            border_width: self.border.width,
            border_color,
            text_color,
            shadow_offset: Vector::new(0.0, 0.0),
        }
    }
}
