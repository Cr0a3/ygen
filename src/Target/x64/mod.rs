//! The x64 Target: used for compiling ir and inline asm into x64 machine code

use std::collections::VecDeque;

use ir::*;

use super::{CallConv, Lexer, Reg, TargetBackendDescr};
mod reg;
pub use reg::*;

pub(crate) mod ir;
pub(crate) mod call;
mod asm;

pub use asm::*;

use crate::Target::Compiler;

/// Initializes the x86-64 target
pub fn initializeX64Target<'a>(call_conv: CallConv) -> TargetBackendDescr<'a> {
    let mut target = TargetBackendDescr::new();

    target.init = Some(initializeX64Target);
    target.buildAsm = Some(buildAsmX86);

    target.lexer = Some(x64Lexer {}.boxed());
    target.compile = Some(x64Parser { tokens: VecDeque::new(), out: None }.boxed());

    target.call = call_conv;

    target.backend.savedRegisters = vec![
        x64Reg::R10.boxed(), x64Reg::R11.boxed(), x64Reg::R12.boxed(), x64Reg::R13.boxed(), x64Reg::R14.boxed(), x64Reg::R15.boxed(),
    ];

    match call_conv {
        CallConv::WindowsFastCall => {
            target.backend.openUsableRegisters64 = VecDeque::from(
                vec![x64Reg::Rsi.boxed(), x64Reg::Rdi.boxed(), 
                x64Reg::R10.boxed(), x64Reg::R11.boxed(), x64Reg::R12.boxed(), x64Reg::R13.boxed(), x64Reg::R14.boxed(), x64Reg::R15.boxed()]
            );
            target.backend.openUsableRegisters32 = VecDeque::from(
                vec![x64Reg::Esi.boxed(), x64Reg::Edi.boxed(), 
                x64Reg::R10d.boxed(), x64Reg::R11d.boxed(), x64Reg::R12d.boxed(), x64Reg::R13d.boxed(), x64Reg::R14d.boxed(), x64Reg::R15d.boxed()]
            );
            target.backend.openUsableRegisters16 = VecDeque::from(
                vec![x64Reg::Si.boxed(), x64Reg::Di.boxed(), 
                x64Reg::R10w.boxed(), x64Reg::R11w.boxed(), x64Reg::R12w.boxed(), x64Reg::R13w.boxed(), x64Reg::R14w.boxed(), x64Reg::R15w.boxed()]
            );
            target.backend.openUsableRegisters8 = VecDeque::from(
                vec![x64Reg::Sil.boxed(), x64Reg::Dil.boxed(), 
                x64Reg::R10b.boxed(), x64Reg::R11b.boxed(), x64Reg::R12b.boxed(), x64Reg::R13b.boxed(), x64Reg::R14b.boxed(), x64Reg::R15b.boxed()]
            );
        },
        CallConv::SystemV => {
            target.backend.openUsableRegisters64 = VecDeque::from(
                vec![x64Reg::R10.boxed(), x64Reg::R11.boxed(), x64Reg::R12.boxed(), x64Reg::R13.boxed(), x64Reg::R14.boxed(), x64Reg::R15.boxed()]
            );
            target.backend.openUsableRegisters32 = VecDeque::from(
                vec![x64Reg::R10d.boxed(), x64Reg::R11d.boxed(), x64Reg::R12d.boxed(), x64Reg::R13d.boxed(), x64Reg::R14d.boxed(), x64Reg::R15d.boxed()]
            );
            target.backend.openUsableRegisters16 = VecDeque::from(
                vec![x64Reg::R10w.boxed(), x64Reg::R11w.boxed(), x64Reg::R12w.boxed(), x64Reg::R13w.boxed(), x64Reg::R14w.boxed(), x64Reg::R15w.boxed()]
            );
            target.backend.openUsableRegisters8 = VecDeque::from(
                vec![x64Reg::R10b.boxed(), x64Reg::R11b.boxed(), x64Reg::R12b.boxed(), x64Reg::R13b.boxed(), x64Reg::R14b.boxed(), x64Reg::R15b.boxed()]
            );
        },
        CallConv::AppleAarch64 => todo!(),
        CallConv::WasmBasicCAbi => todo!(),
    }

    target.setCompileFuncForRetType(CompileRetType);
    target.setCompileFuncForRetVar(CompileRetVar);
    target.setCompileFuncForConstAssign(CompileConstAssign);
    target.setCompileFuncForAddVarVar(CompileAddVarVar);
    target.setCompileFuncForAddTypeType(CompileAddTyTy);
    target.setCompileFuncForSubVarVar(CompileSubVarVar);
    target.setCompileFuncForSubTypeType(CompileSubTyTy);
    target.setCompileFuncForXorVarVar(CompileXorVarVar);
    target.setCompileFuncForXorTypeType(CompileXorTyTy);

    target
}