use crate::Target::{x64Reg, Arch};

/// A shared enum for registers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Reg {
    /// a register of the x64 platform
    x64(x64Reg),
}

impl Reg {
    /// returns the architecture of the register
    pub fn arch(&self) -> Arch {
        match self {
            Reg::x64(_) => Arch::X86_64,
        }
    }
}