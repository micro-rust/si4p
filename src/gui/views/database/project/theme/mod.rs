//! Theme for the project database view.



pub(super) mod projectheader;
pub(super) mod targetinfo;




use marcel::{
    Theme as GlobalTheme,
    border, container,
};

use std::sync::Arc;

use tokio::sync::RwLock;



#[derive(Clone, Copy, Debug)]
pub(super) struct Theme {
    /// Common background of the view.
    pub(super) background: container::Theme,

    /// Theme for the project header.
    pub(super) projectheader: projectheader::Theme,

    /// Theme for the target info.
    pub(super) targetinfo: targetinfo::Theme,
}

impl From<Arc<RwLock<GlobalTheme>>> for Theme {
    fn from(theme: Arc<RwLock<GlobalTheme>>) -> Self {
        // Build the default theme.
        let mut out: Self = Default::default();

        {
            // Wait until the theme is readable.
            let theme = theme.blocking_read();

            // Get the background theme.
            match container::Theme::extract(&theme, String::from("vdbpr-bg")) {
                Some(style) => out.background = style,
                _ => (),
            }
        }

        // Get the project header theme.
        out.projectheader = projectheader::Theme::from(theme.clone());

        // Get the target info theme.
        out.targetinfo = targetinfo::Theme::from(theme.clone());

        out
    }
}

impl Default for Theme {
    fn default() -> Self {
        // Create default background.
        let background = {
            // Background color.
            let color = marcel::color::Color::WHITE;

            // Border theme.
            let border = border::Theme {
                radius: 0.0,
                width: 0.0,
                color: marcel::color::Color::BLACK,
            };

            container::Theme { color, border }
        };

        Theme {
            background,
            projectheader: Default::default(),
            targetinfo: Default::default(),
        }
    }
}
