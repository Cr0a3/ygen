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
mod getelemptr;

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
pub use getelemptr::*;

macro_rules! IrTypeWith3 {
    ($name:tt, $param1:tt, $param2:tt, $param3:tt, $is_func:ident) => {
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

        impl<$param1, $param2, $param3> IsNode for $name<$param1, $param2, $param3> {
            fn $is_func(&self) -> bool {
                true
            }
        }
    };
}
macro_rules! IrTypeWith2 {
    ($name:tt, $param1:tt, $param2:tt, $is_func:ident) => {
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

        impl<$param1, $param2> IsNode for $name<$param1, $param2> {
            fn $is_func(&self) -> bool {
                true
            }
        }
    };
}
macro_rules! IrTypeWith1 {
    ($name:tt, $param1:tt, $is_func:ident) => {
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

        impl<$param1> IsNode for $name<$param1> {
            fn $is_func(&self) -> bool {
                true
            }
        }
    };
}

IrTypeWith1!(Return, T, is_ret);
IrTypeWith3!(Call, T, U, Z, is_call);

IrTypeWith2!(Assign, T, U, is_assign);

IrTypeWith3!(Cast, T, U, Z, is_cast);

IrTypeWith3!(Add, T, U, Z, is_add);
IrTypeWith3!(Sub, T, U, Z, is_sub);
IrTypeWith3!(Xor, T, U, Z, is_xor);
IrTypeWith3!(Or, T, U, Z, is_or);
IrTypeWith3!(And, T, U, Z, is_and);
IrTypeWith3!(Mul, T, U, Z, is_mul);
IrTypeWith3!(Div, T, U, Z, is_div);
IrTypeWith3!(Rem, T, U, Z, is_rem);
IrTypeWith3!(Shl, T, U, Z, is_shl);
IrTypeWith3!(Shr, T, U, Z, is_shr);

IrTypeWith1!(Br, T, is_br);
IrTypeWith3!(BrCond, T, U, Z, is_brcond);

IrTypeWith2!(Alloca, T, U, is_alloca);
IrTypeWith2!(Store, T, U, is_store);
IrTypeWith3!(Load, T, U, Z, is_load);

IrTypeWith2!(Neg, T, U, is_neg);

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
    where T: Debug + Clone + PartialEq + Eq + AsAny,
          U: Debug + Clone + PartialEq + Eq + AsAny,
{
    pub(crate) out: Var,
    pub(crate) cond: Var,

    pub(crate) yes: T,
    pub(crate) no: U,
}

/// checks if the node is another node
#[allow(missing_docs)]
pub trait IsNode {
    fn is_alloca(&self) -> bool { false }
    fn is_assign(&self) -> bool { false }
    fn is_cast(&self) -> bool { false }
    fn is_br(&self) -> bool { false }
    fn is_brcond(&self) -> bool { false }
    fn is_call(&self) -> bool { false }
    fn is_cmp(&self) -> bool { false }
    fn is_debug(&self) -> bool { false }
    fn is_getelemptr(&self) -> bool { false }
    fn is_load(&self) -> bool { false }
    fn is_add(&self) -> bool { false }
    fn is_sub(&self) -> bool { false }
    fn is_xor(&self) -> bool { false }
    fn is_or(&self) -> bool { false }
    fn is_and(&self) -> bool { false }
    fn is_mul(&self) -> bool { false }
    fn is_div(&self) -> bool { false }
    fn is_rem(&self) -> bool { false }
    fn is_shl(&self) -> bool { false }
    fn is_shr(&self) -> bool { false }
    fn is_neg(&self) -> bool { false }
    fn is_phi(&self) -> bool { false }
    fn is_ret(&self) -> bool { false }
    fn is_select(&self) -> bool { false }
    fn is_store(&self) -> bool { false }
    fn is_switch(&self) -> bool { false }
}

use crate::Support::{AsAny, ColorClass, ColorProfile};
/// The ir trait
pub trait Ir: Debug + Any + EvalOptVisitor + IsNode {
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
    fn compile(&self, registry: &mut TargetBackendDescr, module: &mut crate::prelude::Module);

    /// Compiles the node with node information to the given target
    fn compile_dir(&self, compiler: &mut crate::CodeGen::IrCodeGenHelper, block: &crate::prelude::Block, module: &mut crate::prelude::Module);

    /// Returns if the node uses the variable
    fn uses(&self, _: &Var) -> bool {
        false
    }

    /// checks if the node is equal to the other node (used for the implementation of Eq)
    fn is(&self, other: &Box<dyn Ir>) -> bool {
        other.dump() == self.dump()
    }

    /// returns a mutable reference to the vars used by the node as input
    fn inputs_mut(&mut self) -> Vec<&mut Var>;

    /// returns the vars used by the node as input
    fn inputs(&self) -> Vec<Var>;

    /// returns the output var
    fn output(&self) -> Option<Var>;
}

/// A trait used for constant propagination
pub trait EvalOptVisitor {
    /// inlines the variables if possible
    fn maybe_inline(&self, const_values: &HashMap<String, Type>) -> Option<Box<dyn Ir>>;

    /// evaluteas the node
    fn eval(&self) -> Option<Box<dyn Ir>>;
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

/// an operand for ir nodes
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IROperand {
    /// A type (like i64)
    Type(Type),
    /// A variable (like %.)
    Var(Var),
}

impl IROperand {
    #[inline]
    /// Returns if the operand is a type const (like 5 - a constant number)
    pub fn is_type(&self) -> bool {
        matches!(self, IROperand::Type(_))
    }

    #[inline]
    /// Returns if the operand is a var
    pub fn is_var(&self) -> bool {
        matches!(self, IROperand::Var(_))
    }

    #[inline]
    /// Returns the type of the operand
    pub fn get_ty(&self) -> TypeMetadata {
        match self {
            IROperand::Type(ty) => (*ty).into(),
            IROperand::Var(var) => var.ty,
        }
    }

    #[inline]
    /// Returns the unwraped inner type value (else it panics)
    pub fn get_typeconst(&self) -> Type {
        let IROperand::Type(ret) = self else { panic!(); };
        return *ret;
    }

    #[inline]
    /// Returns the unwraped inner var value (else it panics)
    pub fn get_var(&self) -> Var {
        let IROperand::Var(ret) = self else { panic!(); };
        return ret.to_owned();
    }
}

impl std::fmt::Display for IROperand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            IROperand::Type(ty) => match ty {
                Type::u8(i) => format!("{i}"),
                Type::u16(i) => format!("{i}"),
                Type::u32(i) => format!("{i}"),
                Type::u64(i) => format!("{i}"),
    
                Type::i8(i) => format!("{i}"),
                Type::i16(i) => format!("{i}"),
                Type::i32(i) => format!("{i}"),
                Type::i64(i) => format!("{i}"),
    
                Type::ptr(adr) => format!("{adr:#04x}"),
                Type::Void => format!("0"),
    
                Type::f32(i) => format!("{i}"),
                Type::f64(i) => format!("{i}"),
            }
            IROperand::Var(var) => var.name.to_string(),
        })
    }
}

impl AsAny for IROperand {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}