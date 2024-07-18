use std::error::Error;

use super::Token;

/// The lexer trait
pub trait Lexer {
    /// lexes the string
    fn lex(&self, string: String) -> Result<Vec<Token>, Box<dyn Error>>;

    /// Returns self into a boxed lexer trait
    fn boxed(&self) -> Box<dyn Lexer>;
}

impl Clone for Box<dyn Lexer> {
    fn clone(&self) -> Self {
        self.boxed()
    }
}