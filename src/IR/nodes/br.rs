use std::collections::HashMap;

use crate::Support::ColorClass;
use crate::IR::block::BlockId;
use crate::IR::{Function, Type, Var};

use super::{Br, BrCond, Ir};

impl Ir for Br<BlockId> {
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

    fn compile(&self, registry: &mut crate::Target::TargetBackendDescr, module: &mut crate::prelude::Module) {
        registry.compile_br(self, module)
    }
    
    fn uses(&self, _: &crate::prelude::Var) -> bool {
        false
    }
    
    fn is(&self, other: &Box<dyn Ir>) -> bool {
        other.dump() == self.dump()
    }
    
    fn compile_dir(&self, compiler: &mut crate::CodeGen::IrCodeGenHelper, block: &crate::prelude::Block, module: &mut crate::prelude::Module) {
        compiler.compile_br(&self, &block, module)
    }
    
    fn maybe_inline(&self, _: &HashMap<String, Type>) -> Option<Box<dyn Ir>> {
        None
    }
    
    fn eval(&self) -> Option<Box<dyn Ir>> {
        None
    }
    
    fn inputs(&self) -> Vec<Var> {
        vec![]
    }
    
    fn output(&self) -> Option<Var> {
        None
    }
}

impl Ir for BrCond<Var, BlockId, BlockId> {
    fn dump(&self) -> String {
        format!("br cond {} {}, {}", self.inner1.name, self.inner2.name, self.inner3.name)
    }

    fn dumpColored(&self, profile: crate::Support::ColorProfile) -> String {
        format!("{} {} {} {}, {}",
            profile.markup("br", ColorClass::Instr),
            profile.markup("cond", ColorClass::Instr),
            profile.markup(&self.inner1.name, ColorClass::Var), 
            profile.markup(&self.inner2.name, ColorClass::Var), 
            profile.markup(&self.inner3.name, ColorClass::Var),
        )
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn verify(&self, _: crate::prelude::FunctionType) -> Result<(), crate::prelude::VerifyError> {
        // TODO: Check if the blocks and the var exits
        
        Ok(())
    }

    fn clone_box(&self) -> Box<dyn Ir> {
        Box::from( self.clone() )
    }

    fn compile(&self, registry: &mut crate::Target::TargetBackendDescr, module: &mut crate::prelude::Module) {
       registry.compile_br_cond(self, module)
    }

    fn uses(&self, var: &Var) -> bool {
        if self.inner1.name.to_owned() == var.name.to_owned() {
            true
        } else {
            false
        }
    }
    
    fn compile_dir(&self, compiler: &mut crate::CodeGen::IrCodeGenHelper, block: &crate::prelude::Block, module: &mut crate::prelude::Module) {
        compiler.compile_br_cond(&self, &block, module)
    }

    fn maybe_inline(&self, _: &HashMap<String, Type>) -> Option<Box<dyn Ir>> {
        None
    }
    
    fn eval(&self) -> Option<Box<dyn Ir>> {
        if self.inner2.name == self.inner3.name {
            Some(Br::new( self.inner3.to_owned() ))
        } else { None }
    }
    
    fn inputs(&self) -> Vec<Var> {
        vec![self.inner1.to_owned()]
    }
    
    fn output(&self) -> Option<Var> {
        None
    }
}

/// This trait is used for building br nodes
pub trait BuildBr<T> {
    /// Builds a br node
    fn BuildBr(&mut self, val: T);
}

impl BuildBr<&BlockId> for Function {
    fn BuildBr(&mut self, to: &BlockId) {
        let block = self.blocks.back_mut().expect("the IRBuilder needs to have an current block\nConsider creating one");

        block.push_ir(Br::new( to.to_owned() ));
    }
}

/// This trait is used for building br condition nodes
pub trait BuildBrCond<T, U, Z> {
    /// Builds a br condition node
    /// 
    /// Jumps to iftrue if the value is not 0 else to iffalse
    fn BuildBrCond(&mut self, val: T, iftrue: U, iffalse: Z);
}

impl BuildBrCond<Var, &BlockId, &BlockId> for Function {
    fn BuildBrCond(&mut self, val: Var, iftrue: &BlockId, iffalse: &BlockId) {
        let block = self.blocks.back_mut().expect("the IRBuilder needs to have an current block\nConsider creating one");

        block.push_ir( BrCond::new(val, iftrue.to_owned(), iffalse.to_owned()));
    }
}