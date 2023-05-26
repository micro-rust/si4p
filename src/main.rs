//! `defmthost` application.



// TUI is not yet enabled, create compile error if it is built.
#[cfg(feature = "tui")]
compile_error!("Terminal UI is not yet implemented");

// If both UI are enabled, create compile error.
#[cfg(all(feature = "gui", feature = "tui"))]
compile_error!("Attempted to build both GUI and TUI applications");



pub fn main() {
    #[cfg(feature = "gui")]
    defmthost::gui::Application::start();
}
