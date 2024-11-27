use crate::{CodeGen::{dag::{DagOp, DagOpTarget}, memory::Memory, reg::{Reg, TargetReg}}, Target::x86::reg::{X86Reg, X86RegSize}};

use super::{X86MemDispl, X86MemOption, X86Instr, X86Mnemonic, X86Operand};

impl X86Instr {
    /// Creates a new X86 assembly instruction with 0 operands
    pub fn with0(mnemonic: X86Mnemonic) -> X86Instr {
        X86Instr {
            mnemonic: mnemonic,
            op1: None,
            op2: None,
            op3: None,
        }
    }

    /// Creates a new X86 assembly instruction with 1 operand
    pub fn with1(mnemonic: X86Mnemonic, op: X86Operand) -> X86Instr {
        X86Instr {
            mnemonic: mnemonic,
            op1: Some(op),
            op2: None,
            op3: None,
        }
    }

    /// Creates a new X86 assembly instruction with 2 operands
    pub fn with2(mnemonic: X86Mnemonic, op1: X86Operand, op2: X86Operand) -> X86Instr {
        X86Instr {
            mnemonic: mnemonic,
            op1: Some(op1),
            op2: Some(op2),
            op3: None,
        }
    }

    /// Creates a new X86 assembly instruction with 3 operands
    pub fn with3(mnemonic: X86Mnemonic, op1: X86Operand, op2: X86Operand, op3: X86Operand) -> X86Instr {
        X86Instr {
            mnemonic: mnemonic,
            op1: Some(op1),
            op2: Some(op2),
            op3: Some(op3),
        }
    }
}

impl From<DagOp> for X86Operand {
    fn from(dag: DagOp) -> Self {
        if !dag.allocated {
            panic!("operand to use in dag for the X86 backend needs to be allocated");
        }
        
        dag.target.into()
    }
}

impl From<DagOpTarget> for X86Operand {
    fn from(value: DagOpTarget) -> Self {
        match value {
            crate::CodeGen::dag::DagOpTarget::Reg(reg) => match reg.reg {
                crate::CodeGen::reg::TargetReg::X86(X86) => X86Operand::Reg(X86),
            },
            crate::CodeGen::dag::DagOpTarget::Constant(constant) => X86Operand::Const(constant.val() as i64),
            crate::CodeGen::dag::DagOpTarget::Mem(mem) => X86Operand::MemDispl(mem.into()),
            _ => panic!("variables cannot be used as a target in the X86 backend"),
        }
    }
}

impl From<&DagOpTarget> for X86Operand {
    fn from(value: &DagOpTarget) -> Self {
        match value {
            crate::CodeGen::dag::DagOpTarget::Reg(reg) => reg.into(),
            crate::CodeGen::dag::DagOpTarget::Constant(constant) => X86Operand::Const(constant.val() as i64),
            crate::CodeGen::dag::DagOpTarget::Mem(mem) => X86Operand::MemDispl((*mem).into()),
            _ => panic!("variables cannot be used as a target in the X86 backend"),
        }
    }
}

impl From<&Reg> for X86Operand {
    fn from(value: &Reg) -> Self {
        let reg: X86Reg = value.into();

        X86Operand::Reg(reg)
    }
}

impl From<DagOp> for X86Reg {
    #[allow(irrefutable_let_patterns)]
    fn from(value: DagOp) -> Self {
        let DagOpTarget::Reg(codegen_reg) = value.target else {
            panic!("the dag operand {value} has no register as its target")
        };

        let TargetReg::X86(x86) = codegen_reg.reg else {
            panic!("the dag operand {value} doesn't has a X86 reg as its target");
        };

        x86
    }
}

impl From<Memory> for X86MemDispl {
    fn from(mem: Memory) -> Self {
        let base = if mem.fp_relativ {
            Some(X86Reg::Rbp())
        } else if mem.sp_relativ {
            Some(X86Reg::Rsp())
        } else { None };

        X86MemDispl { 
            base: base, 
            option: X86MemOption::Plus, 
            index: None, 
            displ: Some(mem.offset), 
            scale: None, 
            size: mem.size.into(),
        }            
    }
}

impl From<Memory> for X86Operand {
    fn from(mem: Memory) -> Self {
        X86Operand::MemDispl(mem.into())         
    }
}

impl X86MemDispl {
    /// Creates a new X86 mem displacement which gets packed into a X86 operand with
    /// a base, operation and index
    pub fn new(base: X86Reg, op: X86MemOption, index: X86Reg) -> X86Operand {
        X86Operand::MemDispl(Self {
            base: Some(base),
            option: op,
            index: Some(index),
            displ: None,
            scale: None,
            size: X86RegSize::Qword,
        })
    }
}