use std::{any::Any, fmt::Debug, hash::Hash};
use std::collections::HashMap;
use super::{Block, Const, Function, FunctionType, Type, TypeMetadata, Var, VerifyError};
use crate::Target::TargetBackendDescr;

mod assign;
mod call;
mod cast;
mod math;
mod ret;
mod br;
mod cmp;
mod alloca;
mod store;
mod load;
mod debug;
mod phi;
mod switch;
mod neg;
mod select;

pub use assign::*;
pub use call::*;
pub use cast::*;
pub use math::*;
pub use ret::*;
pub use br::*;
pub use cmp::*;
pub use store::*;
pub use debug::*;
pub use switch::*;
pub use select::*;

macro_rules! IrTypeWith3 {
    ($name:tt, $param1:tt, $param2:tt, $param3:tt) => {
        /// An Ir node
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct $name<$param1, $param2, $param3> {
            /// first inner value
            pub(crate) inner1: $param1,
            /// second inner value
            pub(crate) inner2: $param2,
            /// third inner value
            pub(crate) inner3: $param3,
        }


        impl<$param1, $param2, $param3> $name<$param1, $param2, $param3> {
            /// Creates new instance
            #[allow(dead_code)]
            pub fn new(op0: $param1, op1: $param2, op2: $param3) -> Box<Self> {
                Box::from(
                    Self {
                        inner1: op0,
                        inner2: op1,
                        inner3: op2,
                    }
                )
            }
        }
    };
}
macro_rules! IrTypeWith2 {
    ($name:tt, $param1:tt, $param2:tt) => {
        /// An Ir node
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct $name<$param1, $param2> {
            /// first inner value
            pub inner1: $param1,
            /// second inner value
            pub inner2: $param2,
        }


        impl<$param1, $param2> $name<$param1, $param2> {
            /// Creates new instance
            #[allow(dead_code)]
            pub fn new(op0: $param1, op1: $param2) -> Box<Self> {
                Box::from(
                    Self {
                        inner1: op0,
                        inner2: op1,
                    }
                )
            }
        }
    };
}
macro_rules! IrTypeWith1 {
    ($name:tt, $param1:tt) => {
        /// An Ir node
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct $name<$param1> {
            /// inner value
            pub(crate) inner1: $param1,
        }


        impl<$param1> $name<$param1> {
            /// Creates new instance
            #[allow(dead_code)]
            pub(crate) fn new(op0: $param1) -> Box<Self> {
                Box::from(
                    Self {
                        inner1: op0,
                    }
                )
            }
        }
    };
}

IrTypeWith1!(Return, T);
IrTypeWith3!(Call, T, U, Z);

IrTypeWith2!(Assign, T, U);

IrTypeWith3!(Cast, T, U, Z);

IrTypeWith3!(Add, T, U, Z);
IrTypeWith3!(Sub, T, U, Z);
IrTypeWith3!(Xor, T, U, Z);
IrTypeWith3!(Or, T, U, Z);
IrTypeWith3!(And, T, U, Z);
IrTypeWith3!(Mul, T, U, Z);
IrTypeWith3!(Div, T, U, Z);
IrTypeWith3!(Rem, T, U, Z);
IrTypeWith3!(Shl, T, U, Z);
IrTypeWith3!(Lshr, T, U, Z);

IrTypeWith1!(Br, T);
IrTypeWith3!(BrCond, T, U, Z);

IrTypeWith2!(Alloca, T, U);
IrTypeWith2!(Store, T, U);
IrTypeWith3!(Load, T, U, Z);

IrTypeWith2!(Neg, T, U);

/// The cmp node is used to compare values
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cmp {
    pub(crate) mode: cmp::CmpMode,
    pub(crate) ls: Var,
    pub(crate) rs: Var,
    pub(crate) out: Var,
}

impl Cmp {
    /// Creates a new instance
    #[allow(dead_code)]
    pub(crate) fn new(mode: cmp::CmpMode, ls: Var, rs: Var, out: Var) -> Box<Self> {
        Box::from(
            Self {
                mode: mode,
                ls: ls,
                rs: rs,
                out: out,
            }
        )
    }
}

/// The phi node which is used to influence the register allocator
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Phi {
    pub(crate) out: Var,
    pub(crate) recive_from_blocks: Vec<(Block, Var)>,
    pub(crate) typ: TypeMetadata,
}

impl Phi {
    pub(crate) fn new(out: Var, recives: Vec<(Block, Var)>, typ: TypeMetadata) -> Self {
        Self {
            out: out,
            recive_from_blocks: recives,
            typ: typ
        }
    }
}

/// The select ir node
/// ````no-run
/// if cond != 0 {
///     out = yes
/// } else {
///     out = no
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Select<T, U> 
    where T: Debug + Clone + PartialEq + Eq,
          U: Debug + Clone + PartialEq + Eq,
{
    pub(crate) out: Var,
    pub(crate) cond: Var,

    pub(crate) yes: T,
    pub(crate) no: U,
}

use crate::Support::{ColorClass, ColorProfile};
/// The ir trait
pub trait Ir: Debug + Any {
    /// Returns the ir node as his textual representation
    fn dump(&self) -> String;
    /// Returns the ir node as his textual representation with colors
    fn dumpColored(&self, profile: ColorProfile) -> String;

    /// Turns the ir node to an any
    fn as_any(&self) -> &dyn Any;

    /// verifys the instruction (used for return instruction) based on the return type
    fn verify(&self, _: FunctionType) -> Result<(), VerifyError>;

    /// Clones the node into a box of `Box<dyn Ir>`
    fn clone_box(&self) -> Box<dyn Ir>;

    /// Compiles the node based on the given target
    #[allow(dead_code)]
    fn compile(&self, registry: &mut TargetBackendDescr);

    /// Compiles the node with node information to the given target
    fn compile_dir(&self, compiler: &mut crate::CodeGen::IrCodeGenHelper, block: &crate::prelude::Block);

    /// Returns if the node uses the variable
    fn uses(&self, _: &Var) -> bool {
        false
    }

    /// checks if the node is equal to the other node (used for the implementation of Eq)
    fn is(&self, other: &Box<dyn Ir>) -> bool {
        other.dump() == self.dump()
    }

    /// inlines the variables to the types if possible
    fn maybe_inline(&self, const_values: &HashMap<String, Type>) -> Option<Box<dyn Ir>>;

    /// evaluteas the node
    fn eval(&self) -> Option<Box<dyn Ir>>;

    /// returns the vars used by the node as input
    fn inputs(&self) -> Vec<Var>;

    /// returns the output var
    fn output(&self) -> Option<Var>;
}

impl PartialEq for Box<dyn Ir> {
    fn eq(&self, other: &Self) -> bool {
        self.is(other)
    }
}

impl Eq for Box<dyn Ir> { }

impl Clone for Box<dyn Ir> {
    fn clone(&self) -> Box<dyn Ir> {
        self.clone_box()
    }
    
    fn clone_from(&mut self, source: &Self) {
        *self = source.clone()
    }
}

/// Used for sus workaround to replace current ir node
pub trait Replace<T> {
    /// Replaces current ir node
    fn replace(&mut self, other: T);
}

impl Replace<Box<dyn Ir>> for Box<dyn Ir> {
    fn replace(&mut self, other: Box<dyn Ir>) {
        *self = other
    }
}