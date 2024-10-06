use crate::prelude::*;
use super::RegAllocPrep;
use crate::CodeGen::reg_alloc::RegAlloc;

impl RegAllocPrep<Call<FuncId, Vec<Var>, Var>> for RegAlloc {
    fn prep(&mut self, node: &Call<FuncId, Vec<Var>, Var>) {
        let location = self.alloc_rv(node.inner3.ty);
        self.vars.insert(node.inner3.name.to_owned(), location);
        self.var_types.insert(node.inner3.name.to_owned(), node.inner3.ty);
    }
}