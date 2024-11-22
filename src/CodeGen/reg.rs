use std::fmt::{Debug, Display};

use crate::{Target::{x86::reg::X86Reg, Arch}, IR::TypeMetadata};

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
    X86(crate::Target::x86::reg::X86Reg)
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
            TargetReg::X86(x86) => x86.size = new.into(),
        };
    }

    /// Returns if the register is a general pourpus register
    #[inline]
    pub fn is_gr(&self) -> bool {
        match self.reg {
            TargetReg::X86(X86) => X86.is_gr(),
        }
    }

    /// Returns if the register is a floating point register
    #[inline]
    pub fn is_fp(&self) -> bool {
        match self.reg {
            TargetReg::X86(X86) => X86.is_fp(),
        }
    }

    /// Returns if the register is a simd reg (supporting the given vector)
    #[inline]
    pub fn is_simd(&self, vec: &crate::IR::VecTy) -> bool {
        match self.reg {
            TargetReg::X86(X86) => X86.is_simd(vec),
        }
    }

    /// Creates an new X86 register
    #[inline]
    pub fn new_x86(reg: X86Reg) -> Reg {
        Reg {
            size: match reg.size {
                crate::Target::x86::reg::X86RegSize::Byte => 1,
                crate::Target::x86::reg::X86RegSize::Word => 2,
                crate::Target::x86::reg::X86RegSize::Dword => 4,
                crate::Target::x86::reg::X86RegSize::Qword => 8,
                crate::Target::x86::reg::X86RegSize::SimdVec => 16, // in ygen we use sse registers for simd which are 128bit wide
            },
            reg: TargetReg::X86(reg),
        }
    }

    /// Returns an register allocation score for the given register
    /// 
    /// Rules:
    /// 1. Starts at 4
    /// 2. `-1` if it is requires any kind of prefix
    /// 3. `-2` if it is callee saved
    pub fn score(&self) -> usize {
        match self.reg {
            TargetReg::X86(X86) => X86.score(),
        }
    }
}

impl Display for Reg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fmt = match self.reg {
            TargetReg::X86(X86) => format!("{X86}"),
        };

        write!(f, "{fmt}")
    }
}