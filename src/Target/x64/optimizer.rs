use crate::Target::x64::{instr::Operand, X64Reg};

use super::instr::{MemOp, Mnemonic, X64MCInstr};

fn x64_reg_extract(from: Option<Operand>) -> X64Reg {
    if let Some(Operand::Reg(reg)) = from {
        reg
    } else {
        panic!("expected the option to be an register but it wasn't: {:?}", from)
    }
}

/// Performs optimizations on the input assembly strings using the given amount of iterations
pub fn X64AssemblyOptimization(input: Vec<X64MCInstr>, iterations: usize) -> Vec<X64MCInstr> {
    let mut transformed = input;

    for _ in 0..iterations {
        X64AsmOpt(&mut transformed);
    }

    transformed
}

/// Performs optimizations on the input assembly strings using the given amount of iterations
pub(crate) fn X64AsmOpt(instrs: &mut Vec<X64MCInstr>) {
    let mut index = 0;

    while index < instrs.len() {
        if let Some(mut opt_instr) = X64MergeInstrs(&instrs[index..], 3) {
            instrs.drain(index..index + 3);

            opt_instr.reverse();

            for instr in opt_instr {
                instrs.insert(index, instr);
            }
        } else if let Some(mut opt_instr) = X64MergeInstrs(&instrs[index..], 2) {
            instrs.drain(index..index + 2);

            opt_instr.reverse();

            for instr in opt_instr {
                instrs.insert(index, instr);
            }
        } else {
            index += 1;
        }
    }
}

fn X64MergeInstrs(instrs: &[X64MCInstr], n: usize) -> Option<Vec<X64MCInstr>> {
    if instrs.len() < n {
        return None;
    }

    match n {
        3 => X64Merge3Instrs(instrs),
        2 => X64Merge2Instrs(instrs),
        _ => None,
    }
}

fn X64Merge2Instrs(instrs: &[X64MCInstr]) -> Option<Vec<X64MCInstr>> {
    if instrs.len() < 2 {
        return None;
    }

    let instr0 = instrs.get(0).unwrap();
    let instr1 = instrs.get(1).unwrap();

    if let Some(merged) = X64MergeMove(instr0, instr1) { return Some(merged); }
    // TODO: Add more instr combines here

    None
}

fn X64MergeMove(instr0: &X64MCInstr, instr1: &X64MCInstr) -> Option<Vec<X64MCInstr>> {
    if !(instr0.is_mov1(&Operand::Reg(X64Reg::Rax)) || 
       instr0.is_mov1(&Operand::Reg(X64Reg::Eax)) || 
       instr0.is_mov1(&Operand::Reg(X64Reg::Ax))  || 
       instr0.is_mov1(&Operand::Reg(X64Reg::Al)))   {
        return None;
    }

    if !instr0.is_op2_reg() {
        return None;
    }

    if !instr1.is_mov() {
        return None;
    }

    if !(instr1.op2.clone() == Some(Operand::Reg(X64Reg::Rax)) || 
        instr1.op2.clone() == Some(Operand::Reg(X64Reg::Eax)) || 
        instr1.op2.clone() == Some(Operand::Reg(X64Reg::Ax))  || 
        instr1.op2.clone() == Some(Operand::Reg(X64Reg::Al)))   {
        return None;
    }

    Some(vec![X64MCInstr {
        mnemonic: Mnemonic::Mov,
        op1: instr1.op1.clone(),
        op2: instr0.op2.clone(),
        op3: None,
        far: false,
    }])
}

fn X64Merge3Instrs(instrs: &[X64MCInstr]) -> Option<Vec<X64MCInstr>> {
    if instrs.len() < 3 {
        return None;
    }

    let instr0 = instrs.get(0).unwrap();
    let instr1 = instrs.get(1).unwrap();
    let instr2 = instrs.get(2).unwrap();

    if let Some(add) = X64MergeAdd(instr0, instr1, instr2) { return Some(add); }
    if let Some(brcond) = X64MergeBrCond(instr0, instr1, instr2) { return Some(brcond); }
    // TODO: Add more instr combines here

    None
}

