//! Software representation of a peripheral.



use std::sync::Arc;

use svd_parser::svd::{
    Device, Name, PeripheralInfo, MaybeArray,
};

use tokio::sync::RwLock;



#[derive(Clone, Debug)]
pub struct Peripheral {
    /// Name of the peripheral.
    name: String,

    /// Description of the peripheral.
    description: Option<String>,

    /// Base address of the peripheral.
    address: u64,

    /// All registers in this peripheral.
    registers: Vec<Arc<RwLock<super::Register>>>,
}

impl Peripheral {
    /// Creates a peripheral representation from the given information.
    pub fn create(device: &Device, info: &PeripheralInfo) -> Self {
        // Get the name.
        let name = match &info.display_name {
            Some(name) => name.clone(),
            _ => info.name.clone(),
        };

        // Get the description.
        let description = info.description.clone();

        // Get the address.
        let address = info.base_address;

        // Check if the device is derived from another device.
        // This section is very dense and I dont want to debug it.
        let baseinfo = match &info.derived_from {
            Some(derived) => device.peripherals.iter()
                .find(|p| p.name() == derived)
                .expect("Malformed SVD file : Bad peripheral derive"),

            _ => info,
        };

        // Get the registers of the peripheral.
        let mut registers: Vec<super::Register> = baseinfo.registers()
            .filter(|array| array.is_single())
            .map(|r| super::Register::create(info, &r) )
            .collect();

        // Get the array registers.
        let arrays: Vec<super::Register> = baseinfo.registers()
            .filter(|array| array.is_array())
            .map(|array| match array {
                MaybeArray::Array(r, dim) => (r, dim),
                _ => panic!("No single register should reach this point"),
            })
            .map(|(r, dim)| {
                // Create the base register.
                let basereg = super::Register::create(info, r);

                // Get the base name.
                let basename = dim.dim_name.as_ref().unwrap_or(basereg.name()).clone();

                // Create all the variants.
                let indices: Vec<_> = dim.indexes()
                    .enumerate()
                    .map(|(index, substring)| {
                        // Create a new register.
                        let mut new = basereg.clone();

                        // Modify the name.
                        new.name = basename.replace("[%s]", &substring);

                        // Modify the address.
                        new.address += dim.dim_increment as u64 * index as u64;

                        new
                    })
                    .collect();

                indices
            })
            .flatten()
            .collect();

        // Join all registers.
        registers.extend(arrays);

        // Sort by address.
        registers.sort_by( |a, b| a.address().cmp( &b.address() ) );

        // Transform into final container.
        let registers = registers.into_iter()
            .map(|register| Arc::new( RwLock::new( register ) ))
            .collect();

        Self {
            name,
            description,
            address,
            registers
        }
    }

    /// Returns the name of the peripheral.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Returns the description of the peripheral.
    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    /// Returns the base address of the peripheral.
    pub fn address(&self) -> u64 {
        self.address
    }

    /// Returns a reference to the registers.
    pub fn registers(&self) -> &Vec<Arc<RwLock<super::Register>>> {
        &self.registers
    }
}
