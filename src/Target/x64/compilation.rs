use crate::{CodeGen::{calling_convention::MachineCallingConvention, compilation::CompilationHelper, reg_alloc::RegAlloc, ConstImmRules, Reg}, Target::{Arch, CallConv}};

use super::x64Reg;

pub(crate) fn construct_compilation_helper(call_conv: CallConv) -> CompilationHelper {

    let calling_convention = MachineCallingConvention { 
        call_conv: call_conv
    };

    let mut alloc = RegAlloc::new(Arch::X86_64, call_conv);

    alloc.free_registers.push(Arch::X86_64, Reg::x64(x64Reg::Rcx));
    alloc.free_registers.push(Arch::X86_64, Reg::x64(x64Reg::Rdx));
    alloc.free_registers.push(Arch::X86_64, Reg::x64(x64Reg::Rsi));
    alloc.free_registers.push(Arch::X86_64, Reg::x64(x64Reg::Rdi));
    alloc.free_registers.push(Arch::X86_64, Reg::x64(x64Reg::R8));
    alloc.free_registers.push(Arch::X86_64, Reg::x64(x64Reg::R9));
    alloc.free_registers.push(Arch::X86_64, Reg::x64(x64Reg::R10));
    alloc.free_registers.push(Arch::X86_64, Reg::x64(x64Reg::R11));
    alloc.free_registers.push(Arch::X86_64, Reg::x64(x64Reg::R12));
    alloc.free_registers.push(Arch::X86_64, Reg::x64(x64Reg::R13));
    alloc.free_registers.push(Arch::X86_64, Reg::x64(x64Reg::R14));
    alloc.free_registers.push(Arch::X86_64, Reg::x64(x64Reg::R15));

    alloc.free_fpregs.push(Arch::X86_64, Reg::x64(x64Reg::Xmm0));
    alloc.free_fpregs.push(Arch::X86_64, Reg::x64(x64Reg::Xmm1));
    alloc.free_fpregs.push(Arch::X86_64, Reg::x64(x64Reg::Xmm2));
    alloc.free_fpregs.push(Arch::X86_64, Reg::x64(x64Reg::Xmm3));
    alloc.free_fpregs.push(Arch::X86_64, Reg::x64(x64Reg::Xmm4));
    alloc.free_fpregs.push(Arch::X86_64, Reg::x64(x64Reg::Xmm5));
    alloc.free_fpregs.push(Arch::X86_64, Reg::x64(x64Reg::Xmm6));
    alloc.free_fpregs.push(Arch::X86_64, Reg::x64(x64Reg::Xmm7));
    alloc.free_fpregs.push(Arch::X86_64, Reg::x64(x64Reg::Xmm8));
    alloc.free_fpregs.push(Arch::X86_64, Reg::x64(x64Reg::Xmm9));
    alloc.free_fpregs.push(Arch::X86_64, Reg::x64(x64Reg::Xmm10));
    alloc.free_fpregs.push(Arch::X86_64, Reg::x64(x64Reg::Xmm11));
    alloc.free_fpregs.push(Arch::X86_64, Reg::x64(x64Reg::Xmm12));
    alloc.free_fpregs.push(Arch::X86_64, Reg::x64(x64Reg::Xmm13));
    alloc.free_fpregs.push(Arch::X86_64, Reg::x64(x64Reg::Xmm14));
    alloc.free_fpregs.reverse(Arch::X86_64);
    
    // Used as temporary storage
    // alloc.free_fpregs.push(Arch::X86_64, Reg::x64(x64Reg::Xmm15));

    alloc.free_registers.reverse(Arch::X86_64);

    let mut helper = CompilationHelper::new(
        Arch::X86_64, 
        calling_convention, 
        alloc, 
        Reg::x64(x64Reg::Rax)
    );

    helper.lower = Some(super::lower::x64_lower);

    helper.fp_imm = ConstImmRules::CreateConst;

    helper
}