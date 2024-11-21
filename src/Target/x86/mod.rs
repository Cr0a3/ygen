mod lower;
/// X64 registers
pub mod reg;

/// X64 specific register allocation hooks
pub mod alloc;

/// X64 assembly
pub mod asm;

use std::collections::HashMap;
use std::sync::{Mutex, Once};

use reg::X64Reg;

use crate::CodeGen::{dag_lower::DagLower, regalloc_iterated_col::ItRegCoalAllocBase};
use crate::CodeGen::reg::{Reg, TargetReg};
use crate::prelude::TypeMetadata;
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

    let mut free_regs = Vec::new();
    free_regs.push(Reg::new_x64(X64Reg::Rax()));
    free_regs.push(Reg::new_x64(X64Reg::Rbx()));
    free_regs.push(Reg::new_x64(X64Reg::Rcx()));
    free_regs.push(Reg::new_x64(X64Reg::Rdx()));
    free_regs.push(Reg::new_x64(X64Reg::Rdi()));
    free_regs.push(Reg::new_x64(X64Reg::Rsi()));
    free_regs.push(Reg::new_x64(X64Reg::R8()));
    free_regs.push(Reg::new_x64(X64Reg::R9()));
    free_regs.push(Reg::new_x64(X64Reg::R10()));
    free_regs.push(Reg::new_x64(X64Reg::R11()));
    free_regs.push(Reg::new_x64(X64Reg::R12()));
    free_regs.push(Reg::new_x64(X64Reg::R13()));
    free_regs.push(Reg::new_x64(X64Reg::R14()));
    free_regs.push(Reg::new_x64(X64Reg::R15()));
    free_regs.push(Reg::new_x64(X64Reg::Xmm0()));
    free_regs.push(Reg::new_x64(X64Reg::Xmm1()));
    free_regs.push(Reg::new_x64(X64Reg::Xmm2()));
    free_regs.push(Reg::new_x64(X64Reg::Xmm3()));
    free_regs.push(Reg::new_x64(X64Reg::Xmm4()));
    free_regs.push(Reg::new_x64(X64Reg::Xmm5()));
    free_regs.push(Reg::new_x64(X64Reg::Xmm6()));
    free_regs.push(Reg::new_x64(X64Reg::Xmm7()));
    free_regs.push(Reg::new_x64(X64Reg::Xmm8()));
    free_regs.push(Reg::new_x64(X64Reg::Xmm9()));
    free_regs.push(Reg::new_x64(X64Reg::Xmm10()));
    free_regs.push(Reg::new_x64(X64Reg::Xmm11()));
    free_regs.push(Reg::new_x64(X64Reg::Xmm12()));
    free_regs.push(Reg::new_x64(X64Reg::Xmm13()));
    free_regs.push(Reg::new_x64(X64Reg::Xmm14()));
    free_regs.push(Reg::new_x64(X64Reg::Xmm15()));

    let alloc = ItRegCoalAllocBase {
        regs: free_regs,
        arg_processor: Some(alloc::arg_proc),
    };

    BackendInfos {
        dag: DagLower::new(lower::x86_lower, lower::x86_tmps),
        mc: McCompile {},
        asm_printer: AsmPrinter {},
        parser: AsmParser {},
        allocator: alloc,
    }
}

pub(crate) fn ret_reg(ty: TypeMetadata) -> crate::CodeGen::reg::Reg {
    if ty.float() {
        Reg {
            size: ty.byteSize(), // actually xmm registers are 128bit wide but we just say that they exactly fit the float
            reg: TargetReg::X64(reg::X64Reg::Xmm0()),
        }
    } else if ty.isVectorTy() { // simd xmm.. registers
        Reg {
            size: ty.bitSize(),
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

impl CallConv {
    /// Returns if the x64 register is callee saved
    #[inline]
    pub fn x86_is_callee_saved(&self, reg: reg::X64RegVariant) -> bool {
        use reg::X64RegVariant::*;

        match reg {
            Rbx | Rbp | Rsp | R12 | R13 | R14 | R15 => true,
            Xmm6 | Xmm7 | Xmm8 | Xmm9 | Xmm10 |
            Xmm11 | Xmm12 | Xmm13 | Xmm14 |
            Xmm15 => get_call() == CallConv::WindowsFastCall,
            _ => false,
        }
    }
}

static mut BLOCK_RELS: Option<Mutex<HashMap<i64, String>>> = None;
static mut BLOCK_RELS_LAST: i64 = 0;
static BLOCK_RELS_INIT: Once = Once::new();

fn add_block_rel(target: String) -> i64 {
    if unsafe { BLOCK_RELS.is_none()} {
        unsafe {
            BLOCK_RELS_INIT.call_once(|| {
                BLOCK_RELS = Some(Mutex::new(HashMap::new()));
            })
        }
    }

    let map = unsafe { BLOCK_RELS.as_ref().expect("Global hashmap not initialized") };

    let mut lock = map.lock().expect("Locking failed");

    lock.insert(unsafe { BLOCK_RELS_LAST }, target);

    unsafe {
        BLOCK_RELS_LAST += 1;
        BLOCK_RELS_LAST - 1
    } 
}

fn get_block_rel(target: i64) -> String {
    if unsafe { BLOCK_RELS.is_none()} {
        unsafe {
            BLOCK_RELS_INIT.call_once(|| {
                BLOCK_RELS = Some(Mutex::new(HashMap::new()));
            })
        }
    }

    let map = unsafe { BLOCK_RELS.as_ref().expect("Global hashmap not initialized") };

    let lock = map.lock().expect("Locking failed");

    let target = lock.get(&target).expect("unknown block");

    target.to_owned()
}