use crate::Target::x64Reg;

/// A shared enum for registers
pub enum Reg {
    /// a register of the x64 platform
    x64(x64Reg),
}