use crate::IR::{Const, Function, Module};

use super::parser::IrStmt;

/// Emits the ygen ir statements (emitted by the parser) into real definable ir statements
#[allow(unused)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IrGen {
    input: Vec<IrStmt>,

    funcs: Vec<Function>,
    consts: Vec<Const>,
}

impl IrGen {
    /// Creates a new ir generator
    pub fn new(input: Vec<IrStmt>) -> Self {
        Self {
            input: input,

            funcs: vec![],
            consts: vec![],
        }
    }

    /// generates the functions
    pub fn gen_funcs(&mut self) {
        todo!()
    }

    /// generates the consts
    pub fn gen_consts(&mut self) {
        todo!()   
    }

    /// emits the generated functions, constants, .. into a usable module
    pub fn module(&self) -> Module {
        let mut module = Module();

        for func in &self.funcs {
            module.add_raw( func.to_owned() );
        }

        for constant in &self.consts {
            module.add_raw_const( constant.to_owned() );
        }

        module
    }
}