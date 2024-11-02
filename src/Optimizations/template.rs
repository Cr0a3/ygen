use crate::IR::{Block, Function};

/// The trait all Passes need to implement
pub trait Pass {
    /// Runs the pass on a block
    fn run(&self, _block: &mut Block) {}

    /// Runs the pass on the entire function
    fn run_func(&self, _func: &mut Function) {}

    /// Returns the name of the pass
    fn name(&self) -> &'static str;
}