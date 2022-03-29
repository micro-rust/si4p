//! Project module.
//! Contains all necessary information and metadata of an embedded project.



#![deny(warnings)]



mod info;
mod target;



use serde::{ Deserialize, Serialize };



pub use self::info::ProjectInfo;
pub use self::target::TargetInfo;



#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProjectSerial {
    /// Project information.
    pub info: ProjectInfo,

    /// All targets selectable by this project.
    pub targets: Vec<TargetInfo>,
}

impl ProjectSerial {
    /// Creates a new empty `ProjectSerial`.
    pub fn new() -> Self {
        ProjectSerial {
            info: ProjectInfo::new(),
            targets: Vec::new(),
        }
    }

    /// Parses a serialized version of a project.
    pub fn parse(buffer: Vec<u8>) -> Result<Self, ron::Error> {
        let ronfile = String::from_utf8(buffer).unwrap();

        ron::from_str(&ronfile)
    }

    /// Returns the name of the project.
    pub fn name(&self) -> &String {
        &self.info.name
    }

    /// Returns the description of the project.
    pub fn description(&self) -> &String {
        &self.info.description
    }
}
