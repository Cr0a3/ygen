use crate::CodeGen::instr::{MachineMnemonic, MachineOperand, MachineInstr};
use crate::IR::ir::*;
use crate::prelude::{Var, Block, Type};
use super::CompilationHelper;

macro_rules! MathVarVar {
    ($func:ident, $node:ident, $mnemonic:expr) => {
        impl CompilationHelper {
            #[allow(missing_docs)]
            pub(crate) fn $func(&mut self, node: &$node<Var, Var, Var>, mc_sink: &mut Vec<MachineInstr>, _: &Block, _: &mut crate::prelude::Module) {
                let src1 = *self.vars.get(&node.inner1.name).expect("expected valid ls variable");
                let src2 = *self.vars.get(&node.inner2.name).expect("expected valid rs variable");
        
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


MathVarVar!(compile_add_var_var, Add, MachineMnemonic::Add);
MathVarVar!(compile_and_var_var, And, MachineMnemonic::And);
MathVarVar!(compile_div_var_var, Div, MachineMnemonic::Div);
MathVarVar!(compile_mul_var_var, Mul, MachineMnemonic::Mul);
MathVarVar!(compile_or_var_var, Or, MachineMnemonic::Or);
MathVarVar!(compile_sub_var_var, Sub, MachineMnemonic::Sub);
MathVarVar!(compile_xor_var_var, Xor, MachineMnemonic::Xor);
MathVarVar!(compile_rem_var_var, Rem, MachineMnemonic::Rem);
MathVarVar!(compile_shl_var_var, Shl, MachineMnemonic::Shl);
MathVarVar!(compile_shr_var_var, Shr, MachineMnemonic::Shr);

macro_rules! MathVarType {
    ($func:ident, $node:ident, $mnemonic:expr) => {
        impl CompilationHelper {
            #[allow(missing_docs)]
            pub fn $func(&mut self, node: &$node<Var, Type, Var>, mc_sink: &mut Vec<MachineInstr>, _: &Block, _: &mut crate::prelude::Module) {
                let src1 = *self.vars.get(&node.inner1.name).expect("expected valid variable");
                let out = *self.vars.get(&node.inner3.name).unwrap();
        
                let mut instr = MachineInstr::new($mnemonic);
        
                
                instr.add_operand(src1.into());
                instr.add_operand(MachineOperand::Imm(node.inner2.val()));
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

MathVarType!(compile_add_var_type, Add, MachineMnemonic::Add);
MathVarType!(compile_and_var_type, And, MachineMnemonic::And);
MathVarType!(compile_div_var_type, Div, MachineMnemonic::Div);
MathVarType!(compile_mul_var_type, Mul, MachineMnemonic::Mul);
MathVarType!(compile_or_var_type, Or, MachineMnemonic::Or);
MathVarType!(compile_sub_var_type, Sub, MachineMnemonic::Sub);
MathVarType!(compile_xor_var_type, Xor, MachineMnemonic::Xor);
MathVarType!(compile_rem_var_type, Rem, MachineMnemonic::Rem);
MathVarType!(compile_shl_var_type, Shl, MachineMnemonic::Shl);
MathVarType!(compile_shr_var_type, Shr, MachineMnemonic::Shr);

macro_rules! MathTypeType {
    ($func:ident, $node:ident, $mnemonic:expr) => {
        impl CompilationHelper {
            #[allow(missing_docs)]
            pub(crate) fn $func(&mut self, node: &$node<Type, Type, Var>, mc_sink: &mut Vec<MachineInstr>, _: &Block, _: &mut crate::prelude::Module) {
                let out = *self.vars.get(&node.inner3.name).unwrap();
        
                let mut instr = MachineInstr::new($mnemonic);
        
                instr.add_operand(MachineOperand::Imm(node.inner1.val()));
                instr.add_operand(MachineOperand::Imm(node.inner2.val()));
                
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

MathTypeType!(compile_add_type_type, Add, MachineMnemonic::Add);
MathTypeType!(compile_and_type_type, And, MachineMnemonic::And);
MathTypeType!(compile_div_type_type, Div, MachineMnemonic::Div);
MathTypeType!(compile_mul_type_type, Mul, MachineMnemonic::Mul);
MathTypeType!(compile_or_type_type, Or, MachineMnemonic::Or);
MathTypeType!(compile_sub_type_type, Sub, MachineMnemonic::Sub);
MathTypeType!(compile_xor_type_type, Xor, MachineMnemonic::Xor);
MathTypeType!(compile_rem_type_type, Rem, MachineMnemonic::Rem);
MathTypeType!(compile_shl_type_type, Shl, MachineMnemonic::Shl);
MathTypeType!(compile_shr_type_type, Shr, MachineMnemonic::Shr);