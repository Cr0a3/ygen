use crate::CodeGen::{MachineInstr, MachineMnemonic};
use crate::IR::{Block, Var, ir::Neg};

use super::CompilationHelper;

impl CompilationHelper {
    #[allow(missing_docs)]
    pub fn compile_neg(&mut self, node: &Neg<Var, Var>, mc_sink: &mut Vec<MachineInstr>, _: &Block, _: &mut crate::prelude::Module) {
        let op = *self.vars.get(&node.inner1.name).expect("expected valid variable");
        let out = *self.vars.get(&node.inner2.name).expect("expected valid variable");

        let mut instr = MachineInstr::new(MachineMnemonic::Neg);

        instr.set_out( out.into() );
        instr.add_operand( op.into() );

        instr.meta = node.inner2.ty;

        mc_sink.push( instr );

        if let Some(phi_loc) = self.phi_vars.get(&node.inner1.name) {
            let mut instr = MachineInstr::new(MachineMnemonic::Move);
            instr.set_out((*phi_loc).into());
            instr.add_operand(out.into());

            instr.meta = node.inner2.ty;
            
            mc_sink.push(instr);
        }
        
    }
}