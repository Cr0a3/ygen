use super::super::asm::*;
use crate::{prelude::CmpMode, CodeGen::MachineInstr};

pub(crate) fn wasm_lower_cmp(sink: &mut Vec<WasmMCInstr>, instr: &MachineInstr, mode: CmpMode) {
    todo!()
}