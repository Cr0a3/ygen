use std::{any::Any, error::Error, fmt::Display, num::ParseIntError};

use logos::Logos;

use crate::Target::Lexer;

/// An error which can occure during lexing
#[derive(Default, Debug, Clone, PartialEq)]
pub(crate) enum LexingError {
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
#[derive(Logos, Debug, Clone)]
#[logos(skip r"[ \t\n\f]+")]
#[logos(error = LexingError)]
#[doc(hidden)]
pub(crate) enum Token {
	#[regex("[a-zA-Z][a-zA-Z0-9_]*", priority = 5, callback = |lex| lex.slice().to_string())]
	Ident(String),

	#[regex(r"-?[0-9]+(\.[0-9]+)?", |lex| lex.slice().parse::<f64>().unwrap())]
	Num(f64),

	#[token(".")]
	Dot,
}

impl PartialEq for Token {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Self::Ident(l0), Self::Ident(r0)) => l0 == r0,
			(Self::Num(l0), Self::Num(r0)) => l0 == r0,
			_ => core::mem::discriminant(self) == core::mem::discriminant(other),
		}
	}
}

impl Eq for Token {}

/// A temporary structure which implements the Lexer trait 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct wasmLexer {}

impl Lexer for wasmLexer {
	fn lex(&self, string: String) -> Result<Vec<Box<dyn Any>>, Box<dyn Error>> {
		let mut tokens: Vec<Box<dyn Any>> = vec![];
	
		for tok in Token::lexer(&string) {
			tokens.push( Box::new(tok?) );
		}
	
		Ok(tokens)
	}

	fn boxed(&self) -> Box<dyn Lexer> {
		Box::from( self.clone() )
	}
}