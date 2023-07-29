//! Internal events of the console's component.



#[derive(Clone, Debug)]
pub enum Event {
    /// The level filter changed.
    FilterLevel( super::Level ),

    /// The source filter changed.
    FilterSource( super::Source ),
}
