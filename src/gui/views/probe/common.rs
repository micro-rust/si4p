//! Common abstractions for the Probe View.



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Datatype {
    Int8,
    UInt8,
    Int16,
    UInt16,
    Int32,
    UInt32,
    Int64,
    UInt64,

    Float16,
    Float32,

    BFloat16,

    Char,
}


impl core::fmt::Display for Datatype {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", match *self {
            Datatype::Int8 => "i8",
            Datatype::UInt8 => "u8",
            Datatype::Int16 => "i16",
            Datatype::UInt16 => "u16",
            Datatype::Int32 => "i32",
            Datatype::UInt32 => "u32",
            Datatype::Int64 => "i64",
            Datatype::UInt64 => "u64",

            Datatype::Float16 => "f16",
            Datatype::Float32 => "f32",

            Datatype::BFloat16 => "bf16",

            Datatype::Char => "char",
        })
    }
}


pub const DATATYPES: [Datatype; 12] = [
    Datatype::Int8,
    Datatype::UInt8,
    Datatype::Int16,
    Datatype::UInt16,
    Datatype::Int32,
    Datatype::UInt32,
    Datatype::Int64,
    Datatype::UInt64,

    Datatype::Float16,
    Datatype::Float32,

    Datatype::BFloat16,

    Datatype::Char,
];
