//! Internal events of the core view widget.




#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Event {
    /// Emits the command to halt the core.
    Halt,

    /// Emits the command to reset the core.
    Reset,

    /// Emits the command to run the core.
    Run,

    /// Sets the show state of the core registers.
    ShowCoreRegisters( bool ),

    /// Sets the show state of the FPU registers.
    ShowFPURegisters( bool ),
}
