//! Commands related to the SVD files.



use svd_parser::svd::{MaybeArray, RegisterCluster, Access, RegisterProperties, access, WriteConstraint};

use crate::{
    gui::console::{
        Entry, Source,
    },
    library::Library,

    target::Peripheral,
};

use std::sync::Arc;

use super::Message;



/// Async function to read a SVD file.
pub async fn loadSVD(name: String, library: Arc<Library>) -> Message {
    use tokio::{
        fs::File,
        io::AsyncReadExt,
    };

    println!("Loading SVD");

    // Get the path to the SVD file.
    let path = match library.svd.read().await.file(&name) {
        None => return Entry::error(Source::Host, format!("Failed to load SVD file : No file for target {}", name)).into(),
        Some(path) => (*path).clone(),
    };

    // Create the file.
    let file = match File::open(&path.clone()).await {
        Err(_) => return Message::None,
        Ok(f) => f,
    };

    // Create the buffer.
    let mut data = Vec::new();

    // Create the reader.
    let mut reader = tokio::io::BufReader::new(file);

    // Read the file.
    match reader.read_to_end(&mut data).await {
        Err(e) => return Entry::error(Source::Host, format!("Failed to load SVD file {} : {}", path.display(), e)).into(),
        Ok(_) => (),
    }

    // Convert the contents to a string.
    let string = match String::from_utf8( data.clone() ) {
        Err(e) => return Entry::error(Source::Host, format!("Failed to load SVD file {} as string : {}", path.display(), e)).into(),
        Ok(string) => string,
    };

    println!("Loaded file");

    // Parse the SVD data.
    let peripherals = parse( string ).await;

    println!("Parsed file");

    Message::NewSVD( peripherals, path )
}


async fn parse(data: String) -> Vec<Arc<Peripheral>> {
    // Parse the SVD.
    let svd = match svd_parser::parse( &data ) {
        Err(e) => return Vec::new(),
        Ok(svd) => svd,
    };

    // Create the list of peripherals.
    let mut peripherals = Vec::new();

    for maybe in &svd.peripherals {
        // Get the peripheral.
        let peripheral = match maybe {
            MaybeArray::Array(p, _) => p,
            MaybeArray::Single(p) => p,
        };

        // Create the peripheral information.
        let information = Peripheral::create(&svd, peripheral);

        // Add the peripheral to the list.
        peripherals.push( Arc::new( information ) );
    }

    peripherals
}

