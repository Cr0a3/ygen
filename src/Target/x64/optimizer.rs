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
        if let Some(opt_instr) = X64MergeInstrs(&instrs[index..], 3) {
            instrs[index] = opt_instr;
            instrs.drain(index + 1..index + 3);
        } /*else if let Some(opt_instr) = X64MergeInstrs(&instrs[index..], 2) {
            instrs[index] = opt_instr;
            instrs.drain(index + 1..index + 2);
        } */else {
            index += 1;
        }
    }
}

fn X64MergeInstrs(instrs: &[X64MCInstr], n: usize) -> Option<X64MCInstr> {
    if instrs.len() < n {
        return None;
    }

    match n {
        3 => X64Merge3Instrs(instrs),
        2 => X64Merge2Instrs(instrs),
        _ => None,
    }
}

fn X64Merge2Instrs(instrs: &[X64MCInstr]) -> Option<X64MCInstr> {
    if instrs.len() < 2 {
        return None;
    }

    let instr0 = instrs.get(0).unwrap();
    let instr1 = instrs.get(1).unwrap();

    if instr0.op1 == instr1.op2 && 
       (instr0.is_op1_reg() || instr0.is_op2_reg()) && 
       instr1.is_mov() {
        let mut new = instr0.clone();
        new.op1 = instr1.op2.clone();

        Some(new)
    } else {
        None
    }
}

fn X64Merge3Instrs(instrs: &[X64MCInstr]) -> Option<X64MCInstr> {
    if instrs.len() < 3 {
        return None;
    }

    let instr0 = instrs.get(0).unwrap();
    let instr1 = instrs.get(1).unwrap();
    let instr2 = instrs.get(2).unwrap();

    if let Some(add) = X64MergeAdd(instr0, instr1, instr2) { return Some(add); }
    // TODO: Add more instr combines here

    None
}

fn X64MergeAdd(instr0: &X64MCInstr, instr1: &X64MCInstr, instr2: &X64MCInstr) -> Option<X64MCInstr> {
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

    let out = instr2.op1.clone();
    let ls = instr0.op2.clone();
    let rs = instr1.op2.clone();

    Some(X64MCInstr {
        mnemonic: Mnemonic::Lea,
        op1: out,
        op2: Some(Operand::Mem(MemOp {
            base: Some(x64_reg_extract(rs)),
            index: Some(x64_reg_extract(ls)),
            scale: 1,
            displ: 0,
            rip: false,
        })),
        far: false,
    })
}