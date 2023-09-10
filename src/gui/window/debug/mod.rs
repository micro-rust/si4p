//! Debug window GUI.
//! Allows to open and use a debug session with a target.



use iced::{
    Element,

    widget::pane_grid::State as PaneGridState,
};


pub struct DebugWindow {
    /// Window ID.
    /// Differentiates this window from other debug windows.
    id: usize,

    // View of the target's peripherals.

    // View of the target's cores.

    // View of the code execution state of each of the target's cores.

    // Console of this window.

    /// Internal state of the pane grid.
    panegrid: PaneGridState<DebugWindowComponent>,
}

impl DebugWindow {
    pub fn view(&self) -> Element<Message> {
        use iced::{
            widget::{
                PaneGrid,
            },
        };

        // Create the panegrid.
        let panegrid = PaneGrid::new(&self.panegrid, |a, pane,b| match {
            
        });
    }

    pub fn update(&mut self, event: Event) {

    }
}

/// All components of the debug window.
pub enum DebugWindowComponent {
    /// View of the target's cores.
    Cores,

    /// View of the code execution state of the target's cores.
    Code,

    /// View of the target's peripherals.
    Peripherals,
}
