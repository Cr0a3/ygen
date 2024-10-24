use crate::CodeGen::MachineInstr;
use crate::Target::{x64::X64Reg, CallConv};
use crate::Target::x64::asm::instr::*;

pub(crate) fn x64_lower_call(conv: CallConv, sink: &mut Vec<X64MCInstr>, _: &MachineInstr, target: &String) {   
    let func = target;

    if conv == CallConv::SystemV{
        sink.push( X64MCInstr::with2(Mnemonic::Xor, Operand::Reg(X64Reg::Eax), Operand::Reg(X64Reg::Eax)) );
    }

    sink.push( X64MCInstr::with1(Mnemonic::Call, Operand::LinkDestination(func.to_string(), -4)) );
}
