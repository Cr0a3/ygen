use crate::{Support::ColorClass, IR::{Block, IRBuilder}};

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

/// This trait is used for building br nodes
pub trait BuildBr<T> {
    /// Builds a br node
    fn BuildBr(&mut self, val: T);
}

impl BuildBr<&Block> for IRBuilder<'_> {
    fn BuildBr(&mut self, to: &Block) {
        let block = self.blocks.get_mut(self.curr).expect("the IRBuilder needs to have an current block\nConsider creating one");

        block.push_ir(Br::new(Box::from(Block { // creating a new one in order to safe some memory space
            name: to.name.to_owned(),
            nodes: vec![], 
            varCount: 0 
        })));
    }
}