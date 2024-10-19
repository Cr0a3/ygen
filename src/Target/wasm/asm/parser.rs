use std::{any::Any, collections::VecDeque, error::Error, fmt::Display};

use crate::{Support::ColorProfile, Target::Compiler};

use super::{instr::*, lexer::Token};

/// The parser for parsing wasn assembly instructions
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct wasmParser {
    pub(crate) tokens: VecDeque<Token>,
    /// The output instruction
    pub out: Option<WasmMCInstr>,
}

impl wasmParser {
    /// Creates an new x64 assembly parser
    pub(crate) fn new(tokens: Vec<Token>) -> Self {
        Self { 
            tokens: tokens.into(), 
            out: None, 
        }
    }

    /// parses the tokens (output will be saved in `self.out`)
    pub(crate) fn parse(&mut self) -> Result<(), Box<dyn Error>> {
        println!("{:?}", self.tokens);

        let mut instr_string = String::new();
        let prefix_string = if let Some(Token::Ident(prefix)) = self.tokens.front() {
            instr_string = prefix.to_owned();
            Some(prefix.clone())
        } else {
            Err(ParsingError::FirstTokenNeedsToBeIdent)?
        };

        let mut prefix = None;

        self.tokens.pop_front();

        if let Some(Token::Dot) = self.tokens.front() {
            prefix = Some(prefix_string.unwrap().into());

            self.tokens.pop_front();

            instr_string = if let Some(Token::Ident(instr)) = self.tokens.front() {
                instr.to_owned()
            } else {
                Err(ParsingError::MnemonicNeedsToBeIdent)?
            };

            self.tokens.pop_front();
        }

        let instr = instr_string.into();

        if let Some(Token::Num(num)) = self.tokens.front() {
            let num = *num;
            self.tokens.pop_front();

            let mut op = WasmOperand::Var(num as i32);

            // the op is not a var in some special cases for sepific mnemonics
            // for those we now check here
            if instr == WasmMnemonic::Const {
                // and turn it into a imm
                op = WasmOperand::Const(num);
            }

            self.out = Some(WasmMCInstr::with1(prefix, instr, op));
        } else {
            self.out = Some(WasmMCInstr::with0(prefix, instr));
        }

        Ok(())
    }
}

impl Compiler for wasmParser {
    fn new(&self, tokens: Vec<Box<dyn Any>>) -> Box<dyn Compiler> {
        let mut casted = Vec::new();

        for token in tokens {
            casted.push(
                *token.downcast::<Token>().expect("the x64 parser expects that the input tokens are also x64 tokens")
            );
        }

        Box::from( wasmParser::new(casted) )
    }

    fn parse(&mut self) -> Result<(), Box<dyn Error>> {
        self.parse()
    }

    fn out(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(self.out.as_ref().unwrap().encode()?.0)
    }

    fn boxed(&self) -> Box<dyn Compiler> {
        Box::from(self.clone())
    }
    
    fn coloredOut(&self, _: ColorProfile) -> String {
        // not yet supported
        self.out.as_ref().unwrap().to_string()
    }
    
    fn printOut(&self) -> String {
        self.out.as_ref().unwrap().to_string()
    }
}

/// An error which can occure during parsing
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(missing_docs)]
pub(crate) enum ParsingError {
    FirstTokenNeedsToBeIdent,
    MnemonicNeedsToBeIdent
}

impl Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ParsingError::FirstTokenNeedsToBeIdent => "wasm string should start with an ident",
            ParsingError::MnemonicNeedsToBeIdent => "ygen expects that wasm instructions have their mnemonic as idents",
        })
    }
}

impl Error for ParsingError {}