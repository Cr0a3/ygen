//! The x64 Target

use std::collections::VecDeque;

mod compilation;
//use compilation::*;

use super::{CallConv, Lexer, TargetBackendDescr, WhiteList};
mod reg;
use compilation::construct_compilation_helper;
pub use reg::*;

mod asm;
mod lower;


pub use asm::*;

use crate::{CodeGen::MachineMnemonic, Target::Compiler};

/// Initializes the x86-64 target
pub fn initializeX64Target(call_conv: CallConv) -> TargetBackendDescr {
    let mut target = TargetBackendDescr::new();

    target.call = call_conv;

    target.init = Some(initializeX64Target);

    target.lexer = Some(x64Lexer {}.boxed());
    target.compile = Some(x64Parser { tokens: VecDeque::new(), out: None }.boxed());

    target.whitelist = construct_whitelist();

    target.helper = Some(construct_compilation_helper(call_conv));

    target
}

fn construct_whitelist() -> WhiteList {
    let mut whitelist = WhiteList::new();

    // everything is allowed by default
    // so only add illegal stuff here

    whitelist.forbid(MachineMnemonic::FAnd);
    whitelist.forbid(MachineMnemonic::FNeg);
    whitelist.forbid(MachineMnemonic::FOr);
    whitelist.forbid(MachineMnemonic::FXor);
    whitelist.forbid(MachineMnemonic::FRem);
    whitelist.forbid(MachineMnemonic::FShl);
    whitelist.forbid(MachineMnemonic::FShr);

    whitelist
}