use crate::prelude::{Block, Br, BrCond};
use crate::CodeGen::{MachineInstr, MachineMnemonic, MachineOperand};
use crate::IR::Var;

use super::CompilationHelper;

impl CompilationHelper {
    #[allow(missing_docs)]
    pub fn compile_br(&mut self, node: &Br<crate::IR::BlockId>, mc_sink: &mut Vec<MachineInstr>, _: &Block) {
        let block = node.inner1.name.to_owned();

        let instr = MachineInstr::new(
            MachineMnemonic::Br(block)
        );

        mc_sink.push( instr );
    }

    #[allow(missing_docs)]
    pub fn compile_br_cond(&mut self, node: &BrCond<Var, crate::IR::BlockId, crate::IR::BlockId>, mc_sink: &mut Vec<MachineInstr>, _: &Block) {
        // COMPILES TO:
        // if node.inner1 == 0 {
        //     goto node.inner2;
        // } else {
        //     goto node.inner3;
        //}
        
        let iftrue = node.inner2.name.to_owned();
        let iffalse = node.inner3.name.to_owned();

        let src = *self.vars.get(&node.inner1.name).expect("expected valid variable");

        let src = match src {
            super::VarLocation::Reg(reg) => MachineOperand::Reg(reg),
            super::VarLocation::Mem(stack) => MachineOperand::Stack(stack),
        };

        let mut cmp = MachineInstr::new(
            MachineMnemonic::BrCond(iftrue, iffalse)
        );
        cmp.add_operand(src);
        cmp.add_operand(MachineOperand::Imm(0));
        
        mc_sink.push( cmp );
    }
}