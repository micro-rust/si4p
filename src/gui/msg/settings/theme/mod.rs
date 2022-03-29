//! Collection of messages emitted in the theme settings menu.



mod tooltip;



pub use self::tooltip::TooltipThemeMessage;



#[derive(Debug, Clone)]
pub enum ThemeMessage {
    /// A change ocurred in RGB settings (Red channel).
    RGBSettingsRed((usize, usize), String),

    /// A change ocurred in RGB settings (Green channel).
    RGBSettingsGreen((usize, usize), String),

    /// A change ocurred in RGB settings (Blue channel).
    RGBSettingsBlue((usize, usize), String),

    /// A change ocurred in RGB settings (all channels).
    RGBSettingsColor((usize, usize), String),

    /// The color format was changed.
    RGBSettingsHexfmt((usize, usize), bool),

    /// A change in the theme of a tooltip ocurred.
    Tooltip(TooltipThemeMessage),
}
