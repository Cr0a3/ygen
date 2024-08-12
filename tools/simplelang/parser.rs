use std::collections::VecDeque;

use Ygen::IR::TypeMetadata;

use crate::lexer::Token;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Var((String, Option<TypeMetadata>)), // (name, type)
    Binary((Operator, Option<Box<Expr>>, Option<Box<Expr>>)), // (op, left, right)
    LiteralInt(i64),
    Call(CallStmt),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
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
    import: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetStmt {
    var: Option<Expr>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallStmt {
    name: String,
    args: Vec<Expr>,
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
            Token::With | Token::Extern | Token::Import => self.parse_func(),
            Token::Return => self.parse_return(),
            any => todo!("{:?}", any),
        }
    }

    fn parse_func(&mut self) -> Option<Statement> {
        let mut extrn = false;

        if let Some(Token::Extern) = self.tokens.front() {
            extrn = true;
            self.tokens.pop_front();
        };

        let import = if let Some(Token::Import) = self.tokens.front() {
            self.tokens.pop_front();
            true
        } else { false};

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
        
        if import {
            return Some(Statement::Fn(FnStmt {
                name: name,
                body: vec![],
                args: args,
                extrn: false,
                import: import,
            }))
        }

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
            import: false, // we handled imported functions earlier
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
        if let Some(expr) = self.parse_expr() {
            Some(Statement::Expr(expr))
        } else if let Some(call) = self.parse_call() {
            Some(Statement::Expr(call))
        } else {
            None
        }
    }

    fn parse_return(&mut self) -> Option<Statement> {
        let mut to_return = None;

        self.tokens.pop_front(); // the return

        if let Some(expr) = self.parse_expr() {
            to_return = Some(expr);
        } else if let Some(var) = self.parse_var() {
            to_return = Some(Expr::Var(var));
        }


        println!("{:?}", self.tokens.front());

        if let Some(Token::Semicolon) = self.tokens.front() {} else { return None; }

        self.tokens.pop_front();

        Some(Statement::Ret(RetStmt {
            var: to_return,
        }))
    }

    fn parse_expr(&mut self) -> Option<Expr> {
        let mut left = self.parse_term();

        loop {
            if self.tokens.front() == Some(&Token::Add) || self.tokens.front() == Some(&Token::Sub) {
                let op = match self.tokens.front()? {
                    Token::Add => Operator::Add,
                    Token::Sub => Operator::Sub,
                    _ => unreachable!(),
                };

                self.tokens.pop_front();

                let right = self.parse_term();

                let box_left = if let Some(kleft) = left {
                    Some(Box::from(kleft))
                } else { None };

                let box_right = if let Some(kright) = right {
                    Some(Box::from(kright))
                } else { None };

                left = Some(Expr::Binary((op, box_left, box_right)));
            } else {
                break;
            }
        }

        left
    }

    fn parse_term(&mut self) -> Option<Expr> {
        let mut left = self.parse_factor();
        
        loop {
            if self.tokens.front() == Some(&Token::Mul) || self.tokens.front() == Some(&Token::Div) {
                let op = match self.tokens.front()? {
                    Token::Mul => Operator::Mul,
                    Token::Div => Operator::Div,
                    _ => unreachable!(),
                };

                self.tokens.pop_front();

                let right = self.parse_factor();

                let box_left = if let Some(kleft) = left {
                    Some(Box::from(kleft))
                } else { None };

                let box_right = if let Some(kright) = right {
                    Some(Box::from(kright))
                } else { None };

                left = Some(Expr::Binary((op, box_left, box_right)));
                
            } else {
                break;
            }
        }

        left
    }

    fn parse_factor(&mut self) -> Option<Expr> {
        let mut res = None;
        let mut pop = true;

        if let Some(front) = self.tokens.front() {
            res = match front {
                Token::Ident(x) => {
                    let x = x.to_string();

                    let mut out = Some(Expr::Var((x.clone(), None)));

                    if let Some(call) = self.parse_call() {
                        out = Some(call);
                    }
                    pop = false;

                    out

                },
                Token::Number(n) => Some(Expr::LiteralInt(*n)),
                Token::LParam => {
                    self.tokens.pop_front();
                    let out = self.parse_expr();

                    if self.tokens.front() != Some(&Token::RParam) {
                        None
                    } else {
                        out
                    }
                }
                _ => None,
            };
        }

        if res != None && pop {
            self.tokens.pop_front();
        }

        res
    }

    fn parse_call(&mut self) -> Option<Expr> {
        let name = if let Some(Token::Ident(x)) = self.tokens.front() {
            x.to_string()
        } else { return None; };


        self.tokens.pop_front();

        if let Some(Token::LParam) = self.tokens.front() {} else { return None };
        self.tokens.pop_front();

        let mut args = vec![];

        loop {
            if Some(&Token::RParam) == self.tokens.front() {
                break;
            }

            args.push( self.parse_expr()? );

            if Some(&Token::Comma) == self.tokens.front() {
                self.tokens.pop_front();
            }
        }

        self.tokens.pop_front();

        Some(Expr::Call(CallStmt {
            name: name,
            args: args,
        }))
    }
}