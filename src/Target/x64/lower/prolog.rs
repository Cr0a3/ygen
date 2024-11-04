use crate::CodeGen::{MachineInstr, MachineOperand};
use crate::Target::x64::X64Reg;
use crate::Target::x64::asm::instr::*;

pub(crate) fn x64_lower_prolog(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr) {
    //sink.push( X64MCInstr::with0(Mnemonic::Endbr64) );

    for op in instr.operands.iter() { // we remove the stack_off
        let MachineOperand::Reg(crate::CodeGen::Reg::x64(callee_save)) = op else { continue; }; 
        
        if callee_save.is_xmm() {
            sink.extend_from_slice(&[
                X64MCInstr::with2(Mnemonic::Movq, Operand::Mem(MemOp { base: Some(X64Reg::Rsp), index: None, scale: 1, displ: 0, rip: false }), Operand::Reg(*callee_save)),
                X64MCInstr::with2(Mnemonic::Sub, Operand::Reg(X64Reg::Rsp), Operand::Imm(8)),
            ]);
        } else {
            sink.push(X64MCInstr::with1(Mnemonic::Push, Operand::Reg(*callee_save)));
        }
    }

    let Some(MachineOperand::Imm(stack_off)) = instr.operands.get(0) else { panic!("expected valid stack_off for prolog")};
    let stack_off = *stack_off as i64;

    if stack_off > 0 {
        sink.push( X64MCInstr::with1(Mnemonic::Push, Operand::Reg(X64Reg::Rbp) ) );
        sink.push( X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(X64Reg::Rbp), Operand::Reg(X64Reg::Rsp)  ) );
        sink.push( X64MCInstr::with2(Mnemonic::Sub, Operand::Reg(X64Reg::Rbp),  Operand::Imm(stack_off)) );
    }

}

pub(crate) fn x64_lower_epilog(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr) {
    for op in instr.operands.iter().rev() { // we remove the stack_off
        let MachineOperand::Reg(crate::CodeGen::Reg::x64(callee_save)) = op else { continue; }; 
        
        if callee_save.is_xmm() {
            sink.extend_from_slice(&[
                X64MCInstr::with2(Mnemonic::Movq, Operand::Reg(*callee_save), Operand::Mem(MemOp { base: Some(X64Reg::Rsp), index: None, scale: 1, displ: 0, rip: false })),
                X64MCInstr::with2(Mnemonic::Add, Operand::Reg(X64Reg::Rsp), Operand::Imm(8)),
            ]);
        } else {
            sink.push(X64MCInstr::with1(Mnemonic::Pop, Operand::Reg(*callee_save)));
        }
    }

    sink.push( X64MCInstr::with1(Mnemonic::Pop, Operand::Reg(X64Reg::Rbp) ) );
}
