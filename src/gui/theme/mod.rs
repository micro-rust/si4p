//! GUI Theme.
//! A GUI Theme has a collection of themes for the different components of the GUI.



pub mod background;
pub mod border;
pub mod button;
pub mod textinput;
pub mod tooltip;



use serde::{ Deserialize, Serialize };

use std::collections::HashMap;

use tracing::error;



#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Theme {
    /// Name of this theme.
    pub name: String,

    /// Brief description of this theme.
    pub description: String,

    /// A map from a theme to the style implemented.
    /// This is used as a dynamic way to point to styles to reduce duplication.
    /// Each theme has a brief description of what it modifies.
    pub styles: HashMap<String, (String, String)>,

    /// A map of button themes.
    pub button: HashMap<String, button::Theme>,

    /// A map of colors.
    pub color: HashMap<String, (u8, u8, u8, f32)>,

    /// A map of text input themes.
    pub textinput: HashMap<String, textinput::Theme>,

    /// A map of tooltip themes.
    pub tooltip: HashMap<String, tooltip::Theme>,
}

impl Theme {
    /// Parses a string into a theme.
    pub fn parse(buffer: Vec<u8>) -> Result<Self, Option<ron::Error>> {
        let ronfile: String = match String::from_utf8(buffer) {
            Err(e) => {
                error!(origin="app", "Could not parse UTF-8 string from the input buffer: {}", e);
                return Err(None);
            },
            Ok(r) => r,
        };

        match ron::from_str(&ronfile) {
            Err(e) => {
                error!(origin="app", "Could not parse RON file: {}", e);
                return Err(Some(e));
            },
            Ok(t) => Ok(t),
        }
    }
}
