//! `defmt` configuration.



pub struct DefmtConfig {
}

impl crate::gui::common::Widget for DefmtConfig {
    type Event = ();

    fn view(&self) -> iced::Element<crate::gui::Message> {
        iced::widget::Column::new().into()
    }

    fn update(&mut self, _: Self::Event) -> iced::Command<crate::gui::Message> {
        iced::Command::none()
    }
}

impl DefmtConfig {
    /// Creates a new `defmt` configurator.
    pub fn new() -> Self {
        Self {}
    }
}
