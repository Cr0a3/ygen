use crate::CodeGen::MachineInstr;
use crate::Target::x64::X64Reg;
use crate::Target::x64::asm::instr::*;

pub(crate) fn x64_lower_zext(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr) {
    
    let op1 = instr.operands.get(0).expect("expected a first operand");
    let op2 = instr.operands.get(0).expect("expected a secound operand");
    let out = instr.out.expect("expected a output operand");

    let mut movxz = false;

    let op1 = (*op1).into();

    let op2 = (*op2).into();
    
    let out = out.into();

    if let Operand::Reg(op1) = op1 {
        if let Operand::Reg(op2) = op2 {
            if (op1.is_gr16() | op1.is_gr8()) && (op2.is_gr32() | op2.is_gr64()) { // movxz allowes a gr8/16 zext into gr32/64
                movxz = true;
            }
        }
    }

    if movxz {
        let tmp = Operand::Reg(X64Reg::Rax.sub_ty(instr.meta).sub_ty(instr.meta));

        sink.push(X64MCInstr::with2(Mnemonic::Mov, tmp.clone(), op1));
        sink.push(X64MCInstr::with2(Mnemonic::Movzx, tmp.clone(), op2));
        sink.push(X64MCInstr::with2(Mnemonic::Mov, out, tmp));
    } else {
        let tmp = Operand::Reg(X64Reg::Rax.sub_ty(instr.meta).sub_ty(instr.meta));

        if op1 == out {
            sink.push(X64MCInstr::with2(Mnemonic::Mov, op1,  op2));
        } else {
            sink.push(X64MCInstr::with2(Mnemonic::Mov, tmp.clone(), op1));
            sink.push(X64MCInstr::with2(Mnemonic::Mov, tmp.clone(), op2));
            sink.push(X64MCInstr::with2(Mnemonic::Mov, out, tmp));
        }
    }

}
