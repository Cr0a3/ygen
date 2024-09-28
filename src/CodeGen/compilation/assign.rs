use crate::prelude::Assign;
use crate::IR::{Block, Const, Type, Var};
use super::{CompilationHelper, VarLocation};
use crate::CodeGen::{MachineInstr, MachineMnemonic, MachineOperand};

impl CompilationHelper {
    #[allow(missing_docs)]
    pub fn compile_assign_var_type(&mut self, node: &Assign<Var, Type>, mc_sink: &mut Vec<MachineInstr>, _: &Block) {
        let location = *self.vars.get(&node.inner1.name).unwrap();

        let mut instr = MachineInstr::new(MachineMnemonic::Move);

        match location {
            VarLocation::Reg(reg) => instr.set_out(MachineOperand::Reg(reg)),
            VarLocation::Mem(stack) => instr.set_out( MachineOperand::Stack(stack) ),
        }

        instr.add_operand(MachineOperand::Imm(node.inner2.val() as i64));

        instr.meta = node.inner1.ty;

        mc_sink.push( instr );
        
    }

    #[allow(missing_docs)]
    pub fn compile_assign_var_var(&mut self, node: &Assign<Var, Var>, mc_sink: &mut Vec<MachineInstr>, _: &Block) {
        let src1 = *self.vars.get(&node.inner2.name).expect(&format!("{} has no variable location", node.inner2));

        let location = *self.vars.get(&node.inner1.name).unwrap();

        let mut instr = MachineInstr::new(MachineMnemonic::Move);

        match location {
            VarLocation::Reg(reg) => instr.set_out(MachineOperand::Reg(reg)),
            VarLocation::Mem(stack) => instr.set_out( MachineOperand::Stack(stack) ),
        }

        match src1 {
            VarLocation::Reg(reg) => instr.add_operand(MachineOperand::Reg(reg)),
            VarLocation::Mem(stack) => instr.add_operand( MachineOperand::Stack(stack) ),
        }
        
        instr.meta = node.inner1.ty;

        mc_sink.push( instr );
    }
    
    #[allow(missing_docs)]
    pub fn compile_assign_var_const(&mut self, node: &Assign<Var, Const>, mc_sink: &mut Vec<MachineInstr>, _: &Block) {
        let location = *self.vars.get(&node.inner1.name).unwrap();

        let mut instr = MachineInstr::new(
            MachineMnemonic::AdressLoad(node.inner2.name.to_string())
        );

        match location {
            VarLocation::Reg(reg) => instr.set_out(MachineOperand::Reg(reg)),
            VarLocation::Mem(stack) => instr.set_out( MachineOperand::Stack(stack) ),
        }

        
        instr.meta = node.inner1.ty; // is a pointer but i just wrote it here

        mc_sink.push( instr );
    }
}