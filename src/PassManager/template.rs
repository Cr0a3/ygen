use crate::IR::Builder;

/// The trait all Passes need to implement
pub trait Pass {
    /// Returns the pass
    fn run(&self, builder: &mut Builder);
}