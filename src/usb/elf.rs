//! ELF parsing for the defmt decoder.



use defmt_decoder::{
    Encoding, Locations, Table,
};

use std::{
    sync::{
        Arc,
    },
};



pub struct Elf {
    /// Table used by the defmt decoder.
    pub(super) table: Table,

    /// Locations of the defmt data.
    pub(super) locations: Locations,

    /// The encoding of this ELF.
    pub(super) encoding: Encoding,
}

impl Elf {
    /// Parses a given (expected) ELF file.
    pub fn parse(bytes: Arc<[u8]>) -> Result<Self, ()> {
        // Extract the defmt table.
        let table = match defmt_decoder::Table::parse( &bytes[..] ) {
            Err(e) => return Err(()),
            Ok(maybe) => match maybe {
                Some(t) => t,
                _ => return Err(()),
            },
        };

        println!("{:?}", table.encoding());

        // Get the locations used by defmt.
        let locations = match table.get_locations( &bytes[..] ) {
            Ok(l) => l,
            _ => return Err(()),
        };

        // Check that there is enough information.
        if locations.is_empty() || table.indices().any(|idx| !locations.contains_key(&(idx as u64))) {
            return Err(());
        }

        // Get the encoding.
        let encoding = table.encoding();

        Ok( Self {
            table,
            locations,
            encoding,
        })
    }
}

unsafe impl Send for Elf {}
unsafe impl Sync for Elf {}
