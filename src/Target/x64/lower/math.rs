use crate::CodeGen::MachineInstr;
use crate::Target::x64::X64Reg;
use crate::Target::x64::asm::instr::*;
use crate::IR::TypeMetadata;

macro_rules! LowerSimpleMath {
    ($func:ident, $mnemonic:expr) => {
        pub(crate) fn $func(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr) {       
            let op1 = instr.operands.get(0).expect("expected a first operand");
            let op2 = instr.operands.get(1).expect("expected a second operand");
            let out = instr.out.expect("expected a output operand");

            let op1 = (*op1).into();
            let op2 =(*op2).into();
            let out = out.into();

            let rax = || Operand::Reg(X64Reg::Rax.sub_ty(instr.meta));

            sink.push( X64MCInstr::with2(Mnemonic::Mov, rax(), op1).into() );
            sink.push( X64MCInstr::with2($mnemonic, rax(), op2).into() );
            sink.push( X64MCInstr::with2(Mnemonic::Mov, out, rax()).into() );
        }
    };
}

LowerSimpleMath!(x64_lower_add, Mnemonic::Add);
LowerSimpleMath!(x64_lower_and, Mnemonic::And);
LowerSimpleMath!(x64_lower_or, Mnemonic::Or);
LowerSimpleMath!(x64_lower_sub, Mnemonic::Sub);
LowerSimpleMath!(x64_lower_xor, Mnemonic::Xor);

pub(crate) fn x64_lower_mul(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr) {
    let op1 = instr.operands.get(0).expect("expected a first operand");
    let op2 = instr.operands.get(1).expect("expected a second operand");
    let out = instr.out.expect("expected a output operand");

    let op1: Operand = (*op1).into();
    let op2: Operand = (*op2).into();
    let out: Operand = out.into();

    if !op1.is_imm() && op2.is_imm() && out.is_reg() {
        let Operand::Imm(displ) = op2 else { unreachable!() };

        sink.push(X64MCInstr::with3(Mnemonic::Imul, out, op1, Operand::Imm(displ)));
        
        return;
    }

    if op1.is_imm() && op2.is_imm() { // theoraticly we could precalculate it here but if the user wanted us to do this he would use `-O` flag
        sink.extend_from_slice(&[
            X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(X64Reg::Rax.sub_ty(instr.meta)), op1),
            X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(X64Reg::R11.sub_ty(instr.meta)), op2),
            X64MCInstr::with2(Mnemonic::Imul, Operand::Reg(X64Reg::Rax.sub_ty(instr.meta)), Operand::Reg(X64Reg::R11.sub_ty(instr.meta))),
            X64MCInstr::with2(Mnemonic::Mov, out, Operand::Reg(X64Reg::Rax.sub_ty(instr.meta))),
        ]);
        
        return;
    }

    
    if !op2.is_imm() && op1.is_imm() && out.is_reg() {
        let Operand::Imm(displ) = op1 else { unreachable!() };

        sink.push(X64MCInstr::with3(Mnemonic::Imul, out, op2, Operand::Imm(displ)));
        
        return;
    }

    let tmp = X64Reg::Rax.sub_ty(instr.meta);
    
    if op2 != out && out.is_reg() {
        sink.extend_from_slice(&[
            X64MCInstr::with2(Mnemonic::Mov, out.clone(), op1),
            X64MCInstr::with2(Mnemonic::Imul, out, op2),
        ]);
    } else {
        sink.extend_from_slice(&[
            X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(tmp), op1),
            X64MCInstr::with2(Mnemonic::Imul, Operand::Reg(tmp), op2),
            X64MCInstr::with2(Mnemonic::Mov, out, Operand::Reg(tmp)),
        ]);
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
            sink.push(X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(X64Reg::Rax), op));
            sink.push(X64MCInstr::with2(Mnemonic::Mov, out.clone(), Operand::Reg(X64Reg::Rax)));
            sink.push(X64MCInstr::with1(Mnemonic::Neg, out));
        }
    } else {
        sink.push(X64MCInstr::with2(Mnemonic::Mov, out.clone(), op));
        sink.push(X64MCInstr::with1(Mnemonic::Neg, out));
    }
}

