//! Common widget trait.



use crate::gui::Message;

use iced::{
    Command, Element,
};


pub trait Widget {
    type Event;

    fn view(&self) -> Element<Message>;

    fn update(&mut self, event: Self::Event) -> Command<Message> {
        Command::none()
    }
}
