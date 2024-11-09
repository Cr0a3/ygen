use std::collections::{BTreeMap, HashMap, VecDeque};
use std::path::PathBuf;

use crate::prelude::{Alloca, Cmp, CmpMode, DebugNode, GetElemPtr, IROperand, Ir, Load, Neg, Phi, Select, Store, Switch};
use crate::Obj::Linkage;
use crate::IR::block::BlockId;
use crate::IR::{ir, Block, Const, FnTy, Type, TypeMetadata, Var};

use super::lexer::{Loc, Token, TokenType};
use super::IrError;

#[derive(Debug, Clone, Eq)]
#[allow(missing_docs)]
#[allow(private_interfaces)]
pub struct IrInstr {
    pub loc: Loc,
    pub inst: Box<dyn Ir>,
}

impl PartialEq for IrInstr {
    fn eq(&self, other: &Self) -> bool {
        self.loc == other.loc && &self.inst == &other.inst
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(missing_docs)]
pub struct IrBlock {
    pub loc: Loc,
    pub body: Vec<IrInstr>,
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
        args: (BTreeMap<String, TypeMetadata>, /*unlim args*/bool), 
        body: Vec<(String, IrBlock)>,
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
        let mut args = BTreeMap::new();
        
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
            if TokenType::Comma == self.current_token()?.typ {
                self.input.pop_front();
            }

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
            body: Vec::new(),
            scope: Linkage::Extern,
            args: (args, unlim),
            ret: ret,

