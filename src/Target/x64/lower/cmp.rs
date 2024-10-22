use crate::prelude::CmpMode;
use crate::CodeGen::MachineInstr;
use crate::Target::x64::asm::instr::*;

use super::{RegAllocOperand, X64RegAllocInstr};

pub(crate) 
fn x64_lower_cmp(sink: &mut Vec<X64RegAllocInstr>, instr: &MachineInstr, mode: &CmpMode) {
    let ls = instr.operands.get(0).expect("expected valid src operand at 1. place");
    let rs = instr.operands.get(1).expect("expected valid value to compare at 2. place");

    let out = instr.out.expect("expected output");
    let out = out.into();

    let mut ls = (*ls).into();
    let mut rs = (*rs).into();

    sink.extend_from_slice(&vec![
        X64RegAllocInstr::with2(Mnemonic::Mov, RegAllocOperand::Tmp0, ls),
        X64RegAllocInstr::with2(Mnemonic::Mov, RegAllocOperand::Tmp1, rs),
        X64RegAllocInstr::with2(Mnemonic::Mov, RegAllocOperand::Tmp2, RegAllocOperand::Allocated(Operand::Imm(0))),
        X64RegAllocInstr::with2(Mnemonic::Mov, out, RegAllocOperand::Tmp2),
        X64RegAllocInstr::with2(Mnemonic::Cmp, RegAllocOperand::Tmp1, RegAllocOperand::Tmp1),
    ]);

    let mne = match mode {
        CmpMode::Eqal => Mnemonic::Sete,
        CmpMode::NotEqal => Mnemonic::Setne,
        CmpMode::GreaterThan => Mnemonic::Setg,
        CmpMode::LessThan => Mnemonic::Setl,
        CmpMode::GreaterThanOrEqual => Mnemonic::Setge,
        CmpMode::LessThanOrEqual => Mnemonic::Setle,
    };

    sink.push( X64RegAllocInstr::with1(mne, out) );
}
