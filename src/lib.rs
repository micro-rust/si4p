//! `defmthost` crate access. To build as an application, run `cargo install
//! defmthost --features tui` or `cargo install defmthost --features gui`.



pub mod common;
pub mod library;
pub mod usb;

//#[cfg(feature = "gui")]
pub mod gui;
