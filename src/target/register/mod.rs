//! Software representation of a peripheral.



use std::sync::Arc;

use svd_parser::svd::{
    Name, PeripheralInfo, ReadAction, RegisterInfo, MaybeArray,
};

use tokio::sync::RwLock;



#[derive(Clone, Debug)]
pub struct Register {
    /// Name of the register.
    pub(super) name: String,

    /// Description of the register.
    pub(super) description: Option<String>,

    /// Base address of the register.
    pub(super) address: u64,

    /// Returns the raw value of the register.
    pub(super) raw: u32,

    /// Indicates if there are side effects of reading this register.
    pub(super) readaction: Option<ReadAction>,

    /// The list of fields of the register.
    pub(super) fields: Vec<Arc<RwLock<super::Field>>>,
}

impl Register {
    /// Creates a peripheral representation from the given information.
    pub fn create(peripheral: &PeripheralInfo, info: &RegisterInfo) -> Self {
        // Get the name.
        let name = match &info.display_name {
            Some(name) => name.clone(),
            _ => info.name.clone(),
        };

        // Get the description.
        let description = info.description.clone();

        // Get the address.
        let address = peripheral.base_address + (info.address_offset as u64);

        // Check if the register is derived from another register.
        // This section is very dense and I dont want to debug it.
        let baseinfo = match &info.derived_from {
            Some(derived) => peripheral.registers()
                .find(|p| p.name() == derived)
                .expect("Malformed SVD file : Bad peripheral derive"),

            _ => info,
        };

        // Get the fields of the register.
        let mut fields: Vec<super::Field> = baseinfo.fields()
            .filter(|array| array.is_single())
            .map(|f| super::Field::create(peripheral, info, f))
            .collect();

        // Get the array fields.
        let arrays: Vec<super::Field> = baseinfo.fields()
            .filter(|array| array.is_array())
            .map(|array| match array {
                MaybeArray::Array(r, dim) => (r, dim),
                _ => panic!("No single field should reach this point"),
            })
            .map(|(r, dim)| {
                // Create the base field.
                let basefield = super::Field::create(peripheral, info, r);

                // Get the base name.
                let basename = dim.dim_name.as_ref().unwrap_or(&basefield.name).clone();

                // Create all the variants.
                let indices: Vec<_> = dim.indexes()
                    .enumerate()
                    .map(|(index, substring)| {
                        // Create a new register.
                        let mut new = basefield.clone();

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

        // Join all fields.
        fields.extend(arrays);

        // Sort by start bit.
        fields.sort_by(|a, b| a.bitrange.lsb().cmp( &b.bitrange.lsb() ));

        // Check if any field has a read restriction.
        let readaction = match fields.iter().find(|field| field.readaction.is_some()) {
            Some(field) => field.readaction(),
            _ => info.read_action,
        };

        // Transform into final container.
        let fields = fields.into_iter()
            .map(|field| Arc::new( RwLock::new( field ) ))
            .collect();

        Self {
            name,
            description,
            address,
            raw: 0,
            readaction,
            fields,
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

    /// Returns the raw value of the register.
    pub fn raw(&self) -> u32 {
        self.raw
    }

    /// Returns the read side effects of this register.
    pub fn readaction(&self) -> Option<ReadAction> {
        self.readaction
    }
}
