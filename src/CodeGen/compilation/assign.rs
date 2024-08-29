use crate::prelude::{Assign, Ir};
use crate::IR::{Block, Const, Type, Var};
use super::{CompilationHelper, VarLocation};
use crate::CodeGen::{MachineInstr, MachineMnemonic, MachineOperand};

impl CompilationHelper {
    #[allow(missing_docs)]
    pub fn compile_assign_var_type(&mut self, node: &Assign<Var, Type>, mc_sink: &mut Vec<MachineInstr>, block: &Block) {
        let boxed: Box<dyn Ir> = Box::new(node.clone());

        if !block.isVarUsedAfterNode(&boxed,&node.inner1) {
            return; // dead code elimination
        }

        let location = self.alloc(&node.inner1);

        let mut instr = MachineInstr::new(MachineMnemonic::Move);

        match location {
            VarLocation::Reg(reg) => instr.add_operand(MachineOperand::Reg(reg)),
        }

        instr.add_operand(MachineOperand::Imm(node.inner2.val() as i64));

        mc_sink.push( instr );
        
    }

    #[allow(missing_docs)]
    pub fn compile_assign_var_var(&mut self, node: &Assign<Var, Var>, mc_sink: &mut Vec<MachineInstr>, block: &Block) {
        let src1 = *self.vars.get(&node.inner1).expect("expected valid variable");

        let boxed: Box<dyn Ir> = Box::new(node.clone());

        if !block.isVarUsedAfterNode(&boxed, &node.inner2) {
            self.free(&node.inner2);
        }
        if !block.isVarUsedAfterNode(&boxed,&node.inner1) {
            return; // dead code elimination
        }

        let location = self.alloc(&node.inner1);

        let mut instr = MachineInstr::new(MachineMnemonic::Move);

        match location {
            VarLocation::Reg(reg) => instr.add_operand(MachineOperand::Reg(reg)),
        }

        match src1 {
            VarLocation::Reg(reg) => instr.add_operand(MachineOperand::Reg(reg)),
        }

        mc_sink.push( instr );
    }
    
    #[allow(missing_docs)]
    pub fn compile_assign_var_const(&mut self, node: &Assign<Var, Const>, mc_sink: &mut Vec<MachineInstr>, block: &Block) {
        let boxed: Box<dyn Ir> = Box::new(node.clone());

        if !block.isVarUsedAfterNode(&boxed,&node.inner1) {
            return; // dead code elimination
        }

        let location = self.alloc(&node.inner1);

        let mut instr = MachineInstr::new(
            MachineMnemonic::AdressLoad(node.inner2.name.to_string())
        );

        match location {
            VarLocation::Reg(reg) => instr.add_operand(MachineOperand::Reg(reg)),
        }

        mc_sink.push( instr );
    }
}