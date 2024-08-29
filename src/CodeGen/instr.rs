use std::fmt::{Debug, Display};
use std::any::Any;
use super::reg::Reg;

/// a low level instruction which is portable over platforms
pub struct MachineInstr {
    pub(crate) operands: Vec<MachineOperand>,
    pub(crate) out: Option<MachineOperand>,
    pub(crate) mnemonic: MachineMnemonic,
}

impl MachineInstr {
    pub fn new(mne: MachineMnemonic) -> Self {
        Self {
            mnemonic: mne,
            operands: vec![],
            out: None,
        }
    }

    pub fn add_operand(&mut self, op: MachineOperand) {
        self.operands.push( op );
    }

    pub fn set_out(&mut self, out: MachineOperand) {
        self.out = Some(out);
    }
}

/// a low level operand which is portable over platforms
pub enum MachineOperand {
    /// a number
    Imm(i64),
    /// a register
    Reg(Reg),
}

/// The mnemonic to use
pub enum MachineMnemonic {
    Move,
    
    Add,
    And,
    Div,
    Mul,
    Or,
    Sub,
    Xor,

    Call,
    Return,
}

/// a platform specifc instruction
pub trait MCInstr: Any + Debug + Display {
    /// dumps the instruction into a assembly string
    fn dump(&self) -> Vec<String>;
}