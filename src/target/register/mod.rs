//! Software representation of a peripheral.



use svd_parser::svd::{
    PeripheralInfo, RegisterInfo,
};



#[derive(Clone, Debug)]
pub struct Register {
    /// Name of the register.
    pub(super) name: String,

    /// Description of the register.
    pub(super) description: Option<String>,

    /// Base address of the register.
    pub(super) address: u64,
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

        Self {
            name,
            description,
            address,
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
}
