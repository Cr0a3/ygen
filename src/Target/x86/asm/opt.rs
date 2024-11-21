use std::collections::HashMap;

use crate::ydbg;

use super::*;

/// Basic x86 assembly optimizations
pub struct X86BasicOpt;

impl X86BasicOpt {
    /// Optimizes the given input
    pub fn opt(input: &mut Vec<X64Instr>) {
        let is_mov = |instr: X64Instr| {
            matches!(instr.mnemonic, X64Mnemonic::Mov | X64Mnemonic::Movdqa | X64Mnemonic::Movsd | X64Mnemonic::Movss)
        };

        let mut index = 0;
        X86BasicOpt::inline_regs(input);

        for instr in input.clone() {
            let mut removed = false;

            // mov x, x -> nothing
            if is_mov(instr) && instr.op1 == instr.op2 {
                ydbg!("[X86 OPT] removing uneccesary mov");
                input.remove(index);
                removed = true;
            }

            // lea x, [y + x] -> add x, y
            if instr.mnemonic == X64Mnemonic::Lea {
                if let Some(X64Operand::MemDispl(mem)) = instr.op2 {
                    if instr.op1 == Some(X64Operand::Reg(mem.base.unwrap_or(X64Reg::Xmm15()))) && mem.index.is_some() {
                        ydbg!("[X86 OPT] folding lea into add");
                        input[index] = X64Instr {
                            mnemonic: X64Mnemonic::Add,
                            op1: instr.op1,
                            op2: Some(X64Operand::Reg(mem.index.unwrap())),
                            op3: None,
                        };
                    }
                    if instr.op1 == Some(X64Operand::Reg(mem.index.unwrap_or(X64Reg::Xmm15()))) && mem.base.is_some() {
                        ydbg!("[X86 OPT] folding lea into add");
                        input[index] = X64Instr {
                            mnemonic: X64Mnemonic::Add,
                            op1: instr.op1,
                            op2: Some(X64Operand::Reg(mem.base.unwrap())),
                            op3: None,
                        };
                    }
                }
            }
            
            if !removed {
                index += 1;
            }
        }
    }

    /// Inlines some registers for the given assembly code
    pub fn inline_regs(input: &mut Vec<X64Instr>) {
        let is_mov = |instr: X64Instr| {
            matches!(instr.mnemonic, X64Mnemonic::Mov | X64Mnemonic::Movdqa | X64Mnemonic::Movsd | X64Mnemonic::Movss)
        };

        let inline_op = |op: X64Operand, map: &HashMap<X64Reg, X64Operand>| -> X64Operand {
            if let X64Operand::Reg(reg) = op {
                map.get(&reg).cloned().unwrap_or(op)
            } else { op }
        };

        let mut reg_map = HashMap::new();

        for instr in input {
            if is_mov(instr.to_owned()) {
                if let (Some(X64Operand::Reg(dest)), Some(value)) = (instr.op1, instr.op2) {
                    reg_map.insert(dest, value);
                }
            }

            // inline registers

            if !is_mov(instr.to_owned()) {
                instr.op1 = instr.op1.map(|op| inline_op(op, &reg_map));
                instr.op2 = instr.op2.map(|op| inline_op(op, &reg_map));
                instr.op3 = instr.op3.map(|op| inline_op(op, &reg_map));

                if let Some(X64Operand::Reg(dst)) = instr.op1 {
                    reg_map.remove(&dst);
                }
            }
        }
    }
}