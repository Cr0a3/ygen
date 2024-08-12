use std::collections::VecDeque;

use Ygen::IR::TypeMetadata;

use crate::lexer::Token;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Var((String, Option<TypeMetadata>)), // (name, type)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Fn(FnStmt),
    Expr(Expr),
    Ret(RetStmt),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FnStmt {
    name: String,
    body: Vec<Statement>,

    args: Vec<Expr>,

    extrn: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetStmt {
    var: Option<Expr>,
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
            Token::Ident(_) => self.parse_ident(),
            Token::With | Token::Extern => self.parse_func(),
            Token::Return => self.parse_return(),
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

        if let Some(Token::LCurly) = self.tokens.front() {} else { return None; }

        self.tokens.pop_front();  

        let mut body = vec![];

        loop {
            if let Some(Token::RCurly) = self.tokens.front() {
                break;
            } else {
                body.push(self.parse_stmt()?)
            }
        }

        self.tokens.pop_front(); // the }

        Some(Statement::Fn(FnStmt {
            name: name,
            body: body,
            args: args,
            extrn: extrn,
        }))

    }

    fn parse_var(&mut self) -> Option<(String, Option<TypeMetadata>)> {
        let name;
        if let Some(Token::Ident(var)) = self.tokens.front() {
            name = var.to_string();
            self.tokens.pop_front();
        } else { return None; }

        let mut ty = None;

        if let Some(Token::DoubleDot) = self.tokens.front() {
            self.tokens.pop_front();

            let tystring;
    
            if let Some(Token::Ident(ty)) = self.tokens.front() {
                tystring = ty.to_string();
                self.tokens.pop_front();
            } else { return None; }
    
            ty = Some(match tystring.as_str() {
                "u16" => Some(TypeMetadata::u16),
                "u32" => Some(TypeMetadata::u32),
                "u64" => Some(TypeMetadata::u64),
                "i16" => Some(TypeMetadata::i16),
                "i32" => Some(TypeMetadata::i32),
                "i64" => Some(TypeMetadata::i64),
                _ => None,
            }?);
        }

        Some((name, ty))
    }

    fn parse_ident(&mut self) -> Option<Statement> {
        let ident = if let Some(ident) = self.tokens.front() {
            ident 
        } else { None? };

        self.tokens.pop_front();

        None
    }

    fn parse_return(&mut self) -> Option<Statement> {
        let mut to_return = None;

        self.tokens.pop_front(); // the return

        if let Some(var) = self.parse_var() {
            to_return = Some(Expr::Var(var));
        }

        if let Some(Token::Semicolon) = self.tokens.front() {} else { return None; }

        self.tokens.pop_front();

        Some(Statement::Ret(RetStmt {
            var: to_return,
        }))
    }
}