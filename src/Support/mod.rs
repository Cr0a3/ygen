mod color;
mod cli;
mod pad;
mod srcmngr;
mod tokmngr;
mod error;
mod profile;
mod type_switch;

use std::process::exit;

pub use color::{Colorize, ColorEncoder, Color};
pub use profile::{ColorProfile, ColorClass};
pub use cli::Cli;
pub use pad::Pad;
pub use srcmngr::SrcMngr;
pub use tokmngr::TokenMgr;
pub use error::Error;
pub use type_switch::*;

use crate::IR::VerifyError;

/// Prints the error and exits the process
/// If no error occured this function just returns
pub trait PrintErrorAndExit {
    /// Prints the error and exits the process
    /// If no error occured this function just returns
    fn print(&self);
}

impl PrintErrorAndExit for Result<(), VerifyError> {
    fn print(&self) {
        match self {
            Ok(_) => {},
            Err(e) => {
                eprintln!("{}", e);
                exit(-1)
            }
        }
    }
}