use std::{collections::VecDeque, error::Error, fmt::Display, str::FromStr};

use crate::Target::{x64Reg, Compiler, Reg};

use super::{instr::*, Token};

/// The parser for parsing x64 assembly instructions
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct x64Parser {
    pub(crate) tokens: VecDeque<Token>,
    /// The output instruction
    pub out: Option<Instr>,
}

impl x64Parser {
    /// Creates an new x64 assembly parser
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { 
            tokens: tokens.into(), 
            out: None, 
        }
    }

    /// parses the tokens (output will be saved in `self.out`)
    pub fn parse(&mut self) -> Result<(), Box<dyn Error>>{
        let instr = if let Some(Token::Ident(instr)) = self.tokens.front() {
            instr.clone()
        } else {
            Err(ParsingError::FirstTokenNeedsToBeIdent)?
        };

        self.tokens.pop_front(); // advance

        let string = instr.to_ascii_lowercase();
        let parsed = Mnemonic::from_str(&string);
        match parsed {
            Ok(mne) => self.parse_instr(mne)?,
            Err(_) => Err(ParsingError::UnknownInstruction(string))?
        }

        Ok(())
    }

    fn parse_instr(&mut self, mne: Mnemonic) -> Result<(), Box<dyn Error>> {
        let mut instr = Instr {
            mnemonic: mne,
            op1: None,
            op2: None,
        };

        let mut first_op = false;
        if let Some(Token::Num(n)) = self.tokens.front() {
            instr.op1 = Some(Operand::Imm(*n));
            self.tokens.pop_front(); // advance
            first_op = true;
        } else if let Some(Token::Ident(reg)) = self.tokens.front() {
            if let Some(reg) = x64Reg::parse(reg.to_string()) {
                instr.op1 = Some(Operand::Reg(reg.boxed()))
            } else {
                Err(ParsingError::UnknownRegOrUnexpectedIdent(reg.to_string()))?
            }
            self.tokens.pop_front(); // advance
            first_op = true;
        } else if let Some(Token::L_Bracket) = self.tokens.front() {
            instr.op1 = Some(Operand::Mem(self.parse_mem()?));
            first_op = true;
        }

        if first_op {
            if let Some(Token::Comma) = self.tokens.front() {
                self.tokens.pop_front(); // advance
                if let Some(Token::Num(n)) = self.tokens.front() {
                    instr.op2 = Some(Operand::Imm(*n));
                    self.tokens.pop_front(); // advance
                } else if let Some(Token::Ident(reg)) = self.tokens.front() {
                    if let Some(reg) = x64Reg::parse(reg.to_string()) {
                        instr.op2 = Some(Operand::Reg(reg.boxed()))
                    } else {
                        Err(ParsingError::UnknownRegOrUnexpectedIdent(reg.to_string()))?
                    }
                    self.tokens.pop_front(); // advance
                } else if let Some(Token::L_Bracket) = self.tokens.front() {
                    instr.op2 = Some(Operand::Mem(self.parse_mem()?));
                }else {
                    Err(ParsingError::CommaWithoutOperandAfter)?
                }
            } else if self.tokens.len() > 0 {
                Err(ParsingError::UnexpectedTokens(self.tokens.clone().into()))?
            }
        } else if self.tokens.len() > 0 {
            Err(ParsingError::UnexpectedTokens(self.tokens.clone().into()))?
        }

        instr.verify()?;

        self.out = Some(instr);

        Ok(())
    }

    fn parse_mem(&mut self) -> Result<MemOp, Box<dyn Error>> {
        let mut mem = MemOp {
            base: None,
            index: None,
            scale: 1,
            displ: 0,
        };

        self.tokens.pop_front();

        if let Some(Token::Num(n)) = self.tokens.front() {
            mem.displ = *n as isize;
            self.tokens.pop_front(); // advance
        } else if let Some(Token::Ident(reg)) = self.tokens.front() {
            if let Some(reg) = x64Reg::parse(reg.to_string()) {
                mem.base = Some(reg.boxed());
            } else {
                Err(ParsingError::UnknownRegOrUnexpectedIdent(reg.to_string()))?
            }
            self.tokens.pop_front(); // advance
        } else if let Some(Token::R_Bracket) = self.tokens.front() {
            Err(ParsingError::EmptyMemoryDisplacment)?
        } else if let Some(token) = self.tokens.front() { 
            Err(ParsingError::UnexpectedToken(token.clone()))? 
        } else { todo!() }

        if let Some(Token::L_Bracket) = self.tokens.front() {} else {
            let mut sub = false;
            if let Some(Token::Sub) = self.tokens.front() { sub = true; self.tokens.pop_front(); }
            if let Some(Token::Add) = self.tokens.front() { sub = false; self.tokens.pop_front(); }

            if let Some(Token::Num(n)) = self.tokens.front() {
                if sub { mem.displ -= *n as isize; }
                else { mem.displ += *n as isize; }
                self.tokens.pop_front(); // advance
            } else if let Some(Token::Ident(reg)) = self.tokens.front() {
                if let Some(reg) = x64Reg::parse(reg.to_string()) {
                    mem.index = Some(reg.boxed());
                } else {
                    Err(ParsingError::UnknownRegOrUnexpectedIdent(reg.to_string()))?
                }
                self.tokens.pop_front(); // advance
            } else if let Some(token) = self.tokens.front() { 
                Err(ParsingError::UnexpectedToken(token.clone()))? 
            }
        }

        Ok(mem)
    }
}

impl Compiler for x64Parser {
    fn new(&self, tokens: Vec<Token>) -> Box<dyn Compiler> {
        Box::from( x64Parser::new(tokens) )
    }

    fn parse(&mut self) -> Result<(), Box<dyn Error>> {
        self.parse()
    }

    fn out(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(self.out.as_ref().unwrap().compile()?)
    }

    fn boxed(&self) -> Box<dyn Compiler> {
        Box::from(self.clone())
    }
}

/// An error which can occure during parsing
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParsingError {
    /// The first token is not an identifier
    FirstTokenNeedsToBeIdent,
    /// Unknown instruction
    UnknownInstruction(String),
    /// The identifier was unexpected or isn't an valid register
    UnknownRegOrUnexpectedIdent(String),
    /// these tokens are unexpected
    UnexpectedTokens(Vec<Token>),
    /// There is an unexpected comma
    CommaWithoutOperandAfter,
    /// An empty memory displacment []
    EmptyMemoryDisplacment,
    /// Unexpected token
    UnexpectedToken(Token),
}

impl Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ParsingError::FirstTokenNeedsToBeIdent=> "expected an identifier as the first instruction".to_string(),
            ParsingError::UnknownInstruction(name) => format!("unknown instruction: '{}'", name), 
            ParsingError::UnknownRegOrUnexpectedIdent(name) => format!("unexpected ident: '{}' (maybe you missspeld on of the registers)", name), 
            ParsingError::UnexpectedTokens(toks) => format!("unexpected tokens (maybe forgott an comma): {:?}", toks),
            ParsingError::CommaWithoutOperandAfter => "found comma but no valid operand after it".to_string(),
            ParsingError::EmptyMemoryDisplacment => "memory displacments aren't allowed to be empty".to_string(),
            ParsingError::UnexpectedToken(tok) => format!("unexpected token: {:?}", tok),
        })
    }
}

impl Error for ParsingError {}