use crate::{prelude::Cmp, CodeGen::{MachineInstr, MachineMnemonic, MachineOperand}, IR::{Block, TypeMetadata}};

use super::CompilationHelper;

impl CompilationHelper {
    #[allow(missing_docs)]
    pub fn compile_cmp(&mut self, node: &Cmp, mc_sink: &mut Vec<MachineInstr>, _: &Block) {
        let ls = *self.vars.get(&node.ls.name).expect("expected valid variable");
        let rs = *self.vars.get(&node.rs.name).expect("expected valid variable");

        let ls = match ls {
            super::VarLocation::Reg(reg) => MachineOperand::Reg(reg),
            super::VarLocation::Mem(stack) => MachineOperand::Stack(stack),
        };

        let rs = match rs {
            super::VarLocation::Reg(reg) => MachineOperand::Reg(reg),
            super::VarLocation::Mem(stack) => MachineOperand::Stack(stack),
        };

        let out = match *self.vars.get(&node.out.name).unwrap() {
            super::VarLocation::Reg(reg) => MachineOperand::Reg(reg),
            super::VarLocation::Mem(stack) => MachineOperand::Stack(stack),
        };

        let mut cmp = MachineInstr::new(MachineMnemonic::Compare(node.mode) );
        
        cmp.add_operand(ls);
        cmp.add_operand(rs);

        cmp.set_out(out);

        cmp.meta = TypeMetadata::u8;

        mc_sink.push( cmp );

        if let Some(phi_loc) = self.alloc.phi_vars.get(&node.out.name) {
            let mut instr = MachineInstr::new(MachineMnemonic::Move);
            instr.set_out((*phi_loc).into());
            instr.add_operand(out.into());

            instr.meta = TypeMetadata::u8;

            mc_sink.push(instr);
        }
    }
}