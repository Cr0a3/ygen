use crate::CodeGen::MachineInstr;
use crate::Target::x64Reg;
use crate::Target::x64::asm::instr::*;
use crate::IR::{Type, BlockId};

pub(crate) fn x64_lower_switch(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr, cases: &Vec<(Type, BlockId)>) {
    let var = *instr.operands.get(0).expect("switch expectes an variable to switch");
    let mut var = var.into();

    if let Operand::Mem(_) = var {
        sink.push(
            X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rax), var)
        );

        var = Operand::Reg(x64Reg::Rax);
    }

    for (case_type, block) in cases {
        sink.push(
            X64MCInstr::with2(Mnemonic::Cmp, var.clone(), Operand::Imm(case_type.val() as i64)),
        ); 
        sink.push(
            X64MCInstr::with1(Mnemonic::Je, Operand::Imm(0))
        );
        sink.push(
            X64MCInstr::with1(Mnemonic::Link, Operand::BlockLinkDestination(block.name.to_owned(), -4))
        );
    }
}