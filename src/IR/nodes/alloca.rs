use crate::IR::{Function, TypeMetadata, Var};
use crate::Support::ColorClass;

use super::{Alloca, Ir};

impl Ir for Alloca<Var, TypeMetadata> {
    fn dump(&self) -> String {
        format!("{} = alloca {}", self.inner1.name, self.inner2)
    }

    fn dumpColored(&self, profile: crate::Support::ColorProfile) -> String {
        format!("{} = {} {}",
            profile.markup(&self.inner1.name, ColorClass::Var),
            profile.markup("alloca", ColorClass::Instr), 
            profile.markup(&self.inner2.to_string(), ColorClass::Ty),
        )
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn verify(&self, _: crate::prelude::FunctionType) -> Result<(), crate::prelude::VerifyError> {
        Ok(())
    }

    fn clone_box(&self) -> Box<dyn Ir> {
        Box::new( self.clone() )
    }

    fn compile(&self, registry: &mut crate::Target::TargetBackendDescr, module: &mut crate::prelude::Module) {
        registry.compile_alloca(self, module)
    }
    
    fn compile_dir(&self, compiler: &mut crate::CodeGen::IrCodeGenHelper, block: &crate::prelude::Block, module: &mut crate::prelude::Module) {
        compiler.compile_alloca(&self, &block, module)
    }
    
    fn maybe_inline(&self, _: &std::collections::HashMap<String, crate::prelude::Type>) -> Option<Box<dyn Ir>> {
        None
    }
    
    fn eval(&self) -> Option<Box<dyn Ir>> {
        None
    }
    
    fn inputs(&self) -> Vec<Var> {
        vec![]
    }
    
    fn output(&self) -> Option<Var> {
        Some(self.inner1.to_owned())
    }
}

impl Function {
    /// Builds an stack allocation (the out var is the pointer to the allocated stack region)
    pub fn BuildAlloca(&mut self, ty: TypeMetadata) -> Var {
        let block = self.blocks.back_mut().expect("the IRBuilder needs to have an current block\nConsider creating one");
        
        let out = Var::new(block, TypeMetadata::ptr);

        block.push_ir( Alloca::new(out.clone(), ty) );

        out
    }
}