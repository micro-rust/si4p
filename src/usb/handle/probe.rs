//! Connection to the debug probe device.



use crate::{
    common::{
        Entry, Source,
    },

    gui::Message,

    target::core::RegisterType,
};

use probe_rs::{
    CoreStatus, DebugProbeInfo, Permissions, Probe, Session,
};

use std::{
    error::Error,
    sync::Arc,
    time::Duration,
};

use tokio::sync::{
    RwLock,

    mpsc::Sender,
};



pub struct ProbeHandle {
    /// Information on the debug probe selected.
    info: Option<DebugProbeInfo>,

    /// Debug Probe Target.
    target: Option<String>,

    /// USB Debug session.
    session: Option<Session>,
}

impl ProbeHandle {
    /// Creates a new empty connection handle.
    pub fn new() -> Self {
        Self { info: None, target: None, session: None, }
    }

    /// Opens the connection to the given device.
    pub fn open(&mut self, info: DebugProbeInfo) -> Result<bool, Box<dyn Error>> {
        // Save the information of the probe in case of a disconnection.
        self.info = Some(info);

        // Attempt to attach to a session.
        self.session()
    }

    /// Closes the connection to the given device.
    pub fn close(&mut self) -> bool {
        // Save the information of the probe in case of a disconnection.
        self.info = None;

        // Check if a session was open.
        let session = self.session.is_some();

        // Clear the session.
        self.session = None;

        session
    }

    /// Sets the target of the debug session. If a probe is already selected, open the session.
    pub fn target(&mut self, target: String) -> Result<bool, Box<dyn Error>> {
        // Save the target.
        self.target = Some(target);

        // Attempt to attach to a session.
        self.session()
    }

    pub fn notarget(&mut self) -> bool {
        // Clear the target.
        self.target = None;

        // Check if a session was open.
        let session = self.session.is_some();

        // Clear the session.
        self.session = None;

        session
    }

    /// If possible, opens a session with the current probe and target.
    fn session(&mut self) -> Result<bool, Box<dyn Error>> {
        // Attempt to open the session.
        let mut session = match (&self.info, &self.target) {
            (Some(info), Some(target)) => match Probe::open( info ) {
                Ok(probe) => match probe.attach(target, Permissions::new().allow_erase_all()) {
                    Ok(session) => session,

                    Err(e) => return Err( Box::new( e ) ),
                },

                Err(e) => return Err( Box::new( e ) ),
            },

            _ => return Ok( false ),
        };

        // Create the core information.
        let cores = crate::target::CoreInformation::parse(&mut session);

        // Set the global cores.
        *crate::usb::CORES.blocking_write() = cores.into_iter()
            .map(|core| Arc::new( RwLock::new( core ) ))
            .collect();

        // Save the session.
        self.session = Some(session);

        Ok( true )
    }

    /// Halts the given core.
    pub fn halt(&mut self, core: usize) -> Result<bool, probe_rs::Error> {
        // Timeout of the halt command.
        const TIMEOUT: Duration = Duration::from_secs(1);

        match &mut self.session {
            Some(session) => {
                // Get access to the core.
                let mut core = session.core(core)?;

                // Halt the core.
                core.halt( TIMEOUT )?;

                Ok( true )
            },

            _ => Ok( false ),
        }
    }

    /// Runs the given core.
    pub fn run(&mut self, core: usize) -> Result<bool, probe_rs::Error> {
        // Timeout of the halt command.
        const TIMEOUT: Duration = Duration::from_secs(1);

        match &mut self.session {
            Some(session) => {
                // Get access to the core.
                let mut core = session.core(core)?;

                // Run the core.
                core.run()?;

                Ok( true )
            },

            _ => Ok( false ),
        }
    }

    /// Checks for new data in the USB connection.
    pub fn update(&mut self) -> Result<Option<Vec<Entry>>, Entry> {
        // If there is no active session return.
        let session = match &mut self.session {
            Some(session) => session,
            _ => return Ok(None),
        };

        // Get a blocking read on the list of cores.
        let cores = super::super::CORES.blocking_read();

        // List of messages to return.
        let mut messages = Vec::new();

        for lock in cores.iter() {
            // Get a blocking write on each core.
            let mut information = lock.blocking_write();

            // Open the core in the session.
            let mut core = match session.core(information.index) {
                Err(_) => continue,
                Ok(c) => c,
            };

            // Update the status of the core.
            match core.status() {
                Ok(status) => information.status = status,
                _ => information.status = CoreStatus::Unknown,                
            }

            // If the core is not halted, skip reading the registers.
            match information.status {
                CoreStatus::Unknown => messages.push( Self::warn( format!("Failed to update core {} status", information.index) ) ),
                CoreStatus::Halted(_) => (),
                _ => continue,
            }

            // Read the registers of the core.
            for register in &mut information.cregs {
                match core.read_core_reg( register.id ) {
                    Ok(value) => match register.data {
                        RegisterType::FloatingPoint(_) => register.data = RegisterType::FloatingPoint( unsafe { core::mem::transmute(value) } ),
                        RegisterType::Unsigned(_) => register.data = RegisterType::Unsigned( value ),
                    },

                    _ => messages.push( Self::error( format!("Failed to read Core register {:?}", register.id) ) ),
                }
            }

            for register in &mut information.fregs {
                match core.read_core_reg( register.id ) {
                    Ok(value) => match register.data {
                        RegisterType::FloatingPoint(_) => register.data = RegisterType::FloatingPoint( unsafe { core::mem::transmute(value) } ),
                        RegisterType::Unsigned(_) => register.data = RegisterType::Unsigned( value ),
                    },

                    _ => messages.push( Self::error( format!("Failed to read FPU register {:?}", register.id) ) ),
                }
            }
        }

        Ok( Some( messages ) )
    }

