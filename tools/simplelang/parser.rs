use crate::lexer::Token;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parser { 
    tokens: Vec<Token>,

    pub out: Vec<Statement>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { 
            tokens: tokens, 
            out: vec![] 
        }
    }
}