#![allow(non_snake_case)]
#![warn(missing_docs)]
#![warn(unreachable_pub)]
#![allow(clippy::redundant_field_names)]
#![allow(rustdoc::invalid_html_tags)]

//! Ygen - <b>Y</b>et another Code <b>Gen</b>erator

/// The target module: every stuff which has to do with targets. Like:
///  * The target triple
///  * TargetRegistry
pub mod Target;

/// The ir module: functions for building function ir
pub mod IR;

/// The pass manager module:
///  * Includes all passes and their definition
///  * The PassManager
pub mod PassManager;

/// Other utilites like:
///  * Cli args
///  * String coloring and padding
pub mod Support;

/// Writing/Reading object files
pub mod Obj;

/// Most common used functions, classes, enums of this Libary
pub mod prelude {
    pub use crate::IR::*;
    pub use crate::Target::Triple;
    pub use crate::Support::PrintErrorAndExit;
    pub use crate::PassManager::PassManager;
    
    pub use crate::IR::ir::*;
}
