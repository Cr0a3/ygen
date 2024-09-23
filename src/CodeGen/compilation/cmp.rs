use crate::{prelude::Cmp, CodeGen::{MachineInstr, MachineMnemonic, MachineOperand}, IR::Block};

use super::CompilationHelper;

impl CompilationHelper {
    #[allow(missing_docs)]
    pub fn compile_cmp(&mut self, node: &Cmp, mc_sink: &mut Vec<MachineInstr>, block: &Block) {
        let boxed: Box<dyn crate::prelude::Ir> = Box::new( node.clone() );

        let ls = *self.vars.get(&node.ls.name).expect("expected valid variable");
        let rs = *self.vars.get(&node.rs.name).expect("expected valid variable");
        
        if !block.isVarUsedAfterNode(&boxed, &node.ls) {
            self.free(&node.ls);
        }
        if !block.isVarUsedAfterNode(&boxed, &node.rs) {
            self.free(&node.rs);
        }
        if !block.isVarUsedAfterNode(&boxed, &node.out) {
            return; // dead code elimination
        }

        let ls = match ls {
            super::VarLocation::Reg(reg) => MachineOperand::Reg(reg),
            super::VarLocation::Mem(stack) => MachineOperand::Stack(stack),
        };

        let rs = match rs {
            super::VarLocation::Reg(reg) => MachineOperand::Reg(reg),
            super::VarLocation::Mem(stack) => MachineOperand::Stack(stack),
        };

        let out = match self.alloc(&node.out) {
            super::VarLocation::Reg(reg) => MachineOperand::Reg(reg),
            super::VarLocation::Mem(stack) => MachineOperand::Stack(stack),
        };

        /*let mut reset = MachineInstr::new(MachineMnemonic::Move); // integrated into the cmp machine instr
        reset.set_out(out);
        reset.add_operand(MachineOperand::Imm(0));
        mc_sink.push( reset );*/

        let mut cmp = MachineInstr::new(MachineMnemonic::Compare(node.mode) );
        
        cmp.add_operand(ls);
        cmp.add_operand(rs);

        cmp.set_out(out);

        mc_sink.push( cmp );
    }
}