use crate::ydbg;

use super::*;

/// Basic x86 assembly optimizations
pub struct X86BasicOpt;

impl X86BasicOpt {
    /// Optimizes the given input
    pub fn opt(input: &mut Vec<X64Instr>) {
        let mut index = 0;
        for instr in input.clone() {
            let mut removed = false;

            // mov x, x -> nothing
            if matches!(instr.mnemonic, X64Mnemonic::Mov | X64Mnemonic::Movdqa | X64Mnemonic::Movsd | X64Mnemonic::Movss) && instr.op1 == instr.op2 {
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
}