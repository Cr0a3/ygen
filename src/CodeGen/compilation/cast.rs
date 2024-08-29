use crate::prelude::Cast;
use crate::IR::{Block, TypeMetadata, Var};
use super::CompilationHelper;
use crate::CodeGen::MachineInstr;

impl CompilationHelper {
    pub fn compile_cast(&mut self, node: &Cast<Var, TypeMetadata, Var>, mc_sink: &mut Vec<MachineInstr>, block: &Block) {
        todo!()
    }
}