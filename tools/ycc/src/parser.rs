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
            let cur = self.current()?.to_owned();

            match cur.ty {
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
                        loc: cur.pos, 
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
        let ty = self.parse_type()?;
        let name = self.get_ident()?;

        let tok = self.advance().to_owned();

        match tok.ty {
            TokenType::Equal => self.parse_global(ty, name),
            TokenType::LeftParan => self.parse_func(ty, name, false),
            _ => {
                self.errors.push(YccError { 
                    loc: tok.pos, 
                    head: "expected either `=` or `(`", 
                    where_string: "expected either `=` or `(`".into() 
                });
                None
            }
        }
    }

    fn parse_type(&mut self) -> Option<AstType> {
        let current = self.advance().to_owned();

        let mut out_ty = AstType {
            meta: AstTypeMeta::Int,
            signed: false,
            unsigned: false,
        };

        match current.ty {
            TokenType::Unsigned => {out_ty.signed = false; out_ty.unsigned = true},
            TokenType::Signed => {out_ty.signed = true; out_ty.unsigned = false},
            TokenType::Char => out_ty.meta = AstTypeMeta::Char,
            TokenType::Bool => out_ty.meta = AstTypeMeta::Bool,
            TokenType::Short => out_ty.meta = AstTypeMeta::Short,
            TokenType::Int => out_ty.meta = AstTypeMeta::Int,
            TokenType::Long => out_ty.meta = AstTypeMeta::Long,
            TokenType::Float => out_ty.meta = AstTypeMeta::Float,
            TokenType::Double => out_ty.meta = AstTypeMeta::Double,
            TokenType::Void => out_ty.meta = AstTypeMeta::Void,
            TokenType::Struct => out_ty.meta = AstTypeMeta::Struct,
            TokenType::Enum => out_ty.meta = AstTypeMeta::Enum,

            _ => {
                self.errors.push(YccError {
                    loc: self.pos(),
                    head: "expected valid token for types",
                    where_string: "unexpected token".into(),
                });
            }
        }

        if out_ty.signed {
            let current = self.advance().to_owned();
            match current.ty {
                TokenType::Char => out_ty.meta = AstTypeMeta::Char,
                TokenType::Bool => out_ty.meta = AstTypeMeta::Bool,
                TokenType::Short => out_ty.meta = AstTypeMeta::Short,
                TokenType::Int => out_ty.meta = AstTypeMeta::Int,
                TokenType::Long => out_ty.meta = AstTypeMeta::Long,
                TokenType::Float => out_ty.meta = AstTypeMeta::Float,
                TokenType::Double => out_ty.meta = AstTypeMeta::Double,
                TokenType::Void => out_ty.meta = AstTypeMeta::Void,
                TokenType::Struct => out_ty.meta = AstTypeMeta::Struct,
                TokenType::Enum => out_ty.meta = AstTypeMeta::Enum,
    
                _ => {
                    self.errors.push(YccError {
                        loc: self.pos(),
                        head: "expected valid token for types",
                        where_string: "unexpected token".into(),
                    });
                }
            }
        }

        if !out_ty.unsigned { out_ty.signed = true; }

        Some(out_ty)
    }

    fn parse_return(&mut self) -> Option<Stmt> {
        let stmt = Stmt::Return { 
            value: self.parse_expr()? 
        };

        if let TokenType::Semicolon = self.advance().ty {} else { 
            self.errors.push(YccError {
                loc: self.pos(),
                head: "expected semicolon after return",
                where_string: "expected `;`".into(),
            });
            return None;
        }

        Some(stmt)
    }

    fn parse_stmt(&mut self) -> Option<Stmt> {
        let current = self.advance().to_owned();

        let stmt = match current.ty {
            TokenType::Return => self.parse_return()?,
            _ => {
                self.errors.push(YccError {
                    loc: self.pos(),
                    head: "unimplemented",
                    where_string: format!("this statment is still unimplemented: {:?}", current.ty)
                });
                self.critical_error = true;
                return None;
            }
        };

        Some(stmt)
    }

    fn parse_block(&mut self) -> Option<Vec<Stmt>> {
        if let TokenType::LeftBracket = self.advance().ty {} else {
            self.errors.push(YccError {
                loc: self.pos(),
                head: "expected left bracket",
                where_string: "expected `{`".into(),
            });
            self.critical_error = true;
            return None;
        }

        let mut stmts = Vec::new();

        loop {
            if let TokenType::RightBracket = self.current()?.ty {
                self.advance(); // }
                break;
            }

            stmts.push(self.parse_stmt()?);
        }

        Some(stmts)
    }

    fn get_ident(&mut self) -> Option<String> {
        let current = self.advance().to_owned().to_owned();

        if let TokenType::Ident(ident) = current.ty {
            return Some(ident)
        } 

        self.errors.push(YccError {
            loc: self.pos(),
            head: "expected ident",
            where_string: "expected ident".into(),
        });

        None
    }

    fn parse_func(&mut self, ty: AstType, name: String, extrn: bool) -> Option<()> {        
        let mut args = Vec::new();

        let mut first = true;

        loop {
            if !first {
                let current = self.advance();

                if let TokenType::Comma = current.ty {} else {
                    self.errors.push(YccError { 
                        loc: self.pos(),
                        head: "expected comma", 
                        where_string: "expected `,`".to_owned()
                    });
                }
            }

            let arg_ty = self.parse_type()?;
            let arg_name = self.get_ident()?;

            args.push((arg_name, arg_ty));

            first = false;

            if let TokenType::RightParan = self.current()?.ty {
                self.advance(); // )
                break;
            }
        }

        if let TokenType::Semicolon = self.current()?.ty {
            self.out.push(TopLevelStmt::Func(FuncStmt {
                name: name,
                visibility: if extrn { Visibility::Extern } else { Visibility::Private },
                return_type: ty,
                args: args,
                body: Vec::new(),
                only_ty_indector: true,
            }));

            return Some(()) 
        }

        let body = self.parse_block()?;

        self.out.push(TopLevelStmt::Func(FuncStmt {
            name: name,
            visibility: if extrn { Visibility::Extern } else { Visibility::Private },
            return_type: ty,
            args: args,
            body: body,
            only_ty_indector: false,
        }));

        Some(())
    }

    fn parse_global(&mut self, ty: AstType, name: String) -> Option<()> {
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
        self.tokens.front()
    }

    fn parse_expr(&mut self) -> Option<Expr> {
        todo!()
    }
}