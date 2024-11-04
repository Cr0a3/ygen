use crate::CodeGen::MachineInstr;
use crate::Target::x64::X64Reg;
use crate::Target::x64::asm::instr::*;
use crate::IR::{BlockId, Type, TypeMetadata};

pub(crate) fn x64_lower_switch(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr, cases: &Vec<(Type, BlockId)>) {
    if instr.meta.float() {
        return x64_lower_fswitch(sink, instr, cases);
    }
    
    let var = *instr.operands.get(0).expect("switch expectes an variable to switch");
    let mut var = var.into();

    if let Operand::Mem(_) = var {
        sink.push(
            X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(X64Reg::Rax.sub_ty(instr.meta)), var)
        );

        var = Operand::Reg(X64Reg::Rax.sub_ty(instr.meta));
    }

    for (case_type, block) in cases {
        sink.extend_from_slice(&[
            X64MCInstr::with2(Mnemonic::Cmp, var.clone(), Operand::Imm(case_type.val() as i64)),
            X64MCInstr::with1(Mnemonic::Je, Operand::BlockLinkDestination(block.name.to_owned(), -4))
        ]);
    }
}

pub(crate) fn x64_lower_fswitch(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr, cases: &Vec<(Type, BlockId)>) {
    let var = *instr.operands.get(0).expect("switch expectes an variable to switch");
    let var = var.into();

    let move_mne = if instr.meta == TypeMetadata::f32 { Mnemonic::Movd } else { Mnemonic::Movq };
    let size = if instr.meta == TypeMetadata::f32 { TypeMetadata::i32 } else { TypeMetadata::i64 };

    if let Operand::Mem(_) = &var {
        sink.push(
            X64MCInstr::with2(move_mne, Operand::Reg(X64Reg::Xmm15), var.into())
        );
    } else {
        let mne = if instr.meta == TypeMetadata::f32 { Mnemonic::Movss } else { Mnemonic::Movsd };
        
        sink.push(
            X64MCInstr::with2(mne, Operand::Reg(X64Reg::Xmm15), var.into())
        );
    }

    for (case_type, block) in cases {
        let imm = if instr.meta == TypeMetadata::f32 {
            (case_type.val() as f32).to_bits() as i64
        } else {
            case_type.val().to_bits() as i64
        };

        let cmp_mne = if instr.meta == TypeMetadata::f32 { Mnemonic::Ucomiss } else { Mnemonic::Ucomisd };

        sink.extend_from_slice(&[
            X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(X64Reg::R11.sub_ty(size)), Operand::Imm(imm)),
            X64MCInstr::with2(move_mne, Operand::Reg(X64Reg::Xmm14), Operand::Reg(X64Reg::R11.sub_ty(size))),
            X64MCInstr::with2(cmp_mne, Operand::Reg(X64Reg::Xmm15), Operand::Reg(X64Reg::Xmm14)),
            X64MCInstr::with1(Mnemonic::Je, Operand::BlockLinkDestination(block.name.to_owned(), -4)),
        ]);
    }
}