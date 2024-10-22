use crate::CodeGen::MachineInstr;
//use crate::Target::x64Reg;
use crate::Target::x64::asm::instr::*;
use crate::IR::TypeMetadata;

pub(crate) fn x64_lower_downcast(_sink: &mut Vec<X64MCInstr>, _instr: &MachineInstr, _start: &TypeMetadata) {
    todo!()
}
