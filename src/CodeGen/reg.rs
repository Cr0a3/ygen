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
pub enum TargetReg {

}

impl Reg {
    /// Returns the return register
    #[inline]
    pub fn ret(arch: Arch, ty: TypeMetadata) -> Self {
        crate::Target::get_ret_reg(&arch, ty)
    }
}