#![allow(non_snake_case)]
#![warn(missing_docs)]
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

/// Most common used functions, classes, enums of this Libary
pub mod prelude {
    pub use crate::IR::Module;
    pub use crate::IR::IRBuilder;
    pub use crate::IR::Type;
    pub use crate::IR::FunctionType;
    pub use crate::IR::TypeMetadata;
    pub use crate::Target::Triple;
    pub use crate::Support::PrintErrorAndExit;
    
    pub use crate::IR::ir::*;
}
