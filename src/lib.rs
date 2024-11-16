#![allow(non_snake_case)]
#![warn(missing_docs)]
#![warn(unreachable_pub)]
#![allow(clippy::redundant_field_names)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::invalid_codeblock_attributes)]
#![allow(non_camel_case_types)]

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
//! ```no-run
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

pub use proc;

/// The target module: every stuff which has to do with targets. Like:
///  * The target triple
///  * TargetRegistry
pub mod Target;

/// The ir module: functions for building function ir
pub mod IR;

/// The pass manager module:
///  * Includes all passes and their definition
///  * The PassManager
pub mod Optimizations;

/// Other utilites like:
///  * Cli args
///  * String coloring and padding
pub mod Support;

/// Writing/Reading object files
pub mod Obj;

/// Shared code generation classes (mainly used for register based architectures like x86_64, aarch64, ...)
pub mod CodeGen;

/// Debugging information
pub mod debug;

/// Jit execution utillities
#[cfg(feature = "jit")]
pub mod Jit;

/// Most common used functions, classes, enums of this Libary
pub mod prelude {
    pub use crate::IR::*;
    pub use crate::Target::Triple;
    pub use crate::Support::PrintErrorAndExit;
    pub use crate::Optimizations::PassManager;
    
    pub use crate::IR::ir::*;
}


pub(crate) static mut YGEN_DEBUG: bool = false;
pub(crate) static mut YGEN_INITED: bool = false;

/// Activates that ygen now prints debugging information
pub fn activate_ygen_debugging() {
    unsafe { YGEN_DEBUG = true; }
}

/// Initializes ygen
pub fn init_ygen() {
    use self::Support::Colorize;

    if unsafe { !YGEN_INITED } {
        std::panic::set_hook(Box::new(|panic_info| {
            let backtrace = std::backtrace::Backtrace::force_capture();
            eprintln!("\t\t\t{}", "YGEN_PANIC".red());
            eprintln!("\t\t{}", "It seemes like ygen paniced".gray());
            eprintln!("\t{}", "Please create a Github issue with this panic message".gray());
            eprintln!("\t\t\t{}", "Panic:".red());
            eprintln!("{}", panic_info);
            eprintln!("\t\t\t{}", "STACK TRACE:".red());
            let lines = backtrace.to_string();
            let mut lines = lines.split("\n").collect::<Vec<&str>>();

            let mut pos = 0;
            
            // backtraces are formated like this:
            //   13: ygen::IR::module::Module::emitMachineCode
            //              at .\src\IR\module.rs:195
            // so when we see a `ygen::` in the string we need to also skip the next line
            // so that we included the at

            let mut saw_ygen = false;
            
            for line in lines.clone() {
                if !line.contains("ygen")  || line.contains("ygen::init_ygen") {
                    if !saw_ygen {
                        lines.remove(pos);
                        saw_ygen = false;
                        continue;
                    }
                    saw_ygen = false;
                } else {
                    saw_ygen = true;
                }

                pos += 1;
            }

            for line in lines {
                eprintln!("{}", line);
            }
            
            eprintln!("\t\t\t{}", "IMPORTANT".red().bold());
            eprintln!("\t{}{}{}", "Please create a github issue at ".gray(), "https://github.com/Cr0a3/ygen".blue().underline(), " in the following format:".gray());
            eprintln!("  [BUG] ygen paniced");
            eprintln!("  Hello,");
            eprintln!("  Ygen paniced while doing ygen things!");
            eprintln!("  Here's the error message: ");
            eprintln!("  ```");
            eprintln!("  {{This entire error}}");
            eprintln!("  ```");
            eprintln!("  Bye");
        }));
        unsafe { YGEN_INITED = true;}
    }
}

/// The ydbg macro is internally used in ygen to print out more
/// complex debugging information
/// 
/// It will print nothing if the `activate_ygen_debugging` function 
/// was not called
#[macro_export]
macro_rules! ydbg {
    () => {
        if unsafe { $crate::YGEN_DEBUG } {
            print!("\n")
        }
    };
    ($($arg:tt)*) => {{
        if unsafe { $crate::YGEN_DEBUG } {
            println!($($arg)*)
        }
    }};
}