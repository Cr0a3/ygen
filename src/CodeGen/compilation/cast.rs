use crate::prelude::Cast;
use crate::IR::Block;
use super::CompilationHelper;
use crate::CodeGen::{MachineInstr, MachineMnemonic};

impl CompilationHelper {
    #[allow(missing_docs)]
    pub fn compile_cast(&mut self, node: &Cast, mc_sink: &mut Vec<MachineInstr>, _: &Block, _: &mut crate::prelude::Module) {
        let out = *self.vars.get(&node.inner3.name).unwrap();

        let op = {
            if node.inner1.get_ty().float() || node.inner2.float() {
                MachineMnemonic::FCast
            } else if node.inner1.get_ty().bitSize() < node.inner2.bitSize() {
                MachineMnemonic::Zext
            } else if node.inner1.get_ty().bitSize() > node.inner2.bitSize(){
                MachineMnemonic::Downcast
            } else {
                return;
            }  
        }(node.inner1.get_ty());

        let mut instr = MachineInstr::new(op);

        instr.add_operand(node.inner1.into_mi(self));
        instr.set_out(out.into());

        instr.meta = node.inner3.ty;

        mc_sink.push(instr);

        if let Some(phi_loc) = self.phi_vars.get(&node.inner3.name) {
            let mut instr = MachineInstr::new(MachineMnemonic::Move);
            instr.set_out((*phi_loc).into());
            instr.add_operand(out.into());
            instr.meta = node.inner3.ty;
            mc_sink.push(instr);
        }
    }
}