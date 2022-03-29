//! Dynamic Button style.

use iced::{ button, Color };


#[derive(Debug, Clone, Copy)]
pub struct Button {
	/// Background color.
	bg: [Color; 3],

	/// Text color.
	txt: [Color; 3],

	/// Optional border.
	border: [(f32, f32, Color); 3],

	/// Optional shadow offset.
	shadow: [iced::Vector<f32>; 3],
}

impl button::StyleSheet for Button {
	fn active(&self) -> button::Style {
		button::Style {
			background: self.bg[0].into(),
			text_color: self.txt[0].into(),
			border_width: self.border[0].0,
			border_radius: self.border[0].1,
			border_color: self.border[0].2,
			shadow_offset: self.shadow[0],
			..button::Style::default()
		}
	}

	fn hovered(&self) -> button::Style {
		button::Style {
			background: self.bg[1].into(),
			text_color: self.txt[1].into(),
			border_width: self.border[1].0,
			border_radius: self.border[1].1,
			border_color: self.border[1].2,
			shadow_offset: self.shadow[1],
			..button::Style::default()
		}
	}

	fn pressed(&self) -> button::Style {
		button::Style {
			background: self.bg[2].into(),
			text_color: self.txt[2].into(),
			border_width: self.border[2].0,
			border_radius: self.border[2].1,
			border_color: self.border[2].2,
			shadow_offset: self.shadow[2],
			..button::Style::default()
		}
	}
}

impl core::default::Default for Button {
	fn default() -> Self {
		Button {
			bg: [ACTIVE, HOVERED, HOVERED],
			txt: [Color::WHITE; 3],
			border: [(0.0, 1.0, Color::BLACK), (0.0, 1.0, Color::BLACK), (1.0, 1.0, Color::WHITE)],
			shadow: [iced::Vector::new(0.0, 0.0), iced::Vector::new(0.0, 0.0), iced::Vector::new(0.1, 0.1)],
		}
	}
}



const ACTIVE : Color = Color::from_rgb(
	0x72 as f32 / 255.0,
	0x89 as f32 / 255.0,
	0xDA as f32 / 255.0,
);

const HOVERED : Color = Color::from_rgb(
	0x67 as f32 / 255.0,
	0x7B as f32 / 255.0,
	0xC4 as f32 / 255.0,
);
