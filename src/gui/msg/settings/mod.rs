//! Collection of messages emitted by the settings view.



pub mod theme;


use self::theme::ThemeMessage;




#[derive(Debug, Clone)]
pub enum SettingsMessage {
    /// THe back button was pressed.
    BackButton,

    /// The theme settings button was pressed.
    ThemeSettingsButton,

    /// The theme settings button was pressed.
    AppSettingsButton,

    /// Indicates that a theme change ocurred.
    Theme(ThemeMessage),
}