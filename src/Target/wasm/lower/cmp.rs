use super::super::asm::*;
use crate::{prelude::CmpMode, CodeGen::MachineInstr};

pub(crate) fn wasm_lower_cmp(sink: &mut Vec<WasmMCInstr>, instr: &MachineInstr, mode: CmpMode) {
    let ls = instr.operands.get(0).expect("expected ls operand for cmps");
    let rs = instr.operands.get(1).expect("expected rs operand for cmps");

    let mnemonic = match mode {
        CmpMode::Eqal => WasmMnemonic::Eq,
        CmpMode::NotEqal => WasmMnemonic::Ne,
        CmpMode::GreaterThan => if instr.meta.float() {
            WasmMnemonic::Gt
        } else if instr.meta.signed() {
            WasmMnemonic::Gts
        } else {
            WasmMnemonic::Gtu
        },
        CmpMode::LessThan => if instr.meta.float() {
            WasmMnemonic::Lt
        } else if instr.meta.signed() {
            WasmMnemonic::Lts
        } else {
            WasmMnemonic::Ltu
        },
        CmpMode::GreaterThanOrEqual => if instr.meta.float() {
            WasmMnemonic::Ge
        } else if instr.meta.signed() {
            WasmMnemonic::Ges
        } else {
            WasmMnemonic::Geu
        },
        CmpMode::LessThanOrEqual => if instr.meta.float() {
            WasmMnemonic::Le
        } else if instr.meta.signed() {
            WasmMnemonic::Les
        } else {
            WasmMnemonic::Leu
        },
    };

    sink.extend_from_slice(&[
        WasmMCInstr::with1(Some(WasmPrefix::Local), WasmMnemonic::Get, (*ls).into()),
        WasmMCInstr::with1(Some(WasmPrefix::Local), WasmMnemonic::Get, (*rs).into()),
        WasmMCInstr::with0(Some(instr.meta.into()), mnemonic),
    ]);
}