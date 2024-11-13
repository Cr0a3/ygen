use crate::IR::{Function, TypeMetadata, Var};
use crate::Support::ColorClass;

use super::{Alloca, EvalOptVisitor, Ir};

impl Ir for Alloca {
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
    
    fn inputs(&self) -> Vec<Var> {
        vec![]
    }
    
    fn inputs_mut(&mut self) -> Vec<&mut Var> {
        vec![]
    }
    
    fn output(&self) -> Option<Var> {
        Some(self.inner1.to_owned())
    }
}

impl EvalOptVisitor for Alloca {
    fn maybe_inline(&self, _: &std::collections::HashMap<String, crate::prelude::Type>) -> Option<Box<dyn Ir>> {
        None
    }
    
    fn eval(&self) -> Option<Box<dyn Ir>> {
        None
    }
}

impl Alloca {
    /// Returns the output variable
    pub fn getOut(&self) -> Var {
        self.inner1.to_owned()
    }

    /// Returns the type which is allocated
    pub fn getTypeToAlloc(&self) -> TypeMetadata {
        self.inner2
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