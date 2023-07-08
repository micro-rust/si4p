//! Interface to a `defmt` USB logger.



mod command;
mod defmt;
mod handle;
mod hotplug;


pub mod common;
pub mod device;


pub use command::Command;

use crate::common::{
    Entry, Source,
};

use rusb::Context;

use seqid::impls::SeqHashMap;

use std::{
    sync::Arc,
    time::Duration,
};

use tokio::{
    sync::RwLock,
};

use tokio::sync::mpsc::{
    channel,

    Receiver, Sender,

    error::{
        TryRecvError, TrySendError,
    },
};



// Global list of connected USB devices.
lazy_static::lazy_static!{
    pub static ref CONNECTED: Arc<RwLock<SeqHashMap<usize, device::USBDevice>>> = Arc::new( RwLock::new( SeqHashMap::new().unwrap() ) );
}


pub struct USBLogger {
    /// Context of the USB logger.
    context: Context,

    /// A channel to receive commands.
    commands: Receiver<Command>,

    /// A channel to send console entries.
    console: Sender<Entry>,

    /// The connection to the USB `defmt` device.
    defmtusb: handle::DefmtHandle,

    /// Hotplug handler.
    hotplug: hotplug::Hotplug,

    /// Duration of the sleep interval.
    interval: Duration,
}

impl USBLogger {
    /// Attempts to create a new USB `defmt` logger.
    pub fn new(console: Sender<Entry>) -> Option<(Self, Sender<Command>)> {
        // Create a new USB context.
        let context = match Context::new() {
            Err(e) => {
                // Create the entry.
                let entry = Entry::error(Source::Host, format!("Failed to create USB context: {}", e) );

                // Send the error, best effort.
                let _ = console.try_send( entry );

                return None;
            },

            Ok(c) => c,
        };

        // Create a new pair of command channels.
        let (sender, commands) = channel(128);

        // Create the logger.
        let logger = USBLogger {
            context,
            commands,
            console,
            defmtusb: handle::DefmtHandle::new(),
            hotplug: hotplug::Hotplug::new( CONNECTED.clone() ),
            interval: Duration::from_millis(1000),
        };

        Some((logger, sender))
    }

    /// Runs indefinitely the USB logger.
    pub fn run(mut self) {

        'usb: loop {
            // Update the list of connected devices.
            match self.hotplug.update() {
                Ok( (arriving, leaving) ) => {
                    for (vid, pid) in arriving.iter() {
                        self.info( format!("Device {:04X}:{:04X} plugged in", vid, pid) );
                    }
        
                    for (vid, pid) in leaving.iter() {
                        self.info( format!("Device {:04X}:{:04X} plugged out", vid, pid) );
                    }
                },

                Err(e) => self.error( format!("Failed to update available devices : {}", e) ),
            }

            // Check for commands and if the application is closed.
            if self.commands() {
                break 'usb;
            }

            // Check for new data in the defmt USB.
            match self.defmtusb.update() {
                Err(error) => self.txconsole(error),
                Ok(maybe) => match maybe {
                    Some(messages) => for entry in messages.into_iter() {
                        self.txconsole( entry )
                    },
                    _ => (),
                },
            }

            // Sleeps the thread to avoid taking too much CPU.
            // TODO : Make this delay configurable.
            self.sleep();
        }
    }

    /// Checks for commands and executes them.
    /// Returns `true` if a disconnection is necessary.
    fn commands(&mut self) -> bool {
        'cmd: loop {
            // Try to receive the next command.
            let cmd = match self.commands.try_recv() {
                Err(e) => {
                    // Log unexpected diconnections.
                    if e == TryRecvError::Disconnected {
                        self.warn("Command channel closed: Closing USB logger...");
                        return true;
                    }

                    break 'cmd;
                },

                Ok(c) => c,
            };

            // Check which command was received.
            match cmd {
                // Request to open a connection.
                Command::DefmtOpen(target) => match self.defmtusb.open(target) {
                    Ok(maybe) => match maybe {
                        Some((vid, pid)) => self.info( format!("Opened defmt USB device {:04X}:{:04X}", vid, pid) ),
                        _ => self.error( "No defmt USB device was found, it may have been disconnected" ),
                    },

                    Err(e) => self.error( format!("Failed to open defmt USB connection : {}", e) ),
                },

                // Sets the active deftm file.
                Command::SetDefmtFile( bytes ) => match defmt::DefmtInfo::create( bytes ) {
                    Some(encoding) => self.info( format!("Created a new defmt decoder with {:?} encoding", encoding) ),
                    _ => self.error( "Failed to create a defmt decoder from the given file" ),
                },

                // Quit command. Close everything.
                Command::Quit => return true,

                _ => (),
            }
        }

        false
    }

    /// Sleeps the thread to avoid taking too much CPU time. 
    fn sleep(&self) {
        std::thread::sleep( self.interval );
    }

    /// Sends an error entry to the console.
    fn error<S>(&mut self, text: S) where String: From<S> {
        // Create the entry.
        let entry = Entry::error( Source::Host, String::from(text) );

        // Send it to the console.
        self.txconsole(entry);
    }

    /// Sends a warn entry to the console.
    fn warn<S>(&mut self, text: S) where String: From<S> {
        // Create the entry.
        let entry = Entry::warn( Source::Host, String::from(text) );

        // Send it to the console.
        self.txconsole(entry);
    }

    /// Sends an info entry to the console.
    fn info<S>(&mut self, text: S) where String: From<S> {
        // Create the entry.
        let entry = Entry::info( Source::Host, String::from(text) );

        // Send it to the console.
        self.txconsole(entry);
    }

    /// Sends a debug entry to the console.
    fn debug<S>(&mut self, text: S) where String: From<S> {
        // Create the entry.
        let entry = Entry::debug( Source::Host, String::from(text) );

        // Send it to the console.
        self.txconsole(entry);
    }

    /// Sends the given entry to the console.
    fn txconsole(&mut self, entry: Entry) {
        // TODO : Some action to log this.
        match self.console.try_send( entry ) {
            Err(e) => match e {
                TrySendError::Full(_) => (),
                TrySendError::Closed(_) => (),
            },

            _ => (),
        }
    }
}

unsafe impl Send for USBLogger {}
unsafe impl Sync for USBLogger {}
