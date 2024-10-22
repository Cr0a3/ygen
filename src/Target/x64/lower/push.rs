use crate::{CodeGen::MachineInstr, Target::x64::X64Reg};
use crate::Target::x64::asm::instr::*;

use super::{RegAllocOperand, X64RegAllocInstr};

pub(crate) fn x64_lower_push(sink: &mut Vec<X64RegAllocInstr>, instr: &MachineInstr) {
    let input = instr.operands.get(0).expect("push needs an operand");

    sink.push(
        X64RegAllocInstr::with1(Mnemonic::Push, (*input).into())
    );

    sink.push(X64RegAllocInstr::with2(Mnemonic::Sub, RegAllocOperand::Allocated(Operand::Reg(X64Reg::Rsp)), RegAllocOperand::Allocated(Operand::Imm(8)))); // for 16 byte alignment
}

pub(crate) fn x64_lower_push_cleanup(sink: &mut Vec<X64RegAllocInstr>, _: &MachineInstr) {
    sink.push( X64RegAllocInstr::with1(Mnemonic::Pop, RegAllocOperand::Tmp0));
    sink.push(X64RegAllocInstr::with2(Mnemonic::Add, RegAllocOperand::Allocated(Operand::Reg(X64Reg::Rsp)), RegAllocOperand::Allocated(Operand::Imm(8)))); // for 16 byte alignment
}
