use std::collections::HashMap;

use crate::IR::{Const, Function};

use super::parser::IrStmt;

/// Emits the ygen ir statements (emitted by the parser) into real definable ir statements
#[allow(unused)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IrGen {
    input: Vec<IrStmt>,
}

impl IrGen {
    /// Creates a new ir generator
    pub fn new(input: Vec<IrStmt>) -> Self {
        Self {
            input: input,
        }
    }

    /// generates the functions
    #[allow(unused)]
    pub fn gen_funcs(&mut self, list: &mut HashMap<String, Function>) {
        
    }

    /// generates the consts
    #[allow(unused)]
    pub fn gen_consts(&mut self, list: &mut HashMap<String, Const>) {
        
    }
}