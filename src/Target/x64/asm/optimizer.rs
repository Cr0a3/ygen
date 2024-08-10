use crate::Target::x64Reg;

use super::instr::{Instr, Mnemonic, Operand};

/// used for optimizing
pub trait Optimize<T> {
    /// optimizes self
    fn optimize(&mut self) -> Self;
}

impl Optimize<Instr> for Vec<Instr> {
    fn optimize(&mut self) -> Vec<Instr> {
        let mut out: Vec<Instr> = vec![];

        for instr in self.iter() {
            let mut optimized = false;

            

            let last = out.last().cloned();
            if let Some(last) = &last {
                if last.mnemonic == Mnemonic::Mov && instr.mnemonic == Mnemonic::Add {
                    if last.op1 == instr.op1 {
                        if let Some(Operand::Reg(op0)) = &last.op2 {
                            if let Some(Operand::Reg(op2)) = &instr.op2 {
                                if let Some(Operand::Reg(reg)) = &instr.op1 {
                                    out.pop();
                                    out.push(
                                        Instr::with2(
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
                else if instr.op1 == instr.op2 {
                    optimized = true;
                }
                else if instr.uses_mut(&last.op1) {
                    out.pop();
                    optimized = true;
                }
                else if instr.mnemonic == Mnemonic::Ret && last.mnemonic == Mnemonic::Call {
                    out.pop();
                    out.push(Instr::with1(Mnemonic::Jmp, instr.op1.clone().expect("call needs to have only one op")));
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