//! Theme module.


pub mod button;
pub mod container;
pub mod radio;
pub mod txt;



pub struct Theme {
	// Button style.
	pub button: button::Button,

	/// Container style.
	pub container: container::Container,

	/// Radio button style.
	pub radio: radio::Radio,

	/// Text input style.
	pub textinput: txt::TextInput,
}

impl Theme {

}


impl From<Theme> for Box<dyn iced::button::StyleSheet> {
	fn from(theme: Theme) -> Self {
		theme.button.into()
	}
}

impl From<Theme> for Box<dyn iced::container::StyleSheet> {
	fn from(theme: Theme) -> Self {
		theme.container.into()
	}
}

impl From<Theme> for Box<dyn iced::radio::StyleSheet> {
	fn from(theme: Theme) -> Self {
		theme.radio.into()
	}
}

impl From<Theme> for Box<dyn iced::text_input::StyleSheet> {
	fn from(theme: Theme) -> Self {
		theme.textinput.into()
	}
}


impl core::default::Default for Theme {
	fn default() -> Self {
		Theme {
			button: button::Button::default(),
			container: container::Container::default(),
			radio: radio::Radio::default(),
			textinput: txt::TextInput::default(),
		}
	}
}

