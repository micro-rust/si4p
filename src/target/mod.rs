//! Software definition of a target microcontroller.



pub mod core;
pub mod field;
pub mod peripheral;
pub mod register;



pub use core::{ CoreInformation, Register as CoreRegister, };
pub use field::Field;
pub use peripheral::Peripheral;
pub use register::Register;
