use crate::Optimizations::Optimize;

use super::instr::*;

impl Optimize<X64MCInstr> for Vec<X64MCInstr> {
    fn optimize(&mut self) -> Vec<X64MCInstr> {
        let mut out: Vec<X64MCInstr> = vec![];

        let mut optimize = true;

        for instr in self.iter() {
            if instr.mnemonic == Mnemonic::StartOptimization {
                optimize = true;
            } else if instr.mnemonic == Mnemonic::EndOptimization {
                optimize = false;
            }

            if !optimize {
                out.push(instr.to_owned());
                continue;
            }


            // RUN OPTIMIZATIONS HERE
        }

        out
    }
}