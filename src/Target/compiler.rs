use std::error::Error;

use crate::Support::ColorProfile;

use super::Token;

/// An wrapper trait for assembly compilers
pub trait Compiler {
    /// Creates an new assembly compiler
    fn new(&self, tokens: Vec<Token>) -> Box<dyn Compiler>;
    /// compiles an assembly string into machine code
    fn parse(&mut self) -> Result<(), Box<dyn Error>>;
    /// Returns the output machine code
    fn out(&self) -> Result<Vec<u8>, Box<dyn Error>>;
    
    /// Returns the parsed instruction colored
    fn coloredOut(&self, profile: ColorProfile) -> String;

    /// Returns the parsed instruction as a string
    fn printOut(&self) -> String;

    #[doc(hidden)]
    fn boxed(&self) -> Box<dyn Compiler>;
}

impl Clone for Box<dyn Compiler> {
    fn clone(&self) -> Self {
        self.boxed()
    }
}