pub(crate) fn x64_lower_div(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr) {
    // core logic:

    // save rdx
    // mov rax, op1
    // rdx = 0
    // div op2
    // mov out, rax
    // restore rdx

    // but with imms:
    // save rdx
    // rdx = 0
    // mov rax, op1
    // mov rbx, op2
    // div rbx
    // mov out, rax
    // restore rdx

    let op1 = instr.operands.get(0).expect("expected a first operand");
    let op2 = instr.operands.get(1).expect("expected a second operand");
    let out = instr.out.expect("expected a output operand");

    let op1: Operand = (*op1).into();
    let op2: Operand = (*op2).into();
    let out: Operand = out.into();

    let div_mnemonic = if instr.meta.signed() {
        Mnemonic::Idiv
    } else {
        Mnemonic::Div
    };

    let rdx_prep_instr = if instr.meta.signed() {
        X64MCInstr::with0(match instr.meta {
            TypeMetadata::i8 => Mnemonic::Cbw,
            TypeMetadata::i16 => Mnemonic::Cwd,
            TypeMetadata::i32 => Mnemonic::Cdq,
            TypeMetadata::i64 => Mnemonic::Cqo,
            _ => panic!("type {} was labeld as signed but shouldn't be", instr.meta)
        })
    } else {
        X64MCInstr::with2(Mnemonic::Xor, Operand::Reg(X64Reg::Rdx), Operand::Reg(X64Reg::Rdx))
    };

    let out_is_rdx = if let Operand::Reg(reg) = out { reg.sub64() == X64Reg::Rdx } else { false };
    
    if !out_is_rdx {
        sink.push(X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(X64Reg::R15), Operand::Reg(X64Reg::Rdx))); // save rdx
    }

    // assembly code is here
    let div_instr = if op2.is_imm() {
        sink.push(X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(X64Reg::R11.sub_ty(instr.meta)), op2));
        X64MCInstr::with1(div_mnemonic, Operand::Reg(X64Reg::R11.sub_ty(instr.meta)))
    } else if matches!(op2, Operand::Reg(X64Reg::Rdx) | Operand::Reg(X64Reg::Edx)) {
        sink.push(X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(X64Reg::R11.sub_ty(instr.meta)), op2));
        X64MCInstr::with1(div_mnemonic, Operand::Reg(X64Reg::R11.sub_ty(instr.meta)))
    } else {
        X64MCInstr::with1(div_mnemonic, op2)
    };

    sink.extend_from_slice(&[
        X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(X64Reg::Rax.sub_ty(instr.meta)), op1),
        rdx_prep_instr,
        div_instr,
        X64MCInstr::with2(Mnemonic::Mov, out, Operand::Reg(X64Reg::Rax.sub_ty(instr.meta))),
    ]);

    if !out_is_rdx {
        sink.push(X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(X64Reg::R15), Operand::Reg(X64Reg::Rdx))); // restore rdx
    }
}

pub(crate) fn x64_lower_rem(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr) {
        let op1 = instr.operands.get(0).expect("expected a first operand");
    let op2 = instr.operands.get(1).expect("expected a second operand");
    let out = instr.out.expect("expected a output operand");

    let mut op1 = (*op1).into();
    let mut op2 = (*op2).into();
    let out: Operand = out.into();

    let mnemonic = if instr.meta.signed() {
        Mnemonic::Idiv
    } else {
        Mnemonic::Div
    };

    if out != Operand::Reg(X64Reg::Rdx.sub_ty(instr.meta)) {
        sink.push( X64MCInstr::with1(Mnemonic::Push, Operand::Reg(X64Reg::Rdx)).into() );
    }

    let mut pop_rcx = false;
    let mut pop_rsi = false;

    if let Operand::Reg(reg) = op1 {
        if X64Reg::Rdx == reg.sub_ty(TypeMetadata::i64) {
            sink.push( X64MCInstr::with1(Mnemonic::Push, Operand::Reg(X64Reg::Rcx)));
            sink.push( X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(X64Reg::Rcx.sub_ty(instr.meta)), op1) );
            op1 = Operand::Reg( X64Reg::Rcx.sub_ty(instr.meta) );

            pop_rcx = true;
        }
    } 
    if let Operand::Reg(reg) = op2 {
        if X64Reg::Rdx == reg.sub_ty(TypeMetadata::i64) {
            sink.push( X64MCInstr::with1(Mnemonic::Push, Operand::Reg(X64Reg::Rsi)) );
            sink.push( X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(X64Reg::Rsi.sub_ty(instr.meta)), op2) );
            op2 = Operand::Reg( X64Reg::Rsi.sub_ty(instr.meta) );

            pop_rsi = true;
        }
    }

    sink.push( X64MCInstr::with2(Mnemonic::Xor, Operand::Reg(X64Reg::Rdx), Operand::Reg(X64Reg::Rdx)) );

    let rax = || Operand::Reg(X64Reg::Rax.sub_ty(instr.meta));

    sink.push(X64MCInstr::with2(Mnemonic::Mov, rax(), op1.clone()));
    
    // mul/imul only accept r/m
    if let Operand::Imm(_) = op2 {
        sink.push(X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(X64Reg::R11.sub_ty(instr.meta)), op2.clone()));
        sink.push(X64MCInstr::with1(mnemonic, Operand::Reg(X64Reg::R11.sub_ty(instr.meta))));
    } else if let Operand::Mem(_) = op2 {
        sink.push(X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(X64Reg::R11.sub_ty(instr.meta)), op2.clone()));
        sink.push(X64MCInstr::with1(mnemonic, Operand::Reg(X64Reg::R11.sub_ty(instr.meta))));
    } else {
        sink.push(X64MCInstr::with1(mnemonic, op2.clone()));
    }

    if pop_rsi {
        sink.push( X64MCInstr::with1(Mnemonic::Pop, Operand::Reg(X64Reg::Rsi)));
    }
    
    if pop_rcx {
        sink.push( X64MCInstr::with1(Mnemonic::Pop, Operand::Reg(X64Reg::Rcx)));
    }

    sink.push(X64MCInstr::with2(Mnemonic::Mov, out.to_owned(), Operand::Reg(X64Reg::Rdx.sub_ty(instr.meta))));

    if out != Operand::Reg(X64Reg::Rdx.sub_ty(instr.meta)) {
        sink.push( X64MCInstr::with1(Mnemonic::Pop, Operand::Reg(X64Reg::Rdx)).into() );
    }
}

