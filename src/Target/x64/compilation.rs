use crate::{CodeGen::{calling_convention::MachineCallingConvention, compilation::CompilationHelper, reg_alloc::RegAlloc, ConstImmRules, Reg}, Target::{Arch, CallConv}};

use super::X64Reg;

pub(crate) fn construct_compilation_helper(call_conv: CallConv) -> CompilationHelper {

    let calling_convention = MachineCallingConvention { 
        call_conv: call_conv
    };

    let mut alloc = RegAlloc::new(Arch::X86_64, call_conv, false);

    alloc.free_registers.push(Arch::X86_64, Reg::x64(X64Reg::Rcx));
    alloc.free_registers.push(Arch::X86_64, Reg::x64(X64Reg::Rdx));
    alloc.free_registers.push(Arch::X86_64, Reg::x64(X64Reg::Rsi));
    alloc.free_registers.push(Arch::X86_64, Reg::x64(X64Reg::Rdi));
    alloc.free_registers.push(Arch::X86_64, Reg::x64(X64Reg::R8));
    alloc.free_registers.push(Arch::X86_64, Reg::x64(X64Reg::R9));
    alloc.free_registers.push(Arch::X86_64, Reg::x64(X64Reg::R10));
    alloc.free_registers.push(Arch::X86_64, Reg::x64(X64Reg::R11));
    alloc.free_registers.push(Arch::X86_64, Reg::x64(X64Reg::R12));
    alloc.free_registers.push(Arch::X86_64, Reg::x64(X64Reg::R13));
    alloc.free_registers.push(Arch::X86_64, Reg::x64(X64Reg::R14));
    alloc.free_registers.push(Arch::X86_64, Reg::x64(X64Reg::R15));

    alloc.free_fpregs.push(Arch::X86_64, Reg::x64(X64Reg::Xmm0));
    alloc.free_fpregs.push(Arch::X86_64, Reg::x64(X64Reg::Xmm1));
    alloc.free_fpregs.push(Arch::X86_64, Reg::x64(X64Reg::Xmm2));
    alloc.free_fpregs.push(Arch::X86_64, Reg::x64(X64Reg::Xmm3));
    alloc.free_fpregs.push(Arch::X86_64, Reg::x64(X64Reg::Xmm4));
    alloc.free_fpregs.push(Arch::X86_64, Reg::x64(X64Reg::Xmm5));
    alloc.free_fpregs.push(Arch::X86_64, Reg::x64(X64Reg::Xmm6));
    alloc.free_fpregs.push(Arch::X86_64, Reg::x64(X64Reg::Xmm7));
    alloc.free_fpregs.push(Arch::X86_64, Reg::x64(X64Reg::Xmm8));
    alloc.free_fpregs.push(Arch::X86_64, Reg::x64(X64Reg::Xmm9));
    alloc.free_fpregs.push(Arch::X86_64, Reg::x64(X64Reg::Xmm10));
    alloc.free_fpregs.push(Arch::X86_64, Reg::x64(X64Reg::Xmm11));
    alloc.free_fpregs.push(Arch::X86_64, Reg::x64(X64Reg::Xmm12));
    alloc.free_fpregs.push(Arch::X86_64, Reg::x64(X64Reg::Xmm13));
    // Used as temporary storage
    // alloc.free_fpregs.push(Arch::X86_64, Reg::x64(x64Reg::Xmm14));
    // alloc.free_fpregs.push(Arch::X86_64, Reg::x64(x64Reg::Xmm15));
    alloc.free_fpregs.reverse(Arch::X86_64);

    alloc.free_registers.reverse(Arch::X86_64);

    let mut helper = CompilationHelper::new(
        Arch::X86_64, 
        calling_convention, 
        alloc, 
        Reg::x64(X64Reg::Rax)
    );

    helper.lower = Some(super::lower::x64_lower);

    helper.fp_imm = ConstImmRules::CreateConst;

    helper.after_alloc = Some(x64_after_alloc);

    helper
}

fn x64_after_alloc(compiler: &CompilationHelper) {
    if compiler.alloc.stack_off < compiler.call.shadow(compiler.arch) {
        unsafe {
            super::lower::USE_SP_FOR_STACK = true;
        }
    }
}