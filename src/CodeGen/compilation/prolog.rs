use crate::CodeGen::{MachineInstr, MachineMnemonic, MachineOperand};

use super::CompilationHelper;

impl CompilationHelper {
    #[allow(missing_docs)]
    pub fn compile_prolog(&mut self, sink: &mut Vec<MachineInstr>) {
        if self.alloc.stack_off - self.call.align(self.arch) > self.call.shadow(self.arch) {
            let mut instr = MachineInstr::new( MachineMnemonic::Prolog );
            instr.add_operand( MachineOperand::Imm(self.alloc.stack_off as f64) );

            sink.push( instr );
        }
    }

    #[allow(missing_docs)]
    pub fn compile_epilog(&mut self, sink: &mut Vec<MachineInstr>) {
        if self.alloc.stack_off - self.call.align(self.arch)  > self.call.shadow(self.arch) {
            let mut instr = MachineInstr::new( MachineMnemonic::Epilog );
            instr.add_operand( MachineOperand::Imm(self.alloc.stack_off as f64) );

            sink.push( instr );
        }
    }
}