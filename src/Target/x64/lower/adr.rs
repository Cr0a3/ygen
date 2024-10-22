use crate::CodeGen::MachineInstr;
use crate::Target::x64::asm::instr::*;

use super::{RegAllocOperand, X64RegAllocInstr};

pub(crate) fn x64_lower_adr_load(sink: &mut Vec<X64RegAllocInstr>, instr: &MachineInstr, symbol: &String) {
    let out = instr.out.expect("expected a output operand");

    sink.push(
        X64RegAllocInstr::with2(Mnemonic::Lea, RegAllocOperand::Tmp0, RegAllocOperand::Allocated(Operand::Mem(MemOp { base: None, index: None, scale: 1, displ: 7, rip: true })))
    );
    sink.push(
        X64RegAllocInstr::with1(Mnemonic::Link, RegAllocOperand::Allocated(Operand::LinkDestination(symbol.to_string(), -4)))
    );
    sink.push(
        X64RegAllocInstr::with2(Mnemonic::Mov, out.into(), RegAllocOperand::Tmp0)
    );
}

pub(crate) fn x64_lower_adrm(sink: &mut Vec<X64RegAllocInstr>, instr: &MachineInstr) {
    let op = instr.operands.get(0).expect("expected adrm expectes one operand");
    let out = instr.out.expect("expected adrm expectes one operand");

    let op = (*op).into();

    let out = out.into();

    sink.push(X64RegAllocInstr::with2(Mnemonic::Lea, RegAllocOperand::Tmp0, op));
    sink.push(X64RegAllocInstr::with2(Mnemonic::Mov, out, RegAllocOperand::Tmp0));
}
