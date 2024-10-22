use crate::CodeGen::MachineInstr;
use crate::Target::x64::asm::instr::*;
use crate::IR::{BlockId, Type, TypeMetadata};

use super::{RegAllocOperand, X64RegAllocInstr};

pub(crate) fn x64_lower_switch(sink: &mut Vec<X64RegAllocInstr>, instr: &MachineInstr, cases: &Vec<(Type, BlockId)>) {
    if instr.meta.float() {
        return x64_lower_fswitch(sink, instr, cases);
    }
    
    let var = *instr.operands.get(0).expect("switch expectes an variable to switch");
    let mut var = var.into();

    sink.push(
        X64RegAllocInstr::with2(Mnemonic::Mov, RegAllocOperand::Tmp0, var)
    );

    for (case_type, block) in cases {
        sink.extend_from_slice(&vec![
            X64RegAllocInstr::with2(Mnemonic::Cmp, RegAllocOperand::Tmp0, RegAllocOperand::Allocated(Operand::Imm(case_type.val() as i64))),
            X64RegAllocInstr::with1(Mnemonic::Je, RegAllocOperand::Allocated(Operand::Imm(0))),
            X64RegAllocInstr::with1(Mnemonic::Link, RegAllocOperand::Allocated(Operand::BlockLinkDestination(block.name.to_owned(), -4)))
        ]);
    }
}

pub(crate) fn x64_lower_fswitch(sink: &mut Vec<X64RegAllocInstr>, instr: &MachineInstr, cases: &Vec<(Type, BlockId)>) {
    let var = *instr.operands.get(0).expect("switch expectes an variable to switch");
    let var = var.into();

    let mne = if instr.meta == TypeMetadata::f32 { Mnemonic::Movss } else { Mnemonic::Movsd };
    let size = if instr.meta == TypeMetadata::f32 { TypeMetadata::i32 } else { TypeMetadata::i64 };

    sink.push(
        X64RegAllocInstr::with2(mne, RegAllocOperand::Tmp0, var)
    );


    for (case_type, block) in cases {
        let imm = if instr.meta == TypeMetadata::f32 {
            (case_type.val() as f32).to_bits() as i64
        } else {
            case_type.val().to_bits() as i64
        };

        let cmp_mne = if instr.meta == TypeMetadata::f32 { Mnemonic::Ucomiss } else { Mnemonic::Ucomisd };

        sink.extend_from_slice(&[
            X64RegAllocInstr::with2(Mnemonic::Mov, RegAllocOperand::Tmp1, RegAllocOperand::Allocated(Operand::Imm(imm))),
            X64RegAllocInstr::with2(if instr.meta == TypeMetadata::f32 { Mnemonic::Movd } else { Mnemonic::Movq }, RegAllocOperand::Tmp2, RegAllocOperand::Tmp1),
            X64RegAllocInstr::with2(cmp_mne, RegAllocOperand::Tmp0, RegAllocOperand::Tmp1),
            X64RegAllocInstr::with1(Mnemonic::Je, RegAllocOperand::Allocated(Operand::Imm(0))),
            X64RegAllocInstr::with1(Mnemonic::Link, RegAllocOperand::Allocated(Operand::BlockLinkDestination(block.name.to_owned(), -4))),
        ]);
    }
}