use crate::IR::{Function, TypeMetadata, VecTy};

/// A list which allows and forbidds specific types
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(missing_docs)]
pub struct TargetBlackList {
    pub forbidden_types: Vec<TypeMetadata>,
    pub supports_vecs: bool,
    pub supported_vectors: Vec<VecTy>,
}

impl TargetBlackList {
    /// Creates an empty TargetBlackList which allows everything
    pub fn new() -> Self {
        Self {
            forbidden_types: Vec::new(),
            supports_vecs: true,
            supported_vectors: Vec::new(),
        }
    }

    /// Adds a forbidden type
    pub fn add_forbid(&mut self, forbid: TypeMetadata) {
        self.forbidden_types.push(forbid);
    }

    /// Forbidds vectors
    pub fn disalow_vecs(&mut self) {
        self.supports_vecs = false;
    }

    /// Allows vectors
    pub fn allow_vec(&mut self) {
        self.supports_vecs = true;
    }

    /// Adds an allowed vector type
    pub fn add_supported_vec(&mut self, vec: VecTy) {
        self.supported_vectors.push(vec);
    }

    fn check_ty(&self, ty: &TypeMetadata) -> bool {
        if !self.supports_vecs && ty.isVectorTy() {
            return self.supported_vectors.contains(&ty.getVectorTy());
        }

        !self.forbidden_types.contains(&ty)
    }

    /// Checks the function for illegal types
    pub fn check(&self, func: &Function) {
        for (_, arg) in &func.ty.args {
            if !self.check_ty(arg) {
                panic!("function {} contains unsupported type: {}", func.name, arg);
            }
        }

        if !self.check_ty(&func.ty.ret) {
            panic!("function {} returns unsupported type: {}", func.name, func.ty.ret);
        }

        for block in &func.blocks {
            for node in &block.nodes {
                if let Some(ty) = node.ty() {
                    if !self.check_ty(&ty) {
                        panic!("function {} has a node which uses a unsupported type: {}", func.name, ty);
                    }
                }

                if node.is_alloca() {
                    if !self.check_ty(&TypeMetadata::ptr) {
                        panic!("function {} has a unsupported type: {}", func.name, TypeMetadata::ptr);
                    }
                }
            }
        }
    }
}