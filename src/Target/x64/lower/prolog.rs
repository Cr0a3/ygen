use crate::CodeGen::MachineInstr;
use crate::Target::x64::X64Reg;
use crate::Target::x64::asm::instr::*;

pub(crate) fn x64_lower_prolog(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr) {
    //sink.push( X64MCInstr::with0(Mnemonic::Endbr64) );
    sink.push( X64MCInstr::with1(Mnemonic::Push, Operand::Reg(X64Reg::Rbp) ) );
    sink.push( X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(X64Reg::Rbp), Operand::Reg(X64Reg::Rsp)  ) );
    if let Some(op0) = instr.operands.get(0) {
        let op0 = (*op0).into();

        sink.push( X64MCInstr::with2(Mnemonic::Sub, Operand::Reg(X64Reg::Rbp),  op0) );
    }
}

pub(crate) fn x64_lower_epilog(sink: &mut Vec<X64MCInstr>, _: &MachineInstr) {
    sink.push( X64MCInstr::with1(Mnemonic::Pop, Operand::Reg(X64Reg::Rbp) ) );
}
