//! USB configuration views.



#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum USBSelectorView {
    /// USB configuration of `defmt`.
    Defmt,

    /// USB configuration of `probe-rs`.
    Probe,
}