            location: loc,
        })
    }

    fn parse_define(&mut self) -> Result<IrStmt, IrError> {
        let name;
        let mut body = vec![];
        let mut args = BTreeMap::new();
        
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

            if TokenType::Comma == current.typ {
                self.input.pop_front();
            }

            let var_type = self.parse_type()?;
            self.input.pop_front();

            self.expect( TokenType::Var(String::new()) )?;

            let token = self.current_token()?;

            let var_name = match &token.typ {
                TokenType::Var(name) => name.to_string(),
                
                _=> Err(IrError::UnexpectedToken(token.to_owned()))?
            };
            
            self.input.pop_front();

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

            body.push( (name, block) );
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
                "local" | "internal" | "intern" | "private" => {
                    parsed_scope = true;
                    scope = Linkage::Internal
                },
                "public" | "external" | "extern" => {
                    parsed_scope = true;
                    scope = Linkage::External
                },
                "import" => {
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
                        "sub" => self.parse_sub(name)?,
                        "add" => self.parse_add(name)?,
                        "mul" => self.parse_mul(name)?,
                        "cast" => self.parse_cast(name)?,
                        "xor" => self.parse_xor(name)?,
                        "or" => self.parse_or(name)?,
                        "and" => self.parse_and(name)?,
                        "div" => self.parse_div(name)?,
                        "call" => self.parse_call(name)?,
                        "cmp" => self.parse_cmp(name)?,
                        "alloca" => self.parse_alloca(name)?,
                        "load" => self.parse_load(name)?,
                        "phi" => self.parse_phi(name)?,
                        "neg" => self.parse_neg(name)?,
                        "select" => self.parse_select(name)?,
                        "rem" => self.parse_rem(name)?,
                        "shl" => self.parse_shl(name)?,
                        "shr" => self.parse_shr(name)?,
                        "getelemptr" => self.parse_getelemptr(name)?,
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
                    "store" => self.parse_store()?,
                    "switch" => self.parse_switch()?,
                    _ => Err(IrError::UnkownInstrinc{loc: curr.loc.clone(), found: instrinc })?,
                }
            } else if let TokenType::ExclamationMark = curr.typ {
                self.input.pop_front();
                self.expect_ident("dbg".to_owned())?;
                self.input.pop_front();

                self.expect(TokenType::Int(0.0))?;
                let line = if let TokenType::Int(int) = &self.current_token()?.typ {
                    *int
                } else { unreachable!() };
                self.input.pop_front();

                self.expect(TokenType::Dot)?;
                self.input.pop_front();

                self.expect(TokenType::Int(0.0))?;
                let coloumn = if let TokenType::Int(int) = &self.current_token()?.typ {
                    *int
                } else { unreachable!() };
                self.input.pop_front();

                self.expect_ident("in".to_owned())?;
                self.input.pop_front();

                self.expect(TokenType::UnIdent(String::new()))?;
                let file = if let TokenType::UnIdent(string) = &self.current_token()?.typ {
                    string.to_owned()
                } else { unreachable!() };
                self.input.pop_front();

                Box::new(DebugNode {
                    line: line as i64,
                    coloumn: coloumn as i64,
                    file: PathBuf::from(&file),
                })
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

        let ret_op = self.parse_operand(out_ty)?;

        self.input.pop_front();

        Ok(ir::Return::new(ret_op))
    }

    fn parse_operand(&mut self, ty: TypeMetadata) -> Result<IROperand, IrError> {
        let curr = self.current_token()?;

        if let TokenType::Int(numeral) = &curr.typ {
            Ok(IROperand::Type(Type::from_int(ty, *numeral)))
        } else if let TokenType::Var(var) = &curr.typ {
            Ok(IROperand::Var(Var {
                name: var.to_owned(),
                ty: ty,
            }))
        } else {
            Err(IrError::UndeterminedTokenSequence { 
                loc: curr.loc.clone(), 
                expected: "intenger, variable names - for valid constant assignments".to_owned() 
            })
        }
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

        Ok(ir::Call::new(crate::IR::FuncId {
            ty: FnTy(vec![], func_ty),
            name: target,
        }, args, out))
    }

    fn parse_br(&mut self) -> Result<Box<dyn Ir>, IrError> {
        self.input.pop_front(); // br

        let current = self.current_token()?;

        match &current.typ {
            TokenType::Ident(block) => {
                let block = block.to_owned();

                self.input.pop_front();

                Ok(ir::Br::new( BlockId(block) ))
            },
            TokenType::Cond => {
                self.input.pop_front(); // cond

                let var = self.expect(TokenType::Var(String::new()))?;

                let var = if let TokenType::Var(var) = var.typ 
                                                { var } else { unreachable!() };

                self.input.pop_front();

                self.expect(TokenType::Ident(String::new()))?;

                let iftrue;

                if let TokenType::Ident(ident) = &self.current_token()?.typ {
                    iftrue = ident.to_owned();
                } else { unreachable!() }

                self.input.pop_front();

                self.expect(TokenType::Comma)?;
                self.input.pop_front();

                self.expect(TokenType::Ident(String::new()))?;

                let iffalse;

                if let TokenType::Ident(ident) = &self.current_token()?.typ {
                    iffalse = ident.to_owned();
                } else { unreachable!() }
                self.input.pop_front();

                Ok(ir::BrCond::new(Var {
                    name: var,
                    ty: TypeMetadata::Void,
                }, 
                BlockId(iftrue), 
                BlockId(iffalse)
                ))
            },
            _ => Err(IrError::UnexpectedToken(self.current_token()?.clone())),
        }
    }

    fn parse_cast(&mut self, var: String) -> Result<Box<dyn Ir>, IrError> {
        self.input.pop_front();

        let ty = self.parse_type()?;
        self.input.pop_front();

        self.expect(TokenType::Var(String::new()))?;

        let in_var = if let TokenType::Var(name) = &self.current_token()?.typ {
            Var {
                name: name.to_owned(),
                ty: ty,
            }
        } else { unreachable!() };

        self.input.pop_front();

        self.expect_ident("to".into())?;
        self.input.pop_front();

        let out_ty = self.parse_type()?;
        
        self.input.pop_front();

        let out = Var { 
            name: var, 
            ty: out_ty 
        };

        Ok(ir::Cast::new(in_var, out_ty, out))
    }

    fn parse_alloca(&mut self, var: String) -> Result<Box<dyn Ir>, IrError> {
        self.input.pop_front();

        let ty = self.parse_type()?;

        self.input.pop_front();

        let out = Var {
            name: var,
            ty: TypeMetadata::ptr,
        };

        Ok( Alloca::new(out, ty) )
    }

    fn parse_store(&mut self) -> Result<Box<dyn Ir>, IrError> {
        self.input.pop_front();

        let ty = self.parse_type()?;
        self.input.pop_front();

        if let TokenType::Int(imm) = self.current_token()?.typ {
            self.input.pop_front();

            self.expect(TokenType::Comma)?;
            self.input.pop_front();

            self.expect(TokenType::Var(String::new()) )?;

            let var = if let TokenType::Var(var) = &self.current_token()?.typ {
                var.to_owned()
            } else { unreachable!() };
            self.input.pop_front();

            Ok(Store::new(Var {
                name: var,
                ty: TypeMetadata::ptr,
            }, Type::from_int(ty, imm)))
        } else if let TokenType::Var(var) = self.current_token()?.typ.to_owned() {
            self.input.pop_front();

            self.expect( TokenType::Comma )?;
            self.input.pop_front();

            self.expect(TokenType::Var(String::new()))?;
            let src = if let TokenType::Var(var) = &self.current_token()?.typ {
                var.to_owned()
            } else { unreachable!() };
            self.input.pop_front();

            Ok(Store::new(Var {
                name: src,
                ty: TypeMetadata::ptr,
            }, Var {
                name: var,
                ty: ty
            }))
        } else {
            Err(IrError::ExpectedTokenButFoundAnUnexpectedOne { 
                found: self.current_token()?.clone(), 
                expected: Token { typ: TokenType::Var(String::new()), loc: Loc::default() }
            })
        }
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

    fn expect_ident(&mut self, expected: String) -> Result<(), IrError> {
        self.expect(TokenType::Ident(String::new()))?;

        let ident = &self.current_token()?.typ;

        let ident = if let TokenType::Ident(ident) = ident {
            ident
        } else { 
            unreachable!() 
        };

        if ident.to_owned() != expected {
            Err(IrError::ExpectedTokenButFoundAnUnexpectedOne { 
                found: self.current_token()?.clone(), 
                expected: Token { 
                    typ: TokenType::Ident(expected.to_owned()), 
                    loc: self.current_token()?.loc.clone() 
                }
            })?
        }

        Ok(())
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

    fn parse_cmp(&mut self, var: String) -> Result<Box<dyn Ir>, IrError> {
        self.input.pop_front();

        self.expect( TokenType::Ident(String::new()) )?;

        let curr = self.current_token()?;
        let ident = if let TokenType::Ident(ident) = &curr.typ {
            ident.to_owned()
        } else { unreachable!() };

        let mode = match ident.as_str() {
            "eq" => CmpMode::Eqal,
            "ne" => CmpMode::NotEqal,
            "ge" => CmpMode::GreaterThan,
            "le" => CmpMode::LessThan,
            "gte" => CmpMode::GreaterThanOrEqual,
            "lte" => CmpMode::LessThanOrEqual,
            _=> Err(IrError::Unkown { 
                what: "compare mode".to_owned(), 
                name: ident, 
                loc: curr.loc.to_owned()
            })?
        };

        self.input.pop_front();

        let ty = self.parse_type()?;
        self.input.pop_front();

        self.expect( TokenType::Var(String::new()) )?;
        let curr = self.current_token()?;
        let ls = if let TokenType::Var(var) = &curr.typ {
            var.to_owned()
        } else { unreachable!() };
        self.input.pop_front();

        if let TokenType::Comma = self.current_token()?.typ {
            self.input.pop_front();
        }

        self.expect( TokenType::Var(String::new()) )?;
        let curr = self.current_token()?;
        let rs = if let TokenType::Var(var) = &curr.typ {
            var.to_owned()
        } else { unreachable!() };
        self.input.pop_front();

        Ok(Cmp::new(mode, Var {
            name: ls,
            ty: ty,
        }, Var {
            name: rs,
            ty: ty 
        }, Var {
            name: var,
            ty: TypeMetadata::u8
        }))
    }

    fn parse_load(&mut self, var: String) -> Result<Box<dyn Ir>, IrError> {
        self.input.pop_front();

        let ty = self.parse_type()?;

        let out = Var {
            name: var,
            ty: ty
        };

        self.input.pop_front(); // ty
        self.expect(TokenType::Var(String::new()))?;

        let var = if let TokenType::Var(var) = self.current_token()?.typ.to_owned() {
            Var {
                name: var,
                ty: TypeMetadata::ptr
            }
        } else { unreachable!() };

        self.input.pop_front();

        Ok(Load::new(out, var, ty))
    }

    fn parse_phi(&mut self, var: String) -> Result<Box<dyn Ir>, IrError> {
        self.input.pop_front();

        let out_ty = self.parse_type()?;
        let out = Var {
            name: var,
            ty: out_ty
        };

        self.input.pop_front();

        self.expect(TokenType::LSquare)?;
        self.input.pop_front();

        let mut recives = Vec::new();

        loop {
            let current = self.input.pop_front();
            let current = if let Some(current) = current { current } else { Err(IrError::OutOfTokens)? };

            if let TokenType::RSquare = current.typ {
                break;
            }

            if let TokenType::Var(var) = current.typ {
                self.expect(TokenType::Comma)?;
                self.input.pop_front();

                let current = self.input.pop_front().expect("the parser ran out of tokens");
                if let TokenType::Ident(block) = current.typ {
                    recives.push((Block {
                        name: block,
                        nodes: Vec::new(),
                        varCount: 0,
                    }, Var {
                        name: var,
                        ty: out_ty,
                    }));
                } else {
                    Err(IrError::UnexpectedToken(current))?
                }
            } else {
                Err(IrError::UnexpectedToken(current))?
            }

        }

        Ok(Box::new(Phi::new(out, recives, out_ty)))
    }

    fn parse_switch(&mut self) -> Result<Box<dyn Ir>, IrError> {
        self.input.pop_front(); // switch

        let typ = self.parse_type()?;
        self.input.pop_front();

        self.expect(TokenType::Var(String::new()))?;
        let var = if let TokenType::Var(var) = &self.current_token()?.typ {
            Var {
                name: var.to_owned(),
                ty: typ,
            }
        } else { unreachable!() };
        self.input.pop_front();

        self.expect(TokenType::Comma)?;
        self.input.pop_front();

        self.expect_ident("default".into())?;
        self.input.pop_front();

        self.expect(TokenType::Ident(String::new()))?;
        let default = if let TokenType::Ident(ident) = &self.current_token()?.typ {
            ident.to_owned()
        } else { unreachable!() };
        self.input.pop_front();

        self.expect(TokenType::LSquare)?;
        self.input.pop_front();

        let mut cases = HashMap::new();

        loop {
            let current = self.current_token()?;

            if let TokenType::RSquare = current.typ {
                break;
            }

            let ty = self.parse_type()?;
            self.input.pop_front();

            self.expect(TokenType::Int(0.0))?;
            let ty = if let TokenType::Int(int) = &self.current_token()?.typ {
                Type::from_int(ty, *int)
            } else { unreachable!() };
            self.input.pop_front();

            self.expect(TokenType::Comma)?;
            self.input.pop_front();

            self.expect(TokenType::Ident(String::new()))?;
            let block = if let TokenType::Ident(ident) = &self.current_token()?.typ {
                ident.to_owned()
            } else { unreachable!() };
            self.input.pop_front();

            cases.insert(ty, BlockId(block));
        }

        self.input.pop_front();

        Ok(Box::new( Switch::new(var, cases, BlockId(default)) ))
    }

    fn parse_neg(&mut self, var: String) -> Result<Box<dyn Ir>, IrError> {
        self.input.pop_front(); // neg

        let typ = self.parse_type()?;
        self.input.pop_front();

        let out = Var {
            name: var,
            ty: typ
        };

        self.expect(TokenType::Var(String::new()))?;
        let var = if let TokenType::Var(var) = &self.current_token()?.typ {
            Var {
                name: var.to_owned(),
                ty: typ
            }
        } else { unreachable!() };
        self.input.pop_front();

        Ok( Neg::new(var, out) )
    }

    fn parse_select(&mut self, var: String) -> Result<Box<dyn Ir>, IrError> {
        self.input.pop_front(); // select

        let ty = self.parse_type()?;
        self.input.pop_front(); // type

        let out = Var {
            name: var,
            ty: ty
        };

        self.expect(TokenType::Var(String::new()))?;
        let cond = if let TokenType::Var(cond) = &self.current_token()?.typ {
            Var {
                name: cond.to_owned(),
                ty: ty,
            }
        } else { unreachable!() };
        self.input.pop_front(); // cond

        self.expect(TokenType::Comma)?;
        self.input.pop_front(); // ,

        let _op1_ty = self.parse_type()?;
        self.input.pop_front(); // ty

        let current = self.current_token()?;

        if let TokenType::Var(op1) = &current.typ {
            let op1 = Var {
                name: op1.to_owned(),
                ty: ty
            };

            self.input.pop_front(); // var1
            self.expect(TokenType::Comma)?;
            self.input.pop_front(); // ,

            //self.parse_type()?;
            self.input.pop_front();

            let current = self.current_token()?;

            if let TokenType::Var(op2) = &current.typ {
                let op2 = Var {
                    name: op2.to_owned(),
                    ty: ty
                };

                self.input.pop_front();

                Ok(Box::new(Select {
                    out: out,
                    cond: cond,
                    yes: op1,
                    no: op2,
                }))
            } else if let TokenType::Int(op2) = &current.typ {
                let op2 = Type::from_int(ty, *op2);

                self.input.pop_front();

                Ok(Box::new(Select {
                    out: out,
                    cond: cond,
                    yes: op1,
                    no: op2,
                }))
            } else {
                Err(IrError::UnexpectedToken(current.clone()))
            }
        } else if let TokenType::Int(imm) = &current.typ {
            let op1 = Type::from_int(ty, *imm);
            self.input.pop_front();

            self.expect(TokenType::Comma)?;
            self.input.pop_front();

            self.parse_type()?;
            self.input.pop_front();

            let current = self.current_token()?;

            if let TokenType::Var(op2) = &current.typ {
                let op2 = Var {
                    name: op2.to_owned(),
                    ty: ty
                };

                self.input.pop_front();

                Ok(Box::new(Select {
                    out: out,
                    cond: cond,
                    yes: op1,
                    no: op2,
                }))
            } else if let TokenType::Int(op2) = &current.typ {
                let op2 = Type::from_int(ty, *op2);

                self.input.pop_front();

                Ok(Box::new(Select {
                    out: out,
                    cond: cond,
                    yes: op1,
                    no: op2,
                }))
            } else {
                Err(IrError::UnexpectedToken(current.clone()))
            }
        } else {
            Err(IrError::UnexpectedToken(current.clone()))
        }
    }

    fn parse_getelemptr(&mut self, var: String) -> Result<Box<dyn Ir>, IrError> {
        self.input.pop_front();

        let ptr_ty = self.parse_type()?;
        self.input.pop_front();

        self.expect(TokenType::Var(String::new()))?;
        let ptr = if let TokenType::Var(ptr) = &self.current_token()?.typ {
            Var {
                name: ptr.to_owned(),
                ty: ptr_ty
            }
        } else { unreachable!() };
        self.input.pop_front();

        self.expect(TokenType::Comma)?;
        self.input.pop_front();

        let index_ty = self.parse_type()?;
        self.input.pop_front();

        self.expect(TokenType::Var(String::new()))?;
        let index = if let TokenType::Var(index) = &self.current_token()?.typ {
            Var {
                name: index.to_owned(),
                ty: index_ty
            }
        } else { unreachable!() };
        self.input.pop_front();

        self.expect(TokenType::Comma)?;
        self.input.pop_front();

        let out_ty = self.parse_type()?;
        self.input.pop_front();

        Ok(GetElemPtr {
            ptr: ptr,
            ty: out_ty,
            out: Var {
                name: var,
                ty: out_ty
            },
            index: index,
        }.clone_box())
    }
}

macro_rules! ParserImplParseMath {
    ($func:ident, $node:ident) => {
        impl IrParser {
            fn $func(&mut self, var: String) -> Result<Box<dyn Ir>, IrError> {
                self.input.pop_front(); // add/sub/xor/or/and/mul/div

                let ty = self.parse_type()?;
                self.input.pop_front(); // out_ty

                let out = Var {
                    name: var,
                    ty: ty,
                };

                let curr = self.current_token()?;

                Ok(match &curr.typ {
                    TokenType::Int(op1) => {
                        let op1 = *op1;

                        self.input.pop_front(); // num1

                        self.expect(TokenType::Comma)?;
                        self.input.pop_front(); // ,

                        self.expect(TokenType::Int(0.0))?;
                        let op2;
                        if let TokenType::Int(int) = &self.current_token()?.typ {
                            op2 = *int;
                        } else { unreachable!() }

                        self.input.pop_front();

                        let op1 = Type::from_int(ty, op1);
                        let op2 = Type::from_int(ty, op2);

                        ir::$node::new(op1, op2, out)
                    },

                    TokenType::Var(op1) => {
                        let op1 = op1.to_owned();

                        self.input.pop_front(); // op1

                        self.expect(TokenType::Comma)?;
                        self.input.pop_front();

                        if let TokenType::Var(var) = &self.current_token()?.typ {
                            let op1 = Var {
                                name: op1,
                                ty: ty
                            };
    
                            let op2 = Var {
                                name: var.to_owned(),
                                ty: ty
                            };

                            self.input.pop_front();
    
                            ir::$node::new(op1, op2, out)
                        } else if let TokenType::Int(op) = &self.current_token()?.typ {
                            let op1 = Var {
                                name: op1,
                                ty: ty
                            };
    
                            let op2 = Type::from_int(ty, *op);

                            self.input.pop_front();
    
                            ir::$node::new(op1, op2, out)
                        } else { 
                            Err(IrError::ExpectedTokenButFoundAnUnexpectedOne {
                                found: self.current_token()?.to_owned(),

                                expected: Token { typ: TokenType::Var(") or Int()".into()), loc: Loc::default() },
                            })?
                        }
                    },

                    _ => Err(IrError::ExpectedTokenButFoundAnUnexpectedOne {
                        expected: Token { typ: TokenType::Var(String::new()), loc: Loc::default() },
                        found: curr.clone(),
                    })?
                })
            }
        }
    };
}

ParserImplParseMath!(parse_add, Add);
ParserImplParseMath!(parse_sub, Sub);
ParserImplParseMath!(parse_xor, Xor);
ParserImplParseMath!(parse_or,  Or );
ParserImplParseMath!(parse_and, And);
ParserImplParseMath!(parse_mul, Mul);
ParserImplParseMath!(parse_div, Div);
ParserImplParseMath!(parse_rem, Rem);
ParserImplParseMath!(parse_shl, Shl);
ParserImplParseMath!(parse_shr, Shr);