//! Internal state of the target selector.



pub struct State {
    /// The current text input.
    pub(super) input: String,

    /// The currently selected target.
    pub(super) selected: Option<String>,

    /// The indices of the targets that currently match the search.
    pub(super) matches: Option<Vec<usize>>,
}

impl Default for State {
    fn default() -> Self {
        Self { input: String::new(), selected: None, matches: None, }
    }
}
