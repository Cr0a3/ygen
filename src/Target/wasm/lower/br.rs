use super::super::asm::*;
use crate::CodeGen::MachineInstr;

pub(crate) fn wasm_lower_br(sink: &mut Vec<WasmMCInstr>, instr: &MachineInstr, block: String) {
    todo!()
}

pub(crate) fn wasm_lower_brcond(sink: &mut Vec<WasmMCInstr>, instr: &MachineInstr, iftrue: String, iffalse: String) {
    todo!()
}