use crate::prelude::CmpMode;
use crate::CodeGen::MachineInstr;
use crate::Target::x64::instr::*;
use crate::IR::TypeMetadata;

pub(crate) fn x64_lower_fcmp(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr, mode: &CmpMode) {
    let ls = instr.operands.get(0).expect("expected valid src operand at 1. place");
    let rs = instr.operands.get(1).expect("expected valid value to compare at 2. place");
    let out = instr.out.expect("expected output");
    let out: Operand = out.into();
    let ls: Operand = (*ls).into();
    let rs: Operand = (*rs).into();

    sink.push(X64MCInstr::with2(Mnemonic::Mov, out.to_owned(), Operand::Imm(0)));

    let mnemonic = if TypeMetadata::f32 == instr.meta {
        Mnemonic::Ucomiss 
    } else if TypeMetadata::f64 == instr.meta {
        Mnemonic::Ucomisd
    } else {
        panic!("fcmp expects fp args");
    };

    sink.push(X64MCInstr::with2(mnemonic, ls, rs));

    let mne = match mode {
        CmpMode::Equal => Mnemonic::Sete,
        CmpMode::NotEuqal => Mnemonic::Setne,
        CmpMode::GreaterThan => Mnemonic::Setg,
        CmpMode::LessThan => Mnemonic::Setl,
        CmpMode::GreaterThanOrEqual => Mnemonic::Setge,
        CmpMode::LessThanOrEqual => Mnemonic::Setle,
    };

    sink.push( X64MCInstr::with1(mne, out) );
}