//! `defmthost` crate access. To build as an application, run `cargo install
//! defmthost --features tui` or `cargo install defmthost --features gui`.



pub mod common;
pub mod library;
pub mod usb;
pub mod target;

//#[cfg(feature = "gui")]
pub mod gui;



use std::sync::atomic::AtomicBool;



/// Global quit signal.
pub static QUIT: AtomicBool = AtomicBool::new(false);
