use crate::CodeGen::{MachineInstr, MachineMnemonic};
use crate::IR::{Block, ir::Neg};

use super::CompilationHelper;

impl CompilationHelper {
    #[allow(missing_docs)]
    pub fn compile_neg(&mut self, node: &Neg, mc_sink: &mut Vec<MachineInstr>, _: &Block, _: &mut crate::prelude::Module) {
        let op = node.inner1.into_mi(self);
        let out = *self.vars.get(&node.inner2.name).expect("expected valid variable");

        let mut instr = MachineInstr::new(MachineMnemonic::Neg);

        instr.set_out( out.into() );
        instr.add_operand( op.into() );

        instr.meta = node.inner2.ty;

        mc_sink.push( instr );

        if let Some(phi_loc) = self.phi_vars.get(&node.inner2.name) {
            let mut instr = MachineInstr::new(MachineMnemonic::Move);
            instr.set_out((*phi_loc).into());
            instr.add_operand(out.into());

            instr.meta = node.inner2.ty;
            
            mc_sink.push(instr);
        }
        
    }
}