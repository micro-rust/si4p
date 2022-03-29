//! Module for probe control and management.



use probe_rs::{
    Core, DebugProbeInfo, Probe, Session,

    CoreStatus, Error as ProbeError, HaltReason,

    MemoryInterface,

    config::TargetSelector,
};

use tokio::{
    sync::{
        mpsc, oneshot,
    },
};

use tracing::{
    debug, error, info, warn,
};


/// Asynchronous manager of a probe.
pub struct OpenProbe {
    /// Inner probe.
    inner: Session,

    /// A receiver of commands and response channels.
    cmds: mpsc::UnboundedReceiver<(Command, oneshot::Sender<Response>)>,

    /// Currently selected core.
    core: usize,
}

impl OpenProbe {
    /// Creates a new open probe from the given `DebugProbeInfo`.
    pub fn create(info: DebugProbeInfo, target: impl Into<TargetSelector>) -> Result<(Self, mpsc::UnboundedSender<(Command, oneshot::Sender<Response>)>), ProbeError> {
        // Open the probe.
        let probe = info.open()?;

        // Create a session.
        let inner = probe.attach(target)?;

        // Create the channels.
        let (tx, cmds) = mpsc::unbounded_channel();

        // Create the open probe.
        let openprobe = OpenProbe { inner, cmds, core: 0 };

        Ok((openprobe, tx))
    }

    /// Runs in a loop until the quit signal is received.
    pub async fn run(&mut self) {
        loop {
            match self.cmds.recv().await {
                None => {},

                Some(_) => {},
            }
        }
    }

    /// Reads `f32` bits from the given address.
    fn readf32(&mut self, address: u32) -> Result<f32, Error> {
        // Get the currently selected core.
        let mut core = self.getcore()?;

        // Check if the core is in the correct state.
        Self::corehalted(&mut core)?;

        // Perform the read.
        unsafe { core::mem::transmute( Self::rdword32(&mut core, address) ) }
    }

    /// Reads `u32` bits from the given address.
    fn readu32(&mut self, address: u32) -> Result<u32, Error> {
        // Get the currently selected core.
        let mut core = self.getcore()?;

        // Check if the core is in the correct state.
        Self::corehalted(&mut core)?;

        // Perform the read.
        Self::rdword32(&mut core, address)
    }

    /// Reads `u16` from the given address.
    fn readu16(&mut self, address: u32) -> Result<u16, Error> {
        // Get the currently selected core.
        let mut core = self.getcore()?;

        // Check if the core is in the correct state.
        Self::corehalted(&mut core)?;

        // Perform the reads.
        let lo = Self::rdword8(&mut core, address)?;
        let hi = Self::rdword8(&mut core, address + 2)?;

        Ok( ((hi as u16) << 8) | (lo as u16) )
    }

    /// Reads `u8` from the given address.
    fn readu8(&mut self, address: u32) -> Result<u8, Error> {
        // Get the currently selected core.
        let mut core = self.getcore()?;

        // Check if the core is in the correct state.
        Self::corehalted(&mut core)?;

        // Perform the read.
        Self::rdword8(&mut core, address)
    }

    /// Reads `i32` bits from the given address.
    fn readi32(&mut self, address: u32) -> Result<i32, Error> {
        // Get the currently selected core.
        let mut core = self.getcore()?;

        // Check if the core is in the correct state.
        Self::corehalted(&mut core)?;

        // Perform the read.
        unsafe { core::mem::transmute( Self::rdword32(&mut core, address) ) }
    }

    /// Reads `i16` from the given address.
    fn readi16(&mut self, address: u32) -> Result<i16, Error> {
        // Get the currently selected core.
        let mut core = self.getcore()?;

        // Check if the core is in the correct state.
        Self::corehalted(&mut core)?;

        // Perform the reads.
        let lo = Self::rdword8(&mut core, address)?;
        let hi = Self::rdword8(&mut core, address + 2)?;

        let out = ((hi as u16) << 8) | (lo as u16);

        unsafe { Ok( core::mem::transmute( out ) ) }
    }

    /// Reads `i8` from the given address.
    fn readi8(&mut self, address: u32) -> Result<i8, Error> {
        // Get the currently selected core.
        let mut core = self.getcore()?;

        // Check if the core is in the correct state.
        Self::corehalted(&mut core)?;

        // Perform the read.
        unsafe { core::mem::transmute( Self::rdword8(&mut core, address) ) }
    }

    /// Gets the currently selected core.
    fn getcore(&mut self) -> Result<Core, Error> {
        match self.inner.core(self.core) {
            Err(e) => {
                error!(origin="probe", "Could not attach to core {}: {}", self.core, e);
                return Err( Error::CoreNotFound(self.core) );
            },
            Ok(c) => Ok(c),
        }
    }

