use std::collections::VecDeque;

use Ygen::IR::TypeMetadata;

use crate::lexer::Token;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Var((String, TypeMetadata)), // (name, type)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Fn(FnStatement),
    Expr(Expr),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FnStatement {
    name: String,
    body: Vec<Statement>,

    args: Vec<Expr>,

    extrn: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parser { 
    tokens: VecDeque<Token>,

    pub out: Vec<Statement>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { 
            tokens: tokens.into(), 
            out: vec![] 
        }
    }

    pub fn parse(&mut self) {
        while let Some(stmt) = self.parse_stmt() {
            self.out.push(stmt);
        }
    }

    fn parse_stmt(&mut self) -> Option<Statement> {
        match self.tokens.front()? {
            Token::Ident(_) => todo!(),
            Token::With => self.parse_func(),
            Token::Extern => self.parse_func(),
            _ => None,
        }
    }

    fn parse_func(&mut self) -> Option<Statement> {
        let mut extrn = false;

        if let Some(Token::Extern) = self.tokens.front() {
            extrn = true;
            self.tokens.pop_front();
        };

        if let Some(Token::With) = self.tokens.front() {} else { return None; }

        self.tokens.pop_front(); // advance over with

        if let Some(Token::LParam) = self.tokens.front() {} else { return None; }

        self.tokens.pop_front();

        let mut args = vec![];

        loop {
            match self.tokens.front()? {
                Token::Ident(_) => args.push(Expr::Var(self.parse_var()?)),
                Token::RParam => break,
                _ => { return None; },
            }

            if let Some(Token::Comma) = self.tokens.front() {
                self.tokens.pop_front();
            }
        }

        self.tokens.pop_front(); // the )

        let name;
        if let Some(Token::Ident(ident)) = self.tokens.front() {
            name = ident.to_string();
            self.tokens.pop_front();
        } else { return None; }    
        
        
        if let Some(Token::DoubleDot) = self.tokens.front() {} else { return None; }

        self.tokens.pop_front();  

        Some(Statement::Fn(FnStatement {
            name: name,
            body: vec![],
            args: args,
            extrn: extrn,
        }))

    }

    fn parse_var(&mut self) -> Option<(String, TypeMetadata)> {
        let name;
        if let Some(Token::Ident(var)) = self.tokens.front() {
            name = var.to_string();
            self.tokens.pop_front();
        } else { return None; }

        if let Some(Token::DoubleDot) = self.tokens.front() {} else { return None; }

        self.tokens.pop_front();

        let tystring;

        if let Some(Token::Ident(ty)) = self.tokens.front() {
            tystring = ty.to_string();
            self.tokens.pop_front();
        } else { return None; }

        let ty = match tystring.as_str() {
            "u16" => Some(TypeMetadata::u16),
            "u32" => Some(TypeMetadata::u32),
            "u64" => Some(TypeMetadata::u64),
            "i16" => Some(TypeMetadata::i16),
            "i32" => Some(TypeMetadata::i32),
            "i64" => Some(TypeMetadata::i64),
            _ => None,
        }?;

        Some((name, ty))
    }
}