use std::{error::Error, fmt::Display};

use logos::Logos;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum LexingError {
    #[default]
    NonAsciiCharacter,
}

impl Display for LexingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            _ => "",
        })
    }
}

impl Error for LexingError {}

#[derive(Logos, Debug, Clone, PartialEq, Eq)]
#[logos(error = LexingError)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    #[regex("[a-zA-Z0-9]+", |lex| lex.slice().to_string())]
    Ident(String),

    #[token("with")]
    With,

    #[token("(")]
    LParam,

    #[token(")")]
    RParam,

    #[token(",")]
    Comma,

    #[token("extern")]
    Extern,

    #[token(":")]
    DoubleDot,
}
