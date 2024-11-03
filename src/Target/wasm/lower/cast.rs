use super::super::asm::*;
use crate::CodeGen::MachineInstr;
use crate::IR::TypeMetadata;

pub(crate) fn wasm_lower_cast(sink: &mut Vec<WasmMCInstr>, instr: &MachineInstr, start_ty: TypeMetadata) {
    let (prefix, mnemonic) = match (start_ty, instr.meta) {
        (TypeMetadata::i32, TypeMetadata::i64) => (None, WasmMnemonic::Extends),
        (TypeMetadata::u32, TypeMetadata::u64) => (None, WasmMnemonic::Extendu),
        (TypeMetadata::i64, TypeMetadata::i32) => (None, WasmMnemonic::Wrap),
        (TypeMetadata::f32, TypeMetadata::f64) => (None, WasmMnemonic::Promote),
        (TypeMetadata::f64, TypeMetadata::f32) => (None, WasmMnemonic::Demote),

        (TypeMetadata::i32, TypeMetadata::f32) => (Some(WasmPrefix::f32), WasmMnemonic::ConvertI32s),
        (TypeMetadata::i64, TypeMetadata::f32) => (Some(WasmPrefix::f32), WasmMnemonic::ConvertI64s),
        (TypeMetadata::u32, TypeMetadata::f32) => (Some(WasmPrefix::f32), WasmMnemonic::ConvertI32u),
        (TypeMetadata::u64, TypeMetadata::f32) => (Some(WasmPrefix::f32), WasmMnemonic::ConvertI32u),

        (TypeMetadata::i32, TypeMetadata::f64) => (Some(WasmPrefix::f64), WasmMnemonic::ConvertI32s),
        (TypeMetadata::i64, TypeMetadata::f64) => (Some(WasmPrefix::f64), WasmMnemonic::ConvertI64s),
        (TypeMetadata::u32, TypeMetadata::f64) => (Some(WasmPrefix::f64), WasmMnemonic::ConvertI32u),
        (TypeMetadata::u64, TypeMetadata::f64) => (Some(WasmPrefix::f64), WasmMnemonic::ConvertI32u),

        (TypeMetadata::f32, TypeMetadata::i32) => (Some(WasmPrefix::i32), WasmMnemonic::TruncF32s),
        (TypeMetadata::f32, TypeMetadata::i64) => (Some(WasmPrefix::i64), WasmMnemonic::TruncF32s),
        (TypeMetadata::f32, TypeMetadata::u32) => (Some(WasmPrefix::i32), WasmMnemonic::TruncF32u),
        (TypeMetadata::f32, TypeMetadata::u64) => (Some(WasmPrefix::i64), WasmMnemonic::TruncF32u),

        (TypeMetadata::f64, TypeMetadata::i32) => (Some(WasmPrefix::i32), WasmMnemonic::TruncF64s),
        (TypeMetadata::f64, TypeMetadata::i64) => (Some(WasmPrefix::i64), WasmMnemonic::TruncF64s),
        (TypeMetadata::f64, TypeMetadata::u32) => (Some(WasmPrefix::i32), WasmMnemonic::TruncF64u),
        (TypeMetadata::f64, TypeMetadata::u64) => (Some(WasmPrefix::i64), WasmMnemonic::TruncF64u),

        _ => panic!("illegal cast combination for wasm: cast {} ... to {}", start_ty, instr.meta),
    };

    let op = (*instr.operands.get(0).expect("expected operand for cast")).into();
    let out = instr.out.expect("expected output for cast").into();

    if let WasmOperand::Const(_) = op {
        sink.push( WasmMCInstr::with1(Some(instr.meta.into()), WasmMnemonic::Const, op));
    } else {
        sink.push( WasmMCInstr::with1(Some(WasmPrefix::Local), WasmMnemonic::Get, op));
    }
    sink.extend_from_slice(&[
        WasmMCInstr::with0(prefix, mnemonic),
        WasmMCInstr::with1(Some(WasmPrefix::Local), WasmMnemonic::Set, out)
    ]);
}