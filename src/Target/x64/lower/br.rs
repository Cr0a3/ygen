use crate::CodeGen::MachineInstr;
use crate::Target::x64::X64Reg;
use crate::Target::x64::asm::instr::*;


pub(crate) fn x64_lower_br(sink: &mut Vec<X64MCInstr>, _: &MachineInstr, symbol: &String) {
    let target = Operand::BlockLinkDestination(symbol.to_owned(), -4);

    sink.push(
        X64MCInstr::with1(Mnemonic::Jmp, Operand::Imm(0))
    );


    sink.push(
        X64MCInstr::with1(Mnemonic::Link, target)
    );
}

pub(crate) fn x64_lower_cond_br(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr, iftrue: &String, iffalse: &String) {
    let src = instr.operands.get(0).expect("expected valid src operand at 1. place");
    let value = instr.operands.get(1).expect("expected valid value to compare at 1. place");

    let src = (*src).into();
    let value = (*value).into();

    if let Operand::Mem(_) = src {
        sink.push(X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(X64Reg::Rax.sub_ty(instr.meta)), src));
        sink.push(X64MCInstr::with2(Mnemonic::Cmp, Operand::Reg(X64Reg::Rax.sub_ty(instr.meta)), value));
    } else {
        sink.push(X64MCInstr::with2(Mnemonic::Cmp, src, value));
    }
    sink.push(X64MCInstr::with1(Mnemonic::Jne, Operand::Imm(0)));
    sink.push(X64MCInstr::with1(Mnemonic::Link, Operand::BlockLinkDestination(iftrue.to_owned(), -4))); // not 0
    sink.push(X64MCInstr::with1(Mnemonic::Jmp, Operand::Imm(0)));
    sink.push(X64MCInstr::with1(Mnemonic::Link, Operand::BlockLinkDestination(iffalse.to_owned(), -4))); // is 0
}