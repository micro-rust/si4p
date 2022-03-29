//! Logging module.
//! Contains logging abstractions and tracing subscribers.



mod timereport;



use tracing_subscriber::{
    self, EnvFilter, FmtSubscriber,
    fmt::{
        fmt,
        format::{
            Format, Pretty,
        },
    },
};

pub use self::timereport::TimeReport;



pub fn logger() -> FmtSubscriber<Pretty, Format<Pretty, ()>, EnvFilter> {
	// Set the filter directives.
	let filter = EnvFilter::new("warn,error,[{origin}]=debug,database=info");

    fmt()
        .pretty()
        .without_time()
        .with_file(false)
        .with_level(true)
        .with_line_number(false)
        .with_target(true)
        .with_thread_names(true)
        .with_thread_ids(true)
        .with_env_filter(filter)
        .finish()
}
