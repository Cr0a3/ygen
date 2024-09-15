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

        for arg in &node.inner2 {
            let mut instr = MachineInstr::new(MachineMnemonic::Move);
            
            let arg_reg = args.get(reg_args);

            if let Some(arg) = arg_reg {
                instr.set_out(MachineOperand::Reg(*arg));
            } else {
                todo!("implemt arguments which are passed over the stack");
            }

            let src = self.vars.get(&arg.name).expect("expected valid variable");

            match src {
                VarLocation::Reg(reg) => instr.add_operand(MachineOperand::Reg(*reg)),
                VarLocation::Mem(stack) => instr.add_operand( MachineOperand::Stack(*stack) ),
            }

            mc_sink.push( instr );

            reg_args += 1;
        }

        mc_sink.push(MachineInstr::new(
            MachineMnemonic::Call(node.inner1.name.to_string())
        ));

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