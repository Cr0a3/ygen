use super::super::asm::*;
use crate::CodeGen::MachineInstr;

macro_rules! WasmMathLower {
    ($func:ident) => {
        pub(crate) fn $func(sink: &mut Vec<WasmMCInstr>, instr: &MachineInstr) {
            todo!()
        }
    };
}

WasmMathLower!(wasm_lower_add);
WasmMathLower!(wasm_lower_and);
WasmMathLower!(wasm_lower_div);
WasmMathLower!(wasm_lower_mul);
WasmMathLower!(wasm_lower_or);
WasmMathLower!(wasm_lower_sub);
WasmMathLower!(wasm_lower_xor);
WasmMathLower!(wasm_lower_rem);
WasmMathLower!(wasm_lower_neg);
WasmMathLower!(wasm_lower_shl);
WasmMathLower!(wasm_lower_shr);