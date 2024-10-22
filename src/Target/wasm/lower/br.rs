use super::super::asm::*;
use crate::CodeGen::MachineInstr;

pub(crate) fn wasm_lower_br(sink: &mut Vec<WasmMCInstr>, _: &MachineInstr, block: String) {
    sink.extend_from_slice(&[
        todo!(),
        WasmMCInstr::with0(None, WasmMnemonic::BlockLink(block))
    ]);
}

pub(crate) fn wasm_lower_brcond(sink: &mut Vec<WasmMCInstr>, instr: &MachineInstr, iftrue: String, iffalse: String) {
    todo!()
}