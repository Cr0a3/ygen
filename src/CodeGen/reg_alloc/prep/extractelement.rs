use crate::{prelude::GetElemPtr, CodeGen::reg_alloc::RegAlloc};

use super::RegAllocPrep;

impl RegAllocPrep<GetElemPtr> for RegAlloc {
    fn prep(&mut self, node: &GetElemPtr) {
        let location = self.alloc_rv(node.out.ty);
        self.vars.insert(node.out.name.to_owned(), location);
        self.var_types.insert(node.out.name.to_owned(), node.out.ty);
    }
}