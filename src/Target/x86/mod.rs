mod lower;
/// X64 registers
pub mod reg;

/// X64 assembly
pub mod asm;

use crate::{CodeGen::{dag_lower::DagLower, reg::{Reg, TargetReg}}, IR::TypeMetadata};

use super::{asm_printer::AsmPrinter, compile::McCompile, parser::AsmParser, BackendInfos, CallConv};

pub(self) static mut CALLING_CONVENTION: CallConv = CallConv::SystemV;

/// Returns the calling convention used by the x64 backend
pub fn get_call() -> CallConv {
    unsafe { CALLING_CONVENTION }
}

/// Initializes the x86 target
pub fn initializeX86Target(call_conv: CallConv) -> BackendInfos {
    unsafe {
        CALLING_CONVENTION = call_conv
    }

    BackendInfos {
        dag: DagLower::new(lower::x86_lower),
        mc: McCompile {},
        asm_printer: AsmPrinter {},
        parser: AsmParser {},
    }
}

pub(crate) fn ret_reg(ty: TypeMetadata) -> crate::CodeGen::reg::Reg {
    if ty.float() {
        Reg {
            size: ty.byteSize(), // actually xmm registers are 128bit wide but we just say that they exactly fit the float
            reg: TargetReg::X64(reg::X64Reg::Xmm0()),
        }
    } else {
        let mut reg = reg::X64Reg::Rax();
        reg.size = ty.byteSize().into();
        Reg {
            size: ty.byteSize(), // actually xmm registers are 128bit wide but we just say that they exactly fit the float
            reg: TargetReg::X64(reg),
        }
    }
}