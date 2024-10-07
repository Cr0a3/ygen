use crate::{prelude::Select, CodeGen::reg_alloc::RegAlloc, IR::{Type, Var}};

use super::RegAllocPrep;

impl RegAllocPrep<Select<Var, Var>> for RegAlloc {
    fn prep(&mut self, node: &Select<Var, Var>) {
        let location = self.alloc_rv(node.out.ty);
        self.vars.insert(node.out.name.to_owned(), location);
        self.var_types.insert(node.out.name.to_owned(), node.out.ty);
    }
}
impl RegAllocPrep<Select<Var, Type>> for RegAlloc {
    fn prep(&mut self, node: &Select<Var, Type>) {
        let location = self.alloc_rv(node.out.ty);
        self.vars.insert(node.out.name.to_owned(), location);
        self.var_types.insert(node.out.name.to_owned(), node.out.ty);
    }
}
impl RegAllocPrep<Select<Type, Var>> for RegAlloc {
    fn prep(&mut self, node: &Select<Type, Var>) {
        let location = self.alloc_rv(node.out.ty);
        self.vars.insert(node.out.name.to_owned(), location);
        self.var_types.insert(node.out.name.to_owned(), node.out.ty);
    }
}
impl RegAllocPrep<Select<Type, Type>> for RegAlloc {
    fn prep(&mut self, node: &Select<Type, Type>) {
        let location = self.alloc_rv(node.out.ty);
        self.vars.insert(node.out.name.to_owned(), location);
        self.var_types.insert(node.out.name.to_owned(), node.out.ty);
    }
}