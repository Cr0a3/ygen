use std::collections::HashMap;

use crate::Obj::Linkage;
use crate::IR::{Const, FnTy, Function, FunctionType, Type, TypeMetadata, Var};

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
        
        let mut ty = FnTy(fun_args, ret);

        if args.1 {
            ty.activate_dynamic_arguments();
        }

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
                } else if let Some(node) = any.downcast_ref::<Return<Var>>() {
                    self.analiyze_ret_var(&mut vars, node, ret, loc)?;
                } else if let Some(node) = any.downcast_ref::<Assign<Var, Const>>() {
                    self.analiyze_assign_const(&mut vars, node, loc)?;
                } else if let Some(node) = any.downcast_ref::<Assign<Var, Var>>() {
                    self.analiyze_assign_var(&mut vars, node, loc)?;
                } else if let Some(node) = any.downcast_ref::<Assign<Var, Type>>() {
                    self.analiyze_assign_type(&mut vars, node, loc)?;
                } else if let Some(node) = any.downcast_ref::<Call<Function, Vec<Var>, Var>>() {
                    self.analyize_call(&mut vars, node, loc)?;
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

    fn analiyze_ret_var(&mut self, vars: &mut HashMap<String, TypeMetadata>, node: &Return<Var>, fsig: TypeMetadata, loc: Loc) -> Result<(), IrError> {
        let name = node.inner1.name.to_owned();

        if let Some(var) = vars.get(&name) {
            if *var != fsig {
                Err(IrError::FuncWrongReturnTyoe { 
                    expected: fsig, 
                    found: *var, 
                    loc: loc.to_owned()
                })?
            }
        } else {
            Err(IrError::Unkown { 
                what: "variable".to_owned(), 
                name: name.to_owned(), 
                loc: loc.to_owned()
            })?
        }
        
        Ok(())
    }

    fn analiyze_assign_const(&mut self, vars: &mut HashMap<String, TypeMetadata>, node: &Assign<Var, Const>, loc: Loc) -> Result<(), IrError> {
        let name = &node.inner2.name;
        if !self.const_sigs.contains_key(name) {
            Err(IrError::Unkown {
                what: "const".to_owned(),
                loc: loc.to_owned(),
                name: name.to_owned(),
            })?
        }

        let name = node.inner1.name.to_owned();

        if vars.contains_key(&name) {
            Err(IrError::DefinedTwice { 
                loc: loc.to_owned(), 
                name: name.to_owned(),
            })?
        }

        vars.insert(name, TypeMetadata::ptr);

        Ok(())
    }

    fn analiyze_assign_var(&mut self, vars: &mut HashMap<String, TypeMetadata>, node: &Assign<Var, Var>, loc: Loc) -> Result<(), IrError> {
        let name = &node.inner2.name;

        let mut op2 = TypeMetadata::Void;

        if let Some(var) = vars.get(name) {
            op2 = *var;
        } else {
            Err(IrError::Unkown {
                what: "var".to_owned(),
                loc: loc.to_owned(),
                name: name.to_owned(),
            })?
        }

        let name = node.inner1.name.to_owned();

        if vars.contains_key(&name) {
            Err(IrError::DefinedTwice { 
                loc: loc.to_owned(), 
                name: name.to_owned(),
            })?
        }

        vars.insert(name, op2);

        Ok(())
    }

    fn analiyze_assign_type(&mut self, vars: &mut HashMap<String, TypeMetadata>, node: &Assign<Var, Type>, loc: Loc) -> Result<(), IrError> {
        let name = node.inner1.name.to_owned();

        if vars.contains_key(&name) {
            Err(IrError::DefinedTwice { 
                loc: loc.to_owned(), 
                name: name.to_owned(),
            })?
        }

        vars.insert(name, node.inner2.into());

        Ok(())
    }

    fn analyize_call(&mut self, vars: &mut HashMap<String, TypeMetadata>, node: &Call<Function, Vec<Var>, Var>, loc: Loc) -> Result<(), IrError> {
        let name = &node.inner1.name;
        let mut sig = node.inner1.ty.to_owned();

        if let Some((ty, _)) = self.func_sigs.get(&node.inner1.name) {
            sig = ty.to_owned();
        } else  {
            Err(IrError::Unkown {
                what: "function".to_owned(),
                loc: loc.to_owned(),
                name: name.to_owned(),
            })?
        }

        let name = node.inner3.name.to_owned();

        if vars.contains_key(&name) {
            Err(IrError::DefinedTwice { 
                loc: loc.to_owned(), 
                name: name.to_owned(),
            })?
        }

        vars.insert(name, sig.ret);

        let mut index = 0;

        for arg in &node.inner2 {
            let arg = if let Some(var) = vars.get(&arg.name) {
                var
            } else {
                Err(IrError::Unkown { 
                    what: "variable".to_owned(), 
                    name: arg.name.to_owned(), 
                    loc: loc.to_owned(), 
                })?
            };

            if let Some(expected) = sig.args.get(index) {
                if *expected != *arg {
                    Err(IrError::WrongArgument {
                        loc: loc.to_owned(),
                        index: index,
                        expected: Some(*expected),
                        found: *arg,
                    })?
                }
            } else {
                if !sig.any_args {
                    Err(IrError::TooManyArgsVerySupplyed {
                        loc: loc.to_owned(),
                        expected: sig.args.len(),
                    })?
                }
            }

            index += 1;
        }


        Ok(())
    }

    fn analyize_const(&mut self, _: &String, _: &Vec<u8>, _: &Loc, _: Linkage) -> Result<(), IrError> {
        Ok(()) // what can go wrong on constants?
    }
}   