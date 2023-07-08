//! Description of an USB target.



#[derive(Clone, Debug)]
pub struct USBTarget {
    /// Vendor and Product IDs.
    pub ids: (u16, u16),

    /// Bus and address.
    pub bus: (u8, u8),

    /// Configuration used.
    pub config: u8,

    /// Interface used.
    pub iface: u8,

    /// Alternate setting used.
    pub setting: u8,

    /// Endpoint.
    pub endpoint: u8,
}

impl USBTarget {
    /// Creates a new USB target with the given information.
    pub const fn new(ids: (u16, u16), bus: (u8, u8), config: u8, iface: u8, setting: u8, endpoint: u8) -> Self {
        Self { ids, bus, config, iface, setting, endpoint }
    }

    /// Creates a new empty USB target.
    pub const fn empty() -> Self {
        Self { ids: (0, 0), bus: (0, 0), config: 0, iface: 0, setting: 0, endpoint: 0 }
    }

    /// Returns `true` if the given IDs and bus match the target.
    pub fn matches(&self, vid: u16, pid: u16, bus: u8, address: u8) -> bool {
        (self.ids.0 == vid) && (self.ids.1 == pid) && (self.bus.0 == bus) && (self.bus.1 == address)
    }
}