fn X64MergeAdd(instr0: &X64MCInstr, instr1: &X64MCInstr, instr2: &X64MCInstr) -> Option<Vec<X64MCInstr>> {
    if !(instr0.is_mov1(&Operand::Reg(X64Reg::Rax)) || 
       instr0.is_mov1(&Operand::Reg(X64Reg::Eax)) || 
       instr0.is_mov1(&Operand::Reg(X64Reg::Ax))  || 
       instr0.is_mov1(&Operand::Reg(X64Reg::Al)))   {
        return None;
    }

    if !(instr1.is_add1(&Operand::Reg(X64Reg::Rax)) || 
       instr1.is_add1(&Operand::Reg(X64Reg::Eax)) || 
       instr1.is_add1(&Operand::Reg(X64Reg::Ax))  || 
       instr1.is_add1(&Operand::Reg(X64Reg::Al)))   {
        return None;
    }    

    if !instr2.is_mov() {
        return None;
    }

    if !(instr2.op2.clone() == Some(Operand::Reg(X64Reg::Rax)) || 
        instr2.op2.clone() == Some(Operand::Reg(X64Reg::Eax)) || 
        instr2.op2.clone() == Some(Operand::Reg(X64Reg::Ax))  || 
        instr2.op2.clone() == Some(Operand::Reg(X64Reg::Al)))   {
        return None;
    }

    if !(instr0.is_op2_reg() && instr1.is_op2_reg() ||
       instr0.is_op2_imm() && instr1.is_op2_imm() ||
       instr0.is_op2_reg() && instr1.is_op2_imm()) {
        return None;
       }

    let mut out = instr2.op1.clone();
    let mut ls = instr0.op2.clone();
    let mut rs = instr1.op2.clone();

    if let Some(Operand::Reg(reg)) = out {
        if !(reg.is_gr32() || reg.is_gr64()) {
            out = Some(Operand::Reg(reg.sub16()))
        }
    }

    if let Some(Operand::Reg(reg)) = ls {
        if !(reg.is_gr32() || reg.is_gr64()) {
            ls = Some(Operand::Reg(reg.sub64()))
        }
    }

    if let Some(Operand::Reg(reg)) = rs {
        if !(reg.is_gr32() || reg.is_gr64()) {
            rs = Some(Operand::Reg(reg.sub64()))
        }
    }

    if instr0.is_op2_reg() && instr1.is_op2_reg() {
        Some(vec![X64MCInstr {
            mnemonic: Mnemonic::Lea,
            op1: out,
            op2: Some(Operand::Mem(MemOp {
                base: Some(x64_reg_extract(rs)),
                index: Some(x64_reg_extract(ls)),
                scale: 1,
                displ: 0,
                rip: false,
            })),
            op3: None,
            far: false,
        }])
    } else if instr0.is_op2_imm() && instr1.is_op2_reg() {
        let imm = if let Some(Operand::Imm(imm)) = instr0.op1 { imm } else { unreachable!() };

        Some(vec![X64MCInstr {
            mnemonic: Mnemonic::Lea,
            op1: out,
            op2: Some(Operand::Mem(MemOp {
                base: Some(x64_reg_extract(rs)),
                index: None,
                scale: 1,
                displ: imm as isize,
                rip: false,
            })),
            op3: None,
            far: false,
        }])
    } else if instr0.is_op2_reg() && instr1.is_op2_imm() {
        let imm = if let Some(Operand::Imm(imm)) = instr1.op2 { imm } else { unreachable!() };

        Some(vec![X64MCInstr {
            mnemonic: Mnemonic::Lea,
            op1: out,
            op2: Some(Operand::Mem(MemOp {
                base: Some(x64_reg_extract(ls)),
                index: None,
                scale: 1,
                displ: imm as isize,
                rip: false,
            })),
            op3: None,
            far: false,
        }])
    } else { None }
}

fn X64MergeBrCond(instr0: &X64MCInstr, instr1: &X64MCInstr, instr2: &X64MCInstr) -> Option<Vec<X64MCInstr>> {
    // Checks for this pattern:
    // setcc var
    // cmp var, 0
    // jne label
    // And merges it into:
    // jne label

    if !(instr0.is_sete() || 
         instr0.is_setne() || 
         instr0.is_setg()  || 
         instr0.is_setl()  || 
         instr0.is_setge() || 
         instr0.is_setle()) {
        return None;
    }

    if !instr1.is_cmp() {
        return None;
    }

    if !instr2.is_jne() {
        return None;
    }

    let jmp_mne = match instr0.mnemonic {
        Mnemonic::Sete => Mnemonic::Je,
        Mnemonic::Setne => Mnemonic::Jne,
        Mnemonic::Setg => Mnemonic::Jg,
        Mnemonic::Setl => Mnemonic::Jl,
        Mnemonic::Setge => Mnemonic::Jge,
        Mnemonic::Setle => Mnemonic::Jle,
        _ => unreachable!()
    };

    Some(vec![instr0.to_owned(), X64MCInstr {
        mnemonic: jmp_mne,
        op1: instr2.op1.to_owned(),
        op2: None,
        op3: None,
        far: false,
    }])
}