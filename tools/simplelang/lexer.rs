use std::{error::Error, fmt::Display, num::ParseIntError};

use logos::Logos;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum LexingError {
    #[default]
    NonAsciiCharacter,
    UnexpectedCharacter(char),
    NumberError(String),
}

impl Display for LexingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexingError::UnexpectedCharacter(ch) => write!(f, "Error: Unexpected character '{}'", ch),
            _ => write!(f, ""),
        }
    }
}

impl From<ParseIntError> for LexingError {
    fn from(value: ParseIntError) -> Self {
        Self::NumberError(format!("{}", value))
    }
}

impl Error for LexingError {}

#[derive(Logos, Debug, Clone, PartialEq, Eq)]
#[logos(error = LexingError)]
#[logos(skip r"[ \t\n\r\f]+")]
pub enum Token {
    #[regex("[a-zA-Z0-9_]+", |lex| lex.slice().to_string())]
    Ident(String),

    #[regex("[0-9_]+", priority = 3, callback = |lex| lex.slice().parse())]
    Number(i64),

    #[token("with")]
    With,

    #[token("(")]
    LParam,

    #[token(")")]
    RParam,

    #[token("{")]
    LCurly,

    #[token("}")]
    RCurly,

    #[token(",")]
    Comma,

    #[token("+")]
    Add,

    #[token("-")]
    Sub,

    #[token(";")]
    Semicolon,

    #[token("extern")]
    Extern,

    #[token(":")]
    DoubleDot,

    #[token("*")]
    Mul,

    #[token("/")]
    Div,

    #[token("return")]
    Return,
}
