use crate::CodeGen::MachineInstr;
use crate::Target::x64::asm::instr::*;
use crate::Target::x64::X64Reg;
use crate::IR::TypeMetadata;
use super::{RegAllocOperand, X64RegAllocInstr};

pub(crate) fn x64_lower_return(sink: &mut Vec<X64RegAllocInstr>, instr: &MachineInstr) {
    let op = instr.operands.get(0).expect("return expectes operand");
    let op = (*op).into();

    if instr.meta.float() {
        let mnemonic = if instr.meta == TypeMetadata::f32 {
            Mnemonic::Movss
        } else { Mnemonic::Movsd };

        sink.push( X64RegAllocInstr::with2(mnemonic, RegAllocOperand::Allocated(Operand::Reg(X64Reg::Xmm0)), op));
    } else {
        sink.push( X64RegAllocInstr::with2(Mnemonic::Mov, RegAllocOperand::Allocated( Operand::Reg(X64Reg::Rax.sub_ty(instr.meta))), op));
    }

    sink.push( X64RegAllocInstr::with0(Mnemonic::Ret) );
}
