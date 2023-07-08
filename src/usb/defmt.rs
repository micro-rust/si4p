//! Parsing and creation of `defmt` decoders.



use defmt_decoder::{
    Encoding, Locations,
    StreamDecoder, Table,
};



// Currently active defmt information.
pub(super) static mut DEFMT: Option<DefmtInfo> = None;

// Currently active decoder.
pub(super) static mut DECODER: Option<Box<dyn StreamDecoder>> = None;



/// Information about `defmt` extracted from the binary file.
pub struct DefmtInfo {
    /// Table used by the defmt decoder.
    pub(super) table: Table,

    /// Locations of the defmt data.
    pub(super) locations: Locations,

    /// The encoding of this ELF.
    /// Keep this field for debugging.
    #[allow(dead_code)]
    pub(super) encoding: Encoding,
}

impl DefmtInfo {
    /// Parses the given file and stores it as the global `defmt` configuration.
    pub fn create(bytes: std::sync::Arc<[u8]>) -> Option<Encoding> {

        // Extract the defmt table.
        let table = match Table::parse( &bytes[..] ) {
            Ok(maybe) => match maybe {
                Some(table) => table,
                _ => return None,
            },

            _ => return None,
        };

        // Get the locations used by defmt.
        let locations = match table.get_locations( &bytes[..] ) {
            Ok(locations) => locations,
            _ => return None,
        };

        // Check that there is enough information.
        if locations.is_empty() || table.indices().any( |idx| !locations.contains_key(&(idx as u64)) ) {
            return None;
        }

        // Get the encoding.
        let encoding = table.encoding();

        // Set the information and decoder.
        unsafe {
            // Unset the decoder.
            DECODER = None;

            // Set the new information.
            DEFMT = Some( DefmtInfo { table, locations, encoding } );

            // Set the new decoder.
            DECODER = Some( DEFMT.as_mut().unwrap().table.new_stream_decoder() );
        }

        Some(encoding)
    }
}

unsafe impl Send for DefmtInfo {}
unsafe impl Sync for DefmtInfo {}
