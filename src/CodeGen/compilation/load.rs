use crate::CodeGen::{MachineInstr, MachineMnemonic};
use crate::IR::{Block, ir::Load};

use super::CompilationHelper;

impl CompilationHelper {
    #[allow(missing_docs)]
    pub fn compile_load(&mut self, node: &Load, mc_sink: &mut Vec<MachineInstr>, _: &Block, _: &mut crate::prelude::Module) {
        let ptr = node.inner3.into_mi(self);

        let out = *self.vars.get(&node.inner1.name).unwrap();
        let out = out.into();

        let mut instr = MachineInstr::new(MachineMnemonic::Load);

        instr.set_out( out );
        instr.add_operand(ptr);

        instr.meta = node.inner2;

        mc_sink.push( instr );

        if let Some(phi_loc) = self.phi_vars.get(&node.inner1.name) {
            let mut instr = MachineInstr::new(MachineMnemonic::Move);
            instr.set_out((*phi_loc).into());
            instr.add_operand(out.into());
            mc_sink.push(instr);
        }
        
    }
}