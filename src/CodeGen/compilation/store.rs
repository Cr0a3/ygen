use crate::prelude::{Store, Var, Type, Block};
use crate::CodeGen::{MachineInstr, MachineMnemonic, MachineOperand};

use super::CompilationHelper;

impl CompilationHelper {
    #[allow(missing_docs)]
    pub fn compile_store(&mut self, node: &Store<Var, Var>, mc_sink: &mut Vec<MachineInstr>, _: &Block) {
        let in_var = self.vars.get(&node.inner2.name).expect("expected valid variable");
        let in_var = match in_var {
            super::VarLocation::Reg(reg) => MachineOperand::Reg(*reg),
            super::VarLocation::Mem(mem) => MachineOperand::Stack(*mem),
        };

        let ptr = self.vars.get(&node.inner1.name).expect("expected valid variable");
        let ptr = match ptr {
            super::VarLocation::Reg(reg) => MachineOperand::Reg(*reg),
            super::VarLocation::Mem(mem) => MachineOperand::Stack(*mem),
        };

        let mut instr = MachineInstr::new(MachineMnemonic::Store);

        instr.set_out( ptr );
        instr.add_operand(in_var);

        mc_sink.push( instr );
    }
    
    #[allow(missing_docs)]
    pub fn compile_store_ty(&mut self, node: &Store<Var, Type>, mc_sink: &mut Vec<MachineInstr>, _: &Block) {
        let ptr = self.vars.get(&node.inner1.name).expect("expected valid variable");
        let ptr = match ptr {
            super::VarLocation::Reg(reg) => MachineOperand::Reg(*reg),
            super::VarLocation::Mem(mem) => MachineOperand::Stack(*mem),
        };

        let mut instr = MachineInstr::new(MachineMnemonic::Store);

        instr.set_out( ptr );
        instr.add_operand(MachineOperand::Imm(node.inner2.val() as i64));

        mc_sink.push( instr );
    }
}