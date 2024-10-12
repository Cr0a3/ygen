use crate::CodeGen::MachineInstr;
use crate::Target::x64Reg;
use crate::Target::x64::asm::instr::*;

pub(crate) fn x64_lower_move(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr) {
    let op1 = instr.operands.get(0).expect("expected a first operand");
    let out = instr.out.expect("expected a output operand");

    let op1 = (*op1).into();
    
    let out = out.into();

    if let Operand::Mem(_) = out {
        if let Operand::Reg(_) = op1 {} else {
            sink.push( X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rax.sub_ty(instr.meta)), op1) );
            sink.push( X64MCInstr::with2(Mnemonic::Mov, out, Operand::Reg(x64Reg::Rax.sub_ty(instr.meta))) );
            return;
        }
    }

    sink.push( X64MCInstr::with2(Mnemonic::Mov, out, op1).into() );
}