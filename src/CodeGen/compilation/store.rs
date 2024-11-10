use crate::prelude::{Store, Block};
use crate::CodeGen::{MachineInstr, MachineMnemonic};

use super::CompilationHelper;

impl CompilationHelper {
    #[allow(missing_docs)]
    pub fn compile_store(&mut self, node: &Store, mc_sink: &mut Vec<MachineInstr>, _: &Block, _: &mut crate::prelude::Module) {
        let in_var = node.inner2.into_mi(self);

        let ptr = self.vars.get(&node.inner1.name).expect("expected valid variable");
        let ptr = (*ptr).into();

        let mut instr = MachineInstr::new(MachineMnemonic::Store);

        instr.set_out( ptr );
        instr.add_operand(in_var);

        instr.meta = node.inner2.get_ty();

        mc_sink.push( instr );
    }
}