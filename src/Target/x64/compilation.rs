use crate::CodeGen::{calling_convention::MachineCallingConvention, compilation::CompilationHelper, reg_alloc::RegAlloc, ConstImmRules, MCInstr, MachineInstr, Reg};
use crate::Target::{Arch, CallConv};

use super::{lower::x64_lower, X64Reg};

pub(crate) fn construct_compilation_helper(call_conv: CallConv) -> CompilationHelper {

    let calling_convention = MachineCallingConvention { 
        call_conv: call_conv
    };

    let alloc = RegAlloc::new(Arch::X86_64, call_conv);

    let mut helper = CompilationHelper::new(
        Arch::X86_64, 
        calling_convention, 
        alloc, 
        Reg::x64(X64Reg::Rax)
    );

    helper.lower = Some(X64Compile);

    helper.fp_imm = ConstImmRules::CreateConst;

    helper
}

pub(crate) fn X64Compile(compilat: &Vec<MachineInstr>, call: CallConv) -> Vec<Box<dyn MCInstr>> {
    let alloc_instrs = x64_lower(call, compilat.to_owned());

    let mut allocator = super::reg_alloc::X64Allocator::new();

    allocator.instrs = alloc_instrs;

    allocator.allocate();
    allocator.allocate_tmps();

    allocator.bake();

    allocator.out
}