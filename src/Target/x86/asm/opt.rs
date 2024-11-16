use super::*;

/// Basic x86 assembly optimizations
pub struct X86BasicOpt;

impl X86BasicOpt {
    /// Optimizes the given input
    pub fn opt(input: &mut Vec<X64Instr>) {
        let mut index = 0;
        for instr in input.clone() {
            // mov x, x -> nothing
            if instr.mnemonic == X64Mnemonic::Mov && instr.op1 == instr.op2 {
                input.remove(index);
            }

            index += 1;
        }
    }
}