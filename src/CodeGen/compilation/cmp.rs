use crate::{prelude::Cmp, CodeGen::{MachineInstr, MachineMnemonic, MachineOperand}, IR::Block};

use super::CompilationHelper;

impl CompilationHelper {
    #[allow(missing_docs)]
    pub fn compile_cmp(&mut self, node: &Cmp, mc_sink: &mut Vec<MachineInstr>, _: &Block) {
        let ls = self.vars.get(&node.ls.name).expect("expected valid variable");
        let rs = self.vars.get(&node.rs.name).expect("expected valid variable");

        let ls = match ls {
            super::VarLocation::Reg(reg) => MachineOperand::Reg(*reg),
        };

        let rs = match rs {
            super::VarLocation::Reg(reg) => MachineOperand::Reg(*reg),
        };

        let out = match self.alloc(&node.out) {
            super::VarLocation::Reg(reg) => MachineOperand::Reg(reg),
        };

        let mut reset = MachineInstr::new(MachineMnemonic::Move);
        reset.set_out(out);
        reset.add_operand(MachineOperand::Imm(0));
        mc_sink.push( reset );

        let mut cmp = MachineInstr::new(MachineMnemonic::Compare(node.mode) );
        
        cmp.add_operand(ls);
        cmp.add_operand(rs);

        cmp.set_out(out);

        mc_sink.push( cmp );
    }
}