//! Dynamic Radio Button style.


use iced::{ radio, Color };


#[derive(Debug, Clone, Copy)]
pub struct Radio {
	/// Background color.
	bg: [Color; 2],

	/// Inner dot color.
	dot: [Color; 2],

	/// Border.
	border: [(f32, Color); 2],
}

impl radio::StyleSheet for Radio {
	fn active(&self) -> radio::Style {
		radio::Style {
			background: self.bg[0].into(),
			dot_color: self.dot[0].into(),
			border_width: self.border[0].0,
			border_color: self.border[1].1,
		}
	}

	fn hovered(&self) -> radio::Style {
		radio::Style {
			background: self.bg[1].into(),
			dot_color: self.dot[1].into(),
			border_width: self.border[0].0,
			border_color: self.border[1].1,
		}
	}
}

impl core::default::Default for Radio {
	fn default() -> Self {
		Radio {
			bg: [BG, Color { a: 0.5, ..BG }],
			dot: [ACTIVE; 2],
			border: [(1.0, ACTIVE), (1.0, ACTIVE)],
		}
	}
}

const BG: Color = Color::from_rgb(
	0x40 as f32 / 255.0,
	0x44 as f32 / 255.0,
	0x4B as f32 / 255.0,
);

const ACTIVE: Color = Color::from_rgb(
	0x72 as f32 / 255.0,
	0x89 as f32 / 255.0,
	0xDA as f32 / 255.0,
);
