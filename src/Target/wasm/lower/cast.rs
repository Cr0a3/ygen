use super::super::asm::*;
use crate::CodeGen::MachineInstr;
use crate::IR::TypeMetadata;

pub(crate) fn wasm_lower_cast(sink: &mut Vec<WasmMCInstr>, instr: &MachineInstr, start_ty: TypeMetadata) {
    todo!()
}