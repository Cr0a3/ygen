use crate::prelude::*;
use super::RegAllocPrep;
use crate::CodeGen::reg_alloc::RegAlloc;

impl RegAllocPrep<Store<Var, Var>> for RegAlloc {
    fn prep(&mut self, _: &Store<Var, Var>) {
        // nothing needs to be allocated here
    }
}

impl RegAllocPrep<Store<Var, Type>> for RegAlloc {
    fn prep(&mut self, _: &Store<Var, Type>) {
        // nothing needs to be allocated here
    }
}