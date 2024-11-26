use std::collections::HashMap;

use crate::Support::ColorClass;
use crate::IR::{Function, Type, TypeMetadata, Var};

use super::{EvalOptVisitor, IROperand, Ir, Load};

impl Ir for Load {
    fn dump(&self) -> String {
        format!("{} = load {} {}", self.inner1.name, self.inner2, self.inner3.fmt_2())
    }

    fn dumpColored(&self, profile: crate::Support::ColorProfile) -> String {
        format!("{} = {} {} {}", 
            profile.markup(&self.inner1.name, ColorClass::Var), 
            profile.markup("load", ColorClass::Instr), 
            profile.markup(&self.inner2.to_string(), ColorClass::Ty),
            profile.markup(&self.inner3.to_string(), ColorClass::Var),
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
        if let IROperand::Var(ptr) = &self.inner3 {
            if ptr.name == var.name { return true; }
        }
        
        false
    }
    
    fn inputs(&self) -> Vec<Var> {
        let mut inputs = Vec::new();
        if let IROperand::Var(var) = &self.inner3 { inputs.push(var.to_owned()); }
        inputs
    }
    
    fn inputs_mut(&mut self) -> Vec<&mut Var> {
        let mut inputs = Vec::new();
        if let IROperand::Var(var) = &mut self.inner3 { inputs.push(var); }
        inputs
    }
    
    
    fn output(&self) -> Option<Var> {
        Some(self.inner1.to_owned())
    }

    fn ty(&self) -> Option<TypeMetadata> {
        Some(self.inner2)
    }
}

impl EvalOptVisitor for Load {
    fn maybe_inline(&self, _: &HashMap<String, Type>) -> Option<Box<dyn Ir>> {
        None
    }
    
    fn eval(&self) -> Option<Box<dyn Ir>> {
        None
    }
}

impl Function {
    /// the load instruction loads an value from an pointer into a normal variable
    pub fn BuildLoad(&mut self, ptr: Var, ty: TypeMetadata) -> Var {
        let block = self.blocks.back_mut().expect("the IRBuilder needs to have an current block\nConsider creating one");
        
        let out = Var::new(block, ty);

        block.push_ir( Load::new(out.clone(), ty, IROperand::Var(ptr)) );

        out
    }
}