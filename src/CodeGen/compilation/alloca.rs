use crate::CodeGen::{MachineInstr, MachineMnemonic, MachineOperand};
use crate::IR::{Block, TypeMetadata, Var, ir::Alloca};

use super::CompilationHelper;

impl CompilationHelper {
    #[allow(missing_docs)]
    pub fn compile_alloca(&mut self, node: &Alloca<Var, TypeMetadata>, mc_sink: &mut Vec<MachineInstr>, _: &Block) {
        let (_, offset) = self.alloc_custom_stack(&node.inner2);
        let out = self.alloc(&node.inner1);

        let out = match out {
            super::VarLocation::Reg(reg) => MachineOperand::Reg(reg),
            super::VarLocation::Mem(mem) => MachineOperand::Stack(mem),
        };

        let mut instr = MachineInstr::new(MachineMnemonic::StackAlloc);

        instr.set_out(out);
        instr.add_operand(MachineOperand::Imm(offset));

        mc_sink.push(instr);
    }
}