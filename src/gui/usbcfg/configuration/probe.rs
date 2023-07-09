//! `probe-rs` configuration.




pub struct ProbeConfig {
    /// TODO : Create target list.

    /// Protocol of the probe.
    //protocol: Protocol,

    /// Speed of the probe in khz.
    speed: u32,
}

impl crate::gui::common::Widget for ProbeConfig {
    type Event = ();

    fn view(&self) -> iced::Element<crate::gui::Message> {
        iced::widget::Column::new().into()
    }

    fn update(&mut self, _: Self::Event) -> iced::Command<crate::gui::Message> {
        iced::Command::none()
    }
}

impl ProbeConfig {
    /// Creates a new probe configuration.
    pub fn new() -> Self {
        Self {
            //protocol: 
            speed: 1000,
        }
    }
}
