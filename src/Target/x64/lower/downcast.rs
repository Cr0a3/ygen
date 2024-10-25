use crate::CodeGen::MachineInstr;
//use crate::Target::x64Reg;
use crate::Target::x64::asm::instr::*;
use crate::Target::x64::X64Reg;

pub(crate) fn x64_lower_downcast(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr) {
    let out = instr.out.expect("downcast expects output").into();
    let op = match instr.operands.get(0).expect("downcast expects operand") {
        crate::CodeGen::MachineOperand::Imm(i) => Operand::Imm(*i as i64),
        crate::CodeGen::MachineOperand::Reg(reg) => match reg {
            crate::CodeGen::Reg::x64(reg) => Operand::Reg(reg.sub_ty(instr.meta)),
            _ => panic!("x64 backend expects x64 registers")
        }
        crate::CodeGen::MachineOperand::Stack(off) => Operand::Mem(X64Reg::Rbp - *off as u32),
    };

    sink.extend_from_slice(&[
        X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(X64Reg::Rax.sub_ty(instr.meta)), op),
        X64MCInstr::with2(Mnemonic::Mov, out, Operand::Reg(X64Reg::Rax.sub_ty(instr.meta))),
    ]);
}
