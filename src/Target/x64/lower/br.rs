use crate::CodeGen::MachineInstr;
use crate::Target::x64::asm::instr::*;

use super::{RegAllocOperand, X64RegAllocInstr};


pub(crate) fn x64_lower_br(sink: &mut Vec<X64RegAllocInstr>, _: &MachineInstr, symbol: &String) {
    sink.push(
        X64RegAllocInstr::with1(Mnemonic::Jmp, RegAllocOperand::Allocated(Operand::Imm(0)))
    );


    sink.push(
        X64RegAllocInstr::with1(Mnemonic::Link, RegAllocOperand::Allocated(Operand::BlockLinkDestination(symbol.to_owned(), -4)))
    );
}

pub(crate) fn x64_lower_cond_br(sink: &mut Vec<X64RegAllocInstr>, instr: &MachineInstr, iftrue: &String, iffalse: &String) {
    let src = instr.operands.get(0).expect("expected valid src operand at 1. place");
    let value = instr.operands.get(1).expect("expected valid value to compare at 1. place");

    let src = (*src).into();
    let value = (*value).into();

    sink.push(X64RegAllocInstr::with2(Mnemonic::Mov, RegAllocOperand::Tmp0, src));
    sink.push(X64RegAllocInstr::with2(Mnemonic::Cmp, RegAllocOperand::Tmp0, value));
        
    sink.push(X64RegAllocInstr::with1(Mnemonic::Jne, RegAllocOperand::Allocated(Operand::Imm(0))));
    sink.push(X64RegAllocInstr::with1(Mnemonic::Link, RegAllocOperand::Allocated(Operand::BlockLinkDestination(iftrue.to_owned(), -4)))); // not 0
    sink.push(X64RegAllocInstr::with1(Mnemonic::Jmp, RegAllocOperand::Allocated(Operand::Imm(0))));
    sink.push(X64RegAllocInstr::with1(Mnemonic::Link, RegAllocOperand::Allocated(Operand::BlockLinkDestination(iffalse.to_owned(), -4)))); // is 0
}