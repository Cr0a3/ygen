use crate::{prelude::Alloca, CodeGen::reg_alloc::RegAlloc, IR::{TypeMetadata, Var}};

use super::RegAllocPrep;

impl RegAllocPrep<Alloca<Var, TypeMetadata>> for RegAlloc {
    fn prep(&mut self, node: &Alloca<Var, TypeMetadata>) {
        let location = self.alloc_stack(node.inner1.ty);
        self.vars.insert(node.inner1.name.to_owned(), location);
        self.var_types.insert(node.inner1.name.to_owned(), node.inner2);
    }
}