use std::fmt::{Debug, Display};

use crate::{Target::Arch, IR::TypeMetadata};

/// A register
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Reg {
    /// The size of the register (in bits)
    pub size: usize,
    /// The target specific Register
    pub reg: TargetReg,
}

/// A target specific register
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum TargetReg {
    X64(crate::Target::x86::reg::X64Reg)
}

impl Reg {
    /// Returns the return register
    #[inline]
    pub fn ret(arch: Arch, ty: TypeMetadata) -> Self {
        crate::Target::get_ret_reg(&arch, ty)
    }

    /// Returns if the register is a general pourpus register
    #[inline]
    pub fn is_gr(&self) -> bool {
        match self.reg {
            TargetReg::X64(x64) => x64.is_gr(),
        }
    }
}

impl Display for Reg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.reg {
            TargetReg::X64(x64) => std::fmt::Display::fmt(&x64, f),
        }
    }
}