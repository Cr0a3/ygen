use crate::prelude::{Cast, Ir};
use crate::IR::{Block, TypeMetadata, Var};
use super::{CompilationHelper, VarLocation};
use crate::CodeGen::{MachineInstr, MachineMnemonic, MachineOperand};

impl CompilationHelper {
    #[allow(missing_docs)]
    pub fn compile_cast(&mut self, node: &Cast<Var, TypeMetadata, Var>, mc_sink: &mut Vec<MachineInstr>, block: &Block) {
        let src1 = *self.vars.get(&node.inner1).expect("expected valid variable");

        let boxed: Box<dyn Ir> = Box::new(node.clone());

        if !block.isVarUsedAfterNode(&boxed, &node.inner1) {
            self.free(&node.inner1)
        }
        if !block.isVarUsedAfterNode(&boxed, &node.inner3) {
            return;
        }

        let out = self.alloc(&node.inner3);

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
        }

        match out {
            VarLocation::Reg(reg) => instr.set_out(MachineOperand::Reg(reg)),
        }

        instr.meta = node.inner3.ty;

        mc_sink.push(instr);
    }
}