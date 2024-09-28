use crate::prelude::*;
use super::RegAllocPrep;
use crate::CodeGen::reg_alloc::RegAlloc;

impl RegAllocPrep<Return<Type>> for RegAlloc {
    fn prep(&mut self, _: &Return<Type>) {
        // nothing needs to be allocated here
    }
}
impl RegAllocPrep<Return<Var>> for RegAlloc {
    fn prep(&mut self, _: &Return<Var>) {
        // nothing needs to be allocated here
    }
}