use std::collections::HashMap;

use crate::{prelude::{Call, Ir}, CodeGen::{MachineMnemonic, MachineOperand}};
use crate::IR::{Block, Function, Var};
use super::{CompilationHelper, VarLocation};
use crate::CodeGen::MachineInstr;

impl CompilationHelper {
    #[allow(missing_docs)]
    pub fn compile_call(&mut self, node: &Call<Function, Vec<Var>, Var>, mc_sink: &mut Vec<MachineInstr>, block: &Block) {
        let boxed: Box<dyn Ir> = Box::new(node.clone());

        let mut reg_args = 0;

        let args = self.call.args(self.arch);

        let mut saved = HashMap::new();

        
        for (name, loc) in self.vars.to_owned() {
            let typ = *self.var_types.get(&name).unwrap();

            match loc {
                VarLocation::Reg(reg) => {
                    if args.contains(&reg) {
                        // SAVE IT ONTO THE STACK
                        let mut save = MachineInstr::new( MachineMnemonic::Move );
            
                        let (_, off) = self.alloc_custom_stack(&typ);
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
            let mut instr = MachineInstr::new(MachineMnemonic::Move);

            let src = self.vars.get(&arg.name).expect(&format!("expected valid variable: {}", arg.name));

            let arg_reg = args.get(reg_args);

            if let Some(arg) = arg_reg {
                instr.set_out(MachineOperand::Reg(*arg));
            } else {
                instr = MachineInstr::new(MachineMnemonic::Push);
                instr.add_operand(src.into());
                pushes.push(arg.ty);
            }

            let mut op = src.into();

            if let Some((save, _)) = saved.get(&arg.name) {
                op = MachineOperand::Stack(*save);
            }

            instr.add_operand(op);

            mc_sink.push( instr );

            reg_args += 1;
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

        if block.isVarUsedAfterNode(&boxed, &node.inner3) {
            let mut instr = MachineInstr::new(MachineMnemonic::Move);

            let loc = self.alloc(&node.inner3);

            instr.add_operand(
                MachineOperand::Reg(
                    self.call.return_reg(self.arch, node.inner1.ty.ret)
                )
            );

            instr.meta = node.inner1.ty.ret;

            match loc {
                VarLocation::Reg(reg) => instr.set_out(MachineOperand::Reg(reg)),
                VarLocation::Mem(stack) => instr.add_operand( MachineOperand::Stack(stack) ),
            }

            mc_sink.push(instr);
        }
    }
}