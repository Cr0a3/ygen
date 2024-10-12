use crate::{CodeGen::MachineInstr, Target::{x64::instr::{Mnemonic, Operand, X64MCInstr}, x64Reg}};

pub(crate) fn x64_lower_cmov_zero(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr) {
    let cond = instr.operands.get(0).expect("expected condition for valid cmov");
    let cond = (*cond).into();

    let value = instr.operands.get(1).expect("expected value for valid cmov");
    let value = (*value).into();

    let out = instr.out.expect("expected output for valid cmov");
    let out = out.into();

    let cmp = if let Operand::Mem(_) = cond {
        vec![X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rbx), Operand::Imm(0)),
             X64MCInstr::with2(Mnemonic::Cmp, cond, Operand::Reg(x64Reg::Rbx))]
    } else {
        vec![X64MCInstr::with2(Mnemonic::Cmp, cond, Operand::Imm(0))]
    };

    if let Operand::Reg(_) = out {
        if let Operand::Imm(_) = value {
            sink.extend_from_slice(&cmp);
            sink.push( X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rax.sub_ty(instr.meta)), value));
            sink.push( X64MCInstr::with2(Mnemonic::Cmove, out, Operand::Reg(x64Reg::Rax.sub_ty(instr.meta))));
        } else {
            sink.extend_from_slice(&cmp);
            sink.push( X64MCInstr::with2(Mnemonic::Cmove, out, value));
        }
    } else {
        if let Operand::Imm(_) = value {
            sink.push( X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rax.sub_ty(instr.meta)), value));
            sink.extend_from_slice(&cmp);
            sink.push( X64MCInstr::with2(Mnemonic::Cmove, Operand::Reg(x64Reg::Rax.sub_ty(instr.meta)), Operand::Reg(x64Reg::Rax.sub_ty(instr.meta))));
        } else {
            sink.extend_from_slice(&cmp);
            sink.push( X64MCInstr::with2(Mnemonic::Cmove, Operand::Reg(x64Reg::Rax.sub_ty(instr.meta)), value));
        }
        sink.push( X64MCInstr::with2(Mnemonic::Mov, out, Operand::Reg(x64Reg::Rax.sub_ty(instr.meta))));
    }
}  

pub(crate) fn x64_lower_cmov_not_zero(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr) {
    let cond = instr.operands.get(0).expect("expected condition for valid cmov");
    let cond = (*cond).into();

    let value = instr.operands.get(1).expect("expected value for valid cmov");
    let value = (*value).into();

    let out = instr.out.expect("expected output for valid cmov");
    let out = out.into();

    let cmp = if let Operand::Mem(_) = cond {
        vec![X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rbx), Operand::Imm(0)),
             X64MCInstr::with2(Mnemonic::Cmp, cond, Operand::Reg(x64Reg::Rbx))]
    } else {
        vec![X64MCInstr::with2(Mnemonic::Cmp, cond, Operand::Imm(0))]
    };

    if let Operand::Reg(_) = out {
        if let Operand::Imm(_) = value {
            sink.extend_from_slice(&cmp);
            sink.push( X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rax.sub_ty(instr.meta)), value));
            sink.push( X64MCInstr::with2(Mnemonic::Cmovne, out, Operand::Reg(x64Reg::Rax.sub_ty(instr.meta))));
        } else {
            sink.extend_from_slice(&cmp);
            sink.push( X64MCInstr::with2(Mnemonic::Cmovne, out, value));
        }
    } else {
        if let Operand::Imm(_) = value {
            sink.push( X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rax.sub_ty(instr.meta)), value));
            sink.extend_from_slice(&cmp);
            sink.push( X64MCInstr::with2(Mnemonic::Cmovne, Operand::Reg(x64Reg::Rax.sub_ty(instr.meta)), Operand::Reg(x64Reg::Rax.sub_ty(instr.meta))));
        } else {
            sink.extend_from_slice(&cmp);
            sink.push( X64MCInstr::with2(Mnemonic::Cmovne, Operand::Reg(x64Reg::Rax.sub_ty(instr.meta)), value));
        }
        sink.push( X64MCInstr::with2(Mnemonic::Mov, out, Operand::Reg(x64Reg::Rax.sub_ty(instr.meta))));
    }
}  