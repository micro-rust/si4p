//! Commands related to the ELF files.



use crate::common::{
    Entry, Source,
};

use std::path::PathBuf;

use super::Message;



/// Async function to select an ELF file.
pub async fn selectELF(maybe: Option<PathBuf>) -> Message {
    use rfd::FileDialog;

    // Extract the path.
    let path = match maybe {
        Some(path) => path,
        _ => PathBuf::from("/"),
    };

    // Spawn the sync thread to select the file.
    let thread = tokio::task::spawn_blocking(move || {
        return FileDialog::new()
            .set_directory(path)
            .pick_file()
    });

    // Check the result of the file pick.
    match thread.await {
        Err(e) => Message::ConsoleEntry( Entry::error( Source::Host, format!("Failed to select an executable file: {}", e) ) ),

        Ok(maybe) => match maybe {
            Some(path) => Message::LoadELF( PathBuf::from( path ) ),
            None => Message::None,    
        },
    }
}



/// Async function to read an ELF file.
pub async fn loadELF(path: PathBuf) -> Message {
    use std::{
        sync::Arc,
    };

    use tokio::{
        fs::File,
        io::AsyncReadExt,
    };

    // Create the file.
    let file = match File::open(path.clone()).await {
        Err(_) => return Message::None,
        Ok(f) => f,
    };

    // Create the buffer.
    let mut data = Vec::new();

    // Create the reader.
    let mut reader = tokio::io::BufReader::new(file);

    // Read the file.
    match reader.read_to_end(&mut data).await {
        Err(_) => return Message::None,
        Ok(_) => (),
    }

    // Raw data.
    let raw: Arc<[u8]> = Arc::from( data );

    // DO SOME DWARF TESTS HERE
    let elf = {
        use gimli::AttributeValue;

        use std::{
            borrow::Cow,

            ffi::{
                CStr, CString,
            },
        };

        // Create the ELF object.
        let elf = micro_elf::elf::ELFObject::parse(raw.clone()).expect("Failed to parse ELF file");
        /*

        // Create a load section.
        let load = |id: gimli::SectionId| -> Result<Cow<[u8]>, gimli::Error> {
            match elf.section( id.name() ) {
                Some(section) => match elf.content( section ) {
                    Some( content ) => Ok( Cow::Borrowed( &content[..] ) ),
                    _ => Ok( Cow::Borrowed(&[][..]) )
                },
                None => Ok( Cow::Borrowed( &[][..]) ),
            }
        };

        // Get the .debug_str section, if available.
        let debugstr = match elf.section( ".debug_str" ) {
            Some( section ) => elf.content( section ),
            _ => None,
        };

        // Get the .debug_line section, if available.
        let debugstr = match elf.section( ".debug_line" ) {
            Some( section ) => elf.content( section ),
            _ => None,
        };

        // Get the endianness.
        let endian = if elf.endianness().little() { gimli::RunTimeEndian::Little } else { gimli::RunTimeEndian::Big };

        // Load all of the sections.
        let dwarf = gimli::Dwarf::load( &load ).expect("Failed to load DWARF data");

        // Borrow a Cow to create an EndianSlice.
        let borrow: &dyn for<'a> Fn(&'a Cow<[u8]>) -> gimli::EndianSlice<'a, gimli::RunTimeEndian> = &|section| gimli::EndianSlice::new(&*section, endian);

        // Create slices of all sections.
        let dwarf = dwarf.borrow(&borrow);

        // Iterate over the compilation units.
        let mut iter = dwarf.units();

        while let Some(header) = iter.next().expect("Failed to get DWARF unit") {
            println!("Unit @ <debug_info+0x{:X}>", header.offset().as_debug_info_offset().unwrap().0);

            let unit = dwarf.unit(header).expect("Failed to load DWARF unit");

            let mut depth = 0;

            let mut entries = unit.entries();

            while let Some( (delta, entry) ) = entries.next_dfs().expect("Failed to read DFS") {
                depth += delta;

                //println!("  <{}><{:X}> {}", depth, entry.offset().0, entry.tag());

                // Iterate over the attributes in the DIE.
                let mut attrs = entry.attrs();

                while let Some(attr) = attrs.next().expect("Failed to read an attribute") {
                    // Get the debug value of the attribute just in case.
                    let mut value = format!("{:?}", attr.value());

                    // Format the value of the attribute.
                    match attr.value() {
                        // .debug_str reference.
                        AttributeValue::DebugStrRef( offset ) => if let Some( debugstr ) = debugstr {
                            if debugstr.len() > offset.0 {
                                if let Ok( cstr ) = CStr::from_bytes_until_nul( &debugstr[offset.0..] ) {
                                    value = String::from_utf8_lossy( cstr.to_bytes() ).to_string();
                                }    
                            }
                        },

                        // Raw string attribute value.
                        AttributeValue::String( slice ) => value = String::from(slice.to_string_lossy()),

                        _ => (),
                    }

                    // Display the information.
                    //println!("    {}: {}", attr.name(), value);
                }
            }
        }
        */

        elf
    };

    Message::NewELF( raw, Arc::from(elf), path )
}
