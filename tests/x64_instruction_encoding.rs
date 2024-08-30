use ygen::{Optimizations::auto_max_optimize, Target::{x64::instr::*, x64Reg}};

#[test]
pub fn test_mov() {
    let instr = X64MCInstr::with2(
        Mnemonic::Mov, 
        Operand::Reg(x64Reg::Rcx),
        Operand::Mem(MemOp { base: Some(x64Reg::R15), index: None, scale: 1, displ: 5, rip: false })
    );

    assert_eq!(instr.encode(), Ok((vec![0x49, 0x8B, 0x4F, 0x05], None)));

    let instr = X64MCInstr::with2(
        Mnemonic::Mov, 
        Operand::Reg(x64Reg::R12b),
        Operand::Imm(12)
    );

    assert_eq!(instr.encode(), Ok((vec![0x41, 0xC6, 0xC4, 0x0C], None)));
}

#[test]
pub fn test_ret() {
    let instr = X64MCInstr::with0(Mnemonic::Ret);

    assert_eq!(instr.encode(), Ok((vec![0xC3], None)));
}

#[test]
pub fn test_optimization() {
    let mut instrs = vec![
        X64MCInstr::with0(Mnemonic::StartOptimization),
        X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rax), Operand::Reg(x64Reg::Rcx)),
        X64MCInstr::with2(Mnemonic::Add, Operand::Reg(x64Reg::Rax), Operand::Reg(x64Reg::Rdx)),
        X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rcx), Operand::Reg(x64Reg::Rax)),
    ];

    let expected_optimized = vec![
        X64MCInstr::with0(Mnemonic::StartOptimization),
        X64MCInstr::with2(Mnemonic::Lea, Operand::Reg(x64Reg::Rax), Operand::Mem(x64Reg::Rcx + x64Reg::Rdx)),
        X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rcx), Operand::Reg(x64Reg::Rax)),
    ];

    auto_max_optimize(&mut instrs);

    assert_eq!(instrs, expected_optimized);
}