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
}

/// Creates a new variable
pub fn Var(block: &mut Block, ty: TypeMetadata) -> Var {
    Var::new(block, ty)
}