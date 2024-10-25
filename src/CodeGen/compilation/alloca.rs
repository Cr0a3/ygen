use crate::CodeGen::{MachineInstr, MachineMnemonic};
use crate::IR::{Block, TypeMetadata, Var, ir::Alloca};

use super::CompilationHelper;

impl CompilationHelper {
    #[allow(missing_docs)]
    pub fn compile_alloca(&mut self, node: &Alloca<Var, TypeMetadata>, mc_sink: &mut Vec<MachineInstr>, _: &Block, _: &mut crate::prelude::Module) {
        let out = *self.vars.get(&node.inner1.name).unwrap();

        if let Some(phi_loc) = self.alloc.phi_vars.get(&node.inner1.name) {
            let mut instr = MachineInstr::new(MachineMnemonic::AdrMove);
            instr.set_out((*phi_loc).into());
            instr.add_operand(out.into());

            instr.meta = TypeMetadata::ptr;

            mc_sink.push(instr);
        }
    }
}