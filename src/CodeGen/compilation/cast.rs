use crate::prelude::Cast;
use crate::IR::{Block, TypeMetadata, Var};
use super::{CompilationHelper, VarLocation};
use crate::CodeGen::{MachineInstr, MachineMnemonic, MachineOperand};

impl CompilationHelper {
    #[allow(missing_docs)]
    pub fn compile_cast(&mut self, node: &Cast<Var, TypeMetadata, Var>, mc_sink: &mut Vec<MachineInstr>, _: &Block) {
        let src1 = *self.vars.get(&node.inner1.name).expect("expected valid variable");

        let out = *self.vars.get(&node.inner3.name).unwrap();

        let op = {

        if node.inner1.ty.bitSize() > node.inner2.bitSize() {
            MachineMnemonic::Zext
        } else if node.inner1.ty.bitSize() < node.inner2.bitSize(){
            MachineMnemonic::Downcast
        } else {
            return;
        }
        
        };

        let mut instr = MachineInstr::new(op);

        match src1 {
            VarLocation::Reg(reg) => instr.add_operand(MachineOperand::Reg(reg)),
            VarLocation::Mem(stack) => instr.add_operand( MachineOperand::Stack(stack) ),
        }

        match out {
            VarLocation::Reg(reg) => instr.set_out(MachineOperand::Reg(reg)),
            VarLocation::Mem(stack) => instr.add_operand( MachineOperand::Stack(stack) ),
        }

        instr.meta = node.inner3.ty;

        mc_sink.push(instr);

        if let Some(phi_loc) = self.alloc.phi_vars.get(&node.inner3.name) {
            let mut instr = MachineInstr::new(MachineMnemonic::Move);
            instr.set_out((*phi_loc).into());
            instr.add_operand(out.into());
            mc_sink.push(instr);
        }
    }
}