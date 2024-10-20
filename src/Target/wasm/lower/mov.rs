use super::super::asm::*;
use crate::CodeGen::MachineInstr;

pub(crate) fn wasm_lower_mov(sink: &mut Vec<WasmMCInstr>, instr: &MachineInstr) {
    let out = instr.out.expect("move expects out");

    let op = instr.operands.get(0).expect("move expects operand");
    let op = op.into();

    if let WasmOperand::Const(_) = op {
        sink.push( WasmMCInstr::with1(Some(instr.meta.into()), WasmMnemonic::Const, op));
    } else {
        sink.push( WasmMCInstr::with1(Some(WasmPrefix::Local), WasmMnemonic::Get, op));
    }
    
    sink.push( WasmMCInstr::with1(Some(WasmPrefix::Local), WasmMnemonic::Set, out.into()));
}