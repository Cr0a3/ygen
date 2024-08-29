use crate::CodeGen::instr::{MachineMnemonic, MachineOperand, MachineInstr};
use crate::IR::ir::*;
use crate::prelude::Var;
use super::{CompilationHelper, VarLocation};

macro_rules! MathVarVar {
    ($func:ident, $node:ident, $mnemonic:expr) => {
        impl CompilationHelper {
            #[allow(missing_docs)]
            pub(crate) fn $func(&mut self, node: &$node<Var, Var, Var>, mc_sink: &mut Vec<MachineInstr>) {
                let src1 = *self.vars.get(&node.inner1).expect("expected valid variable");
                let src2 = *self.vars.get(&node.inner2).expect("expected valid variable");
        
                let boxed: Box<dyn Ir> = Box::new(node.clone());
        
                if !self.block.isVarUsedAfterNode(&boxed, &node.inner1) { // op1
                    self.free(&node.inner1);
                }
                if !self.block.isVarUsedAfterNode(&boxed, &node.inner2) { // op2
                    self.free(&node.inner2);
                }
                if !self.block.isVarUsedAfterNode(&boxed, &node.inner3) { // out
                    return; 
                }
        
                let out = self.regs.pop(self.arch).expect("ran out of registers");
        
                let mut instr = MachineInstr::new($mnemonic);
        
                match src1 {
                    VarLocation::Reg(reg) => instr.add_operand(MachineOperand::Reg(reg)),
                }
        
                match src2 {
                    VarLocation::Reg(reg) => instr.add_operand(MachineOperand::Reg(reg)),
                }
        
                instr.set_out(MachineOperand::Reg(out));
        
                mc_sink.push(instr);
            }
        }
    };
}


MathVarVar!(compile_add, Add, MachineMnemonic::Add);
MathVarVar!(compile_and, And, MachineMnemonic::And);
MathVarVar!(compile_div, Div, MachineMnemonic::Div);
MathVarVar!(compile_mul, Mul, MachineMnemonic::Mul);
MathVarVar!(compile_or, Or, MachineMnemonic::Or);
MathVarVar!(compile_sub, Sub, MachineMnemonic::Sub);
MathVarVar!(compile_xor, Xor, MachineMnemonic::Xor);


