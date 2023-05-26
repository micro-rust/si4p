//! Interface to a `defmt` USB logger.



mod command;
pub mod device;



pub use command::Command;

use crate::common::{
    Entry, Source,
};

use device::USBDevice;

use rusb::{
    Context, Registration, UsbContext,
};

use seqid::impls::SeqHashMap;

use std::{
    collections::HashMap,
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



/// Global list of connected USB devices.
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

    /// Duration of the sleep interval.
    interval: Duration,}

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
        let mut logger = USBLogger {
            context,
            commands,
            console,
            interval: Duration::from_millis(1000),
        };

        Some((logger, sender))
    }

    /// Runs indefinitely the USB logger.
    pub fn run(mut self) {
        // Handle events timeout. 
        const HANDLETIMEOUT: Duration = Duration::from_millis(250);

        'usb: loop {
            // Handle the events of the USB context.
            match self.context.handle_events( Some(HANDLETIMEOUT) ) {
                Err(e) => self.error( format!("Failed to handle USB context events: {}", e) ),
                _ => (),
            }

            // Update the list of connected devices.
            self.list();

            // Check for commands and if the application is closed.
            if self.commands() {
                break 'usb;
            }

            // Check for new data in the USB.
            //self.update();

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
                // Quit command. Close everything.
                Command::Quit => return true,

                _ => (),
            }
        }

        false
    }

    /// Creates the list of all devices currently connected.
    fn list(&mut self) {
        use rusb::DeviceList;

        // Get the list of devices.
        let list = match DeviceList::new() {
            Err(e) => {
                self.error(format!("Failed to read device list: {}", e));
                return;
            },
            Ok(list) => list,
        };

        // Arriving devices.
        let arriving = self.arriving(&list);

        // Leaving devices.
        let leaving = self.leaving(&list);

        // Insert the devices into the global list.
        let mut glist = CONNECTED.blocking_write();

        // Insert the new devices.
        for new in arriving.into_iter() {
            let ids = new.ids();

            match glist.insert(new) {
                None => self.error(format!("Failed to insert device {:04X}:{:04X} in the map: Ran out of indexable space. Requires an application restart.", ids.0, ids.1)),
                _ => self.info(format!("Device {:04X}:{:04X} plugged in", ids.0, ids.1)),
            }
        }

        // Insert the old devices.
        for old in leaving.iter() {
            match glist.remove(old) {
                None => self.error(format!("Removed device with key {} does not exist", old)),
                Some(device) => self.info(format!("Device {:04X}:{:04X} plugged out", device.ids().0, device.ids().1)),
            }
        }
    }

    /// Checks for arriving devices.
    fn arriving<C: rusb::UsbContext>(&mut self, list: &rusb::DeviceList<C>) -> Vec<USBDevice> {
        // Get a read lock on the device list.
        // It's okay to block because this thread is the only one to write.
        let connected = CONNECTED.blocking_read();

        // Hotplugged devices.
        let mut arriving = Vec::new();

        // Check if all the devices are already listed.
        for device in list.iter() {
            // Get the descriptor.
            let descriptor = match device.device_descriptor() {
                Ok(d) => d,
                _ => continue,
            };

            // Check if the VID and PID are in the list.
            let ids = (descriptor.vendor_id(), descriptor.product_id());

            // If the device is in the list, skip.
            // TODO: Match serial numbers to allow for multiple debuggers to be connected at the same time.
            if connected.values().any(|device| device.ids() == ids) {
                continue;
            }

            // Attempt to build a device info for the device.
            match USBDevice::build(device) {
                Some(d) => arriving.push(d),
                _ => (),
            }
        }

        arriving
    }

    /// Checks for leaving devices.
    fn leaving<C: rusb::UsbContext>(&mut self, list: &rusb::DeviceList<C>) -> Vec<usize> {
        // Get a read lock on the device list.
        // It's okay to block because this thread is the only one to write.
        let connected = CONNECTED.blocking_read();

        // Get all the descriptors of the devices.
        let descriptors: Vec<rusb::DeviceDescriptor> = list.iter()
            .map(|device| device.device_descriptor())
            .filter(|maybe| maybe.is_ok())
            .map(|ok| ok.unwrap())
            .collect();

        // List of leaving devices' indices.
        let mut leaving = Vec::new();

        for (key, device) in connected.iter() {
            // Check if the device is in the currently connected (and accessible) list.
            if descriptors.iter().any(|descriptor| device.ids() == (descriptor.vendor_id(), descriptor.product_id())) {
                continue;
            }

            // The device is no on the list, mark it to be removed.
            leaving.push( key.clone() );
        }

        leaving
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
