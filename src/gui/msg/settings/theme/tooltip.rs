//! Collection of messages emitted in the tooltip theme settings.


#[derive(Debug, Clone)]
pub enum TooltipThemeMessage {
    /// Indicates a change in the Red value of the text color.
    TextRed(String, usize),

    /// Indicates a change in the Green value of the text color.
    TextGreen(String, usize),

    /// Indicates a change in the Blue value of the text color.
    TextBlue(String, usize),

    /// Indicates a change in the Hex value of the text color.
    TextHex(String, usize),

    /// Indicates a change in the Red value of the background color.
    BgRed(String, usize),

    /// Indicates a change in the Green value of the background color.
    BgGreen(String, usize),

    /// Indicates a change in the Blue value of the background color.
    BgBlue(String, usize),

    /// Indicates a change in the Hex value of the background color.
    BgHex(String, usize),

    /// Indicates a change in the Float value of the border radius.
    Radius(String, usize),
}