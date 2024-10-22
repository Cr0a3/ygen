use crate::CodeGen::{MCInstr, MachineOperand, VMem, VReg};

use super::instr::{Mnemonic, Operand};

/// a instruction operand which is usable in x64 register allocation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegAllocOperand {
    Reg(VReg),
    Mem(VMem),
    Allocated(Operand),

    // temporarys
    Tmp0,
    Tmp1,
    Tmp2,
}

impl From<MachineOperand> for RegAllocOperand {
    fn from(value: MachineOperand) -> Self {
        match value {
            MachineOperand::Imm(imm) => RegAllocOperand::Allocated(Operand::Imm(imm as i64)),
            MachineOperand::Reg(vreg) => RegAllocOperand::Reg(vreg),
        }
    }
}

/// a instruction to use in x64 register allocation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct X64RegAllocInstr {
    /// The mnemonic to use
    pub mnemonic: Mnemonic,
    /// First operand
    pub op1: Option<RegAllocOperand>,
    /// Second operand
    pub op2: Option<RegAllocOperand>,

    // for far calls
    pub(crate) far: bool,
}

impl X64RegAllocInstr {
    /// Creates the instruction with 0 operands
    pub fn with0(mne: Mnemonic) -> Self {
        Self {
            mnemonic: mne,
            op1: None,
            op2: None,
            far: false,
        }
    }

    /// Makes the instruction use far calls (for call and so on)
    pub fn make_far(&self) -> Self {
        Self {
            mnemonic: self.mnemonic.to_owned(),
            op1: self.op1.to_owned(),
            op2: self.op2.to_owned(),
            far: true,
        }
    }

    /// Creates the instruction with 1 operand
    pub fn with1(mne: Mnemonic, op: RegAllocOperand) -> Self {
        Self {
            mnemonic: mne,
            op1: Some(op),
            op2: None,
            far: false,
        }
    }

    /// Creates the instruction with 2 operands
    pub fn with2(mne: Mnemonic, op1: RegAllocOperand, op2: RegAllocOperand) -> Self {
        Self {
            mnemonic: mne,
            op1: Some(op1),
            op2: Some(op2),
            far: false,
        }
    }
}

pub struct X64Allocator {
    pub(crate) instrs: Vec<X64RegAllocInstr>,
    pub(crate) alloc: Vec<X64RegAllocInstr>,

    pub(crate) out: Vec<Box<dyn MCInstr>>,
}

impl X64Allocator {
    pub fn new() -> Self {
        Self {
            instrs: Vec::new(),
            alloc: Vec::new(),
            out: Vec::new(),
        }
    }

    pub fn allocate(&mut self) {
        todo!()
    }

    pub fn allocate_tmps(&mut self) {
        todo!()
    }

    pub fn bake(&mut self) {
        todo!()
    }
}