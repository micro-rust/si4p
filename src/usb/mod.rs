//! Interface to a `defmt` USB logger.



mod command;
pub mod device;
mod elf;



pub use command::Command;

use crate::common::{
    Entry, Source,
};

use defmt_decoder::StreamDecoder;

use device::USBDevice;

use elf::Elf;

use rusb::{
    Context, UsbContext, DeviceHandle, GlobalContext,
};

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

// Currently active ELF contents.
static mut ELF: Option<Elf> = None;

// Currently active decoder.
static mut DECODER: Option<Box<dyn StreamDecoder>> = None;


pub struct USBLogger {
    /// Context of the USB logger.
    context: Context,

    /// A channel to receive commands.
    commands: Receiver<Command>,

    /// A channel to send console entries.
    console: Sender<Entry>,

    /// The connection to the USB logger.
    connection: Option<(DeviceHandle<GlobalContext>, u8)>,

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
            connection: None,
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
            //match self.context.handle_events( Some(HANDLETIMEOUT) ) {
            //    Err(e) => self.error( format!("Failed to handle USB context events: {}", e) ),
            //    _ => (),
            //}

            // Update the list of connected devices.
            self.list();

            // Check for commands and if the application is closed.
            if self.commands() {
                break 'usb;
            }

            // Check for new data in the USB connection.
            self.update();

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
                Command::Open(key, idx, num, alt, ep, bus) => self.open(key, idx, num, alt, ep, bus),

                // Sets the active deftm file.
                Command::SetDefmtFile( bytes ) => self.file( bytes ),

                // Quit command. Close everything.
                Command::Quit => return true,

