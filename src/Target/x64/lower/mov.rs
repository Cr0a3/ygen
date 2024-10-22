use crate::CodeGen::MachineInstr;
use crate::Target::x64::asm::instr::*;

use super::{RegAllocOperand, X64RegAllocInstr};

pub(crate) fn x64_lower_move(sink: &mut Vec<X64RegAllocInstr>, instr: &MachineInstr) {
    let op1 = instr.operands.get(0).expect("expected a first operand");
    let out = instr.out.expect("expected a output operand");

    let op1 = (*op1).into();
    
    let out = out.into();

    sink.push( X64RegAllocInstr::with2(Mnemonic::Mov, RegAllocOperand::Tmp0, op1) );
    sink.push( X64RegAllocInstr::with2(Mnemonic::Mov, out, RegAllocOperand::Tmp0) );
}