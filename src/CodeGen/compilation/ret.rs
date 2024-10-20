use crate::{prelude::Return, CodeGen::{MachineInstr, MachineMnemonic, MachineOperand}, IR::{Block, Type, Var}};

use super::CompilationHelper;

impl CompilationHelper {
    #[allow(missing_docs)]
    pub fn compile_ret_ty(&mut self, node: &Return<Type>, mc_sink: &mut Vec<MachineInstr>, _: &Block, _: &mut crate::prelude::Module) {
        let mut instr = MachineInstr::new(MachineMnemonic::Return);
        instr.add_operand(MachineOperand::Imm(node.inner1.val()));
        instr.meta = node.inner1.into();

        mc_sink.push( instr );
    }

    #[allow(missing_docs)]
    pub fn compile_ret_var(&mut self, node: &Return<Var>, mc_sink: &mut Vec<MachineInstr>, _: &Block, _: &mut crate::prelude::Module) {
        let src = *self.vars.get(&node.inner1.name).expect("expected valid variable");
        
        let mut instr = MachineInstr::new(MachineMnemonic::Return);
        instr.add_operand(src.into());

        instr.meta = node.inner1.ty;

        mc_sink.push(instr);
    }
}