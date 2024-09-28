use crate::{prelude::Assign, CodeGen::reg_alloc::RegAlloc, IR::{Const, Type, TypeMetadata, Var}};

use super::RegAllocPrep;

impl RegAllocPrep<Assign<Var, Type>> for RegAlloc {
    fn prep(&mut self, node: &Assign<Var, Type>) {
        let location = self.alloc_rv(node.inner1.ty);
        self.vars.insert(node.inner1.name.to_owned(), location);
        self.var_types.insert(node.inner1.name.to_owned(), node.inner2.into());
    }
}

impl RegAllocPrep<Assign<Var, Var>> for RegAlloc {
    fn prep(&mut self, node: &Assign<Var, Var>) {
        let location = self.alloc_rv(node.inner1.ty);
        self.vars.insert(node.inner1.name.to_owned(), location);
        self.var_types.insert(node.inner1.name.to_owned(), node.inner2.ty);
    }
}
impl RegAllocPrep<Assign<Var, Const>> for RegAlloc {
    fn prep(&mut self, node: &Assign<Var, Const>) {
        let location = self.alloc_rv(node.inner1.ty);
        self.vars.insert(node.inner1.name.to_owned(), location);
        self.var_types.insert(node.inner1.name.to_owned(), TypeMetadata::ptr);
    }
}