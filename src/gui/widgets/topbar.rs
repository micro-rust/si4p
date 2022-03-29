//! A topbar widget.



use crate::gui::msg::Message;
use iced::{
    Align, Column, Container, Element, Length, Row, Text,

    button::{ Button, State },
};



pub struct Topbar {
    /// Buttons to be created.
    buttons: Vec<(State, Message, String)>,

    // Button style of the topbar.
}

impl Topbar {
    /// Creates a new `Topbar`.
    pub fn new() -> Self {
        Topbar { buttons: Vec::new(), }
    }

    /// Adds a new button to the topbar.
    pub fn add(&mut self, msg: Message, text: String) {
        self.buttons.push((State::new(), msg, text));
    }

    /// Builds the topbar GUI.
    pub fn view(&mut self, enabled: bool) -> Element<Message> {
        // Build the list of buttons.
        let buttons: Vec<_> = self.buttons.iter_mut()
            .map(|(state, msg, string)| {
                // Build the text.
                let text = Text::new(string.clone()).size(24);

                // Build a container to center the text.
                let content = Container::new(text)
                    .align_x(Align::Center)
                    .align_y(Align::Center);

                // Build the button.
                let btn = Button::new(state, content)
                    .height(Length::Shrink)
                    .width(Length::Fill)
                    .padding(10);

                if enabled { btn.on_press( msg.clone() ).into() }
                else { btn.into() }
            })
            .collect();

        Row::with_children(buttons)
            .height(Length::Shrink)
            .width(Length::Fill)
            .into()
    }

    /// Builds the topbar GUI as a sidebar.
    pub fn side(&mut self, enabled: bool) -> Element<Message> {
        // Build the list of buttons.
        let buttons: Vec<_> = self.buttons.iter_mut()
            .map(|(state, msg, string)| {
                // Build the text.
                let text = Text::new(string.clone()).size(24);

                // Build a container to center the text.
                let content = Container::new(text)
                    .align_x(Align::Center)
                    .align_y(Align::Center);

                // Build the button.
                let btn = Button::new(state, content)
                    .height(Length::Shrink)
                    .width(Length::Fill)
                    .padding(10);

                if enabled { btn.on_press( msg.clone() ).into() }
                else { btn.into() }
            })
            .collect();

        Column::with_children(buttons)
            .height(Length::Fill)
            .width(Length::Shrink)
            .into()
    }
}
