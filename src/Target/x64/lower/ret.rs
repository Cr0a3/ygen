use crate::CodeGen::MachineInstr;
//use crate::Target::x64Reg;
use crate::Target::x64::asm::instr::*;

pub(crate) fn x64_lower_return(sink: &mut Vec<X64MCInstr>, _: &MachineInstr) {
    sink.push( X64MCInstr::with0(Mnemonic::Ret).into() )
}
