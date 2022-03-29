//! Theme for buttons in the ELF View.


use iced::{ button, Color };


#[derive(Debug, Clone, Copy)]
pub struct ElfViewBtnTheme {
    /// Background color: Normal | Pressed.
    bg: [Color; 2],

    /// Text color: Normal | Pressed.
    text: [Color; 2],

    /// Border: Normal | Pressed.
    border: [(f32, f32, Color); 2],
}


impl ElfViewBtnTheme {
    /// Returns the text color.
    #[inline]
    pub fn textcolor(&self) -> Color {
        self.text[0]
    }
}


impl button::StyleSheet for ElfViewBtnTheme {
    fn active(&self) -> button::Style {
        button::Style {
            background: self.bg[0].into(),
            text_color: self.text[0],
            border_radius: self.border[0].0,
            border_width: self.border[0].1,
            border_color: self.border[0].2,
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            background: self.bg[0].into(),
            text_color: self.text[0],
            border_radius: self.border[1].0,
            border_width: self.border[1].1,
            border_color: self.border[1].2,
            ..button::Style::default()
        }
    }

    fn pressed(&self) -> button::Style {
        button::Style {
            background: self.bg[1].into(),
            text_color: self.text[1],
            border_radius: self.border[1].0,
            border_width: self.border[1].1,
            border_color: self.border[1].2,
            ..button::Style::default()
        }
    }
}


impl core::default::Default for ElfViewBtnTheme {
    fn default() -> Self {
        const ACTIVE : Color = Color::from_rgb(
            0x72 as f32 / 255.0,
            0x89 as f32 / 255.0,
            0xDA as f32 / 255.0,
        );

        const PRESSED : Color = Color::from_rgb(
            0x67 as f32 / 255.0,
            0x7B as f32 / 255.0,
            0xC4 as f32 / 255.0,
        );

        ElfViewBtnTheme {
            bg: [ACTIVE, PRESSED],
            text: [Color::WHITE; 2],
            border: [(0.0, 1.0, Color::BLACK), (1.0, 1.0, Color::WHITE)],
        }
    }
}
