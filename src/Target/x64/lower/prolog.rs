use crate::CodeGen::MachineInstr;
use crate::Target::x64::X64Reg;
use crate::Target::x64::asm::instr::*;

use super::{RegAllocOperand, X64RegAllocInstr};

pub(crate) fn x64_lower_prolog(sink: &mut Vec<X64RegAllocInstr>, instr: &MachineInstr) {
    //sink.push( X64RegAllocInstr::with0(Mnemonic::Endbr64) );
    sink.push( X64RegAllocInstr::with1(Mnemonic::Push, RegAllocOperand::Allocated(Operand::Reg(X64Reg::Rbp)) ) );
    sink.push( X64RegAllocInstr::with2(Mnemonic::Mov, RegAllocOperand::Allocated(Operand::Reg(X64Reg::Rbp)), RegAllocOperand::Allocated(Operand::Reg(X64Reg::Rsp))  ) );
    if let Some(op0) = instr.operands.get(0) {
        let op0 = (*op0).into();

        sink.push( X64RegAllocInstr::with2(Mnemonic::Sub, RegAllocOperand::Allocated(Operand::Reg(X64Reg::Rbp)),  op0) );
    }
}

pub(crate) fn x64_lower_epilog(sink: &mut Vec<X64RegAllocInstr>, _: &MachineInstr) {
    sink.push( X64RegAllocInstr::with1(Mnemonic::Pop, RegAllocOperand::Allocated(Operand::Reg(X64Reg::Rbp)) ) );
}
