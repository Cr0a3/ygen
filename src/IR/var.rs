use super::{Block, TypeMetadata};

/// A variable
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Var {
    pub(crate) name: String,
    pub(crate) ty: TypeMetadata,
}

impl Var {
    /// Creats an new variable
    pub fn new(block: &mut Block, ty: TypeMetadata) -> Self {
        Self {
            name: format!("%{}", block.reqVarName()),
            ty: ty,
        }
    } 
}