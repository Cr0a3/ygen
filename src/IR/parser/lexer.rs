use super::IrError;

/// A location reference.
/// Is recommended to be used for giving tokens locations
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Loc {
    /// The line number
    pub line: u64,
    /// The coloumn
    pub coloumn: u64,
    /// The length of the sequence
    pub length: u64,
    /// The entire source line
    pub line_string: String,
}

/// The token type for parsing ir
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {

}

/// An ir token
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    /// the type
    pub typ: TokenType,
    /// the location
    pub loc: Loc,
}

/// A lexer for lexing ygen ir strings
#[allow(unused)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IrLexer {
    input_stream: String,

    line: String,
    line_no: u64,

    coloumn: u64,

    start: u64,
    current: u64,

    /// The output
    pub out: Vec<Token>,
}

impl IrLexer {
    /// Creates a new ir lexer
    pub fn new(input: String) -> Self {
        Self {
            input_stream: input,
            line: String::new(),
            line_no: 1,
            coloumn: 0,
            start: 0,
            current: 0,

            out: vec![],
        }
    }

    /// "lexes" the input
    pub fn lex(&mut self) -> Result<(), IrError> {
        todo!();

        //Ok(())
    }
}