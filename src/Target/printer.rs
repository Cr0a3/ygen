use std::error::Error;
use crate::IR::Module;
use super::{CallConv, TargetRegistry};

/// The `AsmPrinter`-trait is used to dump assembly code in machine specific style
pub trait AsmPrinter {
    /// prints the assembly code for the module
    fn print(&self, module: &mut Module, registry: &mut TargetRegistry, call_conv: CallConv) -> Result<String, Box<dyn Error>>;

    /// returns the string as commetet
    fn comment(&self, string: &str) -> String {
        format!("// {}", string)
    }

    /// clones itself into a asmprinter boxed object
    fn clone_box(&self) -> Box<dyn AsmPrinter>;
}

impl Clone for Box<dyn AsmPrinter> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}