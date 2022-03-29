//! Dynamic Text Input style.

use iced::{ text_input, Color };


#[derive(Debug, Clone, Copy)]
pub struct TextInput {
	/// Background color.
	bg: [Color; 3],

	/// Optional border.
	border: [(f32, f32, Color); 3],

	/// Placeholder color.
	placeholder: Color,

	/// Value color.
	value: Color,

	/// Selection color.
	selection: Color,
}

impl text_input::StyleSheet for TextInput {
	fn active(&self) -> text_input::Style {
		text_input::Style {
			background: self.bg[0].into(),
			border_width: self.border[0].0,
			border_radius: self.border[0].1,
			border_color: self.border[0].2,
			..text_input::Style::default()
		}
	}

	fn focused(&self) -> text_input::Style {
		text_input::Style {
			background: self.bg[1].into(),
			border_width: self.border[1].0,
			border_radius: self.border[1].1,
			border_color: self.border[1].2,
			..text_input::Style::default()
		}
	}

	fn hovered(&self) -> text_input::Style {
		text_input::Style {
			background: self.bg[2].into(),
			border_width: self.border[2].0,
			border_radius: self.border[2].1,
			border_color: self.border[2].2,
			..text_input::Style::default()
		}
	}

	fn placeholder_color(&self) -> Color {
		self.placeholder
	}

	fn value_color(&self) -> Color {
		self.value
	}

	fn selection_color(&self) -> Color {
		self.selection
	}
}

impl core::default::Default for TextInput {
	fn default() -> Self {
		TextInput {
			bg: [BG, BG, BG],
			border: [(0.0, 2.0, Color::TRANSPARENT), (1.0, 2.0, ACCENT), (1.0, 2.0, Color { a: 0.3, ..ACCENT } )],

			placeholder: Color::from_rgb(0.4, 0.4, 0.4),
			value: Color::WHITE,
			selection: ACTIVE,
		}
	}
}

const BG: Color = Color::from_rgb(
	0x40 as f32 / 255.0,
	0x44 as f32 / 255.0,
	0x4B as f32 / 255.0,
);

const ACCENT: Color = Color::from_rgb(
	0x6F as f32 / 255.0,
	0xFF as f32 / 255.0,
	0xE9 as f32 / 255.0,
);

const ACTIVE: Color = Color::from_rgb(
	0x72 as f32 / 255.0,
	0x89 as f32 / 255.0,
	0xDA as f32 / 255.0,
);
