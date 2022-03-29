//! Information related to a project.



use serde::{ Deserialize, Serialize };



#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProjectInfo {
    /// Name of the project.
    pub name: String,

    /// An optional description comment.
    pub description: String,
}

impl ProjectInfo {
    pub fn new() -> Self {
        ProjectInfo {
            name: String::new(),
            description: String::new(),
        }
    }
}
