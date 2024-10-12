use crate::CodeGen::*;
use super::CompilationHelper;
use crate::prelude::*;

impl CompilationHelper {
    #[allow(missing_docs)]
    pub fn compile_select_tt(&mut self, node: &Select<Type, Type>, mc_sink: &mut Vec<MachineInstr>, _: &Block) {
        let out = self.vars.get(&node.out.name).expect("expected valid variable");
        let cond = self.vars.get(&node.cond.name).expect("expected valid variable");
    
        let out = (*out).into();
        let cond = (*cond).into();

        let yes = MachineOperand::Imm(node.yes.val() as i64);
        let no = MachineOperand::Imm(node.no.val() as i64);

        let mut yes_instr = MachineInstr::new(MachineMnemonic::MovIfZero);
        
        yes_instr.set_out(out);
        yes_instr.add_operand(cond);
        yes_instr.add_operand(no);

        yes_instr.meta = node.yes.into();

        mc_sink.push(yes_instr);
        
        let mut no_instr = MachineInstr::new(MachineMnemonic::MovIfNotZero);
        
        no_instr.set_out(out);
        no_instr.add_operand(cond);
        no_instr.add_operand(yes);
        
        no_instr.meta = node.no.into();

        mc_sink.push(no_instr);
    }

    #[allow(missing_docs)]
    pub fn compile_select_tv(&mut self, node: &Select<Type, Var>, mc_sink: &mut Vec<MachineInstr>, _: &Block) {
        let out = self.vars.get(&node.out.name).expect("expected valid variable");
        let cond = self.vars.get(&node.cond.name).expect("expected valid variable");
        
        let no = self.vars.get(&node.no.name).expect("expected valid variable");
        
        let out = (*out).into();
        let cond = (*cond).into();
        let no = (*no).into();

        let yes = MachineOperand::Imm(node.yes.val() as i64);

        let mut yes_instr = MachineInstr::new(MachineMnemonic::MovIfZero);
        
        yes_instr.set_out(out);
        yes_instr.add_operand(cond);
        yes_instr.add_operand(yes);

        yes_instr.meta = node.yes.into();

        mc_sink.push(yes_instr);

        let mut no_instr = MachineInstr::new(MachineMnemonic::MovIfNotZero);
        
        no_instr.set_out(out);
        no_instr.add_operand(cond);
        no_instr.add_operand(no);

        no_instr.meta = node.no.ty;

        mc_sink.push(no_instr);
    }

    #[allow(missing_docs)]
    pub fn compile_select_vt(&mut self, node: &Select<Var, Type>, mc_sink: &mut Vec<MachineInstr>, _: &Block) {
        let out = self.vars.get(&node.out.name).expect("expected valid variable");
        let cond = self.vars.get(&node.cond.name).expect("expected valid variable");
        
        let yes = self.vars.get(&node.yes.name).expect("expected valid variable");
        
        let out = (*out).into();
        let cond = (*cond).into();
        let yes = (*yes).into();

        let no = MachineOperand::Imm(node.no.val() as i64);

        let mut yes_instr = MachineInstr::new(MachineMnemonic::MovIfZero);
        
        yes_instr.set_out(out);
        yes_instr.add_operand(cond);
        yes_instr.add_operand(no);
    
        yes_instr.meta = node.yes.ty;

        mc_sink.push(yes_instr);
        
        let mut no_instr = MachineInstr::new(MachineMnemonic::MovIfNotZero);
        
        no_instr.set_out(out);
        no_instr.add_operand(cond);
        no_instr.add_operand(yes);

        no_instr.meta = node.no.into();

        mc_sink.push(no_instr);
    }
    
    #[allow(missing_docs)]
    pub fn compile_select_vv(&mut self, node: &Select<Var, Var>, mc_sink: &mut Vec<MachineInstr>, _: &Block) {
        let out = self.vars.get(&node.out.name).expect("expected valid variable");
        let cond = self.vars.get(&node.cond.name).expect("expected valid variable");
        
        let yes = self.vars.get(&node.yes.name).expect("expected valid variable");
        let no = self.vars.get(&node.no.name).expect("expected valid variable");
        
        let out = (*out).into();
        let cond = (*cond).into();
        let yes = (*yes).into();
        let no = (*no).into();

        let mut yes_instr = MachineInstr::new(MachineMnemonic::MovIfZero);
        
        yes_instr.set_out(out);
        yes_instr.add_operand(cond);
        yes_instr.add_operand(no);

        yes_instr.meta = node.yes.ty;

        mc_sink.push(yes_instr);

        let mut no_instr = MachineInstr::new(MachineMnemonic::MovIfNotZero);
        
        no_instr.set_out(out);
        no_instr.add_operand(cond);
        no_instr.add_operand(yes);

        no_instr.meta = node.no.ty;

        mc_sink.push(no_instr);
    }
}