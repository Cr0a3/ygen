use std::fmt::Display;

use crate::Support::ColorClass;
use crate::IR::{Function, Type, TypeMetadata, Var, VerifyError};

use super::{Assign, Cmp, Ir};

/// The "compare mode" (e.g: ls is equal to rs)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CmpMode {
    /// ls == rs
    Eqal,
    /// ls != rs
    NotEqal,
    /// ls > rs
    GreaterThan,
    /// ls < rs
    LessThan,
    /// ls >= rs
    GreaterThanOrEqual,
    /// ls <= rs
    LessThanOrEqual,
}

impl Display for CmpMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            CmpMode::Eqal => "eq",
            CmpMode::NotEqal => "ne",
            CmpMode::GreaterThan => "ge",
            CmpMode::LessThan => "le",
            CmpMode::GreaterThanOrEqual => "gte",
            CmpMode::LessThanOrEqual => "lte",
        })
    }
}

impl Ir for Cmp {
    fn dump(&self) -> String {
        format!("{} = cmp {} {} {}, {}", self.out.name, self.mode, self.ls.ty, self.ls.name, self.rs.name)
    }

    fn dumpColored(&self, profile: crate::Support::ColorProfile) -> String {
        format!("{} = {} {} {} {}, {}",
            profile.markup(&self.out.name, ColorClass::Var),
            profile.markup("cmp", ColorClass::Instr),
            profile.markup(&format!("{}", self.mode), ColorClass::Ty),
            profile.markup(&format!("{}", self.ls.ty), ColorClass::Ty),
            profile.markup(&self.ls.name, ColorClass::Var),
            profile.markup(&self.rs.name, ColorClass::Var),
        )
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn verify(&self, _: crate::prelude::FunctionType) -> Result<(), crate::prelude::VerifyError> {
        if self.ls.ty != self.rs.ty {
            Err(VerifyError::Op0Op1TyNoMatch(self.ls.ty, self.rs.ty))?
        }

        Ok(())
    }

    fn clone_box(&self) -> Box<dyn Ir> {
        Box::new( self.clone() )
    }

    fn compile(&self, registry: &mut crate::Target::TargetBackendDescr, module: &mut crate::prelude::Module) {
        registry.compile_cmp(self, module)
    }

    fn uses(&self, other: &Var) -> bool {
        if other.name == self.ls.name { true }
        else if other.name == self.rs.name { true }
        else { false }
    }
    
    fn compile_dir(&self, compiler: &mut crate::CodeGen::IrCodeGenHelper, block: &crate::prelude::Block, module: &mut crate::prelude::Module) {
        compiler.compile_cmp(&self, &block, module)
    }
    
    fn maybe_inline(&self, _: &std::collections::HashMap<String, crate::prelude::Type>) -> Option<Box<dyn Ir>> {
        None
    }
    
    fn eval(&self) -> Option<Box<dyn Ir>> {
        if self.ls == self.rs {
            let yes = match self.mode {
                CmpMode::Eqal => 1,
                CmpMode::NotEqal => 0,
                CmpMode::GreaterThan => 0,
                CmpMode::LessThan => 0,
                CmpMode::GreaterThanOrEqual => 1,
                CmpMode::LessThanOrEqual => 1,
            };

            Some(Assign::new(self.out.to_owned(), Type::from_int(
                self.out.ty, yes as f64
            )))
        } else { None }
    }
    
    fn inputs(&self) -> Vec<Var> {
        vec![self.ls.to_owned(), self.rs.to_owned()]
    }
    
    fn output(&self) -> Option<Var> {
        Some(self.out.to_owned())
    }
}

/// The trait `BuildCmp` is used to build the cmp node
pub trait BuildCmp {
    /// builds the compare node
    fn BuildCmp(&mut self, mode: CmpMode, ls: Var, rs: Var) -> Var;
}

impl BuildCmp for Function {
    fn BuildCmp(&mut self, mode: CmpMode, ls: Var, rs: Var) -> Var {
        let block = self.blocks.back_mut().expect("the IRBuilder needs to have an current block\nConsider creating one");
        
        let out = Var::new(block, TypeMetadata::i8);

        block.push_ir( Cmp::new(mode, ls, rs, out.to_owned()) );

        out
    }
}