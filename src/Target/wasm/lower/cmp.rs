use super::super::asm::*;
use crate::{prelude::CmpMode, CodeGen::MachineInstr};

pub(crate) fn wasm_lower_cmp(sink: &mut Vec<WasmMCInstr>, instr: &MachineInstr, mode: CmpMode) {
    let ls = instr.operands.get(0).expect("expected ls operand for cmps");
    let rs = instr.operands.get(1).expect("expected rs operand for cmps");

    let ls = (*ls).into();
    let rs = (*rs).into();
    let out: WasmOperand = instr.out.unwrap().into();

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
    if let WasmOperand::Const(_) = ls {
        sink.push( WasmMCInstr::with1(Some(instr.meta.into()), WasmMnemonic::Const, ls));
    } else {
        sink.push( WasmMCInstr::with1(Some(WasmPrefix::Local), WasmMnemonic::Get, ls));
    }

    if let WasmOperand::Const(_) = rs {
        sink.push( WasmMCInstr::with1(Some(instr.meta.into()), WasmMnemonic::Const, rs));
    } else {
        sink.push( WasmMCInstr::with1(Some(WasmPrefix::Local), WasmMnemonic::Get, rs));
    }

    sink.extend_from_slice(&[
        WasmMCInstr::with0(Some(instr.meta.into()), mnemonic),
        WasmMCInstr::with1(Some(WasmPrefix::Local), WasmMnemonic::Set, out)
    ]);
}