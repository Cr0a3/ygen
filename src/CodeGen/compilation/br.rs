use crate::prelude::{Block, Br, BrCond};
use crate::CodeGen::{MachineInstr, MachineMnemonic, MachineOperand};

use super::CompilationHelper;

impl CompilationHelper {
    #[allow(missing_docs)]
    pub fn compile_br(&mut self, node: &Br, mc_sink: &mut Vec<MachineInstr>, _: &Block, _: &mut crate::prelude::Module) {
        let block = node.inner1.name.to_owned();

        let instr = MachineInstr::new(
            MachineMnemonic::Br(block)
        );

        mc_sink.push( instr );
    }

    #[allow(missing_docs)]
    pub fn compile_br_cond(&mut self, node: &BrCond, mc_sink: &mut Vec<MachineInstr>, _: &Block, _: &mut crate::prelude::Module) {
        // COMPILES TO:
        // if node.inner1 == 0 {
        //     goto node.inner2;
        // } else {
        //     goto node.inner3;
        //}
        
        let iftrue = node.inner2.name.to_owned();
        let iffalse = node.inner3.name.to_owned();

        let src = *self.vars.get(&node.inner1.name).expect("expected valid variable");
        let src = src.into();

        let mut cmp = MachineInstr::new(
            MachineMnemonic::BrCond(iftrue, iffalse)
        );
        cmp.add_operand(src);
        cmp.add_operand(MachineOperand::Imm(0.0));
        
        cmp.meta = node.inner1.ty;

        mc_sink.push( cmp );
    }
}