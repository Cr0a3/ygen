use std::{any::Any, fmt::Debug, hash::Hash};
use std::collections::HashMap;
use super::instrincs::Intrinsic;
use super::{Block, BlockId, Const, FuncId, Function, FunctionType, Type, TypeMetadata, Var, VerifyError};

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
mod vec_insr_extr;

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
        pub struct $name {
            /// first inner value
            pub(crate) inner1: $param1,
            /// second inner value
            pub(crate) inner2: $param2,
            /// third inner value
            pub(crate) inner3: $param3,
        }


        impl $name {
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

        impl IsNode for $name {
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
        pub struct $name {
            /// first inner value
            pub inner1: $param1,
            /// second inner value
            pub inner2: $param2,
        }


        impl $name {
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

        impl IsNode for $name {
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
        pub struct $name {
            /// inner value
            pub(crate) inner1: $param1,
        }


        impl $name {
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

        impl IsNode for $name {
            fn $is_func(&self) -> bool {
                true
            }
        }
    };
}

/// The assign ir node
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assign<T, U> where 
    T: Debug + Clone + PartialEq + Eq,
    U: Debug + Clone + PartialEq + Eq
{
    pub(crate) inner1: T,
    pub(crate) inner2: U,
}

impl<T, U>  Assign<T, U> where 
    T: Debug + Clone + PartialEq + Eq,
    U: Debug + Clone + PartialEq + Eq,
    Assign<T, U>: Ir,
{
    /// creates an new assign ir node
    pub fn new(out: T, value: U) -> Box<dyn Ir> {
        Box::new(Self {
            inner1: out,
            inner2: value,
        })
    }
}

impl<T, U> IsNode for Assign<T, U> where 
    T: Debug + Clone + PartialEq + Eq,
    U: Debug + Clone + PartialEq + Eq,
    Assign<T, U>: Ir,
{
    fn is_assign(&self) -> bool {
        true
    }
} 

IrTypeWith1!(Return, IROperand, is_ret);

IrTypeWith3!(Cast, IROperand, TypeMetadata, Var, is_cast);

IrTypeWith3!(Add, IROperand, IROperand, Var, is_add);
IrTypeWith3!(Sub, IROperand, IROperand, Var, is_sub);
IrTypeWith3!(Xor, IROperand, IROperand, Var, is_xor);
IrTypeWith3!(Or, IROperand, IROperand, Var, is_or);
IrTypeWith3!(And, IROperand, IROperand, Var, is_and);
IrTypeWith3!(Mul, IROperand, IROperand, Var, is_mul);
IrTypeWith3!(Div, IROperand, IROperand, Var, is_div);
IrTypeWith3!(Rem, IROperand, IROperand, Var, is_rem);
IrTypeWith3!(Shl, IROperand, IROperand, Var, is_shl);
IrTypeWith3!(Shr, IROperand, IROperand, Var, is_shr);

IrTypeWith1!(Br, BlockId, is_br);
IrTypeWith3!(BrCond, Var, BlockId, BlockId, is_brcond);

IrTypeWith2!(Alloca, Var, TypeMetadata, is_alloca);
IrTypeWith2!(Store, Var, IROperand, is_store);
IrTypeWith3!(Load, Var, TypeMetadata, IROperand, is_load);

IrTypeWith2!(Neg, IROperand, Var, is_neg);

IrTypeWith3!(VecExtract, /*out*/Var, Var, usize, is_vec_extract);

/// Inserts a value into a vector
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(missing_docs)]
pub struct VecInsert {
    pub out: Var,
    pub vec: Var,
    pub elem: IROperand,
    pub position: usize,
}

impl IsNode for VecInsert {
    fn is_vec_insert(&self) -> bool {
        true
    }
}
/// The cmp node is used to compare values
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cmp {
    pub(crate) mode: cmp::CmpMode,
    pub(crate) ls: IROperand,
    pub(crate) rs: IROperand,
    pub(crate) out: Var,
}

impl Cmp {
    /// Creates a new instance
    #[allow(dead_code)]
    pub(crate) fn new(mode: cmp::CmpMode, ls: IROperand, rs: IROperand, out: Var) -> Box<Self> {
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
pub struct Select {
    pub(crate) out: Var,
    pub(crate) cond: Var,

    pub(crate) yes: IROperand,
    pub(crate) no: IROperand,
}

/// Call ir node
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Call {
    pub(crate) out: Var,
    pub(crate) func: FuncId,
    pub(crate) args: Vec<IROperand>,
    /// The call is a instric call then this is the instric
    pub(crate) instric: Option<Intrinsic>,
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
    fn is_vec_insert(&self) -> bool { false }
    fn is_vec_extract(&self) -> bool { false }
}

use crate::Support::{AsAny, ColorClass, ColorProfile};
/// The ir trait
pub trait Ir: Debug + Any + EvalOptVisitor + IsNode + crate::CodeGen::DagVisitor {
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

    /// returns the type of the node
    fn ty(&self) -> Option<TypeMetadata>;
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

    /// Formats the operand without the type
    pub(crate) fn fmt_2(&self) -> String {
        match self {
            IROperand::Type(ty) => match ty {
                Type::u8(i) => format!("{}", i),
                Type::u16(i) => format!("{}", i),
                Type::u32(i) => format!("{}", i),
                Type::u64(i) => format!("{}", i),
    
                Type::i8(i) => format!("{}", i),
                Type::i16(i) => format!("{}", i),
                Type::i32(i) => format!("{}", i),
                Type::i64(i) => format!("{}", i),
    
                Type::ptr(adr) => format!("{:#04x}", adr),
                Type::Void => format!("0"),
    
                Type::f32(i) => format!("{}", i),
                Type::f64(i) => format!("{}", i),
    
                Type::Vector(vec) => format!("<{vec}>"),   
            },
            IROperand::Var(var) => var.name.to_string(),
        }
    }
}

impl std::fmt::Display for IROperand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            IROperand::Type(ty) => ty.to_string(),
            IROperand::Var(var) => format!("{} {}", var.ty, var.name),
        })
    }
}

impl AsAny for IROperand {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}