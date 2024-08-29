use std::error::Error;
use std::fmt::{Debug, Display};
use std::any::Any;
use crate::Obj::Link;

use super::reg::Reg;

/// a low level instruction which is portable over platforms
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MachineInstr {
    pub(crate) operands: Vec<MachineOperand>,
    pub(crate) out: Option<MachineOperand>,
    pub(crate) mnemonic: MachineMnemonic,
}

impl MachineInstr {
    /// Creates a new machine instr
    pub fn new(mne: MachineMnemonic) -> Self {
        Self {
            mnemonic: mne,
            operands: vec![],
            out: None,
        }
    }

    /// Adds an operand
    pub fn add_operand(&mut self, op: MachineOperand) {
        self.operands.push( op );
    }

    /// Sets the output of the instr
    pub fn set_out(&mut self, out: MachineOperand) {
        self.out = Some(out);
    }
}

/// a low level operand which is portable over platforms
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineOperand {
    /// a number
    Imm(i64),
    /// a register
    Reg(Reg),
}

/// The mnemonic to use
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    fn dump(&self) -> Result<Vec<String>, Box<dyn Error>>;

    /// encodes the instruction
    fn encode(&self) -> Result<(Vec<u8>, Option<Link>), Box<dyn Error>>;
}