use crate::CodeGen::{MachineInstr, MachineMnemonic};

use super::CompilationHelper;

impl CompilationHelper {
    #[allow(missing_docs)]
pub fn compile_prolog(&mut self, sink: &mut Vec<MachineInstr>) {
        let instr = MachineInstr::new( MachineMnemonic::Prolog );
        sink.push( instr );
    }

    #[allow(missing_docs)]
    pub fn compile_epilog(&mut self, sink: &mut Vec<MachineInstr>) {
        let instr = MachineInstr::new( MachineMnemonic::Epilog );
        sink.push( instr );
    }
}