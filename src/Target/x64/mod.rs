//! The x64 Target: used for compiling ir and inline asm into x64 machine code

use std::collections::VecDeque;

mod compilation;
//use compilation::*;

use super::{CallConv, Lexer, TargetBackendDescr, WhiteList};
mod reg;
use compilation::construct_compilation_helper;
pub use reg::*;

pub(crate) mod call;
mod asm;
mod lower;

pub use asm::*;

use crate::{CodeGen::MachineMnemonic, Target::Compiler};

/// Initializes the x86-64 target
pub fn initializeX64Target(call_conv: CallConv) -> TargetBackendDescr {
    let mut target = TargetBackendDescr::new();

    target.init = Some(initializeX64Target);

    target.lexer = Some(x64Lexer {}.boxed());
    target.compile = Some(x64Parser { tokens: VecDeque::new(), out: None }.boxed());

    target.whitelist = construct_whitelist();

    target.helper = Some(construct_compilation_helper(call_conv));

    target.call = call_conv;


    target
}

fn construct_whitelist() -> WhiteList {
    let mut whitelist = WhiteList::new();

    whitelist.allow(MachineMnemonic::Move);
    whitelist.allow(MachineMnemonic::Add);
    whitelist.allow(MachineMnemonic::And);
    whitelist.allow(MachineMnemonic::Div);
    whitelist.allow(MachineMnemonic::Mul);
    whitelist.allow(MachineMnemonic::Or);
    whitelist.allow(MachineMnemonic::Sub);
    whitelist.allow(MachineMnemonic::Xor);
    whitelist.allow(MachineMnemonic::Return);

    //whitelist.allow(MachineMnemonic::Zext); todo!()
    //whitelist.allow(MachineMnemonic::Downcast); todo!()

    whitelist
}