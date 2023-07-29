//! USB probe selector.


use probe_rs::{
    DebugProbeInfo, Probe,
};

use crate::usb::Command;


#[derive(Clone)]
pub struct ProbeSelector {
    /// List of all possible probes.
    probes: Vec<DebugProbeInfo>,

    /// The currently selected device.
    selected: Option<DebugProbeInfo>,
}


impl crate::gui::common::Widget for ProbeSelector{
    type Event = ();

    fn view(&self) -> iced::Element<crate::gui::Message> {
        use iced::widget::{
            Button, Column, Row, Tooltip,

            tooltip::Position,
        };

        match &self.selected {
            // A probe is selected, show only the probe that is selected.
            Some(probe) => {
                // Get the probe title.
                let title = Self::probe(probe);

                // Create the close connection button.
                let close = Tooltip::new(
                    Button::new("Close"),
                    "Closes the connection to the debug probe",
                    Position::FollowCursor,
                );

                Row::new()
                    .push(title)
                    .push(close)
                    .into()
            },

            // No probe selected, show all the possible probes.
            _ => {
                self.probes.iter()
                    .map(|probe| {
                        // Create the probe.
                        let title = Self::probe(probe);

                        // Create the button.
                        let open = Button::new("Open")
                            .on_press( Command::ProbeOpen( probe.clone() ).into() );

                        Row::new()
                            .push(title)
                            .push(open)
                    })
                    .fold(Column::new(), |col, row| col.push(row))
                    .into()
            },
        }

    }
}

impl ProbeSelector {
    /// Creates a new Probe Selector.
    pub(super) fn new() -> Self {
        Self { probes: Probe::list_all(), selected: None, }
    }

    /// Rebuilds the list of probes.
    pub(super) fn rebuild(&mut self) {
        self.probes = Probe::list_all();
    }

    /// Creates the UI for a probe's information.
    fn probe(probe: &DebugProbeInfo) -> iced::Element<crate::gui::Message> {
        use iced::widget::{
            Column, Text,
        };

        // Get the IDS of the device.
        let ids = Text::new( format!("Debug Probe {:04X}:{:04X}", probe.vendor_id, probe.product_id) );

        // Get the serial number.
        let serial = Text::new( format!("S/N : {}", probe.serial_number.as_ref().unwrap_or(&String::new())) );

        Column::new()
            .push(ids)
            .push(serial)
            .into()
    }
}