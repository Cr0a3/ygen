use crate::prelude::{Block, Br};
use crate::CodeGen::{MachineInstr, MachineMnemonic};

use super::CompilationHelper;

impl CompilationHelper {
    #[allow(missing_docs)]
    pub fn compile_br(&mut self, node: &Br<Box<Block>>, mc_sink: &mut Vec<MachineInstr>, _: &Block) {
        let block = node.inner1.name.to_owned();

        let instr = MachineInstr::new(
            MachineMnemonic::Br(block)
        );

        mc_sink.push( instr );
    }
}