    /// Checks that the core is halted.
    fn corehalted(core: &mut Core) -> Result<(), Error> {
        match core.status() {
            Err(e) => {
                error!(origin="probe", "Could not get core status: {}", e);
                return Err( Error::UnknownCoreStatus );
            },
            Ok(s) => match s {
                CoreStatus::Halted(reason) => match reason {
                    HaltReason::Breakpoint => debug!(origin="probe", "Core found a breakpoint"),
                    HaltReason::Exception  => debug!(origin="probe", "Core is in exception mode"),
                    HaltReason::External   => debug!(origin="probe", "External halt reason"),
                    HaltReason::Request    => debug!(origin="probe", "Core was requested to halt"),
                    HaltReason::Step       => debug!(origin="probe", "Core stepped one instruction"),
                    HaltReason::Watchpoint => debug!(origin="probe", "Core found a watchpoint"),

                    HaltReason::Multiple => warn!(origin="probe", "Multiple halt reasons, reading memory may fail!"),
                    HaltReason::Unknown   => warn!(origin="probe", "Unknown halt reason, reading memory may fail!"),
                },

                CoreStatus::LockedUp => warn!(origin="probe", "Core is locked up, reading memory may fail!"),

                CoreStatus::Unknown => {
                    error!(origin="probe", "Core is in unknown status");
                    return Err( Error::UnknownCoreStatus );
                },

                CoreStatus::Running => {
                    error!(origin="probe", "Cannot read memory while core is running");
                    return Err( Error::CoreNotHalted );
                },

                CoreStatus::Sleeping => {
                    error!(origin="probe", "Cannot read memory while core is sleeping");
                    return Err( Error::CoreNotHalted );
                },
            },
        }

        Ok(())
    }

    /// Performs a read of a 32 bit word at the given address.
    /// Assumes all validation is performed.
    fn rdword32(core: &mut Core, address: u32) -> Result<u32, Error> {
        match core.read_word_32(address) {
            Err(e) => {
                error!("Failed to read 32 bit word at address {}: {}", address, e);
                Err( Error::Read32Failed(address) )
            },
            Ok(d) => Ok(d)
        }
    }

    /// Performs a read of a 8 bit word at the given address.
    /// Assumes all validation is performed.
    fn rdword8(core: &mut Core, address: u32) -> Result<u8, Error> {
        match core.read_word_8(address) {
            Err(e) => {
                error!("Failed to read 8 bit word at address {}: {}", address, e);
                Err( Error::Read8Failed(address) )
            },
            Ok(d) => Ok(d)
        }
    }

    /// Performs a read of a 8 bit word at the given address.
    /// Assumes all validation is performed.
    fn rdrange(&mut self, core: &mut Core, s: u32, e: u32) -> Result<Vec<u8>, Error> {
        // Check that the start and end are valid.
        if e < s  { return Err( Error::EndBeforeStart ) }
        if e == s { return Ok( Vec::new() )             }

        // Create an output array.
        let mut out = Vec::with_capacity(e as usize - s as usize);

        // Create an aligned start and end and get the length.
        let sa = match s & 0b11 {
            0 => s,
            _ => (s & !(0b11)) + 4,
        };

        let ea = match e & 0b11 {
            0 => e,
            _ => (e & !(0b11)) + 4,
        };

        let la = ea - sa;


        // Check if an unaligned read is necessary at the beginning.
        if sa > s {
            // Create a temp variable to iterate the address.
            let mut st = s;

            // Perform the aligned read.
            while st < sa {
                match Self::rdword8(core, st) {
                    Err(e) => return Err( e ),
                    Ok(b) => out.push(b),
                }

                st += 1;
            }
        }

        // Check if an aligned read is necessary.
        if la > 0 {
            // Create the buffer to store the memory.
            let mut buf = vec![0u8; la as usize];

            // Perform the aligned read.
            match core.read_mem_32bit(s, &mut buf) {
                Err(e) => {
                    error!("Failed to read {} bytes in 32 bit aligned mode from address {}: {}", la, sa, e);
                    return Err( Error::ReadRange32Failed(sa, ea) );
                },
                _ => (),
            }

            // Push the data into the output.
            out.extend_from_slice(&buf);
        }

        // Check if an unaligned read is necessary at the end.
        if ea < e {
            // Create a temp variable to iterate the address.
            let mut et = ea;

            // Perform the aligned read.
            while et < e {
                match Self::rdword8(core, et) {
                    Err(e) => return Err( e ),
                    Ok(b) => out.push(b),
                }

                et += 1;
            }
        }

        Ok( out )
    }
}




#[derive(Clone, Debug)]
pub enum Error {
    CoreNotFound(usize),

    UnknownCoreStatus,

    CoreNotHalted,

    Read32Failed(u32),

    Read8Failed(u32),

    ReadRange32Failed(u32, u32),

    EndBeforeStart,

}



pub enum Response {

}


pub enum Command {
    /// Reads an `i8` at the given address.
    ReadI8(u32),

    /// Reads an `u8` at the given address.
    ReadU8(u32),

    /// Reads an `i16` at the given address.
    ReadI16(u32),

    /// Reads an `u16` at the given address.
    ReadU16(u32),

    /// Reads an `i32` at the given address.
    ReadI32(u32),

    /// Reads an `u32` at the given address.
    ReadU32(u32),

    /// Reads an `i64` at the given address.
    ReadI64(u32),

    /// Reads an `u64` at the given address.
    ReadU64(u32),
}