                _ => (),
            }
        }

        false
    }

    /// Opens the connection to the given device.
    fn open(&mut self, key: usize, idx: u8, num: u8, alt: u8, ep: u8, (bus, address): (u8, u8)) {
        use rusb::DeviceList;

        // List of connected devices.
        let connected = CONNECTED.blocking_read();

        // Get the desired device.
        let device = match connected.get(&key) {
            Some(device) => device,
            _ => {
                self.error( format!("Failed to get device from key {}", key) );
                return;
            },
        };

        // Once the device info is available, get the ids and serial.
        let (vid, pid) = device.ids();
        let serial = device.serial().clone();

        // Get the list of devices.
        let list = match DeviceList::new() {
            Err(e) => {
                self.error(format!("Failed to read device list: {}", e));
                return;
            },
            Ok(list) => list,
        };

        // Exact and Possible devices that match VID:PID.
        let mut exact = None;

        for device in list.iter() {
            // Get device descriptor.
            let descriptor = match device.device_descriptor() {
                Err(e) => {
                    self.error( format!("Failed to read device descriptor: {}", e) );
                    continue;
                },
                Ok(desc) => desc,
            };

            if (descriptor.vendor_id() == vid) && (descriptor.product_id() == pid) && (device.bus_number() == bus) && (device.address() == address) {
                exact = Some(device);
                break;
            }
        }

        // Extract the device.
        let device = match exact {
            Some(d) => d,
            _ => {
                self.error( format!("Could not find device {:04X}:{:04X} @ {:03}:{:03}", vid, pid, bus, address) );
                return;
            },
        };

        self.debug( format!("Found device {:04X}:{:04X} @ {:03}:{:03}", vid, pid, bus, address) );

        // Open the device.
        let mut handle = match device.open() {
            Err(e) => {
                self.error( format!("Failed to open device {:04X}:{:04X} @ {:03}:{:03} : {}", vid, pid, bus, address, e) );
                return;
            },

            Ok(handle) => handle,
        };

        self.info( format!("Opened device {:04X}:{:04X} @ {:03}:{:03}", vid, pid, bus, address) );

        // Check for a kernel driver.
        match handle.kernel_driver_active(num) {
            Ok(true) => {
                self.debug( "Detaching kernel driver..." );
                
                match handle.detach_kernel_driver(num) {
                    Err(e) => {
                        self.error( format!("Failed to detach kernel driver : {}", e) );
                        return;
                    },

                    _ => self.debug( "Kernel driver detached successfully" ),
                }
            },

            Err(e) => {
                self.error("Cannot check if device has kernel driver attatched" );
                return;
            },

            _ => (),
        }

        // Configure the device handle.
        match handle.set_active_configuration(idx) {
            Err(e) => {
                self.error( format!("Failed to set active configuration {} on device {:04X}:{:04X} @ {:03}:{:03} : {}", idx, vid, pid, bus, address, e) );
                return;
            },

            _ => (),
        }

        match handle.claim_interface(num) {
            Err(e) => {
                self.error( format!("Failed to claim interface {} on configuration {} on device {:04X}:{:04X} @ {:03}:{:03} : {}", num, idx, vid, pid, bus, address, e) );
                return;
            },

            _ => (),
        }

        match handle.set_alternate_setting(num, alt) {
            Err(e) => {
                self.error( format!("Failed to set setting {} on interface {} on configuration {} on device {:04X}:{:04X} @ {:03}:{:03} : {}", alt, num, idx, vid, pid, bus, address, e) );
                return;
            },

            _ => (),
        }

        self.info( format!("Configured device {:04X}:{:04X} @ {:03}:{:03}", vid, pid, bus, address) );

        // Set the handle on the logger.
        self.connection = Some((handle, ep));
    }

    /// Checks for new data in the USB connection.
    fn update(&mut self) {
        use defmt_decoder::DecodeError;

        // Check if a connection is open.
        let (handle, endpoint) = match &mut self.connection {
            Some((handle, ep)) => (handle, *ep),
            _ => return,
        };

        // Create a buffer.
        let mut buf = [0u8; 1024];

        // Set the timeout.
        let timeout = Duration::from_millis(250);

        // Try to read from the connection.
        let len = match handle.read_bulk(endpoint | 0x80, &mut buf, timeout) {
            Err(e) => match e {
                rusb::Error::Timeout => return,
                _ => {
                    self.error( format!("USB logger failed to read from endpoint {}: {}", endpoint, e) );
                    return;
                },
            },
            Ok(len) => len,
        };

        self.debug( format!("USB logger read {} bytes", len) );

        // If there is no decoder currently available, skip.
        let decoder = match unsafe { DECODER.as_mut() } {
            Some(d) => d,
            _ => {
                self.error( "No defmt decoder available" );
                return;
            },
        };

        // Stream the bytes into the decoder.
        decoder.received(&buf[0..len]);

        // Decode the stream.
        loop {
            match decoder.decode() {
                Ok(frame) => {
                    // Get the file, mod and line.
                    let (modpath, line) = match unsafe { ELF.as_ref() } {
                        Some(elf) => match elf.locations.get(&frame.index()) {
                            Some(location) => {
                                (location.module.clone(), location.line)
                            },
                            _ => (String::new(), 0),
                        },
                        _ => (String::new(), 0),
                    };

                    // Get the timestamp.
                    let timestamp = frame.display_timestamp()
                        .map(|ts| ts.to_string())
                            .unwrap_or_default();

                    // Create the text.
                    let text = format!(
                        "{{{}}} {}\nLine {} - {}",
                        timestamp, frame.display_message().to_string(),
                        line, modpath,
                    );

                    // Create the entry.
                    let entry = match frame.level().unwrap_or( defmt_parser::Level::Info ) {
                        defmt_parser::Level::Trace => Entry::trace(Source::Target, text),
                        defmt_parser::Level::Debug => Entry::debug(Source::Target, text),
                        defmt_parser::Level::Info  => Entry::info(Source::Target, text),
                        defmt_parser::Level::Warn  => Entry::warn(Source::Target, text),
                        defmt_parser::Level::Error => Entry::error(Source::Target, text),
                    };

                    self.txconsole(entry);
                },

                Err(e) => match e {
                    DecodeError::UnexpectedEof => break,
                    _ => self.warn("Possible data loss / corruption in defmt stream"),
                },
            }
        }
    }

    /// Opens and parses the given ELF file.
    fn file(&mut self, bytes: std::sync::Arc<[u8]>) {
        // Parse the next ELF contents.
        let new = match Elf::parse(bytes) {
            Err(e) => {
                self.error( format!("Failed to parse new ELF file : {:?}", e) );
                return;
            },
            Ok(e) => e,
        };

        self.debug( "Successfully parsed new defmt file" );

        // Get the encoding.
        let encoding = new.encoding;

        unsafe {
            // Delete the current stream decoder.
            // This order is important for lifetimes.
            DECODER = None;

            // Change the ELF file.
            unsafe { ELF = Some(new) };

            // Generate a new decoder.
            DECODER = Some( ELF.as_mut().unwrap().table.new_stream_decoder() );
        }

        self.info( format!("New defmt stream created with encoding {:?}", encoding ) );
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

unsafe impl Send for USBLogger {}
unsafe impl Sync for USBLogger {}
