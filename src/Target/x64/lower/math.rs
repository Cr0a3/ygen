use crate::CodeGen::MachineInstr;
use crate::Target::x64Reg;
use crate::Target::x64::asm::instr::*;

macro_rules! LowerSimpleMath {
    ($func:ident, $mnemonic:expr) => {
        pub(crate) fn $func(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr) {       
            let op1 = instr.operands.get(0).expect("expected a first operand");
            let op2 = instr.operands.get(1).expect("expected a second operand");
            let out = instr.out.expect("expected a output operand");

            let op1 = (*op1).into();
            let op2 =(*op2).into();
            let out = out.into();

            let tmp = || Operand::Reg(x64Reg::Rax.sub_ty(instr.meta));

            sink.push( X64MCInstr::with2(Mnemonic::Mov, tmp(), op1).into() );
            sink.push( X64MCInstr::with2($mnemonic, tmp(), op2).into() );
            sink.push( X64MCInstr::with2(Mnemonic::Mov, out, tmp()).into() );
        }
    };
}

LowerSimpleMath!(x64_lower_add, Mnemonic::Add);
LowerSimpleMath!(x64_lower_and, Mnemonic::And);
LowerSimpleMath!(x64_lower_or, Mnemonic::Or);
LowerSimpleMath!(x64_lower_sub, Mnemonic::Sub);
LowerSimpleMath!(x64_lower_xor, Mnemonic::Xor);

pub(crate) fn x64_lower_div(_sink: &mut Vec<X64MCInstr>, _instr: &MachineInstr) {
    todo!()
}

pub(crate) fn x64_lower_mul(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr) {
    let op1 = instr.operands.get(0).expect("expected a first operand");
    let op2 = instr.operands.get(1).expect("expected a second operand");
    let out = instr.out.expect("expected a output operand");

    let op1 = (*op1).into();
    let op2 = (*op2).into();
    let out: Operand = out.into();

    let mnemonic = if instr.meta.signed() {
        Mnemonic::Imul
    } else {
        Mnemonic::Mul
    };

    if out != Operand::Reg(x64Reg::Rdx.sub_ty(instr.meta)) {
        sink.push( X64MCInstr::with1(Mnemonic::Push, Operand::Reg(x64Reg::Rdx)).into() );
    }

    // MUL node here:
    // mov rax, op1
    // mul/imul op2
    // mov out, rax
    // RDX = is upper slice which will just get destroyed

    let rax = || Operand::Reg(x64Reg::Rax.sub_ty(instr.meta));

    sink.push(X64MCInstr::with2(Mnemonic::Mov, rax(), op1));
    
    // mul/imul only accept r/m
    if let Operand::Imm(_) = op2 {
        sink.push(X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rbx.sub_ty(instr.meta)), op2));
        sink.push(X64MCInstr::with1(mnemonic, Operand::Reg(x64Reg::Rbx.sub_ty(instr.meta))));
    } else if let Operand::Mem(_) = op2 {
        sink.push(X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rbx.sub_ty(instr.meta)), op2));
        sink.push(X64MCInstr::with1(mnemonic, Operand::Reg(x64Reg::Rbx.sub_ty(instr.meta))));
    } else {
        sink.push(X64MCInstr::with1(mnemonic, op2));
    }

    sink.push(X64MCInstr::with2(Mnemonic::Mov, out.to_owned(), rax()));

    if out != Operand::Reg(x64Reg::Rdx.sub_ty(instr.meta)) {
        sink.push( X64MCInstr::with1(Mnemonic::Pop, Operand::Reg(x64Reg::Rdx)).into() );
    }
}

pub(crate) fn x64_lower_neg(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr) {
    let out = instr.out.expect("neg expectes output");
    let op = instr.operands.get(0).expect("neg expectes operand");

    let out: Operand = out.into();
    let op: Operand = (*op).into();

    if op == out {
        sink.push(X64MCInstr::with1(Mnemonic::Neg, out));
        return;
    }

    if let Operand::Mem(_) = op {
        if let Operand::Reg(_) = out {
            sink.push(X64MCInstr::with2(Mnemonic::Mov, out.clone(), op));
            sink.push(X64MCInstr::with1(Mnemonic::Neg, out));
        } else {
            sink.push(X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rax), op));
            sink.push(X64MCInstr::with2(Mnemonic::Mov, out.clone(), Operand::Reg(x64Reg::Rax)));
            sink.push(X64MCInstr::with1(Mnemonic::Neg, out));
        }
    } else {
        sink.push(X64MCInstr::with2(Mnemonic::Mov, out.clone(), op));
        sink.push(X64MCInstr::with1(Mnemonic::Neg, out));
    }
}