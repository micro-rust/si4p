//! Regex database.
//! The regex database contains precompiled regex used in the application.



pub mod cmds;



use crate::log::TimeReport;

use database::{
    Database as DatabaseTrait, Interface,
    common::{
        DBInterface, Status,
    },
};

use regex::Regex;

use std::{
    collections::HashMap,
};

use super::statusupdate;

use tokio::{
    join,
    sync::{ mpsc },
};

use tracing::{
    error, info,

    instrument::WithSubscriber,
};



pub use self::cmds::{
    DBCommand as Command,
    Command as RegexCommand,
    Response as RegexResponse,
};



pub struct Database {
    /// A collection of precompiled regex.
    regex: HashMap<String, Regex>,

    /// Command channel receiver.
    cmds: mpsc::Receiver<Command>,
}

impl Database {
    /// Creates a new `Database`.
    pub fn new(cmds: mpsc::Receiver<Command>) -> Self {
        Database {
            regex: HashMap::new(),
            cmds,
        }
    }

    /// Initializes the `Database` and runs in a separate thread.
    pub async fn run(&mut self) {
        // Create the time reports.
        let mut report = TimeReport::new(String::from("regexdb-init"));
        let rgbreport  = TimeReport::new(String::from("regexdb-compile-rgb"));

        let start = report.start();


        let (rgb,) = join!{
            tokio::spawn( rgb(rgbreport).with_current_subscriber() ),
        };


        // End the report.
        report.end(start);

        // Add the secondary reports to the report.
        match rgb {
            Ok((vec, r)) => {
                for (name, regex) in vec {
                    self.regex.insert(name, regex);
                }

                report.add(r);
            },
            Err(e) => error!(origin="database", db="regex", "Failed to build RGB regex: {}", e),
        }

        info!(origin="database", db="regex", "Regex Database load time:\n{}", report);

        // Start executing database events.
        self.eventloop().await;
    }

    /// Runs the event loop of the database.
    async fn eventloop(&mut self) {
        loop {
            match self.cmds.recv().await {
                Some(cmd) => {
                    // Destructure the command.
                    let (cmd, channel, mut status) = cmd.destructure();

                    statusupdate(&mut status, Status::Acknowledged).await;

                    match cmd {
                        // Lookup the regex requested.
                        RegexCommand::GetRegex(name) => match self.regex.get(&name) {
                            Some(regex) => match channel.send(RegexResponse::Regex(regex.clone())) {
                                Err(_) => {
                                    error!(origin="database", db="regex", "Could not send Regex '{}': Channel closed", name);
                                    statusupdate(&mut status, Status::Failed(None)).await;
                                },
                                _ => {
                                    info!(origin="database", db="regex", "Sent a copy of the Regex {}", name);
                                    statusupdate(&mut status, Status::Completed).await;
                                },
                            },

                            _ => {
                                error!(origin="database", db="regex", "No Regex named {}", name);
                                    statusupdate(&mut status, Status::Failed(None)).await;
                            },
                        },

                        //_ => statusupdate(&mut status, Status::Denied).await,
                    }
                },
                _ => break,
            }
        }
    }
}

impl DatabaseTrait for Database {
    fn spawn() -> Box<dyn Interface> {
        // Create a new channel.
        let (tx, rx) = mpsc::channel(64);

        // Create a new `Database`, spawn it in a new `tokio` task and return the interface.
        let mut database = Database::new(rx);

        tokio::spawn( async move { database.run().with_current_subscriber().await } );

        Box::new( DBInterface::new(tx) )
    }
}



/// Compiles the RGB regex.
async fn rgb(mut report: TimeReport) -> (Vec<(String, Regex)>, TimeReport) {
    // Start the report.
    let start = report.start();

    // Compile the full RGB definition regex.
    let full = {
        // Compile the full RGB definition validation regex.
        let validate = match Regex::new(r"[[0x]|#]?[0-9A-Fa-f]{0,6}") {
            Err(e) => {
                error!("Could not build RGB validation regex: {}", e);
                Regex::new(".*").expect("Could not build empty regex")
            },
            Ok(r) => r,
        };

        // Compile the full RGB definition capture regex.
        let capture = match Regex::new(r"[[0x]|#]?[[(0-9A-Fa-f)(0-9A-Fa-f)(0-9A-Fa-f)]|[(0-9A-Fa-f){2}(0-9A-Fa-f){2}(0-9A-Fa-f){2}]]") {
            Err(e) => {
                error!("Could not build RGB capture regex: {}", e);
                Regex::new(".*").expect("Could not build empty regex")
            },
            Ok(r) => r,
        };

        ((String::from("fullvalid"), validate), (String::from("fullcapture"), capture))
    };

    // Compile the RGB component definition regex.
    let comp = {
        // Compile the full RGB definition validation regex.
        let validate = match Regex::new(r"[[0x]|#]?[0-9A-Fa-f]{0,2}") {
            Err(e) => {
                error!("Could not build RGB component validation regex: {}", e);
                Regex::new(".*").expect("Could not build empty regex")
            },
            Ok(r) => r,
        };

        // Compile the full RGB definition capture regex.
        let capture = match Regex::new(r"[[0x]|#]?[(0-9A-Fa-f){1,2}]") {
            Err(e) => {
                error!("Could not build RGB component capture regex: {}", e);
                Regex::new(".*").expect("Could not build empty regex")
            },
            Ok(r) => r,
        };

        ((String::from("compvalid"), validate), (String::from("compcapture"), capture))
    };

    // Create output.
    let mut out = Vec::with_capacity(4);

    out.push(full.0);
    out.push(full.1);
    out.push(comp.0);
    out.push(comp.1);

    report.end(start);

    (out, report)
}
