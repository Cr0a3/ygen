mod lower;
/// X86 registers
pub mod reg;

/// X86 specific register allocation hooks
pub mod alloc;

/// X86 operation compilation for the dag
pub mod operation;

/// X86 assembly
pub mod asm;

use std::collections::HashMap;
use std::sync::{Mutex, Once};

use reg::X86Reg;

use crate::CodeGen::{dag_lower::DagLower, regalloc_iterated_col::ItRegCoalAllocBase};
use crate::CodeGen::reg::{Reg, TargetReg};
use crate::prelude::TypeMetadata;
use crate::IR::VecTy;
use super::black_list::TargetBlackList;
use super::{asm_printer::AsmPrinter, compile::McCompile, parser::AsmParser, BackendInfos, CallConv};

pub(self) static mut CALLING_CONVENTION: CallConv = CallConv::SystemV;

/// Returns the calling convention used by the X86 backend
pub fn get_call() -> CallConv {
    unsafe { CALLING_CONVENTION }
}

/// Initializes the x86 target
pub fn initializeX86Target(call_conv: CallConv) -> BackendInfos {
    unsafe {
        CALLING_CONVENTION = call_conv
    }

    let mut free_regs = Vec::new();
    free_regs.push(Reg::new_x86(X86Reg::Rax()));
    free_regs.push(Reg::new_x86(X86Reg::Rbx()));
    free_regs.push(Reg::new_x86(X86Reg::Rcx()));
    free_regs.push(Reg::new_x86(X86Reg::Rdx()));
    free_regs.push(Reg::new_x86(X86Reg::Rdi()));
    free_regs.push(Reg::new_x86(X86Reg::Rsi()));
    free_regs.push(Reg::new_x86(X86Reg::R8()));
    free_regs.push(Reg::new_x86(X86Reg::R9()));
    free_regs.push(Reg::new_x86(X86Reg::R10()));
    free_regs.push(Reg::new_x86(X86Reg::R11()));
    free_regs.push(Reg::new_x86(X86Reg::R12()));
    free_regs.push(Reg::new_x86(X86Reg::R13()));
    free_regs.push(Reg::new_x86(X86Reg::R14()));
    free_regs.push(Reg::new_x86(X86Reg::R15()));
    free_regs.push(Reg::new_x86(X86Reg::Xmm0()));
    free_regs.push(Reg::new_x86(X86Reg::Xmm1()));
    free_regs.push(Reg::new_x86(X86Reg::Xmm2()));
    free_regs.push(Reg::new_x86(X86Reg::Xmm3()));
    free_regs.push(Reg::new_x86(X86Reg::Xmm4()));
    free_regs.push(Reg::new_x86(X86Reg::Xmm5()));
    free_regs.push(Reg::new_x86(X86Reg::Xmm6()));
    free_regs.push(Reg::new_x86(X86Reg::Xmm7()));
    free_regs.push(Reg::new_x86(X86Reg::Xmm8()));
    free_regs.push(Reg::new_x86(X86Reg::Xmm9()));
    free_regs.push(Reg::new_x86(X86Reg::Xmm10()));
    free_regs.push(Reg::new_x86(X86Reg::Xmm11()));
    free_regs.push(Reg::new_x86(X86Reg::Xmm12()));
    free_regs.push(Reg::new_x86(X86Reg::Xmm13()));
    free_regs.push(Reg::new_x86(X86Reg::Xmm14()));
    free_regs.push(Reg::new_x86(X86Reg::Xmm15()));

    let alloc = ItRegCoalAllocBase {
        regs: free_regs,
        arg_processor: Some(alloc::arg_proc),
        mem_processor: Some(alloc::mem_proc),
        overwrite_proc: Some(lower::ov_proc),
        stack: -8,
    };

    let mut allow = TargetBlackList::new();

    allow.disalow_vecs(); // so only our allowed vectors are allowed

    allow.add_supported_vec(VecTy { size: 2, ty: TypeMetadata::i64.into() });
    allow.add_supported_vec(VecTy { size: 2, ty: TypeMetadata::u64.into() });
    allow.add_supported_vec(VecTy { size: 2, ty: TypeMetadata::f64.into() });

    allow.add_supported_vec(VecTy { size: 4, ty: TypeMetadata::i32.into() });
    allow.add_supported_vec(VecTy { size: 4, ty: TypeMetadata::u32.into() });
    allow.add_supported_vec(VecTy { size: 4, ty: TypeMetadata::f32.into() });

    allow.add_supported_vec(VecTy { size: 8, ty: TypeMetadata::i16.into() });
    allow.add_supported_vec(VecTy { size: 8, ty: TypeMetadata::u16.into() });

    allow.add_supported_vec(VecTy { size: 16, ty: TypeMetadata::i8.into() });
    allow.add_supported_vec(VecTy { size: 16, ty: TypeMetadata::u8.into() });

    BackendInfos {
        dag: DagLower::new(lower::x86_lower, lower::x86_tmps),
        mc: McCompile {},
        asm_printer: AsmPrinter {},
        parser: AsmParser {},
        allocator: alloc,
        allowment: allow,
    }
}

pub(crate) fn ret_reg(ty: TypeMetadata) -> crate::CodeGen::reg::Reg {
    if ty.float() {
        Reg {
            size: ty.byteSize(), // actually xmm registers are 128bit wide but we just say that they exactly fit the float
            reg: TargetReg::X86(reg::X86Reg::Xmm0()),
        }
    } else if ty.isVectorTy() { // simd xmm.. registers
        Reg {
            size: ty.bitSize(),
            reg: TargetReg::X86(reg::X86Reg::Xmm0()),
        }
    } else {
        let mut reg = reg::X86Reg::Rax();
        reg.size = ty.byteSize().into();
        Reg {
            size: ty.byteSize(), // actually xmm registers are 128bit wide but we just say that they exactly fit the float
            reg: TargetReg::X86(reg),
        }
    }
}

impl CallConv {
    /// Returns if the X86 register is callee saved
    #[inline]
    pub fn x86_is_callee_saved(&self, reg: reg::X86RegVariant) -> bool {
        use reg::X86RegVariant::*;

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