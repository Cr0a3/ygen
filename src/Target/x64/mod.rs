//! The x64 Target: used for compiling ir and inline asm into x64 machine code

use std::{collections::VecDeque, error::Error, fs::OpenOptions, io::Write, path::Path};

use ir::*;

use crate::prelude::Module;

use super::{registry::Reg, CallConv, TargetBackendDescr};
mod reg;
pub(crate) use reg::*;

pub(crate) mod ir;
pub(crate) mod call;
// mod AsmColorize; needs to be implementat
// pub use AsmColorize::AsmColorizer;

/// Initializes the x86-64 target
pub fn initializeX64Target<'a>(call_conv: CallConv) -> TargetBackendDescr<'a> {
    let mut target = TargetBackendDescr::new();

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
    }

    target.setCompileFuncForRetType(CompileRetType);
    target.setCompileFuncForRetVar(CompileRetVar);
    target.setCompileFuncForConstAssign(CompileConstAssign);
    target.setCompileFuncForAddVarVar(CompileAddVarVar);
    target.setCompileFuncForAddTypeType(CompileAddTyTy);

    target
}

impl Module {
    /// Compiles the IR of the module into an string which will then gets written into an asm file using intel syntax
    pub fn emitToAsmFile(&self, path: &Path) -> Result<(), Box<dyn Error>> {
        let call = CallConv::WindowsFastCall; // todo: change it in the future to the actual target triple

        let mut target = initializeX64Target(call);

        let mut file = OpenOptions::new().create(true).write(true)
                                .open(path)?;

        let mut lines = String::new();

        for (name, func) in &self.funcs {
            lines += &format!("{}:\n", name);

            for block in &func.blocks {
                if block.name.to_lowercase() != "entry" {
                    lines += &format!("  {}:\n", block.name)
                }

                let asm_lines = block.buildAsmX86(&func, &call, &mut target);

                for line in asm_lines {
                    lines += &format!("\t{}\n", line);
                }
            }
        }

        file.write_all(lines.as_bytes())?;

        Ok(())
    }
}