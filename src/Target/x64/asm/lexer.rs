use std::{error::Error, fmt::Display, num::ParseIntError};

use logos::Logos;

use crate::Target::Lexer;

/// An error which can occure during lexing
#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexingError {
    /// An invalid intenger
	InvalidInteger(String),
	#[default]
    /// A not supported character
	NonAsciiCharacter,
}

impl Display for LexingError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "error")
	}
}
impl From<Box<dyn Error>> for LexingError {
	fn from(_err: Box<dyn Error>) -> Self {
		LexingError::InvalidInteger(format!("error"))
	}
}

impl std::error::Error for LexingError {}

impl From<ParseIntError> for LexingError {
	fn from(err: ParseIntError) -> Self {
		LexingError::InvalidInteger(format!("{:?}", err.kind()))
	}
}

/// An assembly token
#[derive(Logos, Debug, Clone, PartialEq, Eq)]
#[logos(skip r"[ \t\n\f]+")]
#[logos(error = LexingError)]
#[doc(hidden)]
pub enum Token {
	#[regex("[a-zA-Z0-9_]+", priority = 5, callback = |lex| lex.slice().to_string())]
	Ident(String),

	#[regex(r"(0x[0-9a-fA-F]+|0b[01]+|\d+)", priority = 6, callback = |lex| {
		let string = lex.slice();
		if string.starts_with("0x") {
			i64::from_str_radix(&string.replace("0x", ""), 16)
		} else if string.starts_with("0b") {
			i64::from_str_radix(&string.replace("0b", ""), 2)
		} else if string.starts_with("-") {
			Ok(-(string.replace("-", "").parse::<i64>()?))
		} else {
			string.parse()
		}
	})]
	Num(i64),

    #[token(",")]
    Comma,

    #[token(":")]
    DoubleDot,

	#[token("[")]
	L_Bracket,

	#[token("]")]
	R_Bracket,

	#[token("+")]
	Add,

	#[token("-")]
	Sub,
}

/// A temporary structure which implements the Lexer trait 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct x64Lexer {}

impl Lexer for x64Lexer {
	fn lex(&self, string: String) -> Result<Vec<Token>, Box<dyn Error>> {
		let mut tokens = vec![];
	
		for tok in Token::lexer(&string) {
			tokens.push( tok? );
		}
	
		Ok(tokens)
	}

	fn boxed(&self) -> Box<dyn Lexer> {
		Box::from( self.clone() )
	}
}