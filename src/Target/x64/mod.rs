//! The x64 Target: used for compiling ir and inline asm into x64 machine code

use std::collections::VecDeque;

mod compilation;
//use compilation::*;

use super::{CallConv, Lexer, TargetBackendDescr};
mod reg;
use compilation::construct_compilation_helper;
pub use reg::*;

pub(crate) mod call;
mod asm;

pub use asm::*;

use crate::Target::Compiler;

/// Initializes the x86-64 target
pub fn initializeX64Target(call_conv: CallConv) -> TargetBackendDescr {
    let mut target = TargetBackendDescr::new();

    target.init = Some(initializeX64Target);

    target.lexer = Some(x64Lexer {}.boxed());
    target.compile = Some(x64Parser { tokens: VecDeque::new(), out: None }.boxed());

    target.helper = Some(construct_compilation_helper(call_conv));

    target.call = call_conv;


    target
}