//! Dynamic Conatiner style.

use iced::{ container, Color };


#[derive(Debug, Clone, Copy)]
pub struct Container {
	/// Background color.
	bg: Color,

	/// Text color.
	txt: Color,

	/// Optional border.
	border: (f32, Color),
}

impl container::StyleSheet for Container {
	fn style(&self) -> container::Style {
		container::Style {
			background: self.bg.into(),
			text_color: self.txt.into(),
			border_width: self.border.0,
			border_color: self.border.1,
			..container::Style::default()
		}
	}
}

impl core::default::Default for Container {
	fn default() -> Self {
		Container {
			bg: BG,
			txt: Color::WHITE,
			border: (1.0, Color::BLACK),
		}
	}
}

const BG: Color = Color::from_rgb(
	0x36 as f32 / 255.0,
	0x39 as f32 / 255.0,
	0x3F as f32 / 255.0,
);
