use crate::IR::Block;

/// The trait all Passes need to implement
pub trait Pass {
    /// Returns the pass
    fn run(&self, block: &mut Block);
}