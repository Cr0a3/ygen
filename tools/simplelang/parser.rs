use std::collections::VecDeque;

use ygen::IR::TypeMetadata;
use crate::lexer::Token;
use crate::{ast::*, err, expect, warn};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parser { 
    tokens: VecDeque<Token>,

    pub out: Vec<Statement>,

    error: bool,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { 
            tokens: tokens.into(), 
            out: vec![],

            error: false,
        }
    }

    pub fn parse(&mut self) {
        loop { 
            self.remove_maybe_semicolon();

            if let Some(stmt) = self.parse_stmt() {
                self.out.push(stmt);
            } else {
                break;
            }
        }
    }

    fn parse_stmt(&mut self) -> Option<Statement> {
        match self.tokens.front()? {
            Token::Ident(_) => self.parse_ident(),
            Token::Func | Token::Extern | Token::Import => self.parse_func(),
            Token::Return => self.parse_return(),
            Token::Var => Some(Statement::Expr(self.parse_assign()?)),
            any => {
                err!(self.error, "unexpected token: {:?}", any);
                None
            },
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

        if !expect!(self.tokens.front(), Some(&Token::Func), |tok| {
            err!(self.error, "expected 'func' found: {:?}", tok);
        }) {
            return None;
        }

        self.tokens.pop_front(); // advance over func

        let name;
        if let Some(Token::Ident(ident)) = self.tokens.front() {
            name = ident.to_string();
            self.tokens.pop_front();
        } else { 
            err!(self.error, "expected identifer as the function name found {:?}", self.tokens.front());
            return None; 
        }    

        if !expect!(self.tokens.front(), Some(&Token::LParam), |tok| {
            err!(self.error, "expected '(' found: {:?}", tok);
        }) {
            return None;
        }

        self.tokens.pop_front();

        let mut args = vec![];

        let mut dynamic_args = false;

        loop {
            match self.tokens.front()? {
                Token::Ident(_) => args.push(Expr::Var(self.parse_var()?)),
                Token::TripleDot => {
                    dynamic_args = true;
                    self.tokens.pop_front();
                },
                Token::RParam => break,
                _ => { return None; },
            }

            if let Some(Token::Comma) = self.tokens.front() {
                if dynamic_args {
                    err!(self.error, "after a any arg indicator (...) no arguments are allowed to be there");
                    return None;
                }
                self.tokens.pop_front();
            }
        }

        self.tokens.pop_front(); // the )
        
        let mut ret = TypeMetadata::Void;        

        if let Some(Token::RightArrow) = self.tokens.front() {
            self.tokens.pop_front();  

            let tystring;

            if expect!(self.tokens.front(), Some(&Token::Ident(_)), |tok| {
                err!(self.error, "expected an identifier as the type found: {:?}", tok);
            }) {
                if let Some(Token::Ident(ty)) = self.tokens.front() {
                    tystring = ty.to_string();
                    self.tokens.pop_front();
                } else { unreachable!() }
            } else {
                return None;
            }

            ret = match tystring.as_str() {
                "u16" => Some(TypeMetadata::u16),
                "u32" => Some(TypeMetadata::u32),
                "u64" => Some(TypeMetadata::u64),
                "i16" => Some(TypeMetadata::i16),
                "i32" => Some(TypeMetadata::i32),
                "i64" => Some(TypeMetadata::i64),
                "string" => Some(TypeMetadata::ptr),
                "void" => Some(TypeMetadata::Void),
                any => {
                    err!(self.error, "unknown type: {}", any);
                    None?
                },
            }?; 
        }


        if import {
            return Some(Statement::Fn(FnStmt {
                name: name,
                body: vec![],
                args: args,
                extrn: false,
                import: import,
                dynamic_args: dynamic_args,
                ret: ret,
            }))
        }

        if !expect!(self.tokens.front(), Some(&Token::LCurly), |tok| {
            err!(self.error, "expected '{{' found: {:?}", tok);
        }) {
            return None;
        }

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
            dynamic_args: dynamic_args,
            import: false, // we handled imported functions earlier
            ret: ret,
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
                "string" => Some(TypeMetadata::ptr),
                any => {
                    err!(self.error, "unknown type: {}", any);
                    None
                },
            }?);
        }

        Some((name, ty))
    }

    fn parse_ident(&mut self) -> Option<Statement> {
        if let Some(expr) = self.parse_expr() {
            Some(Statement::Expr(expr))
        } else {
            err!(self.error, "unexpected ident {:?}", self.tokens.front());
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

        if !expect!(self.tokens.front(), Some(&Token::Semicolon), |tok| {
            warn!("expected ';' found: {:?}", tok);
        }) { return None; }

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
                } else { 
                    err!(self.error, "expected left side expression before +, -, * or / found nothing");
                    return None;
                 };

                let box_right = if let Some(kright) = right {
                    Some(Box::from(kright))
                } else { 
                    err!(self.error, "expected right side expression after +, -, * or / found nothing");
                    return None;
                 };

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

                    self.tokens.pop_front();
                    let mut assign = false;
                    let mut call = false;
                    if Some(&Token::LParam) == self.tokens.front() {
                        call = true;
                    } else if   Some(&Token::Assign) == self.tokens.front() ||
                                Some(&Token::AddEqual) == self.tokens.front() ||
                                Some(&Token::SubEqual) == self.tokens.front() || 
                                Some(&Token::MulEqual) == self.tokens.front() || 
                                Some(&Token::DivEqual) == self.tokens.front() {
                        assign = true;
                    }

                    self.tokens.push_front(Token::Ident(x.clone()));

                    if assign {
                        if let Some(assign) = self.parse_assign() {
                            out = Some(assign);
                            pop = false;
                        }
                    }
                    if call {
                        if let Some(call) = self.parse_call() {
                            out = Some(call);
                            pop = false;
                        }
                    }

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
                Token::String(string) => Some(Expr::LiteralString(string.to_string())),
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

        if !expect!(self.tokens.front(), Some(&Token::LParam), |tok| {
            err!(self.error, "expected '(' found: {:?}", tok);
        }) {
            return None;
        }

        self.tokens.pop_front(); // the (

        let mut args = vec![];

        loop {
            if Some(&Token::RParam) == self.tokens.front() {
                break;
            }

            if Some(&Token::Comma) == self.tokens.front() {
                self.tokens.pop_front();
            }

            args.push( self.parse_expr()? );
        }

        self.tokens.pop_front();

        self.remove_maybe_semicolon();

        Some(Expr::Call(CallStmt {
            name: name,
            args: args,
        }))
    }

    fn remove_maybe_semicolon(&mut self) {
        loop {
            if Some(&Token::Semicolon) == self.tokens.front() {
                self.tokens.pop_front();
            } else {
                break;
            }
        }
    }

    fn parse_assign(&mut self) -> Option<Expr> {
        if self.tokens.front() == Some(&Token::Var) { // variable assignment
            self.tokens.pop_front(); // var
        }

        let var = if let Some(var) = self.parse_var() {
            var
        } else {
            err!(self.error, "expected variable after var keyword");
            return None;
        };

        let mut out = (Operator::Assign, Some(Box::from(Expr::Var(var.clone()))), None);
        

        let op = self.tokens.front()?.clone();
        self.tokens.pop_front();
        
        let rhs = if let Some(expr) = self.parse_expr() {
            expr
        } else {
            err!(self.error, "expected right expression after var");
            return None;
        };

        let rhs = Some(Box::from(rhs));

        match op {
            Token::SubEqual => {
                out.2 = Some(Box::from(Expr::Binary((Operator::Sub, Some(Box::from(Expr::Var(var))), rhs))));
            },
            Token::AddEqual => {
                out.2 = Some(Box::from(Expr::Binary((Operator::Add, Some(Box::from(Expr::Var(var))), rhs))));
            },
            Token::MulEqual => {
                out.2 = Some(Box::from(Expr::Binary((Operator::Mul, Some(Box::from(Expr::Var(var))), rhs))));
            },
            Token::DivEqual => {
                out.2 = Some(Box::from(Expr::Binary((Operator::Div, Some(Box::from(Expr::Var(var))), rhs))));
            },
            Token::Assign => {
                out.2 = rhs;
            },
            _ => {
                err!(self.error, "expected either =, +=, -=, *= or /= found {:?}", self.tokens.front());
                return None;
            }
        }

        self.remove_maybe_semicolon();

        Some(Expr::Binary(out))

    }

    pub fn had_errors(&self) -> bool {
        self.error
    }
}