use super::*;

pub(crate) fn x64BuildProlog(_: &Block, registry: &mut TargetBackendDescr) -> Vec<Instr> {
    let mut res = vec![];

    if registry.backend.currStackOffsetForLocalVars != 0 || registry.backend.stackSafe {
        res.push( Instr::with0(Mnemonic::Endbr64) );
        res.push( Instr::with1(Mnemonic::Push, Operand::Reg(x64Reg::Rbp.boxed())) );
        res.push( Instr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rbp.boxed()), Operand::Reg(x64Reg::Rsp.boxed())) );
        res.push( Instr::with2(Mnemonic::Sub, Operand::Reg(x64Reg::Rsp.boxed()), Operand::Imm(registry.backend.shadow + 8)) );
    }

    for backuped in &registry.backend.saveRegister {
        res.push( Instr::with1(Mnemonic::Push, Operand::Reg(backuped.boxed())) )
    }

    res.reverse();

    res
}

pub(crate) fn x64BuildEpilog(_: &Block, registry: &mut TargetBackendDescr) -> Vec<Instr> {
    let mut res = vec![];

    for backuped in &registry.backend.saveRegister {
        res.push( Instr::with1(Mnemonic::Pop, Operand::Reg(backuped.boxed())) )
    }

    if registry.backend.currStackOffsetForLocalVars != 0 || registry.backend.stackSafe {
        res.push( Instr::with2(Mnemonic::Add, Operand::Reg(x64Reg::Rsp.boxed()), Operand::Imm(registry.backend.shadow + 8)) );
        res.push( Instr::with1(Mnemonic::Pop, Operand::Reg(x64Reg::Rbp.boxed())) );
    }

    res.push( Instr::with0(Mnemonic::Ret));

    res
}
