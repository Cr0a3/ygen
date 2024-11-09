use crate::CodeGen::*;
use super::CompilationHelper;
use crate::prelude::*;

impl CompilationHelper {
    #[allow(missing_docs)]
    pub fn compile_select(&mut self, node: &Select<IROperand, IROperand>, mc_sink: &mut Vec<MachineInstr>, _: &Block, _: &mut crate::prelude::Module) {
        let out = (*self.vars.get(&node.out.name).expect("expected valid variable")).into();
        let cond = (*self.vars.get(&node.cond.name).expect("expected valid variable")).into();
        
        let yes = node.yes.into_mi(self);
        let no = node.no.into_mi(self);

        let ty = node.yes.get_ty();

        let mut yes_instr = MachineInstr::new(MachineMnemonic::MovIfZero);
        
        yes_instr.set_out(out);
        yes_instr.add_operand(cond);
        yes_instr.add_operand(yes);

        yes_instr.meta = ty;

        mc_sink.push(yes_instr);

        let mut no_instr = MachineInstr::new(MachineMnemonic::MovIfNotZero);
        
        no_instr.set_out(out);
        no_instr.add_operand(cond);
        no_instr.add_operand(no);

        no_instr.meta = ty;

        mc_sink.push(no_instr);
    }
}