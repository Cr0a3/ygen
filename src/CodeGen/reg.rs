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
    
    /// returns if the list contains on of the register variants
    #[inline]
    pub fn contains_reg(reg: Reg, vec: &Vec<Reg>) -> bool {
        let mut seen = false;

        for vector_reg in vec {
            if seen { break; }

            match vector_reg {
                Reg::x64(x64_reg) => {
                    match &reg {
                        Reg::x64(reg) => seen = x64_reg.sub64() == reg.sub64(),
                    }
                }
            }
        }

        seen
    }

    /// Returns if the register one register variant is a variant of the same register
    /// (E.g: `rcx.is(ecx) = true`)
    pub fn is(&self, other: &Reg) -> bool {
        match self {
            Reg::x64(x64_reg) => {
                match other {
                    Reg::x64(reg) => x64_reg.sub64() == reg.sub64(),
                }
            }
        }  
    }

    /// Returns if the specified register is an fp register
    pub fn is_fp(&self) -> bool {
        match self {
            Reg::x64(x64) => x64.is_xmm(),
        }
    }
}