//! App Messages for Probe events.



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProbeMessage {
    ReloadProbeList,
}