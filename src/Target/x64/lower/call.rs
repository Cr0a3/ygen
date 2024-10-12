use crate::CodeGen::MachineInstr;
use crate::Target::{x64Reg, CallConv};
use crate::Target::x64::asm::instr::*;

pub(crate) fn x64_lower_call(conv: CallConv, sink: &mut Vec<X64MCInstr>, _: &MachineInstr, target: &String) {   
    let func = target;

    if conv.reset_eax() {
        sink.push( X64MCInstr::with2(Mnemonic::Xor, Operand::Reg(x64Reg::Eax), Operand::Reg(x64Reg::Eax)) );
    }

    sink.push( X64MCInstr::with1(Mnemonic::Call, Operand::Imm(5)).into() );
    sink.push( X64MCInstr::with1(Mnemonic::Link, Operand::LinkDestination(func.to_string(), -4)).into() );
}
