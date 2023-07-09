//! This component controls the configuration of the `defmt` and `probe-rs`
//! USB components.



mod configuration;
mod message;
mod view;
mod selector;


pub(self) use configuration::*;

pub use message::Message;
pub use view::USBSelectorView;

use selector::{
    ShowAction, USBSelector,
};

use std::path::PathBuf;



pub(super) struct USBConfiguration {
    /// Currently selected view.
    selected: USBSelectorView,

    /// USB selector for `defmt`.
    defmt: USBSelector<DefmtConfig>,

    /// USB selector for `probe-rs`.
    probe: USBSelector<ProbeConfig>,

    /// Current `defmt` file.
    file: Option<PathBuf>,
}

impl crate::gui::common::Widget for USBConfiguration {
    type Event = Message;

    fn view(&self) -> iced::Element<crate::gui::Message> {
        use iced::widget::Column;

        // Get the view of the selected configuration.
        let view = match self.selected {
            USBSelectorView::Defmt => self.defmt.view(),
            USBSelectorView::Probe => self.probe.view(),
        };

        // Build the topbar and selection buttons.
        let topbar = self.topbar();

        // Create the file path selection.
        let file = self.filepath();

        Column::new()
            .padding(5)
            .spacing(5)
            .width(iced::Length::FillPortion(20))
            .push(file)
            .push(topbar)
            .push(view)
            .into()
    }

    fn update(&mut self, event: Message) -> iced::Command<crate::gui::Message> {
        match event {
            Message::Selected( selected ) => self.selected = selected,

            Message::Defmt( action ) => self.defmt.show( &action ),

            Message::Probe( action ) => self.probe.show( &action ),

            _ => (),
        }

        iced::Command::none()
    }
}

impl USBConfiguration {
    /// Creates a new `USBConfiguration` component.
    pub(super) fn new() -> Self {
        Self {
            selected: USBSelectorView::Defmt,
            defmt: USBSelector::new( defmt, DefmtConfig::new() ),
            probe: USBSelector::new( probe, ProbeConfig::new() ),
            file: None,
        }
    }

    /// Updates the current file path.
    pub(super) fn setpath(&mut self, path: std::path::PathBuf) {
        self.file = Some(path);
    }

    /// Rebuilds the lists of USBs.
    pub(super) fn rebuild(&mut self) {
        // Get a reading lock.
        let connected = crate::usb::CONNECTED.blocking_read();

        self.defmt.rebuild(&connected);
        self.probe.rebuild(&connected);
    }

    /// Creates the view of the file path selection.
    fn filepath(&self) -> iced::Element<crate::gui::Message> {
        use iced::widget::{
            Button, Row,
        };

        // Build the select file button.
        let mut select = Button::new( "Select file" )
            .width(iced::Length::FillPortion(65))
            .on_press( crate::gui::Message::SelectELF( None ) );

        // Build the reload file button.
        let mut reload = Button::new( "Reload" )
            .width(iced::Length::FillPortion(35));

        if let Some( path ) = &self.file {
            // Create the parent path.
            let mut parent = path.clone();
            parent.pop();

            select = select.on_press( crate::gui::Message::SelectELF( Some( parent ) ) );
            reload = reload.on_press( crate::gui::Message::LoadELF( path.clone() ) )
        }

        Row::new()
            .push(select)
            .push(reload)
            .into()
    }

    /// Creates the view of the topbar.
    fn topbar(&self) -> iced::Element<crate::gui::Message> {
        use iced::widget::{
            Button, Row,
        };

        // Create the defmt button.
        let mut defmt = Button::new( "defmt" )
            .width( iced::Length::FillPortion( 50 ) );

        if self.selected == USBSelectorView::Probe {
            defmt = defmt.on_press( Message::Selected( USBSelectorView::Defmt ).into() );
        }

        // Create the defmt button.
        let mut probe = Button::new( "probe-rs" )
            .width( iced::Length::FillPortion( 50 ) );

        if self.selected == USBSelectorView::Defmt {
            probe = probe.on_press( Message::Selected( USBSelectorView::Probe ).into() );
        }

        Row::new()
            .push( defmt )
            .push( probe )
            .into()
    }
}

fn defmt(action: ShowAction) -> crate::gui::Message {
    Message::Defmt( action ).into()
}

fn probe(action: ShowAction) -> crate::gui::Message {
    Message::Probe( action ).into()
}
