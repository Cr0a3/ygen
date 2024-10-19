use std::collections::HashMap;

use crate::{prelude::Call, CodeGen::{MachineMnemonic, MachineOperand, Reg}, IR::{FuncId, TypeMetadata}};
use crate::IR::{Block, Var};
use super::{CompilationHelper, VarLocation};
use crate::CodeGen::MachineInstr;

impl CompilationHelper {
    #[allow(missing_docs)]
    pub fn compile_call(&mut self, node: &Call<FuncId, Vec<Var>, Var>, mc_sink: &mut Vec<MachineInstr>, _: &Block, _: &mut crate::prelude::Module) {
        let mut reg_args = 0;
        let mut fp_reg_args = 0;

        let args = self.call.args(self.arch, TypeMetadata::i64);
        let fp_args = self.call.args(self.arch, TypeMetadata::f64);

        let mut saved = HashMap::new();
        
        for (name, loc) in self.get_vars_to_save_for_call(node) {
            let typ = *self.var_types.get(&name).unwrap();

            match loc {
                VarLocation::Reg(reg) => {
                    if Reg::contains_reg(reg, &args) {
                        // SAVE IT ONTO THE STACK
                        let mut save = MachineInstr::new( MachineMnemonic::Move );
            
                        let off = match self.alloc.alloc_stack(typ) {
                            VarLocation::Mem(off) => off,
                            _ => unreachable!(),
                        };
                        saved.insert(name.to_owned(), (off, loc));

                        save.set_out(MachineOperand::Stack(off));
                        save.add_operand(loc.into());

                        mc_sink.push(save);
                    }
                },
                VarLocation::Mem(_) => {},
            }
        }

        let mut pushes = Vec::new();

        for arg in &node.inner2 {
            let src = self.vars.get(&arg.name).expect(&format!("expected valid variable: {}", arg.name));

            let arg_reg = if TypeMetadata::f32 == arg.ty || TypeMetadata::f64 == arg.ty {
                fp_args.get(fp_reg_args)
            } else { 
                args.get(reg_args)
            };
            let mut arg_reg = arg_reg.cloned();

            if let Some(reg) = arg_reg {
                arg_reg = Some(match reg {
                    Reg::x64(x64) => Reg::x64(x64.sub_ty(arg.ty)),
                    Reg::wasm(i) => Reg::wasm(i),
                })
            }
            
            if let Some(reg) = arg_reg {
                if !self.allocated_vars.contains(&arg.name) {
                    let mut instr = MachineInstr::new(MachineMnemonic::Move);
    
                    let mut op = src.into();
    
                    if let Some((save, _)) = saved.get(&arg.name) {
                        op = MachineOperand::Stack(*save);
                    }
    
                    instr.set_out(MachineOperand::Reg(reg));
                    instr.add_operand(op);

                    instr.meta = arg.ty;

                    mc_sink.push( instr );
                } else {
                    let mut instr = MachineInstr::new(MachineMnemonic::AdrMove);

                    instr.set_out(MachineOperand::Reg(reg));
                    instr.add_operand(src.into());

                    instr.meta = arg.ty;

                    mc_sink.push( instr );
                }
            } else {
                if !self.allocated_vars.contains(&arg.name) {
                    let mut instr = MachineInstr::new(MachineMnemonic::Push);
                    instr.add_operand(src.into());
                    pushes.push(arg.ty);
                    mc_sink.push( instr );
                } else {
                    let mut instr = MachineInstr::new(MachineMnemonic::AdrMove);

                    instr.set_out(MachineOperand::Reg(self.tmp_reg));
                    instr.add_operand(src.into());
                    mc_sink.push( instr );

                    let mut instr = MachineInstr::new(MachineMnemonic::Push);
                    instr.add_operand(MachineOperand::Reg(self.tmp_reg));
                    pushes.push(arg.ty);
                    mc_sink.push( instr );
                }
            }

            if TypeMetadata::f32 == arg.ty || TypeMetadata::f64 == arg.ty {
                fp_reg_args += 1;
            } else { 
                reg_args += 1;
            }
        }

        if !pushes.is_empty() {
            mc_sink.push(MachineInstr::new(MachineMnemonic::CallStackPrepare));
        }

        mc_sink.push(MachineInstr::new(
            MachineMnemonic::Call(node.inner1.name.to_string())
        ));
        
        if !pushes.is_empty() {
            mc_sink.push(MachineInstr::new(MachineMnemonic::CallStackRedo));
        }

        for (_, (stack, original)) in saved {
            let mut restore = MachineInstr::new( MachineMnemonic::Move);

            restore.set_out(original.into());
            restore.add_operand(MachineOperand::Stack(stack));

            mc_sink.push( restore );
        }

        for push_type in pushes {
            let mut instr = MachineInstr::new( MachineMnemonic::PushCleanup );
            instr.meta = push_type;

            mc_sink.push( instr );
        }

        let mut instr = MachineInstr::new(MachineMnemonic::Move);

        let loc = *self.vars.get(&node.inner3.name).unwrap();

        instr.add_operand(
            MachineOperand::Reg(
                self.call.return_reg(self.arch, node.inner1.ty.ret)
            )
        );

        instr.meta = node.inner1.ty.ret;

        match loc {
            VarLocation::Reg(reg) => instr.set_out(MachineOperand::Reg(reg)),
            VarLocation::Mem(stack) => instr.set_out( MachineOperand::Stack(stack) ),
        }

        mc_sink.push(instr);

        if let Some(phi_loc) = self.alloc.phi_vars.get(&node.inner1.name) {
            let mut instr = MachineInstr::new(MachineMnemonic::Move);
            instr.set_out((*phi_loc).into());
            instr.add_operand(loc.into());
            mc_sink.push(instr);
        }
    }
}