use std::collections::HashMap;

use crate::IR::{Function, Type, Var};
use crate::Support::ColorClass;

use super::{EvalOptVisitor, IROperand, Ir, Store};

impl Ir for Store {
    fn dump(&self) -> String {
        format!("store {} {}, {}", self.inner2.get_ty(), self.inner2, self.inner1.name)
    }

    fn dumpColored(&self, profile: crate::Support::ColorProfile) -> String {
        format!("{} {} {} {}",
            profile.markup("store", ColorClass::Instr),
            profile.markup(&self.inner2.get_ty().to_string(), ColorClass::Ty),
            profile.markup(&self.inner2.to_string(), ColorClass::Var),
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

    fn uses(&self, var: &Var) -> bool {
        if let IROperand::Var(value) = &self.inner2 {
            if value.name == var.name {
                return true;
            }
        }
        
        false
    }
    
    fn inputs(&self) -> Vec<Var> {
        let mut inputs = Vec::new();
        
        if let IROperand::Var(var) = &self.inner2 {
            inputs.push(var.to_owned());
        }

        inputs.push(self.inner1.to_owned());

        inputs
    }
    
    fn inputs_mut(&mut self) -> Vec<&mut Var> {
        let mut inputs = Vec::new();
        
        if let IROperand::Var(var) = &mut self.inner2 {
            inputs.push(var);
        }

        inputs.push(&mut self.inner1);

        inputs
    }
    
    fn output(&self) -> Option<Var> {
        None // technicly the ptr is the output
    }

    fn ty(&self) -> Option<crate::prelude::TypeMetadata> {
        Some(self.inner2.get_ty())
    }
}

impl EvalOptVisitor for Store {
    fn maybe_inline(&self, const_values: &HashMap<String, Type>) -> Option<Box<dyn Ir>> {
        if let IROperand::Var(value) = &self.inner2 {
            if let Some(constant) = const_values.get(&value.name) {
                return Some( Store::new(self.inner1.to_owned(), IROperand::Type(*constant)) );
            } 
        }
        
        None
    }
    
    fn eval(&self) -> Option<Box<dyn Ir>> {
        None
    }
    
}

/// The `BuildStore` trait is used for overloading the `BuildStore` method
pub trait BuildStore<T, U> {
    /// the `store` node, stores a value into a allocted pointer
    fn BuildStore(&mut self, target: T, value: U);
}

impl BuildStore<Var, Var> for Function {
    fn BuildStore(&mut self, target: Var, value: Var) {
        let block = self.blocks.back_mut().expect("the IRBuilder needs to have an current block\nConsider creating one");

        block.push_ir( Store::new(target, IROperand::Var(value)) );
    }
}

impl BuildStore<Var, Type> for Function {
    fn BuildStore(&mut self, target: Var, value: Type) {
        let block = self.blocks.back_mut().expect("the IRBuilder needs to have an current block\nConsider creating one");

        block.push_ir( Store::new(target, IROperand::Type(value)) );
    }
}