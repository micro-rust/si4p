//! Show actions of the USB selector.



/// Common trait for all show actions.
pub trait ShowAction: core::fmt::Debug + Send + Sync + 'static {
    /// Returns the show state.
    fn state(&self) -> bool;

    /// Returns the key of the device to show.
    fn device(&self) -> usize;

    /// Returns the key of the configuration to show.
    fn configuration(&self) -> Option<u8> {
        None
    }

    /// Returns the key of the interface to show.
    fn interface(&self) -> Option<u8> {
        None
    }

    /// Returns the key of the endpoint to show.
    fn endpoint(&self) -> Option<u8> {
        None
    }
}



/// Show action for device level.
#[derive(Clone, Copy, Debug)]
pub struct DevShowAction {
    /// The show state.
    state: bool,

    /// The key of the device.
    key: usize,
}

impl DevShowAction {
    /// Creates a new `DevShowAction`.
    pub const fn new(state: bool, key: usize) -> Self {
        Self { state, key, }
    }
}

impl ShowAction for DevShowAction {
    fn state(&self) -> bool {
        self.state
    }

    fn device(&self) -> usize {
        self.key
    }
}

unsafe impl Send for DevShowAction {}

unsafe impl Sync for DevShowAction {}



/// Show action for configuration level.
#[derive(Clone, Copy, Debug)]
pub struct CfgShowAction {
    /// The show state.
    state: bool,

    /// The key of the device.
    key: usize,

    /// The key of the configuration.
    idx: u8,
}

impl CfgShowAction {
    /// Creates a new `CfgShowAction`.
    pub const fn new(state: bool, key: usize, idx: u8) -> Self {
        Self { state, key, idx, }
    }
}

impl ShowAction for CfgShowAction {
    fn state(&self) -> bool {
        self.state
    }

    fn device(&self) -> usize {
        self.key
    }

    fn configuration(&self) -> Option<u8> {
        Some(self.idx)
    }
}

unsafe impl Send for CfgShowAction {}

unsafe impl Sync for CfgShowAction {}



/// Show action for configuration level.
#[derive(Clone, Copy, Debug)]
pub struct IfaceShowAction {
    /// The show state.
    state: bool,

    /// The key of the device.
    key: usize,

    /// The key of the configuration.
    idx: u8,

    /// The key of the interface.
    num: u8,
}

impl IfaceShowAction {
    /// Creates a new `IfaceShowAction`.
    pub const fn new(state: bool, key: usize, idx: u8, num: u8) -> Self {
        Self { state, key, idx, num, }
    }
}

impl ShowAction for IfaceShowAction {
    fn state(&self) -> bool {
        self.state
    }

    fn device(&self) -> usize {
        self.key
    }

    fn configuration(&self) -> Option<u8> {
        Some(self.idx)
    }

    fn interface(&self) -> Option<u8> {
        Some(self.num)
    }
}

unsafe impl Send for IfaceShowAction {}

unsafe impl Sync for IfaceShowAction {}
