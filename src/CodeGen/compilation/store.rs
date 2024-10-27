use crate::prelude::{Store, Var, Type, Block};
use crate::CodeGen::{MachineInstr, MachineMnemonic, MachineOperand};

use super::CompilationHelper;

impl CompilationHelper {
    #[allow(missing_docs)]
    pub fn compile_store(&mut self, node: &Store<Var, Var>, mc_sink: &mut Vec<MachineInstr>, _: &Block, _: &mut crate::prelude::Module) {
        let in_var = self.vars.get(&node.inner2.name).expect("expected valid variable");
        let in_var = (*in_var).into();

        let ptr = self.vars.get(&node.inner1.name).expect("expected valid variable");
        let ptr = (*ptr).into();

        let mut instr = MachineInstr::new(MachineMnemonic::Store);

        instr.set_out( ptr );
        instr.add_operand(in_var);

        instr.meta = node.inner2.ty;

        mc_sink.push( instr );
    }
    
    #[allow(missing_docs)]
    pub fn compile_store_ty(&mut self, node: &Store<Var, Type>, mc_sink: &mut Vec<MachineInstr>, _: &Block, _: &mut crate::prelude::Module) {
        let ptr = self.vars.get(&node.inner1.name).expect("expected valid variable");
        let ptr = (*ptr).into();

        let mut instr = MachineInstr::new(MachineMnemonic::Store);

        instr.set_out( ptr );
        instr.add_operand(MachineOperand::Imm(node.inner2.val()));

        instr.meta = node.inner2.into();

        mc_sink.push( instr );
    }
}