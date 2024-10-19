use super::super::asm::*;
use crate::CodeGen::MachineInstr;
use crate::IR::{Type, BlockId};

pub(crate) fn wasm_lower_switch(sink: &mut Vec<WasmMCInstr>, instr: &MachineInstr, cases: Vec<(Type, BlockId)>) {
    todo!()
}