use crate::CodeGen::dag::DagOp;

use super::{X64Instr, X64Mnemonic, X64Operand};

impl X64Instr {
    /// Creates a new x64 assembly instruction with 0 operands
    pub fn with0(mnemonic: X64Mnemonic) -> X64Instr {
        X64Instr {
            mnemonic: mnemonic,
            op1: None,
            op2: None,
            op3: None,
        }
    }

    /// Creates a new x64 assembly instruction with 1 operand
    pub fn with1(mnemonic: X64Mnemonic, op: X64Operand) -> X64Instr {
        X64Instr {
            mnemonic: mnemonic,
            op1: Some(op),
            op2: None,
            op3: None,
        }
    }

    /// Creates a new x64 assembly instruction with 2 operands
    pub fn with2(mnemonic: X64Mnemonic, op1: X64Operand, op2: X64Operand) -> X64Instr {
        X64Instr {
            mnemonic: mnemonic,
            op1: Some(op1),
            op2: Some(op2),
            op3: None,
        }
    }

    /// Creates a new x64 assembly instruction with 3 operands
    pub fn with3(mnemonic: X64Mnemonic, op1: X64Operand, op2: X64Operand, op3: X64Operand) -> X64Instr {
        X64Instr {
            mnemonic: mnemonic,
            op1: Some(op1),
            op2: Some(op2),
            op3: Some(op3),
        }
    }
}

impl From<DagOp> for X64Operand {
    fn from(dag: DagOp) -> Self {
        if !dag.allocated {
            panic!("operand to use in dag for the x64 backend needs to be allocated");
        }
        
        match dag.target {
            crate::CodeGen::dag::DagOpTarget::Reg(reg) => match reg.reg {
                crate::CodeGen::reg::TargetReg::X64(x64) => X64Operand::Reg(x64),
            },
            crate::CodeGen::dag::DagOpTarget::Constant(constant) => X64Operand::Const(constant.val() as i64),
            crate::CodeGen::dag::DagOpTarget::Mem(mem) => X64Operand::MemDispl(mem),
            _ => panic!("variables cannot be used as a target in the x64 backend"),
        }
    }
}