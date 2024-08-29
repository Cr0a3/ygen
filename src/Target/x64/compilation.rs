use crate::{CodeGen::{calling_convention::MachineCallingConvention, compilation::CompilationHelper, Reg}, Target::{Arch, CallConv}};

use super::x64Reg;

pub(crate) fn construct_compilation_helper(call_conv: CallConv) -> CompilationHelper {

    let calling_convention = MachineCallingConvention { 
        call_conv: call_conv
    };

    let mut helper = CompilationHelper::new(Arch::X86_64, calling_convention);

    helper.regs.push(Arch::X86_64, Reg::x64(x64Reg::Rcx));
    helper.regs.push(Arch::X86_64, Reg::x64(x64Reg::Rdx));
    helper.regs.push(Arch::X86_64, Reg::x64(x64Reg::Rsi));
    helper.regs.push(Arch::X86_64, Reg::x64(x64Reg::Rdi));
    helper.regs.push(Arch::X86_64, Reg::x64(x64Reg::R8));
    helper.regs.push(Arch::X86_64, Reg::x64(x64Reg::R9));
    helper.regs.push(Arch::X86_64, Reg::x64(x64Reg::R10));
    helper.regs.push(Arch::X86_64, Reg::x64(x64Reg::R11));
    helper.regs.push(Arch::X86_64, Reg::x64(x64Reg::R12));
    helper.regs.push(Arch::X86_64, Reg::x64(x64Reg::R13));
    helper.regs.push(Arch::X86_64, Reg::x64(x64Reg::R14));
    helper.regs.push(Arch::X86_64, Reg::x64(x64Reg::R15));

    helper.regs.reverse(Arch::X86_64);

    helper
}