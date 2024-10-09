use crate::prelude::*;
use super::RegAllocPrep;
use crate::CodeGen::reg_alloc::RegAlloc;

macro_rules! PrepMvv {
    ($name:ident) => {
        impl RegAllocPrep<$name<Var, Var, Var>> for RegAlloc {
            fn prep(&mut self, node: &$name<Var, Var, Var>) {
                let location = self.alloc_rv(node.inner3.ty);
                self.vars.insert(node.inner3.name.to_owned(), location);
                self.var_types.insert(node.inner3.name.to_owned(), node.inner3.ty);
            }
        }
    };
}

PrepMvv!(Add);
PrepMvv!(And);
PrepMvv!(Div);
PrepMvv!(Mul);
PrepMvv!(Or);
PrepMvv!(Sub);
PrepMvv!(Xor);
PrepMvv!(Rem);
PrepMvv!(Shl);
PrepMvv!(Shr);

macro_rules! PrepMvt {
    ($name:ident) => {
        impl RegAllocPrep<$name<Var, Type, Var>> for RegAlloc {
            fn prep(&mut self, node: &$name<Var, Type, Var>) {
                let location = self.alloc_rv(node.inner3.ty);
                self.vars.insert(node.inner3.name.to_owned(), location);
                self.var_types.insert(node.inner3.name.to_owned(), node.inner3.ty);
            }
        }
    };
}

PrepMvt!(Add);
PrepMvt!(And);
PrepMvt!(Div);
PrepMvt!(Mul);
PrepMvt!(Or);
PrepMvt!(Sub);
PrepMvt!(Xor);
PrepMvt!(Rem);
PrepMvt!(Shl);
PrepMvt!(Shr);

macro_rules! PrepMtt {
    ($name:ident) => {
        impl RegAllocPrep<$name<Type, Type, Var>> for RegAlloc {
            fn prep(&mut self, node: &$name<Type, Type, Var>) {
                let location = self.alloc_rv(node.inner3.ty);
                self.vars.insert(node.inner3.name.to_owned(), location);
                self.var_types.insert(node.inner3.name.to_owned(), node.inner3.ty);
            }
        }
    };
}


PrepMtt!(Add);
PrepMtt!(And);
PrepMtt!(Div);
PrepMtt!(Mul);
PrepMtt!(Or);
PrepMtt!(Sub);
PrepMtt!(Xor);
PrepMtt!(Rem);
PrepMtt!(Shl);
PrepMtt!(Shr);