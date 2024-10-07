use crate::CodeGen::MachineInstr;
use crate::Target::x64Reg;
use crate::Target::x64::asm::instr::*;

pub(crate) fn x64_lower_push(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr) {
    let input = instr.operands.get(0).expect("push needs an operand");

    sink.push(
        X64MCInstr::with1(Mnemonic::Push, (*input).into())
    );

    sink.push(X64MCInstr::with2(Mnemonic::Sub, Operand::Reg(x64Reg::Rsp), Operand::Imm(8))); // for 16 byte alignment
}

pub(crate) fn x64_lower_push_cleanup(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr) {
    sink.push( X64MCInstr::with1(Mnemonic::Pop, Operand::Reg(x64Reg::Rax.sub_ty(instr.meta))));
    sink.push(X64MCInstr::with2(Mnemonic::Add, Operand::Reg(x64Reg::Rsp), Operand::Imm(8))); // for 16 byte alignment
}
