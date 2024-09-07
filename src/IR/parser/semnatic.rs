use std::collections::HashMap;

use crate::Obj::Linkage;
use crate::IR::{Const, FnTy, FunctionType, Type, TypeMetadata, Var};

use crate::prelude::ir::*;

use super::parser::{IrStmt, IrBlock};
use super::lexer::Loc;
use super::IrError;

/// semantic analaysiz for ir stmts
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IrSemnatic<'a> {
    input: &'a Vec<IrStmt>,

    const_sigs: HashMap<String, Linkage>,
    func_sigs: HashMap<String, (FunctionType, Linkage)>,
}

impl<'a> IrSemnatic<'a> {
    /// Creates an new ir semnatic analyzizer
    pub fn new(exprs: &'a Vec<IrStmt>) -> Self {
        Self {
            input: exprs,

            const_sigs: HashMap::new(),
            func_sigs: HashMap::new(),
        }
    }

    /// verifys the input
    pub fn verify(&mut self) -> Result<(), IrError> {
        for stmt in self.input {
            match stmt {
                IrStmt::Func { name, ret, args, body: _, scope, location } => self.add_func(name, *ret, args, scope, location)?,
                IrStmt::Const { name, data: _, location, scope } => self.add_const(name, scope, location)?
            }
        }

        for stmt in self.input {
            match stmt {
                IrStmt::Func { name, ret, args, body, scope, location } => self.analizye_func(name, *ret, args, body, *scope, location)?,
                IrStmt::Const { name, data, location, scope } => self.analyize_const(name, data, location, *scope)?,
            }
        }

        Ok(())
    }

    fn add_func(&mut self, name: &String, ret: TypeMetadata, args: &(HashMap<String, TypeMetadata>, bool),  scope: &Linkage, loc: &Loc) -> Result<(), IrError> {
        if self.func_sigs.contains_key(name) {
            Err(IrError::DefinedTwice {
                loc: loc.clone(),
                name: name.to_owned()
            })?
        }
        
        let mut fun_args = vec![];

        for (_, arg) in &args.0 {
            fun_args.push( *arg );
        }
        
        let ty = FnTy(fun_args, ret);

        self.func_sigs.insert(name.to_owned(), (ty, *scope));

        Ok(())
    }

    fn add_const(&mut self, name: &String, scope: &Linkage, loc: &Loc) -> Result<(), IrError> {
        if self.func_sigs.contains_key(name) {
            Err(IrError::DefinedTwice {
                loc: loc.clone(),
                name: name.to_owned()
            })?
        }

        self.const_sigs.insert(name.to_owned(), *scope);
        Ok(())
    }

    fn analizye_func(&mut self, name: &String, ret: TypeMetadata, args: &(HashMap<String, TypeMetadata>, bool), body: &HashMap<String, IrBlock>, scope: Linkage, loc: &Loc) -> Result<(), IrError> {
        let mut vars = HashMap::new();

        let mut blocks = vec![];

        for (name, ty) in &args.0 {
            vars.insert(name.to_owned(), *ty);
        }

        if Linkage::Extern == scope && body.len() > 0 {
            Err(IrError::ExternFunWithBody {
                name: name.to_owned(),
                loc: loc.to_owned(),
            })?
        }

        for (name, block) in body {
            if blocks.contains(name) {
                Err(IrError::DefinedTwice {
                    loc: block.loc.to_owned(),
                    name: name.to_owned()
                })?
            }

            blocks.push(name.to_owned());

            for node in &block.body {
                let instr = &node.inst;
                let loc = node.loc.to_owned();

                let any = instr.as_any();

                if let Some(node) = any.downcast_ref::<Return<Type>>() {
                    self.analiyze_ret_int(node, ret, loc)?;
                } else if let Some(node) = any.downcast_ref::<Assign<Var, Const>>() {
                    self.analiyze_assign_const(node, loc)?;
                }
            }
        }

        Ok(())
    }

    fn analiyze_ret_int(&mut self, node: &Return<Type>, fsig: TypeMetadata, loc: Loc) -> Result<(), IrError> {
        let ret: TypeMetadata = node.inner1.into();

        if ret != fsig {
            Err(IrError::FuncWrongReturnTyoe {
                expected: fsig,
                found: ret,
                loc: loc,
            })?
        }

        Ok(())
    }

    fn analiyze_assign_const(&mut self, node: &Assign<Var, Const>, loc: Loc) -> Result<(), IrError> {
        let name = &node.inner2.name;
        if !self.const_sigs.contains_key(name) {
            Err(IrError::Unkown {
                what: "const".to_owned(),
                loc: loc,
                name: name.to_owned(),
            })?
        }

        Ok(())
    }

    fn analyize_const(&mut self, _: &String, _: &Vec<u8>, _: &Loc, _: Linkage) -> Result<(), IrError> {
        Ok(()) // what can go wrong on constants?
    }
}   