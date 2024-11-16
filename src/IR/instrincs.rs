use std::str::FromStr;

use super::TypeMetadata;

/// The Intrinsic class in ygen describes a instrinc which is just like in llvm
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Intrinsic {
    pub(crate) instrinc: DefinedIntrinsic,
    pub(crate) ty: super::FunctionType,
}

/// A enum of pre defined Intrinsic
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DefinedIntrinsic {
    /// Returns the stack pointer
    GetStackPtr,
    /// Returns the frame pointer
    GetFramePtr,
}

impl Intrinsic {
    /// The `ygen.instrincs.getStackPtr` Intrinsic returns the stack pointer
    /// 
    /// ### Inputs
    /// None
    /// 
    /// ### Outputs
    /// A `ptr` referencing the stack pointer
    pub fn get_sp() -> Self {
        Self {
            instrinc: DefinedIntrinsic::GetStackPtr,
            ty: super::FnTy(Vec::new(), TypeMetadata::ptr),
        }
    }

    /// The `ygen.instrincs.getFramePtr` Intrinsic returns the frame pointer
    /// 
    /// ### Inputs
    /// None
    /// 
    /// ### Outputs
    /// A `ptr` referencing the frame pointer
    pub fn get_fp() -> Self {
        Self {
            instrinc: DefinedIntrinsic::GetFramePtr,
            ty: super::FnTy(Vec::new(), TypeMetadata::ptr),
        }
    }

    /// Returns the name of the instrinc
    pub fn name(&self) -> &'static str {
        match self.instrinc {
            DefinedIntrinsic::GetStackPtr => "ygen.getStackPtr",
            DefinedIntrinsic::GetFramePtr => "ygen.getFramePtr",
        }
    }

    /// Returns the return type of the instrinc
    #[inline]
    pub fn ret(&self) -> TypeMetadata {
        self.ty.ret
    }

    /// Returns the arguments of the instrinc
    #[inline]
    pub fn args(&self) -> Vec<TypeMetadata> {
        self.ty.args.iter().map(|(_, ty)| *ty).collect()
    }
}

impl FromStr for Intrinsic {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ygen.getStackPtr" => Ok(Intrinsic::get_sp()),
            "ygen.getFramePtr" => Ok(Intrinsic::get_fp()),
            _ => Err(()),
        }
    }
}