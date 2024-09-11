use crate::prelude::{Block, Br, BrCond, Ir};
use crate::CodeGen::{MachineInstr, MachineMnemonic, MachineOperand};
use crate::IR::Var;

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

    #[allow(missing_docs)]
    pub fn compile_br_cond(&mut self, node: &BrCond<Var, Block, Block>, mc_sink: &mut Vec<MachineInstr>, block: &Block) {
        let boxed: Box<dyn Ir> = Box::new(node.clone());

        let iftrue = node.inner2.name.to_owned();
        let iffalse = node.inner3.name.to_owned();

        if !block.isVarUsedAfterNode(&boxed, &node.inner1) {
            self.free(&node.inner1);
        }

        let src = *self.vars.get(&node.inner1.name).expect("expected valid variable");

        let src = match src {
            super::VarLocation::Reg(reg) => MachineOperand::Reg(reg),
        };

        let mut cmp = MachineInstr::new(
            MachineMnemonic::Compare(iftrue, iffalse)
        );
        cmp.add_operand(src);
        cmp.add_operand(MachineOperand::Imm(1));
        
        mc_sink.push( cmp );
    }
}