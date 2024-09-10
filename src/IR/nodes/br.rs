use crate::{Support::ColorClass, IR::Block};

use super::{Br, Ir};

impl Ir for Br<Box<Block>> {
    fn dump(&self) -> String {
        format!("br {}", self.inner1.name)
    }

    fn dumpColored(&self, profile: crate::Support::ColorProfile) -> String {
        format!("{} {}",
            profile.markup("br", ColorClass::Instr),
            profile.markup(&self.inner1.name, ColorClass::Var),
        )
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn verify(&self, _: crate::prelude::FunctionType) -> Result<(), crate::prelude::VerifyError> {
        // TODO: Check if block exists
        
        Ok(())
    }

    fn clone_box(&self) -> Box<dyn Ir> {
        Box::from( self.clone() )
    }

    fn compile(&self, registry: &mut crate::Target::TargetBackendDescr) {
        registry.compile_br(self)
    }
    
    fn uses(&self, _: &crate::prelude::Var) -> bool {
        false
    }
    
    fn is(&self, other: &Box<dyn Ir>) -> bool {
        other.dump() == self.dump()
    }
}