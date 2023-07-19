//! Possible values of the field of a register.



use super::EnumeratedValue;
use svd_parser::svd::WriteConstraintRange;



pub enum WriteValues {
    /// Must be written as read.
    WriteAsRead,

    /// The field is written as it's raw value.
    Value,

    /// The field is written as a value within a range.
    Range( WriteConstraintRange ),

    /// The field is written as one of all possible enumerated values.
    Enumeration( Vec<EnumeratedValue> ),
}
