use std::collections::HashMap;

use crate::IR::{Function, Type, TypeMetadata, Var};
use crate::Support::ColorClass;

use super::{Store, Ir};

impl Ir for Store<Var, Var> {
    fn dump(&self) -> String {
        format!("store {} {}, {}", self.inner2.ty, self.inner2.name, self.inner1.name)
    }

    fn dumpColored(&self, profile: crate::Support::ColorProfile) -> String {
        format!("{} {} {} {}",
            profile.markup("store", ColorClass::Instr),
            profile.markup(&self.inner2.ty.to_string(), ColorClass::Ty),
            profile.markup(&self.inner2.name, ColorClass::Var),
            profile.markup(&self.inner1.name, ColorClass::Var),
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
        registry.compile_store(self)
    }

    fn uses(&self, var: &Var) -> bool {
        if self.inner2.name == var.name {
            true
        } else {
            false
        }
    }
    
    fn compile_dir(&self, compiler: &mut crate::CodeGen::IrCodeGenHelper, block: &crate::prelude::Block) {
        compiler.compile_store(&self, &block)
    }
    
    fn maybe_inline(&self, const_values: &HashMap<String, Type>) -> Option<Box<dyn Ir>> {
        if let Some(constant) = const_values.get(&self.inner2.name) {
            Some( Store::new(self.inner1.to_owned(), *constant) )
        } else { None }
    }
    
    fn eval(&self) -> Option<Box<dyn Ir>> {
        None
    }
    
    fn inputs(&self) -> Vec<Var> {
        vec![self.inner2.to_owned()]
    }
    
    fn output(&self) -> Option<Var> {
        Some(self.inner1.to_owned())
    }
}

impl Ir for Store<Var, Type> {
    fn dump(&self) -> String {
        let tmp: TypeMetadata = self.inner2.into();
        format!("store {} {}, {}", tmp, self.inner2.val(), self.inner1.name)
    }

    fn dumpColored(&self, profile: crate::Support::ColorProfile) -> String {
        let tmp: TypeMetadata = self.inner2.into();

        format!("{} {} {}, {}",
            profile.markup("store", ColorClass::Instr),
            profile.markup(&tmp.to_string(), ColorClass::Ty),
            profile.markup(&self.inner2.val().to_string(), ColorClass::Value),
            profile.markup(&self.inner1.name, ColorClass::Var),
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
        registry.compile_store_ty(self)
    }
    
    fn compile_dir(&self, compiler: &mut crate::CodeGen::IrCodeGenHelper, block: &crate::prelude::Block) {
        compiler.compile_store_ty(&self, &block)
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
        Some(self.inner1.to_owned())
    }
}

/// The `BuildStore` trait is used for overloading the `BuildStore` method
pub trait BuildStore<T, U> {
    /// the `store` node, stores a value into a allocted pointer
    fn BuildStore(&mut self, target: T, value: U);
}

impl BuildStore<Var, Var> for Function {
    fn BuildStore(&mut self, target: Var, value: Var) {
        let block = self.blocks.get_mut(self.blocks.len() - 1).expect("the IRBuilder needs to have an current block\nConsider creating one");

        block.push_ir( Store::new(target, value) );
    }
}

impl BuildStore<Var, Type> for Function {
    fn BuildStore(&mut self, target: Var, value: Type) {
        let block = self.blocks.get_mut(self.blocks.len() - 1).expect("the IRBuilder needs to have an current block\nConsider creating one");

        block.push_ir( Store::new(target, value) );
    }
}