async fn parse_(data: String) {
    // Parse the SVD.
    let svd = match svd_parser::parse( &data ) {
        Err(e) => return,
        Ok(svd) => svd,
    };

    println!("Found device from {:?} named {:?} description {:?}", svd.vendor, svd.name, svd.description);

    println!("CPU on device: {:?}", svd.cpu);

    println!("Address unit {} bits\nSingle transfer bits {}", svd.address_unit_bits, svd.width);

    for p in &svd.peripherals {
        if p.is_array() {
            print!("Cluster peripheral ");
        } else {
            print!("Single peripheral ");
        }

        // Get the peripheral.
        let peripheral = match p {
            MaybeArray::Array(p, dim) => p,
            MaybeArray::Single(p) => p,
        };

        println!("{} ({:?}) @ 0x{:08X}", peripheral.name, peripheral.display_name, peripheral.base_address);

        if let Some(alternate) = &peripheral.alternate_peripheral {
            println!("  - alternate {}", alternate);
        }

        if let Some(derived) = &peripheral.derived_from {
            println!("  - derived from {}", derived);
        }

        println!("  - {:?}", peripheral.interrupt);

        println!("  - {:?}", peripheral.address_block);

        // Get the base address.
        let base = peripheral.base_address;

        // Get the default peripheral access.
        let defaultprop = peripheral.default_register_properties;

        if let Some(registers) = &peripheral.registers {
            println!("  - {} registers", registers.len());

            for r in registers {
                print!("    > Register ");

                match r {
                    RegisterCluster::Cluster(c) => {
                        match c {
                            MaybeArray::Array(p, dim) => {
                                println!("(Cluster) {{Array}} {}", p.name);
                            },

                            MaybeArray::Single(p) => {
                                println!("(Cluster) {{Single}} {}", p.name);
                            },
                        }
                    },

                    RegisterCluster::Register(c) => {
                        // Get the register.
                        let register = match c {
                            MaybeArray::Array(p, dim) => {
                                println!("(Register) {{Array}} {} [{:?}] @ 0x{:08X}", p.name, p.display_name, base + (p.address_offset as u64));
                                p
                            },

                            MaybeArray::Single(p) => {
                                println!("(Register) {{Single}} {} [{:?}] @ 0x{:08X}", p.name, p.display_name, base + (p.address_offset as u64));
                                p
                            },
                        };

                        // Is it derived.
                        if let Some(derived) = &register.derived_from {
                            println!("      · Derived from {}", derived);
                        }

                        // Get the register access mode.
                        let access = access(&register.properties, &defaultprop);

                        print!("      · {}", match access {
                                Access::ReadOnly      => "RO",
                                Access::ReadWrite     => "RW",
                                Access::ReadWriteOnce => "RW1",
                                Access::WriteOnce     => "W1",
                                Access::WriteOnly     => "WO",
                        });

                        // Get the register size.
                        let size = size( &register.properties, &defaultprop );

                        println!(" {:?} bits", size);

                        // Get the reset value.
                        let reset = resetvalue( &register.properties, &defaultprop );

                        println!("      · Reset value 0x{:08X}", reset);

                        // Get the register protection.
                        println!("      · Protection {:?}", register.properties.protection);

                        // Modified write values.
                        println!("      · Write effects {:?}", register.modified_write_values);

                        // Get the write constraints.
                        let constraint = match &register.write_constraint {
                            Some(c) => match c {
                                WriteConstraint::WriteAsRead(true) => String::from("Write as Read"),
                                WriteConstraint::UseEnumeratedValues(true) => String::from("Write only Enumerated"),
                                WriteConstraint::Range(range) => format!("{}:{}", range.min, range.max),

                                _ => String::from("None"),
                            },

                            _ => String::from("None"),
                        };

                        // Write constraints.
                        println!("      · Write constraints {:?}", constraint);

                        // Read action.
                        println!("      · Read action {:?}", register.read_action);

                        if let Some(fields) = &register.fields {
                            println!("      · {} fields", fields.len());

                            for f in fields {
                                let field = match f {
                                    MaybeArray::Array(x, dim) => {
                                        println!("        + Field {{Cluster}} {}", x.name);
                                        x
                                    },
                                    MaybeArray::Single(x) => {
                                        println!("        + Field {{Single}} {}", x.name);
                                        x
                                    },
                                };

                                // Build the field.
                                /*
                                let field = crate::target::field::Field::create( &peripheral, &register, field ).expect("Failed to parse the field");

                                println!("          = Bits {}..{}", field.bitrange.lsb(), field.bitrange.msb());
                                println!("          = Access {:?}", field.access);

                                match field.values {
                                    WriteValues::Value => println!("          = Raw value"),
                                    WriteValues::WriteAsRead => println!("          = Write as read"),
                                    WriteValues::Range(range) => println!("          = Range {}..{}", range.min, range.max),
                                    WriteValues::Enumeration( list ) => println!("\n\n\n          = {} values", list.len()),
                                }
                                */
                            }
                        }
                    },
                }

                println!("");
            }
        }

        println!("");
    }
}



/// Gets the access of the register or it's default if it's not present.
pub fn access(properties: &RegisterProperties, default: &RegisterProperties) -> Access {
    match properties.access {
        None => default.access.expect("No default register access configuration"),
        Some(access) => access,
    }
}

/// Gets the register size or it's default if it's not present.
pub fn size(properties: &RegisterProperties, default: &RegisterProperties) -> u32 {
    match properties.size {
        None => default.size.expect("No default register size configuration"),
        Some(size) => size,
    }
}

/// Gets the register reset value or it's default if it's not present.
pub fn resetvalue(properties: &RegisterProperties, default: &RegisterProperties) -> u64 {
    // Get the reset mask.
    let mask = match properties.reset_mask {
        None => default.reset_mask.expect("No default register size configuration"),
        Some(size) => size,
    };

    // Get the reset value.
    let value = match properties.reset_mask {
        None => default.reset_value.expect("No default register size configuration"),
        Some(size) => size,
    };

    value & mask
}
