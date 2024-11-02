use std::fmt::Display;

use crate::Support::{ColorClass, ColorProfile};

use super::{Block, TypeMetadata};

/// A variable
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Var {
    pub(crate) name: String,
    pub(crate) ty: TypeMetadata,
}

impl Var {
    /// Creats a new variable
    pub fn new(block: &mut Block, ty: TypeMetadata) -> Self {
        Self {
            name: format!("%{}", block.reqVarName()),
            ty: ty,
        }
    } 

    /// same as Display::fmt but with colors
    pub fn to_colored_string(&self, profile: ColorProfile) -> String {
        format!("{} {}", 
            profile.markup(&self.ty.to_string(), ColorClass::Ty),
            profile.markup(&self.name, ColorClass::Var)
        )
    }
}

impl crate::Support::AsAny for Var {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Creates a new variable
pub fn Var(block: &mut Block, ty: TypeMetadata) -> Var {
    Var::new(block, ty)
}

impl Display for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.ty, self.name)
    }
}