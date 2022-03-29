//! Target of a project.
//! A target is a chip / board or device that is targeted through a probe to load and debug code on it.


#![allow(dead_code)]



use serde::{ Deserialize, Serialize };

use std::path::PathBuf;



#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TargetInfo {
    /// Name of the target.
    pub name: String,

    /// Target chip name.
    pub target: String,

    /// Full path to the ELF binary file.
    pub binary: String,
}

impl TargetInfo {
    /// Return the binary file path if it exists.
    pub fn binary(&self) -> Option<PathBuf> {
        let path = PathBuf::from(self.binary.clone());

        if path.exists() && path.is_file() {
            return Some(path);
        }

        None
    }
}
