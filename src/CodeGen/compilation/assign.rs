use crate::prelude::Assign;
use crate::IR::{Block, Const, Type, Var};
use super::CompilationHelper;
use crate::CodeGen::{MachineInstr, MachineMnemonic, MachineOperand};

impl CompilationHelper {
    #[allow(missing_docs)]
    pub fn compile_assign_var_type(&mut self, node: &Assign<Var, Type>, mc_sink: &mut Vec<MachineInstr>, _: &Block, _: &mut crate::prelude::Module) {
        let out = *self.vars.get(&node.inner1.name).unwrap();
        let mut instr = MachineInstr::new(MachineMnemonic::Move);

        instr.set_out(out.into());

        instr.add_operand(MachineOperand::Imm(node.inner2.val()));

        instr.meta = node.inner1.ty;

        mc_sink.push( instr );

        if let Some(phi_loc) = self.alloc.phi_vars.get(&node.inner1.name) {
            let mut instr = MachineInstr::new(MachineMnemonic::Move);
            instr.set_out((*phi_loc).into());
            instr.add_operand(out.into());

            mc_sink.push(instr);
        }
        
    }

    #[allow(missing_docs)]
    pub fn compile_assign_var_var(&mut self, node: &Assign<Var, Var>, mc_sink: &mut Vec<MachineInstr>, _: &Block, _: &mut crate::prelude::Module) {
        let src1 = *self.vars.get(&node.inner2.name).expect(&format!("{} has no variable location", node.inner2));
        let out = *self.vars.get(&node.inner1.name).unwrap();

        let mut instr = MachineInstr::new(MachineMnemonic::Move);

        instr.set_out(out.into());

        instr.add_operand(src1.into());
        
        instr.meta = node.inner1.ty;

        mc_sink.push( instr );

        if let Some(phi_loc) = self.alloc.phi_vars.get(&node.inner1.name) {
            let mut instr = MachineInstr::new(MachineMnemonic::Move);
            instr.set_out((*phi_loc).into());
            instr.add_operand(out.into());

            mc_sink.push(instr);
        }
    }
    
    #[allow(missing_docs)]
    pub fn compile_assign_var_const(&mut self, node: &Assign<Var, Const>, mc_sink: &mut Vec<MachineInstr>, _: &Block, _: &mut crate::prelude::Module) {
        let out = *self.vars.get(&node.inner1.name).unwrap();

        let mut instr = MachineInstr::new(
            MachineMnemonic::AdressLoad(node.inner2.name.to_string())
        );

        instr.set_out(out.into());

        
        instr.meta = node.inner1.ty; // is a pointer but i just wrote it here

        mc_sink.push( instr );

        if let Some(phi_loc) = self.alloc.phi_vars.get(&node.inner1.name) {
            let mut instr = MachineInstr::new(MachineMnemonic::Move);
            instr.set_out((*phi_loc).into());
            instr.add_operand(out.into());
            mc_sink.push(instr);
        }
    }
}