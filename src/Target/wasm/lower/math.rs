use super::super::asm::*;
use crate::CodeGen::MachineInstr;

macro_rules! WasmMathLower {
    ($func:ident, $mnemonic:expr) => {
        pub(crate) fn $func(sink: &mut Vec<WasmMCInstr>, instr: &MachineInstr) {
            let out = instr.out.expect("math ops expect out");

            let ls = instr.operands.get(0).expect("math ops expect ls operand");
            let rs = instr.operands.get(0).expect("math ops expect rs operand");

            let ls = ls.into();
            let rs = rs.into();
        
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
            
            sink.push( WasmMCInstr::with0(Some(instr.meta.into()), $mnemonic) );

            sink.push( WasmMCInstr::with1(Some(WasmPrefix::Local), WasmMnemonic::Set, out.into()));
        }
    };
}

WasmMathLower!(wasm_lower_add, WasmMnemonic::Add);
// WasmMathLower!(wasm_lower_and);
WasmMathLower!(wasm_lower_mul, WasmMnemonic::Mul);
// WasmMathLower!(wasm_lower_or);
WasmMathLower!(wasm_lower_sub, WasmMnemonic::Sub);
// WasmMathLower!(wasm_lower_xor);
// WasmMathLower!(wasm_lower_neg);
// WasmMathLower!(wasm_lower_shl);
// WasmMathLower!(wasm_lower_shr);

pub(crate) fn wasm_lower_div(sink: &mut Vec<WasmMCInstr>, instr: &MachineInstr) {
    let out = instr.out.expect("math ops expect out");

    let ls = instr.operands.get(0).expect("math ops expect ls operand");
    let rs = instr.operands.get(0).expect("math ops expect rs operand");

    let ls = ls.into();
    let rs = rs.into();

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
    
    let mnemonic = if instr.meta.float() {
        WasmMnemonic::Div
    } else if instr.meta.signed() {
        WasmMnemonic::Divs
    } else {
        WasmMnemonic::Divu
    };

    sink.push( WasmMCInstr::with0(Some(instr.meta.into()), mnemonic) );

    sink.push( WasmMCInstr::with1(Some(WasmPrefix::Local), WasmMnemonic::Set, out.into()));
}


pub(crate) fn wasm_lower_rem(sink: &mut Vec<WasmMCInstr>, instr: &MachineInstr) {
    let out = instr.out.expect("math ops expect out");

    let ls = instr.operands.get(0).expect("math ops expect ls operand");
    let rs = instr.operands.get(0).expect("math ops expect rs operand");

    let ls = ls.into();
    let rs = rs.into();

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
    
    let mnemonic = if instr.meta.signed() {
        WasmMnemonic::Rems
    } else {
        WasmMnemonic::Remu
    };

    sink.push( WasmMCInstr::with0(Some(instr.meta.into()), mnemonic) );

    sink.push( WasmMCInstr::with1(Some(WasmPrefix::Local), WasmMnemonic::Set, out.into()));
}