use crate::Support::ColorClass;
use crate::IR::{IRBuilder, TypeMetadata, Var};

use super::{Ir, Load};

impl Ir for Load<Var, Var, TypeMetadata> {
    fn dump(&self) -> String {
        format!("{} = load {} {}", self.inner1.name, self.inner3, self.inner2.name)
    }

    fn dumpColored(&self, profile: crate::Support::ColorProfile) -> String {
        format!("{} = {} {} {}", 
            profile.markup(&self.inner1.name, ColorClass::Var), 
            profile.markup("load", ColorClass::Instr), 
            profile.markup(&self.inner3.to_string(), ColorClass::Ty),
            profile.markup(&self.inner2.name, ColorClass::Var),
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

    fn compile(&self, registry: &mut crate::Target::TargetBackendDescr) {
        registry.compile_load(self)
    }

    fn uses(&self, var: &Var) -> bool {
        if self.inner2.name == var.name {
            true
        } else {
            false
        }
    }
}

impl IRBuilder<'_> {
    /// the load instruction loads an value from an pointer into a normal variable
    pub fn BuildLoad(&mut self, ptr: Var, ty: TypeMetadata) -> Var {
        let block = self.blocks.get_mut(self.curr).expect("the IRBuilder needs to have an current block\nConsider creating one");
        
        let out = Var::new(block, ty);

        block.push_ir( Load::new(out.clone(), ptr, ty) );

        out
    }
}