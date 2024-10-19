use crate::{CodeGen::{MCInstr, MachineInstr}, Optimizations::Optimize, Target::CallConv};

pub(crate) fn wasm_lower_instr(sink: &mut Vec<super::asm::WasmMCInstr>, instr: MachineInstr) {
    match instr.mnemonic {
        crate::CodeGen::MachineMnemonic::Move => todo!(),
        crate::CodeGen::MachineMnemonic::Add => todo!(),
        crate::CodeGen::MachineMnemonic::And => todo!(),
        crate::CodeGen::MachineMnemonic::Div => todo!(),
        crate::CodeGen::MachineMnemonic::Mul => todo!(),
        crate::CodeGen::MachineMnemonic::Or => todo!(),
        crate::CodeGen::MachineMnemonic::Sub => todo!(),
        crate::CodeGen::MachineMnemonic::Xor => todo!(),
        crate::CodeGen::MachineMnemonic::Rem => todo!(),
        crate::CodeGen::MachineMnemonic::Neg => todo!(),
        crate::CodeGen::MachineMnemonic::Shl => todo!(),
        crate::CodeGen::MachineMnemonic::Shr => todo!(),
        crate::CodeGen::MachineMnemonic::FMove => todo!(),
        crate::CodeGen::MachineMnemonic::FAdd => todo!(),
        crate::CodeGen::MachineMnemonic::FAnd => todo!(),
        crate::CodeGen::MachineMnemonic::FDiv => todo!(),
        crate::CodeGen::MachineMnemonic::FMul => todo!(),
        crate::CodeGen::MachineMnemonic::FOr => todo!(),
        crate::CodeGen::MachineMnemonic::FSub => todo!(),
        crate::CodeGen::MachineMnemonic::FXor => todo!(),
        crate::CodeGen::MachineMnemonic::FRem => todo!(),
        crate::CodeGen::MachineMnemonic::FNeg => todo!(),
        crate::CodeGen::MachineMnemonic::FShl => todo!(),
        crate::CodeGen::MachineMnemonic::FShr => todo!(),
        crate::CodeGen::MachineMnemonic::FCompare(cmp_mode) => todo!(),
        crate::CodeGen::MachineMnemonic::FCast(type_metadata) => todo!(),
        crate::CodeGen::MachineMnemonic::BrCond(iftrue, iffalse) => todo!(),
        crate::CodeGen::MachineMnemonic::Compare(cmp_mode) => todo!(),
        crate::CodeGen::MachineMnemonic::Zext => todo!(),
        crate::CodeGen::MachineMnemonic::Downcast => todo!(),
        crate::CodeGen::MachineMnemonic::Call(func) => todo!(),
        crate::CodeGen::MachineMnemonic::Br(block_name) => todo!(),
        crate::CodeGen::MachineMnemonic::Return => todo!(),
        crate::CodeGen::MachineMnemonic::AdressLoad(const_name) => todo!(),
        crate::CodeGen::MachineMnemonic::StackAlloc => todo!(),
        crate::CodeGen::MachineMnemonic::Store => todo!(),
        crate::CodeGen::MachineMnemonic::Load => todo!(),
        crate::CodeGen::MachineMnemonic::Prolog => todo!(),
        crate::CodeGen::MachineMnemonic::Epilog => todo!(),
        crate::CodeGen::MachineMnemonic::Push => todo!(),
        crate::CodeGen::MachineMnemonic::PushCleanup => todo!(),
        crate::CodeGen::MachineMnemonic::CallStackPrepare => todo!(),
        crate::CodeGen::MachineMnemonic::CallStackRedo => todo!(),
        crate::CodeGen::MachineMnemonic::AdrMove => todo!(),
        crate::CodeGen::MachineMnemonic::Switch(cases) => todo!(),
        crate::CodeGen::MachineMnemonic::MovIfZero => todo!(),
        crate::CodeGen::MachineMnemonic::MovIfNotZero => todo!(),
    }
}

/// The function used for lowering general `MachineInstr` into `MCInstr`
pub(crate) fn wasm_lower(_: CallConv, instrs: Vec<MachineInstr>) -> Vec<Box<dyn MCInstr>> {
    let mut out = Vec::new();

    for instr in instrs {
        wasm_lower_instr(&mut out, instr.clone());
    }

    out.optimize();

    let mut mc_instrs = vec![];

    for instr in out {
        mc_instrs.push( instr.into() );
    }

    mc_instrs
}