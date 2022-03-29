//! Theme for each target in the project create/edit body.





use marcel::{
    Theme as GlobalTheme,
    border, button, container, textinput,
};

use std::sync::Arc;

use tokio::sync::RwLock;



#[derive(Clone, Copy, Debug)]
pub struct Theme {
    /// The background of the theme.
    pub background: container::Theme,

    /// The theme of a text input.
    pub textinput: textinput::Theme,

    /// The theme of the remove button.
    pub button: button::Theme,
}

impl From<Arc<RwLock<GlobalTheme>>> for Theme {
    fn from(theme: Arc<RwLock<GlobalTheme>>) -> Self {
        // Wait until the theme is readable.
        let theme = theme.blocking_read();

        // Build the default theme.
        let mut out: Self = Default::default();

        // Get the text input theme.
        match container::Theme::extract(&theme, String::from("vdbpr-ph-bg")) {
            Some(style) => out.background = style,
            _ => (),
        }


        // Get the text input theme.
        match textinput::Theme::extract(&theme, String::from("vdbpr-ti-txtin")) {
            Some(style) => out.textinput = style,
            _ => (),
        }


        // Get the button theme.
        match button::Theme::extract(&theme, String::from("vdbpr-ti-btn")) {
            Some(style) => out.button = style,
            _ => (),
        }

        out
    }
}

impl Default for Theme {
    fn default() -> Self {
        // Build the background color.
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

        // Build the text input theme.
        let textinput = {
            // Build the active state theme.
            let active = {
                // Background color.
                let background = marcel::color::Color::WHITE;

                // Border theme.
                let border = border::Theme {
                    radius: 10.0,
                    width: 2.0,
                    color: marcel::color::Color::RED,
                };

                textinput::StateTheme { background, border }
            };

            // Build the focused state theme.
            let focused = {
                // Background color.
                let background = marcel::color::Color::WHITE;

                // Border theme.
                let border = border::Theme {
                    radius: 10.0,
                    width: 2.0,
                    color: marcel::color::Color::RED,
                };

                textinput::StateTheme { background, border }
            };

            // Build the hovered state theme.
            let hovered = {
                // Background color.
                let background = marcel::color::Color::WHITE;

                // Border theme.
                let border = border::Theme {
                    radius: 10.0,
                    width: 2.0,
                    color: marcel::color::Color::RED,
                };

                textinput::StateTheme { background, border }
            };

            // Build the default colors.
            let colors = {
                // Placeholder color.
                let placeholder = marcel::color::Color::BLACK;

                // Value color.
                let value = marcel::color::Color::RED;

                // Selection color.
                let selection = marcel::color::Color::BLACK;

                [placeholder, value, selection]
            };

            textinput::Theme {
                active,
                focused: focused,
                hovered: hovered,
                colors,
            }
        };

        // Build the button theme.
        let button = {
            // Build the active state theme.
            let active = {
                // Background color.
                let background = marcel::color::Color::WHITE;

                // Border theme.
                let border = border::Theme {
                    radius: 10.0,
                    width: 0.0,
                    color: marcel::color::Color::BLACK,
                };

                // Text color.
                let textcolor = marcel::color::Color::BLACK;

                button::StateTheme { background, border, textcolor }
            };

            // Build the active state theme.
            let hovered = {
                // Background color.
                let background = marcel::color::Color::WHITE;

                // Border theme.
                let border = border::Theme {
                    radius: 10.0,
                    width: 0.0,
                    color: marcel::color::Color::BLACK,
                };

                // Text color.
                let textcolor = marcel::color::Color::BLACK;

                button::StateTheme { background, border, textcolor }
            };

            // Build the active state theme.
            let pressed = {
                // Background color.
                let background = marcel::color::Color::WHITE;

                // Border theme.
                let border = border::Theme {
                    radius: 10.0,
                    width: 0.0,
                    color: marcel::color::Color::BLACK,
                };

                // Text color.
                let textcolor = marcel::color::Color::BLACK;

                button::StateTheme { background, border, textcolor }
            };

            // Build the active state theme.
            let disabled = {
                // Background color.
                let background = marcel::color::Color::WHITE;

                // Border theme.
                let border = border::Theme {
                    radius: 10.0,
                    width: 0.0,
                    color: marcel::color::Color::BLACK,
                };

                // Text color.
                let textcolor = marcel::color::Color::BLACK;

                button::StateTheme { background, border, textcolor }
            };

            button::Theme {
                active,
                hovered,
                pressed,
                disabled,
            }
        };


        Theme { background, textinput, button }
    }
}
