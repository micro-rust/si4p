//! State for a Debug view.


use iced::{
    button, pick_list, scrollable,
};



pub struct DebugViewState {
    /// State for the probe pick list.
    pub(super) probelist: pick_list::State<String>,

    /// Currently selected probe.
    pub(super) probe: Option<String>,
}


impl DebugViewState {
    /// Creates a new Debug View Theme.
    pub fn new() -> DebugViewState {
        DebugViewState {
        	probelist: pick_list::State::new(),
        	probe: None,
        }
    }
}
