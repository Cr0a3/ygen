use crate::CodeGen::MachineInstr;
use crate::Target::x64::X64Reg;
use crate::Target::x64::asm::instr::*;

pub(crate) fn x64_lower_adr_load(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr, symbol: &String) {
    let out = instr.out.expect("expected a output operand");

    let out = out.into();

    sink.push(
        X64MCInstr::with2(Mnemonic::Lea, Operand::Reg(X64Reg::Rax.sub_ty(instr.meta)), Operand::Mem(MemOp { base: None, index: None, scale: 1, displ: 7, rip: true })).into()
    );
    sink.push(
        X64MCInstr::with1(Mnemonic::Link, Operand::LinkDestination(symbol.to_string(), -4)).into()
    );
    sink.push(
        X64MCInstr::with2(Mnemonic::Mov, out, Operand::Reg(X64Reg::Rax.sub_ty(instr.meta))).into()
    );
}

pub(crate) fn x64_lower_adrm(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr) {
    let op = instr.operands.get(0).expect("expected adrm expectes one operand");
    let out = instr.out.expect("expected adrm expectes one operand");

    let op = (*op).into();

    let out = out.into();

    if let Operand::Reg(_) = out {
        if let Operand::Mem(_) = op {
            sink.push(X64MCInstr::with2(Mnemonic::Lea, out, op));
        } else {
            sink.push(X64MCInstr::with2(Mnemonic::Mov, out, op));
        }
    } else {
        if let Operand::Mem(_) = op {
            sink.push(X64MCInstr::with2(Mnemonic::Lea, Operand::Reg(X64Reg::Rax.sub_ty(instr.meta)), op));
        } else {
            sink.push(X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(X64Reg::Rax.sub_ty(instr.meta)), op));
        }
        sink.push(X64MCInstr::with2(Mnemonic::Mov, out, Operand::Reg(X64Reg::Rax.sub_ty(instr.meta))));
    }
}
