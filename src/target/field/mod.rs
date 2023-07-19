//! Software definition of a register field.



mod enumerated;
mod values;



pub use enumerated::EnumeratedValue;
pub use values::WriteValues;



use svd_parser::svd::{
    Access, BitRange, FieldInfo, PeripheralInfo, RegisterInfo,
    WriteConstraint, WriteConstraintRange,
};



pub struct Field {
    /// Name of the field.
    pub name: String,

    /// Address of the register containing the field.
    pub address: u64,

    /// Bit range of the field.
    pub bitrange: BitRange,

    /// Access restrictions to this field.
    pub access: Access,

    /// Possible values of this field.
    pub values: WriteValues,

    /// Current raw value of the field.
    pub raw: u64,
}

impl Field {
    /// Creates the field representation from the available information.
    pub fn create(peripheral: &PeripheralInfo, register: &RegisterInfo, field: &FieldInfo) -> Option<Field> {
        // Get the name of the field.
        let name = field.name.clone();

        // Get the address of the register.
        let address = peripheral.base_address + (register.address_offset as u64);

        // Get the bit range of the field.
        let bitrange = field.bit_range;

        // Get the access (or default).
        let access = getaccess(peripheral, register, field)?;

        // Get the value restrictions of the field.
        let values = match field.write_constraint {
            Some( WriteConstraint::UseEnumeratedValues(true) ) => WriteValues::Enumeration(Vec::new()),
            Some( WriteConstraint::WriteAsRead(true) ) => WriteValues::WriteAsRead,
            Some( WriteConstraint::Range(range) ) => WriteValues::Range( range ),
            _ => WriteValues::Value,
        };

        Some( Self {
            name,
            address,
            bitrange,
            access,
            values,
            raw: 0,
        })
    }
}



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
