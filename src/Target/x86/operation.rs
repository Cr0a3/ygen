use crate::{CodeGen::{dag::{DagOp, DagOpTarget, DagTmpInfo, OperationHandler}, reg::TargetReg}, IR::TypeMetadata};

use super::asm::{X86Instr, X86MemDispl, X86Mnemonic, X86Operand};

/// The handler for operantions in the x86 backend
pub struct X86OperationHandler;

impl OperationHandler for X86OperationHandler {
    type Operand = X86Operand;

    type Instr = X86Instr;

    fn just_op(&self, op: &DagOp) -> bool {
        // just_ops are all operations except constant fps and adress moves
        !(op.is_operation_cfp() || op.is_operation_adrm())
    }

    fn inserts_instrs(&self, op: &DagOp) -> bool {
        // cfp and adrms insert new instruction
        op.is_operation_cfp() || op.is_operation_adrm()
    }

    fn requires_new_const(&self, op: &DagOp) -> bool {
        // there are no constants required except for constant fps
        !op.is_operation_cfp()
    }

    fn compile_op(&self, op: &DagOp, _: Option<&crate::IR::Const>) -> Option<Self::Operand> {
        if op.is_operation_cfp() {
            panic!("constant fp operations require to be compiled using the `compile_instrs` function");
        }

        if op.is_operation_adrm() {
            panic!("adrms need to be compiled using the `compile_instrs` function");
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

    fn compile_instrs(&self, op: &DagOp, _constant: Option<&crate::IR::Const>, tmp: DagTmpInfo) -> Option<Vec<Self::Instr>> {
        if op.is_operation_adrm() {
            // lea target, [adr]
            let mut insts = Vec::new();

            let target = op.get_adrm_target().unwrap();

            let rel = X86MemDispl::rip(target);

            insts.push(X86Instr::with2(X86Mnemonic::Lea, X86Operand::Tmp(tmp.tmp_num), rel));

            return Some(insts)
        }

        if op.is_operation_cfp() {
            // movd target, [rel {}]
    
            todo!("implement constant fp operation")
        }

        None
    }

    fn tmp(&self, op: &DagOp, num: usize) -> Vec<DagTmpInfo> {
        if op.is_operation_cfp() {
            let mut tmp = DagTmpInfo::new(num, TypeMetadata::f64);
            tmp.require_fp();

            vec![tmp]
        } else if op.is_operation_adrm() {
            let mut tmp = DagTmpInfo::new(num, TypeMetadata::ptr);
            tmp.require_fp();

            vec![tmp]
        } else { Vec::new() }
    }
}

impl X86OperationHandler {
    /// Creates a new Operation Handler for the X86 backend
    pub fn new() -> Self {
        Self {}
    }
}