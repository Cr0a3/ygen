//! The x64 Target: used for compiling ir and inline asm into x64 machine code

use std::collections::VecDeque;

use ir::*;

use super::{CallConv, TargetBackendDescr};

pub(crate) mod ir;
pub(crate) mod call;

/// Initializes the x86-64 target
pub fn initializeX64Target(call_conv: CallConv) -> TargetBackendDescr {
    let mut target = TargetBackendDescr::new();

    match call_conv {
        CallConv::WindowsFastCall => {
            target.backend.openUsableRegisters64 = VecDeque::from(
                vec!["rsi".into(), "rdi".into(), 
                "r10".into(), "r11".into(), "r12".into(), "r13".into(), "r14".into(), "r15".into()]
            );
            target.backend.openUsableRegisters32 = VecDeque::from(
                vec!["esi".into(), "edi".into(), 
                "r10d".into(), "r11d".into(), "r12d".into(), "r13d".into(), "r14d".into(), "r15d".into()]
            );
            target.backend.openUsableRegisters16 = VecDeque::from(
                vec!["si".into(), "di".into(), 
                "r10w".into(), "r11w".into(), "r12w".into(), "r13w".into(), "r14w".into(), "r15w".into()]
            );
            target.backend.openUsableRegisters8 = VecDeque::from(
                vec!["sil".into(), "dil".into(), 
                "r10b".into(), "r11b".into(), "r12b".into(), "r13b".into(), "r14b".into(), "r15b".into()]
            );
        },
        CallConv::SystemV => {
            target.backend.openUsableRegisters64 = VecDeque::from(
                vec!["r10".into(), "r11".into(), "r12".into(), "r13".into(), "r14".into(), "r15".into()]
            );
            target.backend.openUsableRegisters32 = VecDeque::from(
                vec!["r10d".into(), "r11d".into(), "r12d".into(), "r13d".into(), "r14d".into(), "r15d".into()]
            );
            target.backend.openUsableRegisters16 = VecDeque::from(
                vec!["r10w".into(), "r11w".into(), "r12w".into(), "r13w".into(), "r14w".into(), "r15w".into()]
            );
            target.backend.openUsableRegisters8 = VecDeque::from(
                vec!["r10b".into(), "r11b".into(), "r12b".into(), "r13b".into(), "r14b".into(), "r15b".into()]
            );
        },
    }

    target.setCompileFuncForRetType(CompileRetType);
    target.setCompileFuncForRetVar(CompileRetVar);
    target.setCompileFuncForConstAssign(CompileConstAssign);
    target.setCompileFuncForAddVarVar(CompileAddVarVar);
    target.setCompileFuncForAddTypeType(CompileAddTyTy);

    target
}