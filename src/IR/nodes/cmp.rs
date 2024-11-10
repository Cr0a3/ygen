use std::fmt::Display;

use crate::Support::ColorClass;
use crate::IR::{Function, Type, TypeMetadata, Var, VerifyError};

use super::{Assign, Cmp, EvalOptVisitor, IROperand, Ir, IsNode};

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
        format!("{} = cmp {} {} {}, {}", self.out.name, self.mode, self.ls.get_ty(), self.ls, self.rs)
    }

    fn dumpColored(&self, profile: crate::Support::ColorProfile) -> String {
        format!("{} = {} {} {} {}, {}",
            profile.markup(&self.out.name, ColorClass::Var),
            profile.markup("cmp", ColorClass::Instr),
            profile.markup(&format!("{}", self.mode), ColorClass::Ty),
            profile.markup(&format!("{}", self.ls.get_ty()), ColorClass::Ty),
            profile.markup(&self.ls.to_string(), ColorClass::Var),
            profile.markup(&self.rs.to_string(), ColorClass::Var),
        )
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn verify(&self, _: crate::prelude::FunctionType) -> Result<(), crate::prelude::VerifyError> {
        if self.ls.get_ty() != self.rs.get_ty() {
            Err(VerifyError::Op0Op1TyNoMatch(self.ls.get_ty(), self.rs.get_ty()))?
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
        if let IROperand::Var(ls) = &self.ls {
            if other.name == ls.name { return true; }
        }
        if let IROperand::Var(rs) = &self.rs {
            if other.name == rs.name { return true; }
        }

        false
    }
    
    fn compile_dir(&self, compiler: &mut crate::CodeGen::IrCodeGenHelper, block: &crate::prelude::Block, module: &mut crate::prelude::Module) {
        compiler.compile_cmp(&self, &block, module)
    }
    
    fn inputs(&self) -> Vec<Var> {
        let mut inputs = Vec::new();

        if let IROperand::Var(ls) = &self.ls {
            inputs.push(ls.to_owned());
        }
        if let IROperand::Var(rs) = &self.rs {
            inputs.push(rs.to_owned());
        }

        inputs
    }
    
    fn inputs_mut(&mut self) -> Vec<&mut Var> {
        let mut inputs = Vec::new();

        if let IROperand::Var(ls) = &mut self.ls {
            inputs.push(ls);
        }
        if let IROperand::Var(rs) = &mut self.rs {
            inputs.push(rs);
        }

        inputs
    }
    
    fn output(&self) -> Option<Var> {
        Some(self.out.to_owned())
    }
}

fn calc_based_on_mode(mode: &CmpMode, ls: &Type, rs: &Type, out: Var) -> Option<Box<dyn Ir>> {
    let condition_met = match mode {
        CmpMode::Eqal => ls.val() == rs.val(),
        CmpMode::NotEqal => ls.val() != rs.val(),
        CmpMode::GreaterThan => ls.val() > rs.val(),
        CmpMode::LessThan => ls.val() < rs.val(),
        CmpMode::GreaterThanOrEqual => ls.val() >= rs.val(),
        CmpMode::LessThanOrEqual => ls.val() <= rs.val(),
    };

    Some(Assign::new(out, Type::from_int(TypeMetadata::u8, condition_met as i8 as f64)))
}

impl EvalOptVisitor for Cmp {
    fn maybe_inline(&self, consts: &std::collections::HashMap<String, crate::prelude::Type>) -> Option<Box<dyn Ir>> {
        match (&self.ls, &self.rs) {
            (IROperand::Var(ls), IROperand::Var(rs)) => {
                match (consts.get(&ls.name), consts.get(&rs.name)) {
                    (Option::Some(ls), Option::Some(rs)) => calc_based_on_mode(&self.mode, ls, rs, self.out.to_owned()),
                    _ => None,
                }
            },
            _ => None
        }
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
}

impl IsNode for Cmp {
    fn is_cmp(&self) -> bool {
        true
    }
}

impl Cmp {
    /// Returns the mode with which the node compares
    pub fn getCmpMode(&self) -> CmpMode {
        self.mode
    }

    /// Returns the left side operand
    pub fn getLsVar(&self) -> Var {
        self.ls.get_var()
    }

    /// Returns the left side operand
    pub fn getLsConst(&self) -> Type {
        self.ls.get_typeconst()
    }
    
    /// Returns if the left side operand is a var
    pub fn isLsVar(&self) -> bool { self.ls.is_var() }
    
    /// Returns the right side operand
    pub fn getRsVar(&self) -> Var {
        self.rs.get_var()
    }

    /// Returns the right side operand
    pub fn getRsConst(&self) -> Type {
        self.rs.get_typeconst()
    }
    
    /// Returns if the rright side operand is a var
    pub fn isRsVar(&self) -> bool { self.rs.is_var() }

    /// Returns the output variable
    pub fn getOutput(&self) -> Var {
        self.out.to_owned()
    }

    /// Returns the type of the node
    pub fn getType(&self) -> TypeMetadata {
        self.out.ty
    }
}

/// The trait `BuildCmp` is used to build the cmp node
pub trait BuildCmp<T, U> {
    /// builds the compare node
    fn BuildCmp(&mut self, mode: CmpMode, ls: T, rs: U) -> Var;
}

impl BuildCmp<Var, Var> for Function {
    fn BuildCmp(&mut self, mode: CmpMode, ls: Var, rs: Var) -> Var {
        let block = self.blocks.back_mut().expect("the IRBuilder needs to have an current block\nConsider creating one");
        
        let out = Var::new(block, TypeMetadata::u8);

        block.push_ir( Cmp::new(mode, IROperand::Var(ls), IROperand::Var(rs), out.to_owned()) );

        out
    }
}

impl BuildCmp<Var, Type> for Function {
    fn BuildCmp(&mut self, mode: CmpMode, ls: Var, rs: Type) -> Var {
        let block = self.blocks.back_mut().expect("the IRBuilder needs to have an current block\nConsider creating one");
        
        let out = Var::new(block, TypeMetadata::u8);

        block.push_ir( Cmp::new(mode, IROperand::Var(ls), IROperand::Type(rs), out.to_owned()) );

        out
    }
}

impl BuildCmp<Type, Var> for Function {
    fn BuildCmp(&mut self, mode: CmpMode, ls: Type, rs: Var) -> Var {
        let block = self.blocks.back_mut().expect("the IRBuilder needs to have an current block\nConsider creating one");
        
        let out = Var::new(block, TypeMetadata::u8);

        block.push_ir( Cmp::new(mode, IROperand::Type(ls), IROperand::Var(rs), out.to_owned()) );

        out
    }
}

impl BuildCmp<Type, Type> for Function {
    fn BuildCmp(&mut self, mode: CmpMode, ls: Type, rs: Type) -> Var {
        let block = self.blocks.back_mut().expect("the IRBuilder needs to have an current block\nConsider creating one");
        
        let out = Var::new(block, TypeMetadata::u8);

        block.push_ir( Cmp::new(mode, IROperand::Type(ls), IROperand::Type(rs), out.to_owned()) );

        out
    }
}