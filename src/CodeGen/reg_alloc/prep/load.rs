use crate::prelude::*;
use super::RegAllocPrep;
use crate::CodeGen::reg_alloc::RegAlloc;

impl RegAllocPrep<Load<Var, Var, TypeMetadata>> for RegAlloc {
    fn prep(&mut self, node: &Load<Var, Var, TypeMetadata>) {
        let location = self.alloc_rv(node.inner3);
        self.vars.insert(node.inner1.name.to_owned(), location);
        self.var_types.insert(node.inner1.name.to_owned(), node.inner3);
    }
}