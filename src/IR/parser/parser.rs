use std::collections::{HashMap, VecDeque};

use crate::prelude::Ir;
use crate::Obj::Linkage;
use crate::IR::{ir, Block, Const, FnTy, Function, Type, TypeMetadata, Var};

use super::lexer::{Loc, Token, TokenType};
use super::IrError;

#[derive(Debug, Clone, Eq)]
#[allow(missing_docs)]
pub struct IrInstr {
    pub(crate) loc: Loc,
    pub(crate) inst: Box<dyn Ir>,
}

impl PartialEq for IrInstr {
    fn eq(&self, other: &Self) -> bool {
        self.loc == other.loc && &self.inst == &other.inst
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(missing_docs)]
pub struct IrBlock {
    pub(crate) loc: Loc,
    pub(crate) body: Vec<IrInstr>,
}

/// An ir statement
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(private_interfaces)]
#[allow(missing_docs)]
pub enum IrStmt {
    /// a function
    Func{
        name: String,
        ret: TypeMetadata, 
        args: (HashMap<String, TypeMetadata>, /*unlim args*/bool), 
        body: HashMap<String, IrBlock>,
        scope: Linkage,

        location: Loc,
    },
    /// a constant
    Const{
        name: String,
        data: Vec<u8>, 
        location: Loc,
        scope: Linkage,
    },
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
        let name;
        let mut args = HashMap::new();
        
        self.expect( TokenType::Declare )?;
        self.input.pop_front(); // advance over declare

        let ret = self.parse_type()?;
        self.input.pop_front();

        self.expect( TokenType::Func(String::new()) )?;

        let tok = self.current_token()?;
        let loc = tok.loc.to_owned();
        if let TokenType::Func(func) = &tok.typ {
            name = func.to_string();
        } else { unreachable!() }

        self.input.pop_front();
        self.expect( TokenType::LParam )?;

        self.input.pop_front();

        let mut unlim = false;

        loop {
            let current = self.current_token()?;

            if TokenType::RParam == current.typ {
                break;
            }

            if TokenType::TripleDot == current.typ {
                self.input.pop_front();
                unlim = true;
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

            self.input.pop_front();
        }

        self.expect(TokenType::RParam)?;
        self.input.pop_front(); // the closing param )

        Ok(IrStmt::Func { 
            name: name, 
            body: HashMap::new(),
            scope: Linkage::Extern,
            args: (args, unlim),
            ret: ret,

            location: loc,
        })
    }

    fn parse_define(&mut self) -> Result<IrStmt, IrError> {
        let name;
        let mut body = HashMap::new();
        let mut args = HashMap::new();
        
        let mut link = Linkage::External;

        self.expect( TokenType::Define )?;
        self.input.pop_front(); // advance over define

        let ret = self.parse_type()?;
        self.input.pop_front();

        let curr = self.current_token()?;

        if let TokenType::Ident(ident) = &curr.typ {
            link = match ident.as_str() {
                "local" | "internal" | "private" => Linkage::Internal,
                "public" | "external" => Linkage::External,
                _ => Err(IrError::Unkown { 
                    what: "linkage".to_owned(), 
                    name: ident.to_owned(), 
                    loc: curr.loc.clone()
                })?
            };
        }

        self.expect( TokenType::Func(String::new()) )?;

        let tok = self.current_token()?;
        let loc = tok.loc.to_owned();
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

            let (name, block) = self.parse_block()?;

            body.insert( name, block );
        }

        self.input.pop_front(); // }

