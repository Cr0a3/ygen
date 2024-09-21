use crate::{CodeGen::IrCodeGenArea, Target::CallConv};

use super::{instr::X64MCInstr, lower::x64_lower_instr};

#[allow(unused)]
pub(crate) fn x64_lower_ir_debug(area: &IrCodeGenArea, call: CallConv) -> Vec<X64MCInstr> {
    let mut sink = vec![];

    for instr in &area.compiled {
        x64_lower_instr(call, &mut sink, instr.to_owned());
    }

    sink
}