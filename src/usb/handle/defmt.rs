//! Connection to the USB defmt device.
//! 



use crate::common::{
    Entry, Source,
};

use defmt_decoder::Frame;

use rusb::{
    DeviceHandle, GlobalContext,
};

use super::{
    common::USBTarget,
    decoder::{
        DefmtInfo, DEFMT, DECODER,
    },
};



pub struct DefmtHandle {
    /// USB device handle.
    device: Option<DeviceHandle<GlobalContext>>,

    /// Endpoint used in the connection.
    endpoint: u8,
}

impl DefmtHandle {
    /// Creates a new empty connection handle.
    pub fn new() -> Self {
        Self { device: None, endpoint: 0u8, }
    }

    /// Opens the connection to the given device.
    pub fn open(&mut self, target: USBTarget) -> Result<Option<(u16, u16)>, rusb::Error> {
        use rusb::DeviceList;

        // Get the list of devices.
        let list = DeviceList::new()?;

        // Selected device (if found).
        let mut selected = None;

        for device in list.iter() {
            // Get the device descriptor.
            let descriptor = match device.device_descriptor() {
                Ok(descriptor) => descriptor,
                _ => continue,
            };

            // Get the necessary device information.
            let (vid, pid) = (descriptor.vendor_id(), descriptor.product_id());
            let (bus, address) = (device.bus_number(), device.address());

            // Check if the device matches the information.
            if target.matches(vid, pid, bus, address) {
                selected = Some((device, (vid, pid)));
                break;
            }
        }

        // Unwrap the device.
        let (device, ids) = match selected {
            Some(device) => device,
            _ => return Ok(None),
        };

        // Open the device.
        let mut handle = device.open()?;

        // Check for a kernel driver and detach it.
        if handle.kernel_driver_active(target.iface)? {
            handle.detach_kernel_driver(target.iface)?;
        }

        // Configure the device handle.
        handle.set_active_configuration(target.config)?;
        handle.claim_interface(target.iface)?;
        handle.set_alternate_setting(target.iface, target.setting)?;

        // Set the handle on the logger.
        self.device = Some(handle);
        self.endpoint = target.endpoint | 0x80;

        Ok(Some(ids))
    }

    /// Checks for new data in the USB connection.
    pub fn update(&mut self) -> Result<Option<Vec<Entry>>, Entry> {
        use defmt_decoder::DecodeError;
        use rusb::Error;
        use std::time::Duration;

        // Check if a connection is open.
        let handle = match &mut self.device {
            Some(handle) => handle,
            _ => return Ok(None),
        };

        // Check if there is defmt information available.
        let info = match unsafe { DEFMT.as_mut() } {
            Some(d) => d,
            _ => return Err( self.error( "No defmt information available" ) ),
        };

        // Check if there is a decoder available.
        let decoder = match unsafe { DECODER.as_mut() } {
            Some(d) => d,
            _ => return Err( self.error( "No defmt decoder available" ) ),
        };

        // Create a buffer for the incoming data.
        let mut buf = [0u8; 1024];

        // Set the timeout.
        // TODO : Make this configurable
        let timeout = Duration::from_millis(250);

        // Try to read from the connection.
        let len = match handle.read_bulk(self.endpoint, &mut buf, timeout) {
            Err(e) => {
                // Message to return to the app.
                let message = match e {
                    // Device is busy or no data is ready : Wait for device
                    Error::Busy | Error::Timeout => Ok(None),

                    // Device removed or reconfigured : Close connection
                    Error::Access | Error::NoDevice | Error::NotFound => Err( self.error( format!("USB logger failed to read from device (Closing connection) : {}", e) ) ),

                    // Operation not supported : Close connection
                    Error::NotSupported => Err( self.error( format!("USB logging not supported on this platform (Closing connection) : {}", e) ) ),

                    // Recoverable errors : Warn user and wait
                    Error::Interrupted | Error::NoMem | Error::BadDescriptor |
                    Error::Pipe | Error::Io | Error::InvalidParam => Err( self.warn( format!("USB logger failed to read from device (Recoverable error) : {}", e) ) ),

                    // Buffer oveflow : Close connection
                    Error::Overflow => Err( self.error( format!("USB buffer overflow, for security reasons the connection will be closed : {}", e) ) ),

                    // Unknown error : Close connection
                    Error::Other => Err( self.error( "Unknown USB error (Closing connection)" ) ),
                };

                // This checks if the connection must be closed.
                let closed = match e {
                    Error::Access | Error::NoDevice | Error::NotFound |
                    Error::Overflow | Error::Other => true,
                    _ => false,
                };

                // Close the connection if necesary.
                if closed {
                    self.device = None;
                }

                return message;
            },

            Ok(len) => len,
        };

        // Stream the bytes into the decoder.
        decoder.received( &buf[0..len] );

        // Create the list of new messages.
        let mut mail = Vec::new();

        // Decode the frames.
        loop {
            match decoder.decode() {
                Ok(frame) => mail.push( self.deframe( frame, info ) ),

                Err(e) => match e {
                    DecodeError::Malformed => mail.push( self.warn("Possible data loss / corruption in defmt stream") ),
                    DecodeError::UnexpectedEof => break,
                },
            }
        }

        Ok( Some( mail ) )
    }

    /// Converts a message frame to a console message.
    fn deframe(&self, frame: Frame, info: &mut DefmtInfo) -> Entry {
        use defmt_parser::Level as DefmtLevel;

        // Get the file, module and line.
        let (modpath, line) = match info.locations.get(&frame.index()) {
            Some(location) => (location.module.clone(), location.line),
            _ => (String::new(), 0),
        };

        // Get the timestamp.
        let timestamp = match frame.display_timestamp() {
            Some(ts) => ts.to_string(),
            _ => String::new(),
        };

        // Get the display message.
        let message = frame.display_message().to_string();

        // Create the text.
        let text = format!("{{{}}} {}\nLine {} - {}", timestamp, message, line, modpath);

        match frame.level().unwrap_or( DefmtLevel::Info ) {
            DefmtLevel::Trace => Entry::trace(Source::Target, text),
            DefmtLevel::Debug => Entry::debug(Source::Target, text),
            DefmtLevel::Info  => Entry::info(Source::Target, text),
            DefmtLevel::Warn  => Entry::warn(Source::Target, text),
            DefmtLevel::Error => Entry::error(Source::Target, text),
        }
    }

    /// Creates an error message.
    fn error<S>(&mut self, text: S) -> Entry where String: From<S> {
        Entry::error( Source::Host, String::from(text) )
    }

    /// Creates a warn message.
    fn warn<S>(&mut self, text: S) -> Entry where String: From<S> {
        Entry::warn( Source::Host, String::from(text) )
    }
}
