//! Theme for the Elf View of the GUI.



mod button;
mod text;
mod row;

use self::button::ElfViewBtnTheme;
use self::row::ElfViewRowTheme;
use self::text::ElfViewTextInfoTheme;

use iced::{ Font };


#[derive(Debug, Clone, Copy)]
pub struct ElfViewTheme {
    /// Main button theme.
    pub(super) mainbtn: ElfViewBtnTheme,

    /// Sub button theme.
    pub(super) subbtn: ElfViewBtnTheme,

    /// Section and Program header row theme.
    pub(super) subrow: ElfViewRowTheme,

    /// Info theme
    pub(super) textinfo: ElfViewTextInfoTheme,
}



impl core::default::Default for ElfViewTheme {
    fn default() -> Self {
        Self {
            mainbtn: ElfViewBtnTheme::default(),
            subbtn: ElfViewBtnTheme::default(),
            subrow: ElfViewRowTheme::default(),
            textinfo: ElfViewTextInfoTheme::default(),
        }
    }
}

pub(super) const MONO: Font = Font::External {
    name: "UbuntuMono",

    #[cfg(target_os = "linux")]
    bytes: include_bytes!("../../../../../res/font/UbuntuMono-R.ttf"),

    #[cfg(target_os = "windows")]
    bytes: include_bytes!("..\\..\\..\\..\\..\\res\\font\\UbuntuMono-R.ttf"),
};

