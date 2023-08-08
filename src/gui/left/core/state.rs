//! Internal state of a core view.




#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct State {
    /// Show state of the core registers.
    pub(super) core: bool,

    /// Show state of the FPU registers.
    pub(super) fpu: bool,
}
