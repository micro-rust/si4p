//! Theme for a background.



use iced::{
    Color,
    container::{
        Style, StyleSheet,
    },
};



#[derive(Clone, Copy, Debug, Default)]
pub struct BackgroundTheme {
    /// Color of the background.
    pub background: Color,

    /// Radius of the border.
    pub radius: f32,
}

impl StyleSheet for BackgroundTheme {
    fn style(&self) -> Style {
        Style {
            background: Some( self.background.into() ),
            border_radius: self.radius,
            ..Style::default()
        }
    }
}
