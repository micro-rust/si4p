//! Describes the enumerated values of a field.



pub struct EnumeratedValue {
    /// Name of the enumerated value.
    pub name: String,

    /// Description of the value.
    pub description: String,

    /// Value of the enumerated value.
    pub value: u64,

    /// Indicates if this is the default value of the enumeration.
    pub default: bool,
}
