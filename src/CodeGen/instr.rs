use std::fmt::{Debug, Display};
use std::any::Any;
use super::reg::Reg;

/// a low level instruction which is portable over platforms
pub struct MachineInstr {
    pub(crate) operands: Vec<MachineOperand>,
    pub(crate) mnemonic: MachineMnemonic,
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

}

pub trait MCInstr: Any + Debug + Display {
    fn dump(&self) -> Vec<String>;
}