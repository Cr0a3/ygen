use crate::Target::x64Reg;

use super::instr::{X64MCInstr, Mnemonic, Operand};

/// used for optimizing
pub trait Optimize<T> {
    /// optimizes self
    fn optimize(&mut self) -> Self;
}

impl Optimize<X64MCInstr> for Vec<X64MCInstr> {
    fn optimize(&mut self) -> Vec<X64MCInstr> {
        let mut out: Vec<X64MCInstr> = vec![];

        let mut optimize = false;

        for instr in self.iter() {
            let mut optimized = false;

            if instr.mnemonic == Mnemonic::StartOptimization {
                optimize = true;
            } else if instr.mnemonic == Mnemonic::EndOptimization {
                optimize = false;
            }

            if !optimize {
                out.push(instr.to_owned());
                continue;
            }

            let last = out.last().cloned();
            if let Some(last) = &last {
                if last.mnemonic == Mnemonic::Mov && instr.mnemonic == Mnemonic::Add {
                    if last.op1 == instr.op1 {
                        if let Some(Operand::Reg(op0)) = &last.op2 {
                            if let Some(Operand::Reg(op2)) = &instr.op2 {
                                if let Some(Operand::Reg(reg)) = &instr.op1 {
                                    out.pop();
                                    out.push(
                                        X64MCInstr::with2(
                                            Mnemonic::Lea, 
                                            Operand::Reg(reg.clone()), 
                                            Operand::Mem(*op0.as_any().downcast_ref::<x64Reg>().unwrap() 
                                                + 
                                                *op2.as_any().downcast_ref::<x64Reg>().unwrap()
                                            ) 
                                        ));
                                    optimized = true;
                                }
                            }
                        }
                    }
                } 
                if instr.op1 == instr.op2 {
                    if instr.mnemonic == Mnemonic::Mov {
                        optimized = true;
                    }
                }
                if instr.op1 == last.op1 && instr.mnemonic == Mnemonic::Mov && last.mnemonic == Mnemonic::Mov {
                    if let Some(Operand::Reg(_)) = instr.op1 {
                        out.pop();
                    }
                }
                if instr.op2 == last.op1 && instr.mnemonic == Mnemonic::Mov && last.mnemonic == Mnemonic::Mov {
                    optimized = true;
                }
                if instr.mnemonic == Mnemonic::Ret && last.mnemonic == Mnemonic::Call {
                    out.pop();
                    out.push(X64MCInstr::with1(Mnemonic::Jmp, instr.op1.clone().expect("call needs to have one op")));
                    optimized = true;
                } 
                if instr.mnemonic == Mnemonic::Ret {
                    out.push(instr.clone());
                }

                if !optimized {
                    if instr.invert_of(last) {
                        out.pop();
                    } else {
                        out.push(instr.to_owned()) 
                    }
                }
            } else { out.push(instr.to_owned()) }
        }

        out
    }
}