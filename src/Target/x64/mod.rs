//! The x64 Target: used for compiling ir and inline asm into x64 machine code

use std::collections::VecDeque;

use ir::*;

use super::{registry::TARGETS, Arch, CallConv};

pub(crate) mod ir;
pub(crate) mod call;

/// Initializes the x86-64 target
pub fn initializeX64Target(call_conv: CallConv) {
    let target = &mut TARGETS.lock().unwrap();

    target.set_inited(Arch::X86_64);

    match call_conv {
        CallConv::WindowsFastCall => {
            target.backend.openUsableRegisters = VecDeque::from(
                vec!["rsi".into(), "rdi".into(), 
                "r10".into(), "r11".into(), "r12".into(), "r13".into(), "r14".into(), "r15".into()]
            );
        },
        CallConv::SystemV => {
            target.backend.openUsableRegisters = VecDeque::from(
                vec!["r10".into(), "r11".into(), "r12".into(), "r13".into(), "r14".into(), "r15".into()]
            );
        },
    }

    target.setCompileFuncForRetType(Arch::X86_64, CompileRetType);
    target.setCompileFuncForRetVar(Arch::X86_64, CompileRetVar);
    target.setCompileFuncForConstAssign(Arch::X86_64, CompileConstAssign);
    target.setCompileFuncForAddVarVar(Arch::X86_64, CompileAddVarVar);
    target.setCompileFuncForAddTypeType(Arch::X86_64, CompileAddTyTy);
}