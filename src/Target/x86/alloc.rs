use std::collections::HashMap;

use crate::ydbg;
use crate::CodeGen::dag::DagOpTarget;
use crate::CodeGen::memory::Memory;
use crate::CodeGen::reg::Reg;
use crate::CodeGen::{dag::DagTmpInfo, regalloc_iterated_col::ItRegCoalAlloc};
use crate::Target::x86::asm::X86Operand;
use crate::Target::CallConv;
use crate::IR::TypeMetadata;

use super::asm::X86Instr;
use super::reg::X86Reg;

/// Runs X86 argument allocation
pub fn arg_proc(alloc: &mut ItRegCoalAlloc) {
    ydbg!("[X86] Running argument register allocation");

    let Some(func) = alloc.curr_func else {
        panic!("no current function")
    };

    let call = super::get_call();

    let mut arg_num = 0;
    let mut arg_stack_off = 8;

    // windows does stuff a bit different
    // there the first argument is rsp + 32
    if call == CallConv::WindowsFastCall {
        arg_stack_off += 32;
    }

    for (name, ty) in &func.ty.args {
        if let Some(mut reg) = call.get_x86_arg(arg_num, *ty) {
            ydbg!("[REG] allocating a register for arg {name}");

            reg.size = ty.byteSize().into();

            alloc.vars.insert(name.to_owned(), DagOpTarget::Reg(Reg::new_x86(reg)));
            
            alloc.regs.retain(|listed_reg| *listed_reg != Reg::new_x86(reg) );
        } else {
            ydbg!("[IRC] allocating memory for arg {name}");
            // stack vars are a little harder
            // the stack is build like that:
            // rsp
            // rsp + 8 -> first arg
            // rsp + 8 + sizeof(first arg) -> second arg
            // e.g: the size of first arg is 8 bytes (so a long long)
            // rsp + 8 -> first arg
            // rsp + 16 -> second arg

            let memory = Memory {
                offset: arg_stack_off,
                sp_relativ: true, // would be rsp + off
                fp_relativ: false, // would be rbp + off
                size: ty.byteSize(),
            };

            arg_stack_off += ty.byteSize() as i32;

            alloc.vars.insert(name.to_owned(), DagOpTarget::Mem(memory));
        }

        arg_num += 1;
    }
}

pub(super) fn resolve(tmp_infos: Vec<DagTmpInfo>, asm: &mut Vec<X86Instr>, alloc: &mut ItRegCoalAlloc) {    
    let mut tmps = HashMap::new();

    for tmp in &tmp_infos {
        let loc = alloc.request_tmp(tmp);
        tmps.insert(tmp.tmp_num, loc);
    }

    for instr in asm.iter_mut() {
        if let Some(op1) = &mut instr.op1 {
            if let X86Operand::Tmp(t) = op1.to_owned() {
                let mut resolved = false;
                for (num,  loc) in &tmps {
                    if *num == t {
                        *op1 = loc.into();
                        resolved = true;
                        break;
                    }
                }
                if !resolved {
                    panic!("unable to resolve tmp {t}");
                }
            }
        }
        
        if let Some(op2) = &mut instr.op2 {
            if let X86Operand::Tmp(t) = op2.to_owned() {
                let mut resolved = false;
                for (num,  loc) in &tmps {
                    if *num == t {
                        *op2 = loc.into();
                        resolved = true;
                        break;
                    }
                }
                if !resolved {
                    panic!("unable to resolve tmp {t}");
                }
            }
        }

        if let Some(op3) = &mut instr.op3 {
            if let X86Operand::Tmp(t) = op3.to_owned() {
                let mut resolved = false;
                for (num,  loc) in &tmps {
                    if *num == t {
                        *op3 = loc.into();
                        resolved = true;
                        break;
                    }
                }
                if !resolved {
                    panic!("unable to resolve tmp {t}");
                }
            }
        }
    }

    for (_, tmp) in tmps {
        if let DagOpTarget::Reg(reg) = tmp {
            alloc.regs.push(reg);
        }
    }

    alloc.sort();
} 

impl CallConv {
    /// Returns the nth x86 argument
    #[inline]
    pub fn get_x86_arg(&self, num: usize, ty: TypeMetadata) -> Option<X86Reg> {
        if ty.float() { self.get_x86_arg_fp(num) }
        else if ty.isVectorTy() { self.get_x86_arg_fp(num) } // in ygen we do vectors using sse
        else { self.get_x86_arg_gr(num) }
    }

    /// Returns the nth x86 gr argument
    #[inline]
    pub fn get_x86_arg_gr(&self, num: usize) -> Option<X86Reg> {
        match self {
            CallConv::WindowsFastCall => match num {
                0 => Some(X86Reg::Rcx()),
                1 => Some(X86Reg::Rdx()),
                2 => Some(X86Reg::R8()),
                3 => Some(X86Reg::R9()),
                _ => None,
            },
            CallConv::SystemV => match num {
                0 => Some(X86Reg::Rdi()),
                1 => Some(X86Reg::Rsi()),
                2 => Some(X86Reg::Rcx()),
                3 => Some(X86Reg::Rdx()),
                4 => Some(X86Reg::R8()),
                5 => Some(X86Reg::R9()),
                _ => None,
            },
            _ => panic!("the calling convention {self:?} is not usable in the x86 backend")
        }
    }

    /// Returns the nth x86 fp argument
    #[inline]
    pub fn get_x86_arg_fp(&self, num: usize) -> Option<X86Reg> {match self {
        CallConv::WindowsFastCall | CallConv::SystemV => match num {
            0 => Some(X86Reg::Xmm0()),
            1 => Some(X86Reg::Xmm1()),
            2 => Some(X86Reg::Xmm2()),
            3 => Some(X86Reg::Xmm3()),
            4 => Some(X86Reg::Xmm4()),
            5 => Some(X86Reg::Xmm5()),
            6 => Some(X86Reg::Xmm6()),
            7 => Some(X86Reg::Xmm7()),
            _ => None,
        },
        _ => panic!("the calling convention {self:?} is not usable in the x86 backend")
    }
    }
}