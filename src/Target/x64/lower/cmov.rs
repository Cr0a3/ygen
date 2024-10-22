use crate::CodeGen::MachineInstr; 
use crate::Target::x64::instr::{Mnemonic, Operand};
use crate::IR::TypeMetadata;

use super::{RegAllocOperand, X64RegAllocInstr};

pub(crate) fn x64_lower_cmov_zero(sink: &mut Vec<X64RegAllocInstr>, instr: &MachineInstr) {
    if instr.meta.float() {
        return x64_lower_fcmov0(sink, instr);
    }

    let cond = instr.operands.get(0).expect("expected condition for valid cmov");
    let cond = (*cond).into();

    let value = instr.operands.get(1).expect("expected value for valid cmov");
    let value = (*value).into();

    let out = instr.out.expect("expected output for valid cmov");
    let out = out.into();

    let cmp = vec![
        X64RegAllocInstr::with2(Mnemonic::Mov, RegAllocOperand::Tmp0, RegAllocOperand::Allocated(Operand::Imm(0))),
        X64RegAllocInstr::with2(Mnemonic::Cmp, cond, RegAllocOperand::Tmp0)
    ];

    sink.push( X64RegAllocInstr::with2(Mnemonic::Mov, RegAllocOperand::Tmp0, value));
    sink.extend_from_slice(&cmp);
    sink.push( X64RegAllocInstr::with2(Mnemonic::Cmove, RegAllocOperand::Tmp1, RegAllocOperand::Tmp0) );
    sink.push( X64RegAllocInstr::with2(Mnemonic::Mov, out, RegAllocOperand::Tmp1, ) );
}  

pub(crate) fn x64_lower_cmov_not_zero(sink: &mut Vec<X64RegAllocInstr>, instr: &MachineInstr) {
    if instr.meta.float() {
        return x64_lower_fcmovne0(sink, instr);
    }

    let cond = instr.operands.get(0).expect("expected condition for valid cmov");
    let cond = (*cond).into();

    let value = instr.operands.get(1).expect("expected value for valid cmov");
    let value = (*value).into();

    let out = instr.out.expect("expected output for valid cmov");
    let out = out.into();

    let cmp = vec![
        X64RegAllocInstr::with2(Mnemonic::Mov, RegAllocOperand::Tmp0, RegAllocOperand::Allocated(Operand::Imm(0))),
        X64RegAllocInstr::with2(Mnemonic::Cmp, cond, RegAllocOperand::Tmp0)
    ];

    sink.push( X64RegAllocInstr::with2(Mnemonic::Mov, RegAllocOperand::Tmp0, value));
    sink.extend_from_slice(&cmp);
    sink.push( X64RegAllocInstr::with2(Mnemonic::Cmovne, RegAllocOperand::Tmp1, RegAllocOperand::Tmp0) );
    sink.push( X64RegAllocInstr::with2(Mnemonic::Mov, out, RegAllocOperand::Tmp1, ) );
}  

pub(crate) fn x64_lower_fcmov0(sink: &mut Vec<X64RegAllocInstr>, instr: &MachineInstr) {
    let cond = instr.operands.get(0).expect("expected condition for valid cmov");
    let cond = (*cond).into();

    let value = instr.operands.get(1).expect("expected value for valid cmov");
    let value = (*value).into();

    let out = instr.out.expect("expected output for valid cmov");
    let out: RegAllocOperand = out.into();

    let mnemonic = if instr.meta == TypeMetadata::f32 {
        Mnemonic::Movd
    } else {
        Mnemonic::Movq
    };

    sink.extend_from_slice(&vec![
        X64RegAllocInstr::with2(Mnemonic::Mov, RegAllocOperand::Tmp0, RegAllocOperand::Allocated(Operand::Imm(0))),
        X64RegAllocInstr::with2(Mnemonic::Cmp, cond, RegAllocOperand::Tmp0),
        X64RegAllocInstr::with2(Mnemonic::Mov, RegAllocOperand::Tmp0, out.to_owned()),
        X64RegAllocInstr::with2(mnemonic,  RegAllocOperand::Tmp1, value),
        X64RegAllocInstr::with2(Mnemonic::Cmove, RegAllocOperand::Tmp0,  RegAllocOperand::Tmp1),
        X64RegAllocInstr::with2(mnemonic, out, RegAllocOperand::Tmp0)
    ]);
} 

pub(crate) fn x64_lower_fcmovne0(sink: &mut Vec<X64RegAllocInstr>, instr: &MachineInstr) {
    let cond = instr.operands.get(0).expect("expected condition for valid cmov");
    let cond = (*cond).into();

    let value = instr.operands.get(1).expect("expected value for valid cmov");
    let value = (*value).into();

    let out = instr.out.expect("expected output for valid cmov");
    let out: RegAllocOperand = out.into();

    let mnemonic =if instr.meta == TypeMetadata::f32 {
        Mnemonic::Movd
    } else {
        Mnemonic::Movq
    };

    sink.extend_from_slice(&vec![
        X64RegAllocInstr::with2(Mnemonic::Mov, RegAllocOperand::Tmp0, RegAllocOperand::Allocated(Operand::Imm(0))),
        X64RegAllocInstr::with2(Mnemonic::Cmp, cond, RegAllocOperand::Tmp0),
        X64RegAllocInstr::with2(Mnemonic::Mov, RegAllocOperand::Tmp0, out.to_owned()),
        X64RegAllocInstr::with2(mnemonic,  RegAllocOperand::Tmp1, value),
        X64RegAllocInstr::with2(Mnemonic::Cmovne, RegAllocOperand::Tmp0,  RegAllocOperand::Tmp1),
        X64RegAllocInstr::with2(mnemonic, out, RegAllocOperand::Tmp0)
    ]);
} 
