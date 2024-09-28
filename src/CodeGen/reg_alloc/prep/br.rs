use crate::prelude::*;
use super::RegAllocPrep;
use crate::CodeGen::reg_alloc::RegAlloc;

impl RegAllocPrep<Br<Box<Block>>> for RegAlloc {
    fn prep(&mut self, _: &Br<Box<Block>>) {
        // nothing to allocate
    }
}
impl RegAllocPrep<BrCond<Var, Block, Block>> for RegAlloc {
    fn prep(&mut self, _: &BrCond<Var, Block, Block>) {
        // nothing to allocate
    }
}