pub(crate) fn x64_lower_shl(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr) {
    let out = instr.out.expect("expected output for valid shl instruction");
    let op1 = instr.operands.get(0).expect("expected 2 operands for valid shl instruction");
    let op2 = instr.operands.get(1).expect("expected 2 operands for valid shl instruction");

    let out: Operand = out.into();

    let op1: Operand = (*op1).into();
    let op2: Operand = (*op2).into();

    if let Operand::Reg(reg) = out {
        if X64Reg::Rcx != reg.sub_ty(TypeMetadata::i64) {
            sink.push( X64MCInstr::with1(Mnemonic::Push, Operand::Reg(X64Reg::Rcx)));
        }
    }

    let mut else_clause = true;

    if let Operand::Reg(reg) = op1 {
        if reg.sub64() == X64Reg::Rcx {
            sink.push( X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(X64Reg::Rax.sub_ty(instr.meta)), op1.clone()));
            sink.push( X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(X64Reg::Rcx.sub_ty(instr.meta)), op2.clone()));
            else_clause = false;
        }
    } 
    
    if else_clause {
        sink.push( X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(X64Reg::Rcx.sub_ty(instr.meta)), op2));
        sink.push( X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(X64Reg::Rax.sub_ty(instr.meta)), op1));
    }

    sink.push( X64MCInstr::with2(Mnemonic::Sal, Operand::Reg(X64Reg::Rax.sub_ty(instr.meta)), Operand::Reg(X64Reg::Cl)));
    sink.push( X64MCInstr::with2(Mnemonic::Mov, out.clone(), Operand::Reg(X64Reg::Rax.sub_ty(instr.meta))));

    if let Operand::Reg(reg) = out {
        if X64Reg::Rcx != reg.sub_ty(TypeMetadata::i64) {
            sink.push( X64MCInstr::with1(Mnemonic::Pop, Operand::Reg(X64Reg::Rcx)));
        }
    }
}

pub(crate) fn x64_lower_shr(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr) {
    let out = instr.out.expect("expected output for valid shl instruction");
    let op1 = instr.operands.get(0).expect("expected 2 operands for valid shl instruction");
    let op2 = instr.operands.get(1).expect("expected 2 operands for valid shl instruction");

    let out: Operand = out.into();

    let op1: Operand = (*op1).into();
    let op2: Operand = (*op2).into();

    if let Operand::Reg(reg) = out {
        if X64Reg::Rcx != reg.sub_ty(TypeMetadata::i64) {
            sink.push( X64MCInstr::with1(Mnemonic::Push, Operand::Reg(X64Reg::Rcx)));
        }
    }

    let mut else_clause = true;

    let mne = match instr.meta.signed() {
        true => Mnemonic::Sar,
        false => Mnemonic::Shr,
    };

    if let Operand::Reg(reg) = op1 {
        if reg.sub64() == X64Reg::Rcx {
            sink.push( X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(X64Reg::Rax.sub_ty(instr.meta)), op1.clone()));
            sink.push( X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(X64Reg::Rcx.sub_ty(instr.meta)), op2.clone()));
            else_clause = false;
        }
    } 
    
    if else_clause {
        sink.push( X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(X64Reg::Rcx.sub_ty(instr.meta)), op2));
        sink.push( X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(X64Reg::Rax.sub_ty(instr.meta)), op1));
    }

    sink.push( X64MCInstr::with2(mne, Operand::Reg(X64Reg::Rax.sub_ty(instr.meta)), Operand::Reg(X64Reg::Cl)));
    sink.push( X64MCInstr::with2(Mnemonic::Mov, out.clone(), Operand::Reg(X64Reg::Rax.sub_ty(instr.meta))));

    if let Operand::Reg(reg) = out {
        if X64Reg::Rcx != reg.sub_ty(TypeMetadata::i64) {
            sink.push( X64MCInstr::with1(Mnemonic::Pop, Operand::Reg(X64Reg::Rcx)));
        }
    }
}
