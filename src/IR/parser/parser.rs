use std::collections::{HashMap, VecDeque};

use crate::prelude::Ir;
use crate::IR::TypeMetadata;

use super::lexer::{Loc, Token, TokenType};
use super::IrError;

/// An ir statement
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(private_interfaces)]
#[allow(missing_docs)]
pub enum IrStmt {
    /// a function
    Func{name: String, ret: TypeMetadata, args: HashMap<String, TypeMetadata>, body: Vec<(Box<dyn Ir>, Loc)>},
    /// a constant
    Const{name: String, data: Vec<u8>, location: Loc},
}

/// Parses ir tokens into ir statements with location data
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IrParser {
    /// The output
    pub out: Vec<IrStmt>,

    input: VecDeque<Token>,
}

impl IrParser {
    /// Creates an new ir parser
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            out: vec![],
            input: VecDeque::from(tokens),
        }
    }

    /// parses the input
    pub fn parse(&mut self) -> Result<(), IrError> {
        while self.input.len() != 0 {
            let stmt = self.parse_stmt()?;
            self.out.push( stmt );
        }

        Ok(())
    }

    fn parse_stmt(&mut self) -> Result<IrStmt, IrError> {
        let tok = if let Some(token) = self.input.front() {
            token
        } else {
            Err(IrError::OutOfTokens)?
        };

        match &tok.typ {
            TokenType::Declare => self.parse_declare(),
            TokenType::Define => self.parse_define(),
            TokenType::Const => self.parse_const(),

            _ => Err(IrError::UnexpectedToken(tok.clone())),
        }
    }

    fn parse_declare(&mut self) -> Result<IrStmt, IrError> {
        todo!()
    }

    fn parse_define(&mut self) -> Result<IrStmt, IrError> {
        let name;
        let mut body= vec![];
        let mut args = HashMap::new();
        

        self.expect( TokenType::Define )?;
        self.input.pop_front(); // advance over define

        let ret = self.parse_type()?;
        self.input.pop_front();

        self.expect( TokenType::Func(String::new()) )?;

        let tok = self.current_token()?;
        if let TokenType::Func(func) = &tok.typ {
            name = func.to_string();
        } else { unreachable!() }

        self.input.pop_front();
        self.expect( TokenType::LParam )?;

        self.input.pop_front();

        loop {
            let current = self.current_token()?;

            if TokenType::RParam == current.typ {
                break;
            }

            let var_type = self.parse_type()?;
            self.input.pop_front();

            self.expect( TokenType::Var(String::new()) )?;

            let token = self.current_token()?;

            let var_name = match &token.typ {
                TokenType::Var(name) => name.to_string(),
                
                _=> Err(IrError::UndeterminedTokenSequence {
                    loc: token.loc.clone(), 
                    expected: String::from("%s for a valid variable"),
                })?
            };

            args.insert(var_name, var_type );
        }

        self.input.pop_front(); // the closing param )

        
        self.expect(TokenType::LBracket)?;
        self.input.pop_front();

        loop {
            let current = self.current_token()?;

            if TokenType::RBracket == current.typ {
                break;
            }

            body.push( self.parse_instruction()? );
        }

        Ok(IrStmt::Func { 
            name: name, 
            body: body,
            args: args,
            ret: ret,
        })
    }

    fn parse_const(&mut self) -> Result<IrStmt, IrError> {
        self.expect(TokenType::Const)?;

        self.input.pop_front();

        /* 
        PARSE NAME
        */

        let name;

        let mut location;

        self.expect( TokenType::Ident(String::new()) )?;

        let tok = self.current_token()?;
        if let TokenType::Ident(ident) = &tok.typ {
            name = ident.to_string();
            location = tok.loc.clone();
        } else { unreachable!() }

        self.input.pop_front();

        self.expect(TokenType::Equal)?;
        self.input.pop_front();

        let current = self.current_token()?;

        let mut data = vec![];

        match &current.typ {
            TokenType::String(x) => data = x.as_bytes().to_vec(),
            TokenType::LSquare => data = self.parse_data_array()?, 

            _=> Err(IrError::ExpectedTokenButFoundAnUnexpectedOne { 
                found: current.clone(), 
                expected: Token { 
                    typ: TokenType::LSquare, 
                    loc: current.loc.clone() 
                } 
            })?
        };

        let current = &self.current_token()?.loc;

        if location.line == current.line {
            location.length = current.coloumn - location.coloumn;
        }

        self.input.pop_front();

        Ok(IrStmt::Const { 
            name: name, 
            data: data,
            location: location,
        })
    }

    fn parse_instruction(&mut self) -> Result<(Box<dyn Ir>, Loc), IrError> {
        todo!()
    }

    fn parse_data_array(&mut self) -> Result<Vec<u8>, IrError> {
        self.expect(TokenType::LSquare)?;
        self.input.pop_front();
        
        let mut data = vec![];

        loop {
            let current = self.current_token()?;

            match &current.typ {
                TokenType::Int(int) => data.push(*int as u8),

                TokenType::RSquare => break,
                _ => Err(IrError::UnexpectedToken(current.clone()))?,
            };

            self.input.pop_front();

            let current = self.current_token()?;

            if TokenType::Comma == current.typ {
                self.input.pop_front();
            }
        }

        Ok(data)
    }

    fn current_token(&self) -> Result<&Token, IrError> {
        if let Some(token) = self.input.front() {
            Ok(token)
        } else { Err(IrError::OutOfTokens) }
    }

    fn expect(&mut self, typ: TokenType) -> Result<Token, IrError> {
        let token = self.current_token()?;

        if typ.name() == token.typ.name() {
            Ok(token.clone())
        } else {
            Err(IrError::ExpectedTokenButFoundAnUnexpectedOne { 
                found: token.clone(), 
                expected: Token { 
                    typ: typ, 
                    loc: token.loc.clone()
                } 
            })?
        }   
    }

    fn parse_type(&mut self) -> Result<TypeMetadata, IrError> {
        let token = self.current_token()?;

        let mut ident = String::new();

        if let TokenType::Ident(text) = &token.typ {
            ident = text.to_string();
        } else {
            Err(IrError::ExpectedTokenButFoundAnUnexpectedOne { 
                found: token.clone(), 
                expected: Token { 
                    typ: TokenType::Ident("abc".to_string()), 
                    loc: token.loc.clone()
                } 
            })?
        }

        if let Some(typ) = TypeMetadata::parse(ident) {
            Ok(typ)
        } else {
            Err(IrError::UnkownType(token.clone()) )
        }
    }
}