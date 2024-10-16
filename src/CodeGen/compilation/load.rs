use crate::CodeGen::{MachineInstr, MachineMnemonic, MachineOperand};
use crate::IR::{Block, TypeMetadata, Var, ir::Load};

use super::CompilationHelper;

impl CompilationHelper {
    #[allow(missing_docs)]
    pub fn compile_load(&mut self, node: &Load<Var, Var, TypeMetadata>, mc_sink: &mut Vec<MachineInstr>, _: &Block, _: &mut crate::prelude::Module) {
        let ptr = *self.vars.get(&node.inner2.name).expect("expected valid variable");
        let ptr = match ptr {
            super::VarLocation::Reg(reg) => MachineOperand::Reg(reg),
            super::VarLocation::Mem(mem) => MachineOperand::Stack(mem),
        };

        let out = *self.vars.get(&node.inner1.name).unwrap();
        let out = match out {
            super::VarLocation::Reg(reg) => MachineOperand::Reg(reg),
            super::VarLocation::Mem(mem) => MachineOperand::Stack(mem),
        };

        let mut instr = MachineInstr::new(MachineMnemonic::Load);

        instr.set_out( out );
        instr.add_operand(ptr);

        instr.meta = node.inner3;

        instr.meta = node.inner3;

        mc_sink.push( instr );

        if let Some(phi_loc) = self.alloc.phi_vars.get(&node.inner1.name) {
            let mut instr = MachineInstr::new(MachineMnemonic::Move);
            instr.set_out((*phi_loc).into());
            instr.add_operand(out.into());
            mc_sink.push(instr);
        }
        
    }
}