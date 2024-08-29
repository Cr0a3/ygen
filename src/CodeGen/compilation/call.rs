use crate::prelude::Call;
use crate::IR::{Block, Function, Var};
use super::CompilationHelper;
use crate::CodeGen::MachineInstr;

impl CompilationHelper {
    #[allow(missing_docs)]
    pub fn compile_call(&mut self, node: &Call<Function, Vec<Var>, Var>, mc_sink: &mut Vec<MachineInstr>, block: &Block) {
        todo!()
    }
}