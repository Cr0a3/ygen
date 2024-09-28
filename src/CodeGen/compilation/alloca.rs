use crate::CodeGen::{MachineInstr, MachineMnemonic, MachineOperand};
use crate::IR::{Block, TypeMetadata, Var, ir::Alloca};

use super::{CompilationHelper, VarLocation};

impl CompilationHelper {
    #[allow(missing_docs)]
    pub fn compile_alloca(&mut self, node: &Alloca<Var, TypeMetadata>, mc_sink: &mut Vec<MachineInstr>, _: &Block) {
        let off = match self.alloc.alloc_stack(node.inner2) {
            VarLocation::Mem(off) => off,
            _ => unreachable!(),
        };

        let out = *self.vars.get(&node.inner1.name).unwrap();

        let out = match out {
            super::VarLocation::Reg(reg) => MachineOperand::Reg(reg),
            super::VarLocation::Mem(mem) => MachineOperand::Stack(mem),
        };

        let mut instr = MachineInstr::new(MachineMnemonic::StackAlloc);

        instr.set_out(out);
        instr.add_operand(MachineOperand::Imm(off));

        mc_sink.push(instr);
    }
}