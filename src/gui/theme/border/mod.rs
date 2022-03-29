//! Theme of the border of a widget.



use serde::{ Deserialize, Serialize };



#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct Theme {
	/// Radius of the border.
	pub radius: f32,

	/// Width of the border.
	pub width: f32,

	/// Color of the border.
	pub color: (u8, u8, u8, f32),
}
