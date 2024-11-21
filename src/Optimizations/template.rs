use crate::IR::{Block, Function, Module};

/// The trait all Passes need to implement
#[allow(unused_variables)]
pub trait Pass {
    /// Runs the pass on a block
    fn run(&self, block: &mut Block) {}

    /// Runs the pass on the entire function
    fn run_func(&self, func: &mut Function) {}

    /// Runs the pass on the entire module
    fn run_mod(&self, module: &mut Module) {}

    /// Returns the name of the pass
    fn name(&self) -> &'static str;
}