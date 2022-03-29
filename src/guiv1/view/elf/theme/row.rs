//! Theme for buttons in the ELF View.


use iced::{ container, Color };

#[derive(Debug, Clone, Copy)]
pub struct ElfViewRowTheme {
    /// Text color | Background color.
    color: [Color; 2],

    /// Border style.
    border: (f32, f32, Color),
}


impl container::StyleSheet for ElfViewRowTheme {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: self.color[0].into(),
            background: self.color[1].into(),

            border_radius: self.border.0,
            border_width: self.border.1,
            border_color: self.border.2,
        }
    }
}

impl core::default::Default for ElfViewRowTheme {
    fn default() -> Self {
        const BG : Color = Color::from_rgb(
            0x36 as f32 / 255.0,
            0x39 as f32 / 255.0,
            0x3F as f32 / 255.0,
        );

        ElfViewRowTheme {
            color: [Color::WHITE, BG],
            border: (1.0, 0.0, Color::WHITE),
        }
    }
}
