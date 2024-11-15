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

    /// Returns if the register is a general pourpus register
    #[inline]
    pub fn is_gr(&self) -> bool {
        match self.reg {
            TargetReg::X64(x64) => x64.is_gr(),
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
}

impl Display for Reg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fmt = match self.reg {
            TargetReg::X64(x64) => format!("{x64}"),
        };

        write!(f, "{fmt}")
    }
}