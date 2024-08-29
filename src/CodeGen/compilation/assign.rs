use crate::prelude::ConstAssign;
use crate::IR::{Block, Const, Type, Var};
use super::CompilationHelper;
use crate::CodeGen::MachineInstr;

impl CompilationHelper {
    #[allow(missing_docs)]
    pub fn compile_assign_var_type(&mut self, node: &ConstAssign<Var, Type>, mc_sink: &mut Vec<MachineInstr>, block: &Block) {
        todo!()
    }

    #[allow(missing_docs)]
    pub fn compile_assign_var_var(&mut self, node: &ConstAssign<Var, Var>, mc_sink: &mut Vec<MachineInstr>, block: &Block) {
        todo!()
    }
    
    #[allow(missing_docs)]
    pub fn compile_assign_var_const(&mut self, node: &ConstAssign<Var, Const>, mc_sink: &mut Vec<MachineInstr>, block: &Block) {
        todo!()
    }
}