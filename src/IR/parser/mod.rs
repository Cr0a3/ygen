//! This module contains the logic required for parsing ygen-ir files
//! It defines the Lexer and the Parser
//! Additionaly it provides an easy to use function which is a wrapper around the lexing, parsing unit

use std::{error::Error, fmt::Display};
use lexer::Loc;

use crate::Support::{self, Colorize};

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
#[derive(Debug)]//, Clone, PartialEq, Eq)]
pub enum IrError {
    /// Unexpected token
    UnexpectedToken(lexer::Token),

    /// Unexpected character
    UnexpectedCharacter{
        /// The refering character
        chr: char, 
        /// The character location
        loc: Loc
    },

    /// the lexer ran out of characters
    OutOfChars,

    /// the parser ran out of tokens
    OutOfTokens,

    /// expected an end to the sequence, but found no end
    UndeterminedTokenSequence{
        /// The character location
        loc: Loc,
        /// expected either one of these
        expected: String,
    },

    /// An error of another type
    Boxed{
        /// The location
        loc: Loc,
        /// The box of the error
        err: Box<dyn Error>,
    },

    /// Expected token
    ExpectedTokenButFoundAnUnexpectedOne{ 
        /// the token which was found
        found: lexer::Token, 
        /// the token which was expected
        expected: lexer::Token 
    },

    /// unkown instruction
    UnkownInstrinc{
        /// the location
        loc: Loc,
        /// what was found
        found: String,
    },

    /// A unkown type
    UnkownType(lexer::Token),
}

impl Display for IrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            IrError::UnexpectedToken(token) => {
                let mut fab = Support::Error::new("unexpected token", "", token.loc.line.to_string(), token.loc.coloumn.to_string());

                fab.deactivateLocationDisplay();

                fab.setCodeLine(token.loc.line_string.to_string());
                let mut length = token.loc.length;

                if 0 == length {
                    length = 1;
                }

                fab.addWhere("unexpected token", token.loc.coloumn, length);

                fab.to_string()
            },
            
            IrError::UnexpectedCharacter {chr, loc} => {
                let mut fab = Support::Error::new(
                    "unexpected character", 
                    loc.line_string.to_string(), 
                    loc.line.to_string(), 
                    loc.coloumn.to_string()
                );

                fab.setCodeLine(loc.line_string.to_string());
                fab.addWhere(format!("unexpected character: {}", chr), loc.coloumn, 1);

                fab.deactivateLocationDisplay();

                fab.to_string()
            },

            IrError::OutOfChars => format!("{}: the lexer ran out of characters", "error".red().bold()),
            IrError::OutOfTokens => format!("{}: the parser ran out of tokens", "error".red().bold()),

            IrError::UndeterminedTokenSequence {loc, expected} => {
                let mut fab = Support::Error::new(
                    "undetermined token sequence", 
                    loc.line_string.to_string(), 
                    loc.line.to_string(), 
                    loc.coloumn.to_string()
                );

                fab.setCodeLine(loc.line_string.to_string());
                fab.addWhere(format!("expected either one of these: {}, found nothing", expected), loc.coloumn, 1);

                fab.deactivateLocationDisplay();

                fab.to_string()
            },

            IrError::Boxed {loc, err} => {
                let mut fab = Support::Error::new(
                    format!("{}", err), 
                    loc.line_string.to_string(), 
                    loc.line.to_string(), 
                    loc.coloumn.to_string()
                );

                fab.setCodeLine(loc.line_string.to_string());

                fab.deactivateLocationDisplay();

                fab.to_string()
            }
            
            IrError::ExpectedTokenButFoundAnUnexpectedOne{found, expected} => {
                let mut fab = Support::Error::new("expected a specific token but found another one", "", found.loc.line.to_string(), found.loc.coloumn.to_string());

                fab.deactivateLocationDisplay();

                fab.setCodeLine(found.loc.line_string.to_string());

                let mut length = found.loc.length;

                if 0 == length {
                    length = 1;
                }

                fab.addWhere(format!("expected following token: {:?} but found {:?}", expected.typ.name(), found.typ), found.loc.coloumn, length);

                fab.to_string()                
            }
            
            IrError::UnkownType(typ) => {
                let mut fab = Support::Error::new("unknown type", "", typ.loc.line.to_string(), typ.loc.coloumn.to_string());

                fab.deactivateLocationDisplay();

                fab.setCodeLine(typ.loc.line_string.to_string());
                fab.addWhere("unkown type", typ.loc.coloumn, typ.loc.length);

                fab.to_string()

            }
            
            IrError::UnkownInstrinc { loc, found } => {
                let mut fab = Support::Error::new(format!("unknown instric: {}", found), "", loc.line.to_string(), loc.coloumn.to_string());

                fab.deactivateLocationDisplay();

                fab.setCodeLine(loc.line_string.to_string());
                fab.addWhere("unkown ir instr", loc.coloumn, loc.length);

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

        let mut lexer = lexer::IrLexer::new(input);
        lexer.lex()?;

        let mut parser = parser::IrParser::new(lexer.out);
        parser.parse()?;

        semnatic::IrSemnatic::new(&parser.out).verify()?;

        let mut gen = gen::IrGen::new(parser.out);

        gen.gen_funcs();
        gen.gen_consts();

        Ok(gen.module())
    }
}