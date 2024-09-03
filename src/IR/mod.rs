mod module;
mod func;
mod typ;
mod builder;
mod block;
mod var;
mod constant;
mod nodes;
pub mod parser;

/// Stores all ir nodes and the ir trait
pub mod ir {
    pub use super::nodes::*;
}

use std::error::Error;
use std::fmt::Display;

pub use module::Module;
pub use func::{Function, FunctionType, FnTy, Func};
pub use constant::Const;
pub use typ::Type;
pub use typ::TypeMetadata;
pub use builder::IRBuilder;
pub use block::Block;
pub use var::Var;

/// An error which stores if an ir node is invalid
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerifyError {
    /// The type of the ret node doesn't match the function return type
    RetTyNotFnTy(TypeMetadata, TypeMetadata),
    /// The type of op0 operand doesn't match the type of the op1 operand
    Op0Op1TyNoMatch(TypeMetadata, TypeMetadata),
    /// The given argument type doesn't overllap with the actual argument type
    InvalidArgumentTypeFound,
    /// To many arguments were supplyed
    ToManyArgumentsWereSupplyed,
}

impl Display for VerifyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match &self {
            VerifyError::RetTyNotFnTy(retTy, fnTy) => {
                format!(
                        "The return type of the return node needs to be the same as the one of the function:\n{}",
                format!("  ret {} ... but the function return type is {}", retTy, fnTy),
                )
            },
            VerifyError::Op0Op1TyNoMatch(op0Ty, op1Ty) => {
                format!(
                        "The type of the 1. operand needs to be the same as the one of the second operand:\n{}",
                format!("  op0 {} ... but the op1 type is {}", op0Ty, op1Ty),
                )
                
            },
            VerifyError::InvalidArgumentTypeFound => {
                format!(
                    "an unexpected argument type was found"
                )
            },
            VerifyError::ToManyArgumentsWereSupplyed => {
                format!(
                    "too many arguments were supplyed"
                )
            }
        })
    }
}
impl Error for VerifyError {}