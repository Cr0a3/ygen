use crate::prelude::{IROperand, Return, Block};
use crate::CodeGen::{MachineInstr, MachineMnemonic};

use super::CompilationHelper;

impl CompilationHelper {
    #[allow(missing_docs)]
    pub fn compile_ret(&mut self, node: &Return<IROperand>, mc_sink: &mut Vec<MachineInstr>, _: &Block, _: &mut crate::prelude::Module) {
        let mut instr = MachineInstr::new(MachineMnemonic::Return);
        instr.add_operand(node.inner1.into_mi(self));
        instr.meta = node.inner1.get_ty();

        mc_sink.push( instr );
    }
}