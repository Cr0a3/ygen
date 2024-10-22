use super::super::asm::*;
use crate::CodeGen::MachineInstr;

pub(crate) fn wasm_lower_alloc(sink: &mut Vec<WasmMCInstr>, instr: &MachineInstr) {
    todo!()
}

pub(crate) fn wasm_lower_store(sink: &mut Vec<WasmMCInstr>, instr: &MachineInstr) {
    todo!()
}

pub(crate) fn wasm_lower_load(sink: &mut Vec<WasmMCInstr>, instr: &MachineInstr) {
    todo!()
}

pub(crate) fn wasm_lower_adress_load(sink: &mut Vec<WasmMCInstr>, instr: &MachineInstr, constant: String) {
    todo!()
}

pub(crate) fn wasm_lower_push(sink: &mut Vec<WasmMCInstr>, instr: &MachineInstr) {
    let op = instr.operands.get(0).expect("push expects operand");
    let op = (*op).into();

    if let WasmOperand::Const(_) = op {
        sink.push( WasmMCInstr::with1(Some(instr.meta.into()), WasmMnemonic::Const, op));
    } else {
        sink.push( WasmMCInstr::with1(Some(instr.meta.into()), WasmMnemonic::Get, op));
    }
}