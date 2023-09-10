//! Enumerates all window types to allow for multi window environment (`iced` >= 0.11).



pub mod binary;
//pub mod debug;



pub use self::{
    binary::BinaryWindow,
    //debug::DebugWindow,
};



//#[derive(Clone)]
pub enum Window {
    /// A binary inspection window.
    Binary( BinaryWindow ),

    // A debug session window.
    //Debug( DebugWindow ),
}
