use crate::CodeGen::{MachineInstr, MachineMnemonic, MachineOperand};

use super::CompilationHelper;

impl CompilationHelper {
    #[allow(missing_docs)]
    pub fn compile_prolog(&mut self, sink: &mut Vec<MachineInstr>) {
        if self.stack_off > self.call.shadow(self.arch) {
            let mut instr = MachineInstr::new( MachineMnemonic::Prolog );
            instr.add_operand( MachineOperand::Imm(self.stack_off) );

            sink.push( instr );
        }
    }

    #[allow(missing_docs)]
    pub fn compile_epilog(&mut self, sink: &mut Vec<MachineInstr>) {
        if self.stack_off > self.call.shadow(self.arch) {
            let mut instr = MachineInstr::new( MachineMnemonic::Epilog );
            instr.add_operand( MachineOperand::Imm(self.stack_off) );

            sink.push( instr );
        }
    }
}