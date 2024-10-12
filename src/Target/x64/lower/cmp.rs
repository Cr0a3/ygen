use crate::prelude::CmpMode;
use crate::CodeGen::MachineInstr;
use crate::Target::x64Reg;
use crate::Target::x64::asm::instr::*;

pub(crate) 
fn x64_lower_cmp(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr, mode: &CmpMode) {
    let ls = instr.operands.get(0).expect("expected valid src operand at 1. place");
    let rs = instr.operands.get(1).expect("expected valid value to compare at 2. place");

    let out = instr.out.expect("expected output");
    let out = out.into();

    let mut ls = (*ls).into();

    let mut rs = (*rs).into();

    if let Operand::Imm(_) = ls {
        let tmp = ls;
        ls = rs;
        rs = tmp;
    }

    if let Operand::Mem(_) = ls {
        if ls == out {
            sink.push(X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rbx.sub_ty(instr.meta)), ls.clone()));
            sink.push(X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rax.sub_ty(instr.meta)), Operand::Imm(0)));
            sink.push(X64MCInstr::with2(Mnemonic::Mov, ls, Operand::Reg(x64Reg::Rax.sub_ty(instr.meta))));
            sink.push(X64MCInstr::with2(Mnemonic::Cmp, Operand::Reg(x64Reg::Rbx.sub_ty(instr.meta)), rs));
        } else if rs == out {
            sink.push(X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rbx.sub_ty(instr.meta)), rs.clone()));
            sink.push(X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rax.sub_ty(instr.meta)), Operand::Imm(0)));
            sink.push(X64MCInstr::with2(Mnemonic::Mov, rs, Operand::Reg(x64Reg::Rax.sub_ty(instr.meta))));
            sink.push(X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rax.sub_ty(instr.meta)), ls));
            sink.push(X64MCInstr::with2(Mnemonic::Cmp, Operand::Reg(x64Reg::Rax.sub_ty(instr.meta)), Operand::Reg(x64Reg::Rbx.sub_ty(instr.meta))));
        } else {
            if let Operand::Reg(_) = out {
                sink.push(X64MCInstr::with2(Mnemonic::Mov, out.clone(), Operand::Imm(0)));
            } else {
                sink.push(X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rax.sub_ty(instr.meta)), Operand::Imm(0)));
                sink.push(X64MCInstr::with2(Mnemonic::Mov, out.clone(), Operand::Reg(x64Reg::Rax.sub_ty(instr.meta))));
            }
            sink.push(X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rax.sub_ty(instr.meta)), ls));
            sink.push(X64MCInstr::with2(Mnemonic::Cmp, Operand::Reg(x64Reg::Rax.sub_ty(instr.meta)), rs));
        }
    } else {
        if ls == out {
            sink.push(X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rax.sub_ty(instr.meta)), ls.clone()));
            sink.push(X64MCInstr::with2(Mnemonic::Mov, ls, Operand::Imm(0)));
            sink.push(X64MCInstr::with2(Mnemonic::Cmp, Operand::Reg(x64Reg::Rax.sub_ty(instr.meta)), rs));
        } else if rs == out {
            sink.push(X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rax.sub_ty(instr.meta)), rs.clone()));
            sink.push(X64MCInstr::with2(Mnemonic::Mov, rs, Operand::Imm(0)));
            sink.push(X64MCInstr::with2(Mnemonic::Cmp, ls, Operand::Reg(x64Reg::Rax.sub_ty(instr.meta))));
        } else {
            if let Operand::Reg(_) = out {
                sink.push(X64MCInstr::with2(Mnemonic::Mov, out.clone(), Operand::Imm(0)));
            } else {
                sink.push(X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rax.sub_ty(instr.meta)), Operand::Imm(0)));
                sink.push(X64MCInstr::with2(Mnemonic::Mov, out.clone(), Operand::Reg(x64Reg::Rax.sub_ty(instr.meta))));
            }
            sink.push(X64MCInstr::with2(Mnemonic::Cmp, ls, rs));
        }
    }

    let out = match out {
        Operand::Reg(reg) => Operand::Reg(reg.sub8()),
        _ => out,
    };

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
