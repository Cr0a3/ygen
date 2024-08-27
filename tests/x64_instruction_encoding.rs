use ygen::{Optimizations::auto_max_optimize, Target::{instr::*, x64Reg, Reg}};

#[test]
pub fn test_mov() {
    let instr = Instr::with2(
        Mnemonic::Mov, 
        Operand::Reg(x64Reg::Rcx.boxed()),
        Operand::Mem(MemOp { base: Some(x64Reg::R15.boxed()), index: None, scale: 1, displ: 5, rip: false })
    );

    assert_eq!(instr.encode(), Ok((vec![0x49, 0x8B, 0x4F, 0x05], None)));

    let instr = Instr::with2(
        Mnemonic::Mov, 
        Operand::Reg(x64Reg::R12b.boxed()),
        Operand::Imm(12)
    );

    assert_eq!(instr.encode(), Ok((vec![0x41, 0xC6, 0xC4, 0x0C], None)));
}

#[test]
pub fn test_ret() {
    let instr = Instr::with0(Mnemonic::Ret);

    assert_eq!(instr.encode(), Ok((vec![0xC3], None)));
}

#[test]
pub fn test_optimization() {
    let mut instrs = vec![
        Instr::with0(Mnemonic::StartOptimization),
        Instr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rax.boxed()), Operand::Reg(x64Reg::Rcx.boxed())),
        Instr::with2(Mnemonic::Add, Operand::Reg(x64Reg::Rax.boxed()), Operand::Reg(x64Reg::Rdx.boxed())),
        Instr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rcx.boxed()), Operand::Reg(x64Reg::Rax.boxed())),
    ];

    let expected_optimized = vec![
        Instr::with0(Mnemonic::StartOptimization),
        Instr::with2(Mnemonic::Lea, Operand::Reg(x64Reg::Rax.boxed()), Operand::Mem(x64Reg::Rcx + x64Reg::Rdx)),
        Instr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rcx.boxed()), Operand::Reg(x64Reg::Rax.boxed())),
    ];

    auto_max_optimize(&mut instrs);

    assert_eq!(instrs, expected_optimized);
}