    /// Flashes the given ELF file to the target.
    pub fn flash(&mut self, bytes: &[u8], console: &mut Sender<Message>) {
        use probe_rs::flashing::{ DownloadOptions, FlashProgress, ProgressEvent, };
        use std::io::Cursor;

        // Default timeout for flash operations.
        const TIMEOUT: Duration = Duration::from_secs(1);

        // Log the start of the flash process.
        let _ = console.try_send( Self::info( "Flashing the target" ).into() );

        // Check that there is a session open.
        let session = match &mut self.session {
            None => {
                // Send an error message.
                let _ = console.try_send( Self::error( "Failed to flash the target : No open session available" ).into() );

                return;
            },

            Some(session) => session,
        };

        // Log the start of the core halting.
        let _ = console.try_send( Self::debug( "Halting all cores" ).into() );

        // Halt all cores.
        let cores = session.list_cores();

        for (core_index, _) in cores {
            // Get the core and halt it.
            let _ = match session.core(core_index) {
                Ok(mut core) => match core.halt( TIMEOUT ) {
                    Err(e) => console.try_send( Self::warn( format!("Failed to halt core {} : {}", core_index, e) ).into() ),

                    _ => console.try_send( Self::debug( format!("Halted core {}", core_index) ).into() ),
                },

                Err(e) => console.try_send( Self::warn( format!("Failed to halt core {} : {}", core_index, e) ).into() ),
            };
        }

        // At least we have tried to nicely halt all cores.
        // Log the start of the flash process.
        let _ = console.try_send( Self::debug( "Downloading executable file" ).into() );

        // Get the flash loader.
        let mut loader = session.target().flash_loader();

        // Add the ELF file to the flash loader.
        let _ = match loader.load_elf_data( &mut Cursor::new(bytes) ) {
            Err(e) => {
                let _ = console.try_send( Self::error( format!("Failed to parse ELF data for flashing : {}", e) ).into() );
                return;
            },
            Ok(_) => console.try_send( Self::debug( "Successfully parsed ELF data for flashing" ).into() ),
        };

        // Create a console copy.
        let copy = console.clone();

        // Create the flash progress handler.
        let handler = move |event| {
            let _ = match event {
                ProgressEvent::Initialized { flash_layout } => copy.try_send( Self::debug( format!("Initiated flashing : {:?}", flash_layout) ).into() ),

                ProgressEvent::StartedErasing => copy.try_send( Self::debug( "Start erasing flash" ).into() ),
                ProgressEvent::StartedFilling => copy.try_send( Self::debug( "Start filling flash" ).into() ),
                ProgressEvent::StartedProgramming => copy.try_send( Self::debug( "Start programming flash" ).into() ),

                ProgressEvent::FailedErasing => copy.try_send( Self::error( "Failed erasing flash" ).into() ),
                ProgressEvent::FailedFilling => copy.try_send( Self::error( "Failed filling flash" ).into() ),
                ProgressEvent::FailedProgramming => copy.try_send( Self::error( "Failed programming flash" ).into() ),

                ProgressEvent::FinishedErasing => copy.try_send( Self::info( "Finished erasing flash" ).into() ),
                ProgressEvent::FinishedFilling => copy.try_send( Self::info( "Finished filling flash" ).into() ),
                ProgressEvent::FinishedProgramming => copy.try_send( Self::info( "Finished programming flash" ).into() ),

                ProgressEvent::SectorErased { size, time } => copy.try_send( Self::debug( format!("Erased sector ({} bytes) in {} ms", size, time.as_millis()) ).into() ),
                ProgressEvent::PageFilled { size, time } => copy.try_send( Self::debug( format!("Filled page ({} bytes) in {} ms", size, time.as_millis()) ).into() ),
                ProgressEvent::PageProgrammed { size, time } => copy.try_send( Self::debug( format!("Programmed page ({} bytes) in {} ms", size, time.as_millis()) ).into() ),

                ProgressEvent::DiagnosticMessage { message } => copy.try_send( Self::info( format!("Flash programmer message: {}", message) ).into() ),
            };
        };

        // Create the download options.
        let mut options = DownloadOptions::new();
        options.progress = Some( FlashProgress::new(handler) );

        // Log the start of the flash process.
        let _ = console.try_send( Self::debug( "Flashing target" ).into() );

        // Commit the download.
        let _ = match loader.commit(session, options) {
            Err(e) => console.try_send( Self::error( format!("Flashing failed : {}", e) ).into() ),
            Ok(_) => console.try_send( Self::info("Successfully flashed target").into() ),
        };
    }

    /// Creates a debug message.
    fn debug<S>(text: S) -> Entry where String: From<S> {
        Entry::debug( Source::Host, String::from(text) )
    }

    /// Creates an error message.
    fn error<S>(text: S) -> Entry where String: From<S> {
        Entry::error( Source::Host, String::from(text) )
    }

    /// Creates an info message.
    fn info<S>(text: S) -> Entry where String: From<S> {
        Entry::info( Source::Host, String::from(text) )
    }

    /// Creates a warn message.
    fn warn<S>(text: S) -> Entry where String: From<S> {
        Entry::warn( Source::Host, String::from(text) )
    }
}
