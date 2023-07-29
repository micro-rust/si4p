//! Core information abstraction.



use probe_rs::{
    Architecture, CoreRegister, CoreStatus, CoreType, RegisterId, Session,
};



#[derive(Clone, Debug)]
pub struct CoreInformation {
    /// Index of the core.
    pub index: usize,

    /// Architecture of the core.
    pub architecture: Architecture,

    /// Type of the core.
    pub coretype: CoreType,

    /// List of core registers.
    pub cregs: Vec<Register>,

    /// List of FPU registers.
    pub fregs: Vec<Register>,

    /// Last known status of the core.
    pub status: CoreStatus,
}

impl CoreInformation {
    /// Get the core information from the `probe-rs` information.
    pub fn parse(session: &mut Session) -> Vec<Self> {
        // Get the list of cores.
        let corelist = session.list_cores();

        // Create the list of cores.
        let mut list = Vec::new();

        for (index, coretype) in corelist.iter() {
            match session.core(*index) {
                Ok(core) => {
                    // Get all the core registers.
                    let all = core.registers();

                    // Parse the core registers.
                    let cregs = all.core_registers().map(|r| Register::parse(r, RegisterType::Unsigned(0))).collect();

                    // Parse the FPU registers.
                    let fregs = match all.fpu_registers() {
                        Some(iter) => iter.map(|r| Register::parse(r, RegisterType::FloatingPoint(0.0))).collect(),
                        _ => Vec::new(),
                    };

                    // Create the new core.
                    let new = Self {
                        index: *index,
                        architecture: core.architecture(),
                        coretype: core.core_type(),
                        cregs,
                        fregs,
                        status: CoreStatus::Unknown,
                    };

                    // Add the core to the list.
                    list.push( new );
                },

                Err(e) => println!("Failed to read the information of a core : {}", e),
            }
        }

        list
    }
}



#[derive(Clone, Debug)]
pub struct Register {
    /// Name of the register.
    pub name: String,

    /// ID of the register.
    pub id: RegisterId,

    /// Size of the register in bytes.
    pub bytes: usize,

    /// Type of data in this register.
    pub data: RegisterType,
}

impl Register {
    /// Parse the given register.
    pub fn parse(register: &CoreRegister, data: RegisterType) -> Self {
        // Name of the register.
        let name = String::from( register.name() );

        // ID of the register.
        let id = register.id();

        // Size in bytes of the register.
        let bytes = register.size_in_bytes();

        Self { name, id, bytes, data,}
    }
}


#[derive(Clone, Copy, Debug)]
pub enum RegisterType {
    Unsigned(u64),
    FloatingPoint(f64),
}
