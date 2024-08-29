use std::{error::Error, fmt::Display, num::ParseIntError};
use logos::Logos;
use unescaper::unescape;

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
            LexingError::UnexpectedCharacter(ch) => write!(f, "unexpected character '{}'", ch),
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

    #[regex(r#""[^"]*""#, |lex| unescape(&lex.slice().to_string().replace("\"", "")).unwrap())]
    String(String),

    #[regex(r"(0x[0-9a-fA-F]+|0b[01]+|\d+)", priority = 3, callback = |lex| {
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
    Number(i64),

    #[token("func")]
    Func,

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

    #[token("*")]
    Mul,

    #[token("/")]
    Div,

    #[token("+=", priority=5)]
    AddEqual,

    #[token("-=", priority=5)]
    SubEqual,

    #[token("*=", priority=5)]
    MulEqual,

    #[token("/=", priority=5)]
    DivEqual,

    #[token(";")]
    Semicolon,

    #[token("extern")]
    Extern,

    #[token("var")]
    Var,

    #[token("=")]
    Assign,

    #[token("import")]
    Import,

    #[token(":")]
    DoubleDot,

    #[token("...")]
    TripleDot,

    #[token("return")]
    Return,

    #[token("->")]
    RightArrow,

    #[regex(r"//[^\n]*", logos::skip)]
    Comment,

    #[regex(r"/\*([^*]|\*[^/])*\*/", logos::skip)]
    MultiComment,
}