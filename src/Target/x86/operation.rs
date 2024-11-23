use crate::CodeGen::{dag::{DagOp, DagOpTarget, DagOperandOption, OperationHandler}, reg::TargetReg};

use super::asm::{X86Instr, X86Operand};

/// The handler for operantions in the x86 backend
pub struct X86OperationHandler;

impl OperationHandler for X86OperationHandler {
    type Operand = X86Operand;

    type Instr = X86Instr;

    fn just_op(&self, op: &DagOp) -> bool {
        // just_ops are all operations except constant fps
        !op.is_operation_cfp()
    }

    fn inserts_instrs(&self, op: &DagOp) -> bool {
        // just_ops are all operations except constant fps
        !op.is_operation_cfp()
    }

    fn requires_new_const(&self, op: &DagOp) -> bool {
        // there are no constants required except for constant fps
        !op.is_operation_cfp()
    }

    fn compile_op(&self, op: &DagOp, _: Option<&crate::IR::Const>) -> Option<Self::Operand> {
        if op.is_operation_cfp() {
            panic!("constant fp operations require to be compiled using the `compile_instrs` function");
        }

        let mut operand = None;

        if !op.allocated {
            panic!("compiling with unallocated operands is impossible"); // What should i say more about it
        }

        if op.is_operation_cimm() {
            let DagOpTarget::Constant(imm) = op.target else {
                panic!("the combination `operation is cimm and target is not cimm` is impossible");
            };

            operand = Some(X86Operand::Const(imm.val() as i64));
        }

        if op.is_operation_load() {
            operand = Some(match op.target {
                DagOpTarget::Mem(mem) => X86Operand::MemDispl(mem.into()),
                DagOpTarget::Reg(reg) => X86Operand::Reg({
                    let TargetReg::X86(x86) = reg.reg;// else { panic!("a non x86 reg in the x86 target is sub-optimial"); };

                    x86
                }),
                _ => panic!("the combination `operation is load but target is not either a mem or a reg` is impossible"),
            });
        }

        operand
    }

    fn compile_instrs(&self, op: &DagOp, _constant: Option<&crate::IR::Const>) -> Option<Vec<Self::Instr>> {
        assert_eq!(op.get_operation(), DagOperandOption::ConstantFp);

        todo!("implement constant fp operation")
    }
}