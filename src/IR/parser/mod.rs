//! This module contains the logic required for parsing ygen-ir files
//! It defines the Lexer and the Parser
//! Additionaly it provides an easy to use function which is a wrapper around the lexing, parsing unit

use std::collections::HashMap;
use std::{error::Error, fmt::Display};
use crate::Support;

use super::Module;

/// Ygen-Ir lexing
pub mod lexer;

/// Ygen-Ir parsing
pub mod parser;

/// Ygen-Ir semnatic checks
pub mod semnatic;

/// Ygen-Ir ir emittment
pub mod gen;

/// An error which can occure during one of the ygen-ir lexing, parsing, gen steps
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IrError {
    /// Unexpected token
    UnexpectedToken(lexer::Token),
}

impl Display for IrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            IrError::UnexpectedToken(token) => {
                let mut fab = Support::Error::new("unexpected token", "", token.loc.line.to_string(), token.loc.coloumn.to_string());

                fab.deactivateLocationDisplay();

                fab.addWhere("unexpected token", token.loc.coloumn, token.loc.length);

                fab.to_string()
            }
        })
    }
}

impl Error for IrError {}

impl Module {
    /// parses the input string into a module
    pub fn parse<T: Into<String>>(input: T) -> Result<Self, IrError> {
        let input = input.into();

        let mut functions = HashMap::new();
        let mut consts = HashMap::new();

        let mut lexer = lexer::IrLexer::new(input);
        lexer.lex()?;

        let mut parser = parser::IrParser::new(lexer.out);
        parser.parse()?;

        semnatic::IrSemnatic::new(&parser.out).verify()?;

        let mut gen = gen::IrGen::new(parser.out);

        gen.gen_funcs(&mut functions);
        gen.gen_consts(&mut consts);

        Ok(Module {
            funcs: functions,
            consts: consts,
        })
    }
}