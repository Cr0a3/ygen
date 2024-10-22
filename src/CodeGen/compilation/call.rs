use crate::{prelude::Call, CodeGen::MachineMnemonic, IR::FuncId};
use crate::IR::{Block, Var};
use super::CompilationHelper;
use crate::CodeGen::MachineInstr;

impl CompilationHelper {
    #[allow(missing_docs)]
    pub fn compile_call(&mut self, node: &Call<FuncId, Vec<Var>, Var>, mc_sink: &mut Vec<MachineInstr>, _: &Block, _: &mut crate::prelude::Module) {
        let out = (*self.vars.get(&node.inner3.name).expect("expected valid output variable")).into();

        let mut index = 0;
        for arg in &node.inner2 {
            let src = self.vars.get(&arg.name).expect(&format!("expected valid variable: {}", arg.name));

            let mut instr = MachineInstr::new(MachineMnemonic::ArgMove(index));
            instr.add_operand(src.into());            

            index += 1;
        }

        let mut instr = MachineInstr::new(MachineMnemonic::Call(node.inner1.name.to_string()));

        instr.set_out(out);
        instr.meta = node.inner1.ty.ret;

        mc_sink.push(instr);

        if let Some(phi_loc) = self.alloc.phi_vars.get(&node.inner1.name) {
            let mut instr = MachineInstr::new(MachineMnemonic::Move);
            instr.set_out((*phi_loc).into());
            instr.add_operand(out);
            mc_sink.push(instr);
        }
    }
}