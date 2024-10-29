use std::collections::HashMap;
use crate::CodeGen::{calling_convention::MachineCallingConvention, compilation::CompilationHelper, Allocator, ConstImmRules, Reg};
use crate::Target::{Arch, CallConv};

use super::X64Reg;
use super::reg_alloc;

pub(crate) fn construct_compilation_helper(call_conv: CallConv) -> CompilationHelper {

    let calling_convention = MachineCallingConvention { 
        call_conv: call_conv
    };

    let mut alloc = Allocator {
        alloc: Some(reg_alloc::x64_alloc),
        alloc_rv: Some(reg_alloc::x64_alloc_rv),
        alloc_stack: Some(reg_alloc::x64_alloc_stack),
        free: Some(reg_alloc::x64_free),
        after_alloc: Some(x64_after_alloc),
        vars: HashMap::new(),
        var_types: HashMap::new(),
        allocated_vars: Vec::new(),
        epilog: false,
        scopes: HashMap::new(),
        phi_vars: HashMap::new(),
        stack_off: 8,
        ffpregs: vec![
            Reg::x64(X64Reg::Xmm0), 
            Reg::x64(X64Reg::Xmm1), 
            Reg::x64(X64Reg::Xmm2), 
            Reg::x64(X64Reg::Xmm3), 
            Reg::x64(X64Reg::Xmm4), 
            Reg::x64(X64Reg::Xmm5), 
            Reg::x64(X64Reg::Xmm6), 
            Reg::x64(X64Reg::Xmm7), 
            Reg::x64(X64Reg::Xmm8), 
            Reg::x64(X64Reg::Xmm9), 
            Reg::x64(X64Reg::Xmm10), 
            Reg::x64(X64Reg::Xmm11), 
            Reg::x64(X64Reg::Xmm12), 
            Reg::x64(X64Reg::Xmm13)],

        fregs: vec![
            Reg::x64(X64Reg::Rcx),
            Reg::x64(X64Reg::Rdx),
            Reg::x64(X64Reg::Rsi),
            Reg::x64(X64Reg::Rdi),
            Reg::x64(X64Reg::R8),
            Reg::x64(X64Reg::R9),
            Reg::x64(X64Reg::R10),
            Reg::x64(X64Reg::R11),
            Reg::x64(X64Reg::R12),
            Reg::x64(X64Reg::R12),
            Reg::x64(X64Reg::R13),
            Reg::x64(X64Reg::R14),
            Reg::x64(X64Reg::R15),
        ],

        call: calling_convention,
    };

    alloc.fregs.reverse();
    alloc.ffpregs.reverse();

    let mut helper = CompilationHelper::new(
        Arch::X86_64, 
        calling_convention, 
        alloc, 
        Reg::x64(X64Reg::Rax)
    );

    helper.lower = Some(super::lower::x64_lower);

    helper.fp_imm = ConstImmRules::CreateConst;


    helper
}

fn x64_after_alloc(compiler: &CompilationHelper) {
    /*if compiler.alloc.stack_off - 8 < compiler.call.shadow(compiler.arch) {
        unsafe {
            super::lower::USE_SP_FOR_STACK = true;
            if compiler.call.call_conv == CallConv::WindowsFastCall {
                super::lower::SP_OFF = 32;
            }
        }
    }*/
}