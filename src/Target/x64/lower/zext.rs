use crate::{CodeGen::MachineInstr, IR::TypeMetadata};
use crate::Target::x64::X64Reg;
use crate::Target::x64::asm::instr::*;

use super::{RegAllocOperand, X64RegAllocInstr};

pub(crate) fn x64_lower_zext(sink: &mut Vec<X64RegAllocInstr>, instr: &MachineInstr, start: &TypeMetadata) {
    let op1 = instr.operands.get(0).expect("expected a first operand");
    let out = instr.out.expect("expected a output operand");

    let mut movxz = false;

    let op1 = (*op1).into();
    
    let out = out.into();

    if (start.bitSize() == 8 || start.bitSize() == 16) && (instr.meta.bitSize() == 32 || instr.meta.bitSize() == 64) { // movxz allowes a gr8/16 zext into gr32/64
        movxz = true;
    }


    if movxz {
        sink.push(X64RegAllocInstr::with2(Mnemonic::Mov, RegAllocOperand::Tmp0, op1));
        sink.push(X64RegAllocInstr::with2(Mnemonic::Movzx, RegAllocOperand::Tmp0, RegAllocOperand::Tmp1));
        sink.push(X64RegAllocInstr::with2(Mnemonic::Mov, out, RegAllocOperand::Tmp0));
    } else {
        let tmp = Operand::Reg(X64Reg::Rax.sub_ty(instr.meta).sub_ty(instr.meta));

        sink.push(X64RegAllocInstr::with2(Mnemonic::Mov, RegAllocOperand::Tmp0, op1));
        sink.push(X64RegAllocInstr::with2(Mnemonic::Mov, out, RegAllocOperand::Tmp0));
    }

}
