//! Internal events of the target selector.



#[derive(Clone, Debug)]
pub enum Event {
    /// The text input changed.
    TextChange( String ),

    /// Selects the given target.
    SelectTarget( String ),

    /// Deselects the given target.
    DeselectTarget,
}
