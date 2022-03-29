//! Silicon 4+ GUI debugger and probe.


mod database;
mod log;
mod probe;
mod project;
mod gui;
mod init;



use tracing_subscriber::util::SubscriberInitExt;



fn main() {
    // Initialize logger.
    let logger = log::logger();
    logger.init();

    // Launch the application.
    gui::Application::start( () );
}
