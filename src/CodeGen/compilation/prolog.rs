use crate::CodeGen::{MachineInstr, MachineMnemonic, MachineOperand};

use super::CompilationHelper;

impl CompilationHelper {
    #[allow(missing_docs)]
    pub fn compile_prolog(&mut self, sink: &mut Vec<MachineInstr>) {
        let mut instr = MachineInstr::new( MachineMnemonic::Prolog );
        instr.add_operand( MachineOperand::Imm(self.alloc.stack_off as f64) );

        for save in &self.alloc.callee_save_registers {
            instr.add_operand( MachineOperand::Reg(*save) );
        }

        sink.push( instr );
    }

    #[allow(missing_docs)]
    pub fn compile_epilog(&mut self, sink: &mut Vec<MachineInstr>) {
        let mut instr = MachineInstr::new( MachineMnemonic::Epilog );
        instr.add_operand( MachineOperand::Imm(self.alloc.stack_off as f64) );

        for save in &self.alloc.callee_save_registers {
            instr.add_operand( MachineOperand::Reg(*save) );
        }

        sink.push( instr );
    }
}