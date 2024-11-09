use std::collections::VecDeque;
use crate::error::ErrorLoc;
use crate::{error::YccError, lexer::*};
use crate::ast::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parser<'a> {
    tokens: VecDeque<&'a Token>,

    pub errors: Vec<YccError>,
    critical_error: bool,

    pub out: Vec<TopLevelStmt>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        let mut prepared_tokens = VecDeque::new();

        for tok in tokens {
            prepared_tokens.push_back(tok);
        }

        Self {
            tokens: prepared_tokens,
            errors: Vec::new(),
            critical_error: false,
            out: Vec::new(),
        }
    }

    pub fn parse(&mut self) -> Option<()> {
        while self.tokens.len() != 0 {
            match self.advance().ty {
                TokenType::Unsigned => self.parse_func_or_global(),
                TokenType::Signed => self.parse_func_or_global(),
                TokenType::Char => self.parse_func_or_global(),
                TokenType::Bool => self.parse_func_or_global(),
                TokenType::Short => self.parse_func_or_global(),
                TokenType::Int => self.parse_func_or_global(),
                TokenType::Long => self.parse_func_or_global(),
                TokenType::Float => self.parse_func_or_global(),
                TokenType::Double => self.parse_func_or_global(),
                TokenType::Void => self.parse_func_or_global(),
                TokenType::Const => self.parse_const(),
                TokenType::Enum => self.parse_enum(),
                TokenType::Struct => self.parse_struct(),
                TokenType::Union => todo!("unions are currently unsupported"),
                TokenType::Typedef => todo!("typedefs are currently unsupported"),
                TokenType::Extern => self.parse_extern(),
                TokenType::Static => self.parse_static(),
                TokenType::Volatile => todo!("volatiles are currently unsupported"),
                _ => {
                    self.errors.push(YccError { 
                        loc: self.pos(), 
                        head: "unexpected token", 
                        where_string: format!("unexpected token"), 
                    });

                    None
                },
            };

            if self.critical_error {
                break;
            }
        }

        Some(())
    }

    fn parse_func_or_global(&mut self) -> Option<()> {
        todo!()
    }

    fn parse_func(&mut self) -> Option<()> {
        todo!()
    }

    fn parse_global(&mut self) -> Option<()> {
        todo!()
    }

    fn parse_const(&mut self) -> Option<()> {
        todo!()
    }

    fn parse_enum(&mut self) -> Option<()> {
        todo!()
    }

    fn parse_struct(&mut self) -> Option<()> {
        todo!()
    }

    fn parse_extern(&mut self) -> Option<()> {
        todo!()
    }

    fn parse_static(&mut self) -> Option<()> {
        todo!()
    }

    #[inline]
    fn advance(&mut self) -> &Token {
        self.tokens.pop_front().expect("parser ran out of tokens")
    }

    #[inline]
    fn pos(&self) -> ErrorLoc {
        self.current().expect("parser ran out of tokens").pos
    }

    #[inline]
    fn current(&self) -> Option<&&Token> {
        self.tokens.back()
    }
}