#![allow(non_snake_case)]
#![warn(missing_docs)]
#![warn(unreachable_pub)]
#![allow(clippy::redundant_field_names)]
#![allow(rustdoc::invalid_html_tags)]

//! # Ygen - <b>Y</b>et another Code <b>Gen</b>erator
//! ### Description
//! Ygen is a libary to build modern compilers and tools <br>
//! It includes many features like IR Optimization, Argument parsing, and much more <br>
//! There are many utility functions and classes to help you write many asspects of your compiler: <br>
//!  - Like the Colorize trait to colorize your strings (maybe used for printing errors)
//!  - The TokenMngr in combination with the SrcMngr to easily write a lexer and store the source location (usefull for including
//!     debugging information)
//! 
//! #### The YGEN-IR
//! The Ygen Internal Representation isn't very different to LLVMs IR. <br>
//! A tip: <br>
//!  To dump the Ir of a YGen-Module use the `dump`-Function which dumps the entire IR into a string <br>
//!  If you want to print the IR out the stdout consider using the `dumpColored`-Function which includes Color Information <br>
//!  But if you pipe that into a file it also includes the Color bytes which then look a bit sus so consider printing it to stderr
//! 
//! ###### Here is a quick introduction to the YGEN-IR:
//! A function is defined like this:
//! ```
//! define i32 @add( i32 %0,  i32 %1 ) {
//! entry:
//!  %2 = add i32 %0, %1
//!  ret i32 %2
//!}
//! ```
//! So `define` then the return type of the function, a `@`, the function name and the arguments. <br>
//! An important thing to understand is that you can only define every variable once because
//! it follows the SSA form. <br>
//! 
//! In YGEN-iR there are many inbuild instructions
//! Like in our add function example:
//!  - `add` adds two numbers
//!  - `ret` returns an constant or a variable 

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
