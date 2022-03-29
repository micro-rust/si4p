//! Theme related app messages.



use crate::gui::theme::{
	button::{ ButtonTheme, ButtonStateTheme },
};



#[derive(Debug, Clone)]
pub enum ThemeMessage {
	/// An update on a button state theme.
	ButtonUpdate(String, ButtonTheme),

	/// An update on a button state theme.
	ButtonStateUpdate(String, ButtonStateTheme),
}
