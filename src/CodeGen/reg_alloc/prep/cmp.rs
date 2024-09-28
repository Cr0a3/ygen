use crate::prelude::*;
use super::RegAllocPrep;
use crate::CodeGen::reg_alloc::RegAlloc;

impl RegAllocPrep<Cmp> for RegAlloc {
    fn prep(&mut self, node: &Cmp) {
        let location = self.alloc_rv(node.out.ty);
        self.vars.insert(node.out.name.to_owned(), location);
        self.var_types.insert(node.out.name.to_owned(), node.out.ty);
    }
}