        Ok(IrStmt::Func { 
            name: name, 
            body: body,
            args: (args, false),
            scope: link,
            ret: ret,

            location: loc,
        })
    }

    fn parse_const(&mut self) -> Result<IrStmt, IrError> {
        self.expect(TokenType::Const)?;

        self.input.pop_front();

        /* 
        PARSE NAME
        */

        let mut scope = Linkage::External;
        let mut parsed_scope = false;

        let mut name = "unreachable".into();

        let mut location = Loc::default();

        self.expect( TokenType::Ident(String::new()) )?;

        let tok = self.current_token()?;
        if let TokenType::Ident(ident) = &tok.typ {
            match ident.as_str() {
                "local" | "internal" | "private" => {
                    parsed_scope = true;
                    scope = Linkage::Internal
                },
                "public" | "external" => {
                    parsed_scope = true;
                    scope = Linkage::External
                },
                _ => {
                    name = ident.to_string();
                    location = tok.loc.clone();
                }
            }
        } else { unreachable!() }

        self.input.pop_front();

        if parsed_scope {
            self.expect(TokenType::Ident(String::new()))?;
            let tok = self.current_token()?;

            if let TokenType::Ident(ident) = &tok.typ {
                name = ident.to_owned();
                location = tok.loc.clone();
            } else { unreachable!() }

            self.input.pop_front();
        }

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
            scope: scope,
        })
    }

    fn parse_block(&mut self) -> Result<(String, IrBlock), IrError> {
        self.expect(TokenType::Block(String::new()))?;

        let name;
        let loc;
        let curr_token = self.current_token()?;
        if let TokenType::Block(ident) = &curr_token.typ {
            name = ident.to_string();
            loc = curr_token.loc.clone();
        } else { unreachable!() }

        self.input.pop_front();

        let mut body = vec![];


        loop {
            let curr = self.current_token()?;

            if TokenType::RBracket == curr.typ {
                break;
            }

            if let TokenType::Block(_) = curr.typ {
                break;
            }

            body.push( self.parse_instruction()? );
        }

        Ok((name, IrBlock {
            loc: loc,
            body: body,
        }))
    }

    fn parse_instruction(&mut self) -> Result<IrInstr, IrError> {
        let curr = self.current_token()?.clone();

        let mut var = false;

        if let TokenType::Var(_) = curr.typ.clone() {
                var = true;
        }
        let node = {
            if var {
                let name = self.input.pop_front(); // var name
                
                let name = if let TokenType::Var(name) = name.expect("unreachble").typ {
                    name
                } else { unreachable!() };

                self.expect(TokenType::Equal)?;
                self.input.pop_front(); // =

                self.expect(TokenType::Ident(String::new()))?; // node
                if let TokenType::Ident(instrinc) = &self.current_token()?.typ {
                   match instrinc.as_str() {
                      "call" => self.parse_call(name)?,
                      _ => {
                          let ty = self.parse_type()?;
                          self.input.pop_front(); // the type
                          self.parse_const_assing(name, ty)?
                      }
                    }
                } else { unreachable!() }
            } else if let TokenType::Ident(instrinc) = curr.typ {
                match instrinc.as_str() {
                    "ret" => self.parse_ret()?,
                    "br" => self.parse_br()?,
                    _ => Err(IrError::UnkownInstrinc{loc: curr.loc.clone(), found: instrinc })?,
                }
            } else {
                Err(IrError::UnexpectedToken(curr.clone()))?
             }
        };

        let loc = curr.loc;

        Ok(IrInstr { 
            loc: loc, 
            inst: node
        })

    }

    fn parse_ret(&mut self) -> Result<Box<dyn Ir>, IrError> {
        self.input.pop_front(); // ret

        let out_ty = self.parse_type()?;
        self.input.pop_front();

        let curr = self.current_token()?;

        let out:  Result<Box<dyn Ir>, IrError> = if let TokenType::Int(numeral) = &curr.typ {
            Ok(ir::Return::new(Type::from_int(out_ty, *numeral)))
        } else if let TokenType::Var(var) = &curr.typ {
            Ok(ir::Return::new(Var {
                name: var.to_owned(),
                ty: out_ty,
            }))
        } else {
            Err(IrError::UndeterminedTokenSequence { 
                loc: curr.loc.clone(), 
                expected: "ints, vars - for valid return nodes".to_owned(), 
            })
        };

        self.input.pop_front();

        out
    }

    fn parse_const_assing(&mut self, var: String, ty: TypeMetadata) -> Result<Box<dyn Ir>, IrError> {
        let out = Var {
            name: var,
            ty: ty,
        };

        let curr = self.current_token()?;

        let out:  Result<Box<dyn Ir>, IrError>  = if let TokenType::Int(numeral) = &curr.typ {
            Ok(ir::Assign::new(out, Type::from_int(ty, *numeral)))
        } else if let TokenType::Var(var) = &curr.typ {
            Ok(ir::Assign::new(out, Var { 
                name: var.to_owned(),
                ty: ty,
            }))
        } else if let TokenType::Ident(cons) = &curr.typ {
            Ok(ir::Assign::new(out, Const::new(cons.to_string())))
        } else {
            Err(IrError::UndeterminedTokenSequence { 
                loc: curr.loc.clone(), 
                expected: "intenger, variable names - for valid constant assignments".to_owned() 
            })
        };

        self.input.pop_front();

        out
    }

    fn parse_call(&mut self, var: String) -> Result<Box<dyn Ir>, IrError> {
        self.input.pop_front(); // call

        let func_ty = self.parse_type()?;
        self.input.pop_front();

        self.expect(TokenType::Ident(String::from("func name")))?;

        let target = &self.current_token()?;

        let target = if let TokenType::Ident(ident) = &target.typ {
            ident.to_owned()
        } else { unreachable!() };

        self.input.pop_front(); // function name

        let out = Var {
            name: var,
            ty: func_ty
        };

        let mut args = vec![];

        loop {
            let ty = if let TokenType::Ident(ty) = &self.current_token()?.typ {
                if let Some(ty) = TypeMetadata::parse(ty.to_owned()) {
                    ty
                } else {
                    break
                }
            } else { break; };

            self.input.pop_front();

            let var = self.current_token()?;

            if let TokenType::Var(name) = &var.typ {
                args.push(Var {
                    name: name.to_string(),
                    ty: ty
                });
            }

            self.input.pop_front();
        }

        Ok(ir::Call::new(Function { 
            ty: FnTy(vec![], func_ty),
            name: target, 
            linkage: Linkage::External, 
            blocks: VecDeque::new(), 
        }, args, out))
    }

    fn parse_br(&mut self) -> Result<Box<dyn Ir>, IrError> {
        self.input.pop_front(); // br

        self.expect( TokenType::Ident(String::new()) )?; // block names are idents
        
        let block = if let TokenType::Ident(name) = &self.current_token()?.typ {
            name.to_owned()
        } else { unreachable!() };

        Ok(ir::Br::new(Box::from(Block { 
            name: block, 
            nodes: vec![], 
            varCount: 0
        })))
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