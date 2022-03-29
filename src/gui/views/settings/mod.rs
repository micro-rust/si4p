//! Settings view.
//! GUI View that allows changing the settings of the application.



#![deny(warnings)]



mod theme;



use crate::gui::msg::{
    Message, SettingsMessage,
};

use iced::{
    Element, Column, Row, Command,

    Length,

    Text,

    button::{ self, Button },
};


pub struct SettingsView {
    /// Current internal view displayed.
    view: Option<SettingsSubview>,

    /// State of the back button.
    backbtn: button::State,

    /// State of the theme settings button.
    themebtn: button::State,

    /// State of the app settings button.
    appbtn: button::State,
}

impl SettingsView {
    pub fn new() -> Self {
        SettingsView {
            view: None,
            backbtn: button::State::new(),
            themebtn: button::State::new(),
            appbtn: button::State::new(),
        }
    }

    /// Updates the view.
    pub fn update(&mut self, msg: SettingsMessage) -> Command<Message> {
        match msg {
            // Go back one level.
            SettingsMessage::BackButton => {
                self.view = None;
                Command::none()
            },

            // Open the theme settings.
            SettingsMessage::ThemeSettingsButton => {
                self.view = Some(SettingsSubview::ThemeSettings);
                Command::none()
            },

            // Open the app settings.
            SettingsMessage::AppSettingsButton => {
                self.view = Some(SettingsSubview::AppSettings);
                Command::none()
            },

            _ => Command::none(),
        }
    }

    /// Builds the GUI.
    pub fn view(&mut self) -> Element<Message> {
        // Create the back button.
        let back = {
            let text = Text::new("BACK").size(28);

            let button = Button::new(&mut self.backbtn, text)
                .height(Length::Units(150))
                .width(Length::Units(150));

            match self.view {
                Some(_) => button.on_press( Message::Settings( SettingsMessage::BackButton ) ),
                _ => button,
            }
        };

        // Create the content.
        let content: Element<_> = match self.view {
            Some(_) => Row::new().into(),

            None => {
                let themebtn = Button::new(&mut self.themebtn, Text::new("Theme Settings").size(28))
                    .height(Length::Shrink)
                    .width(Length::Fill)
                    .on_press( Message::Settings( SettingsMessage::ThemeSettingsButton ) );

                let appbtn = Button::new(&mut self.appbtn, Text::new("App Settings").size(28))
                    .height(Length::Shrink)
                    .width(Length::Fill)
                    .on_press( Message::Settings( SettingsMessage::AppSettingsButton ) );

                Column::new()
                    .push(themebtn)
                    .push(appbtn)
                    .height(Length::Fill)
                    .width(Length::Fill)
                    .into()
            },
        };


        Row::new()
            .push(back)
            .push(content)
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsSubview {
    ThemeSettings,
    AppSettings,
}