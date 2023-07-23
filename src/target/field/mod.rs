//! Software definition of a register field.



//mod enumerated;
//mod values;



//pub use enumerated::EnumeratedValue;
//pub use values::WriteValues;



use svd_parser::svd::{
    Access, BitRange, FieldInfo, Name, PeripheralInfo, ReadAction, RegisterInfo,
    WriteConstraint, WriteConstraintRange,
};



#[derive(Clone, Debug)]
pub struct Field {
    /// Name of the field.
    pub(super) name: String,

    /// Address of the register containing the field.
    pub(super) address: u64,

    /// Bit range of the field.
    pub(super) bitrange: BitRange,

    /// Side effects of reading this field.
    pub(super) readaction: Option<ReadAction>,
}

impl Field {
    /// Creates the field representation from the available information.
    pub fn create(peripheral: &PeripheralInfo, register: &RegisterInfo, info: &FieldInfo) -> Field {
        // Get the name of the field.
        let name = info.name.clone();

        // Get the address of the register.
        let address = peripheral.base_address + (register.address_offset as u64);

        // Check if the field is derived from another field.
        // This section is very dense and I dont want to debug it.
        let baseinfo = match &info.derived_from {
            Some(derived) => register.fields()
                .find(|p| p.name() == derived)
                .expect("Malformed SVD file : Bad peripheral derive"),

            _ => info,
        };

        // Get the bit range of the field.
        let bitrange = baseinfo.bit_range;

        // Get the read side effect.s
        let readaction = baseinfo.read_action;

        Self {
            name,
            address,
            bitrange,
            readaction,
        }
    }

    /// Returns the read side effect of this field.
    pub fn readaction(&self) -> Option<ReadAction> {
        self.readaction
    }
}



/*
/// Gets the access permissions of the field.
fn getaccess(peripheral: &PeripheralInfo, register: &RegisterInfo, field: &FieldInfo) -> Option<Access> {
    match field.access {
        None => match register.properties.access {
            None => peripheral.default_register_properties.access,

            access => access,
        },

        access => access,
    }
}


/// Get the access constraint of the field.
fn getconstraint(peripheral: &PeripheralInfo, register: &RegisterInfo, field: &FieldInfo) -> Option<WriteConstraint> {
    match field.write_constraint {
        None => register.write_constraint,

        access => access,
    }
}
*/