use crate::prelude::CmpMode;
use crate::CodeGen::MachineInstr;
use crate::Target::x64::x64Reg;
use crate::Target::x64::instr::*;

pub(crate) fn x64_lower_fcmp(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr, mode: &CmpMode) {
    let ls = instr.operands.get(0).expect("expected valid src operand at 1. place");
    let rs = instr.operands.get(1).expect("expected valid value to compare at 2. place");
    let out = instr.out.expect("expected output");
    let out = out.into();
    let mut ls: Operand = (*ls).into();
    let mut rs: Operand = (*rs).into();

    todo!();

    let mne = match mode {
        CmpMode::Eqal => Mnemonic::Sete,
        CmpMode::NotEqal => Mnemonic::Setne,
        CmpMode::GreaterThan => Mnemonic::Setg,
        CmpMode::LessThan => Mnemonic::Setl,
        CmpMode::GreaterThanOrEqual => Mnemonic::Setge,
        CmpMode::LessThanOrEqual => Mnemonic::Setle,
    };

    sink.push( X64MCInstr::with1(mne, out) );
}