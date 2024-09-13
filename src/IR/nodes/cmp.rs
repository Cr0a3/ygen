use std::fmt::Display;

use crate::{Support::ColorClass, IR::{IRBuilder, Var, VerifyError}};

use super::{Cmp, Ir};

/// The "compare mode" (e.g: ls is equal to rs)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
        format!("cmp {} {} {}, {}", self.mode, self.ls.ty, self.ls.name, self.rs.name)
    }

    fn dumpColored(&self, profile: crate::Support::ColorProfile) -> String {
        format!("{} {} {} {}, {}",
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

    fn compile(&self, registry: &mut crate::Target::TargetBackendDescr) {
        registry.compile_cmp(self)
    }

    fn uses(&self, other: &Var) -> bool {
        if other.name == self.ls.name { true }
        else if other.name == self.rs.name { true }
        else { false }
    }
}

/// The trait `BuildCmp` is used to build the cmp node
pub trait BuildCmp {
    /// builds the compare node
    fn BuildCmp(&mut self, mode: CmpMode, ls: Var, rs: Var) -> Var;
}

impl BuildCmp for IRBuilder<'_> {
    fn BuildCmp(&mut self, mode: CmpMode, ls: Var, rs: Var) -> Var {
        let block = self.blocks.get_mut(self.curr).expect("the IRBuilder needs to have an current block\nConsider creating one");
        
        let out = Var::new(block, ls.ty);

        block.push_ir( Cmp::new(mode, ls, rs, out.to_owned()) );

        out
    }
}