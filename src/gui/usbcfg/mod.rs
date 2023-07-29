//! This component controls the configuration of the `defmt` and `probe-rs`
//! USB components.



mod configuration;
mod message;
mod view;
mod probe;
mod selector;
mod target;


pub(self) use configuration::*;

pub use message::Message;
pub use view::USBSelectorView;

use probe::ProbeSelector;

use selector::{
    ShowAction, USBSelector,
};

use std::path::PathBuf;

use target::TargetSelection;



pub(super) struct USBConfiguration {
    /// Currently selected view.
    selected: USBSelectorView,

    /// USB selector for `defmt`.
    defmt: USBSelector<DefmtConfig>,

    /// USB selector for `probe-rs`.
    probe: ProbeSelector,

    /// Target.
    target: crate::gui::right::target::TargetSelector,

    /// Current ELF.
    //file: Option<PathBuf>,
    file: crate::gui::right::elf::ELFSelector,
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
        //let file = self.filepath();
        let file = iced::widget::component( self.file );

        // Create the target selection.
        //let target = self.target.view();
        let target = iced::widget::component( self.target.clone() );

        Column::new()
            .padding(5)
            .spacing(5)
            .width(iced::Length::FillPortion(20))
            .push(file)
            .push(target)
            .push(topbar)
            .push(view)
            .into()
    }

    fn update(&mut self, event: Message) -> iced::Command<crate::gui::Message> {
        match event {
            Message::Selected( selected ) => self.selected = selected,

            Message::Defmt( action ) => self.defmt.show( &action ),

            //Message::TargetTextChange( new ) => self.target.textinput( new ),

            _ => (),
        }

        iced::Command::none()
    }
}

impl USBConfiguration {
    /// Creates a new `USBConfiguration` component.
    pub(super) fn new(library: std::sync::Arc<crate::library::Library>) -> Self {
        Self {
            selected: USBSelectorView::Defmt,
            defmt: USBSelector::new( defmtaction, defmtselect, DefmtConfig::new() ),
            probe: ProbeSelector::new(),
            //target: TargetSelection::new( library ),
            target: crate::gui::right::target::TargetSelector::new( library ),
            //file: None,
            file: crate::gui::right::elf::ELFSelector::new(),
        }
    }

    /// Updates the current file path.
    pub(super) fn setpath(&mut self, path: std::path::PathBuf) {
        //self.file = Some(path);
    }

    /// Rebuilds the lists of USBs.
    pub(super) fn rebuild(&mut self) {
        // Get a reading lock.
        let connected = crate::usb::CONNECTED.blocking_read();

        self.defmt.rebuild(&connected);
        self.probe.rebuild();
    }

    /// Marks the given target as selected.
    pub(super) fn select(&mut self, name: String) {
        //self.target.mark( name );
    }

    /// UnMarks the given target as selected.
    pub(super) fn deselect(&mut self) {
        //self.target.unmark();
    }

    /*
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
    */

    /// Creates the view of the topbar.
    /// This includes a defmt and probe tab view selector.
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

fn defmtaction(action: ShowAction) -> crate::gui::Message {
    Message::Defmt( action ).into()
}

fn probeaction(action: ShowAction) -> crate::gui::Message {
    Message::Probe( action ).into()
}

fn defmtselect(target: crate::usb::common::USBTarget) -> crate::gui::Message {
    crate::gui::Message::USB( crate::usb::Command::DefmtOpen( target ) )
}

fn probeselect(target: crate::usb::common::USBTarget) -> crate::gui::Message {
    crate::gui::Message::None
}
