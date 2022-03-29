//! Organization of the internal state of the probe GUI view.



use iced::{
    button,
    pick_list,
    text_input,
};

use super::Datatype;



pub(super) struct State {
    /// Project picklist state.
    pub(super) projectlist: pick_list::State<String>,

    /// Probe picklist state.
    pub(super) probelist: pick_list::State<String>,

    /// A list of states for the buttons.
    pub(super) button: ButtonStates,

    /// Probe read datatype state.
    pub(super) rddatatype: pick_list::State<Datatype>,

    /// Currently selected datatype.
    pub(super) seldatatype: Option<Datatype>,

    /// A list of states for the text inputs.
    pub(super) textinput: TextInputStates,
}

impl State {
    pub fn new() -> Self {
        State {
            projectlist: Default::default(),
            probelist: Default::default(),
            button: Default::default(),
            rddatatype: Default::default(),
            seldatatype: None,
            textinput: Default::default(),
        }
    }
}



#[derive(Default)]
pub(super) struct TextInputStates {
    /// Current read address.
    pub(super) readaddr: text_input::State,

    /// Current value of the read address.
    pub(super) readaddrval: String,

    /// Current read range start address.
    pub(super) saddr: text_input::State,

    /// Current value of the read range start address.
    pub(super) saddrval: String,

    /// Current read range end address.
    pub(super) eaddr: text_input::State,

    /// Current value of the read range end address.
    pub(super) eaddrval: String,
}



#[derive(Default)]
pub(super) struct ButtonStates {
    /// State of the load button.
    pub(super) load: button::State,

    /// State of the stop button.
    pub(super) stop: button::State,

    /// State of the reset button.
    pub(super) reset: button::State,

    /// State of the run button.
    pub(super) run: button::State,

    /// State of the step button.
    pub(super) step: button::State,

    /// State of the dump button.
    pub(super) dump: button::State,

    /// State of the read button.
    pub(super) read: button::State,

    /// State of the read range button.
    pub(super) range: button::State,

    /// State of the read symbol button.
    pub(super) symbol: button::State,
}
