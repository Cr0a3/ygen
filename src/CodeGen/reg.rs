use std::fmt::{Debug, Display};

use crate::{Target::{x86::reg::X64Reg, Arch}, IR::TypeMetadata};

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

    /// Sets the size of the register
    #[inline]
    pub fn set_size(&mut self, new: usize) {
        self.size = new;

        match &mut self.reg {
            TargetReg::X64(x64_reg) => x64_reg.size = new.into(),
        };
    }

    /// Returns if the register is a general pourpus register
    #[inline]
    pub fn is_gr(&self) -> bool {
        match self.reg {
            TargetReg::X64(x64) => x64.is_gr(),
        }
    }

    /// Returns if the register is a floating point register
    #[inline]
    pub fn is_fp(&self) -> bool {
        match self.reg {
            TargetReg::X64(x64) => x64.is_fp(),
        }
    }

    /// Creates an new x64 register
    #[inline]
    pub fn new_x64(reg: X64Reg) -> Reg {
        Reg {
            size: match reg.size {
                crate::Target::x86::reg::X64RegSize::Byte => 1,
                crate::Target::x86::reg::X64RegSize::Word => 2,
                crate::Target::x86::reg::X64RegSize::Dword => 4,
                crate::Target::x86::reg::X64RegSize::Qword => 8,
            },
            reg: TargetReg::X64(reg),
        }
    }

    /// Returns an register allocation score for the given register
    /// 
    /// Rules:
    /// 1. Starts at 2
    /// 2. `-1` if it is not callee saved
    /// 3. `-1` if it is doesn't require a reg prefix
    pub fn score(&self) -> usize {
        match self.reg {
            TargetReg::X64(x64) => x64.score(),
        }
    }
}

impl Display for Reg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fmt = match self.reg {
            TargetReg::X64(x64) => format!("{x64}"),
        };

        write!(f, "{fmt}")
    }
}