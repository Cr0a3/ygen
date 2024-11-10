use crate::CodeGen::instr::{MachineMnemonic, MachineInstr};
use crate::IR::ir::*;
use crate::prelude::Block;
use super::CompilationHelper;

macro_rules! MathImpls {
    ($func:ident, $node:ident, $mnemonic:expr) => {
        impl CompilationHelper {
            #[allow(missing_docs)]
            pub(crate) fn $func(&mut self, node: &$node, mc_sink: &mut Vec<MachineInstr>, _: &Block, _: &mut crate::prelude::Module) {
                let src1 = node.inner1.into_mi(self);
                let src2 = node.inner2.into_mi(self);
        
                let out = *self.vars.get(&node.inner3.name).unwrap();
        
                let mut instr = MachineInstr::new($mnemonic);
        
                instr.add_operand(src1.into());
                instr.add_operand(src2.into());
                instr.set_out(out.into());
        
                instr.meta = node.inner3.ty;
        
                mc_sink.push(instr);

                if let Some(phi_loc) = self.phi_vars.get(&node.inner3.name) {
                    let mut instr = MachineInstr::new(MachineMnemonic::Move);
                    instr.set_out((*phi_loc).into());
                    instr.add_operand(out.into());
                    mc_sink.push(instr);
                }
            }
        }
    };
}


MathImpls!(compile_add, Add, MachineMnemonic::Add);
MathImpls!(compile_and, And, MachineMnemonic::And);
MathImpls!(compile_div, Div, MachineMnemonic::Div);
MathImpls!(compile_mul, Mul, MachineMnemonic::Mul);
MathImpls!(compile_or, Or, MachineMnemonic::Or);
MathImpls!(compile_sub, Sub, MachineMnemonic::Sub);
MathImpls!(compile_xor, Xor, MachineMnemonic::Xor);
MathImpls!(compile_rem, Rem, MachineMnemonic::Rem);
MathImpls!(compile_shl, Shl, MachineMnemonic::Shl);
MathImpls!(compile_shr, Shr, MachineMnemonic::Shr);