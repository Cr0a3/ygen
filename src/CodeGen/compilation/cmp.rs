use crate::{prelude::Cmp, CodeGen::{MachineInstr, MachineMnemonic}, IR::{Block, TypeMetadata}};

use super::CompilationHelper;

impl CompilationHelper {
    #[allow(missing_docs)]
    pub fn compile_cmp(&mut self, node: &Cmp, mc_sink: &mut Vec<MachineInstr>, _: &Block, _: &mut crate::prelude::Module) {
        let ls = *self.vars.get(&node.ls.name).expect("expected valid variable");
        let rs = *self.vars.get(&node.rs.name).expect("expected valid variable");
        let out =  *self.vars.get(&node.out.name).unwrap();

        let ls = ls.into();
        let rs = rs.into();

        let out =  out.into();

        let mut cmp = MachineInstr::new(MachineMnemonic::Compare(node.mode) );
        
        cmp.add_operand(ls);
        cmp.add_operand(rs);

        cmp.set_out(out);

        cmp.meta = node.ls.ty;

        mc_sink.push( cmp );

        if let Some(phi_loc) = self.phi_vars.get(&node.out.name) {
            let mut instr = MachineInstr::new(MachineMnemonic::Move);
            instr.set_out((*phi_loc).into());
            instr.add_operand(out.into());

            instr.meta = TypeMetadata::u8;

            mc_sink.push(instr);
        }
    }
}