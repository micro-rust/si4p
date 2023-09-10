//! List of all possible views in the inspection view.




#[derive(Clone, Debug)]
pub enum View {
    /// The view of a section.
    Section,

    /// The view of a symbol.
    Symbol,
}
