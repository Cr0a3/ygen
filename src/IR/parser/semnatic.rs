use std::collections::{BTreeMap, HashMap};

use crate::Obj::Linkage;
use crate::IR::{BlockId, Const, FnTy, FuncId, FunctionType, Type, TypeMetadata, Var};

use crate::prelude::ir::*;

use super::parser::{IrStmt, IrBlock};
use super::lexer::Loc;
use super::IrError;

/// semantic analyze for ir stmts
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IrSemnatic<'a> {
    input: &'a Vec<IrStmt>,

    const_sigs: HashMap<String, Linkage>,
    func_sigs: HashMap<String, (FunctionType, Linkage, /*the blocks*/Vec<String>)>,
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
                IrStmt::Func { name, ret, args, body, scope, location } => self.add_func(name, *ret, args, scope, body, location)?,
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

    fn add_func(&mut self, name: &String, ret: TypeMetadata, args: &(BTreeMap<String, TypeMetadata>, bool),  scope: &Linkage, body: &Vec<(String, IrBlock)>, loc: &Loc) -> Result<(), IrError> {
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

        let mut blocks = vec![];

        for (name, _) in body {
            blocks.push( name.to_owned() );
        }

        self.func_sigs.insert(name.to_owned(), (ty, *scope, blocks));

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

    fn analizye_func(&mut self, name: &String, ret: TypeMetadata, args: &(BTreeMap<String, TypeMetadata>, bool), body: &Vec<(String, IrBlock)>, scope: Linkage, loc: &Loc) -> Result<(), IrError> {
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

        let func = name;

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
                } else if let Some(node) = any.downcast_ref::<Call<FuncId, Vec<Var>, Var>>() {
                    self.analyize_call(&mut vars, node, loc)?;
                } else if let Some(node) = any.downcast_ref::<Br<BlockId>>() {
                    self.analiyze_block(func, node, loc)?;
                } else if let Some(node) = any.downcast_ref::<Add<Var, Type, Var>>() {
                    self.analaysiz_add_var_ty(&mut vars, node, loc)?;
                } else if let Some(node) = any.downcast_ref::<Sub<Var, Type, Var>>() {
                    self.analaysiz_sub_var_ty(&mut vars, node, loc)?;
                } else if let Some(node) = any.downcast_ref::<Xor<Var, Type, Var>>() {
                    self.analaysiz_xor_var_ty(&mut vars, node, loc)?;
                } else if let Some(node) = any.downcast_ref::<Or<Var, Type, Var>>() {
                    self.analaysiz_or_var_ty(&mut vars, node, loc)?;
                } else if let Some(node) = any.downcast_ref::<And<Var, Type, Var>>() {
                    self.analaysiz_and_var_ty(&mut vars, node, loc)?;
                } else if let Some(node) = any.downcast_ref::<Mul<Var, Type, Var>>() {
                    self.analaysiz_mul_var_ty(&mut vars, node, loc)?;
                } else if let Some(node) = any.downcast_ref::<Div<Var, Type, Var>>() {
                    self.analaysiz_div_var_ty(&mut vars, node, loc)?;
                } else if let Some(node) = any.downcast_ref::<Rem<Var, Type, Var>>() {
                    self.analaysiz_rem_var_ty(&mut vars, node, loc)?;
                } else if let Some(node) = any.downcast_ref::<Shl<Var, Type, Var>>() {
                    self.analaysiz_shl_var_ty(&mut vars, node, loc)?;
                } else if let Some(node) = any.downcast_ref::<Shr<Var, Type, Var>>() {
                    self.analaysiz_shr_var_ty(&mut vars, node, loc)?;
                } else if let Some(node) = any.downcast_ref::<Add<Type, Type, Var>>() {
                    self.analaysiz_add_ty_ty(&mut vars, node, loc)?;
                } else if let Some(node) = any.downcast_ref::<Sub<Type, Type, Var>>() {
                    self.analaysiz_sub_ty_ty(&mut vars, node, loc)?;
                } else if let Some(node) = any.downcast_ref::<Xor<Type, Type, Var>>() {
                    self.analaysiz_xor_ty_ty(&mut vars, node, loc)?;
                } else if let Some(node) = any.downcast_ref::<Or<Type, Type, Var>>() {
                    self.analaysiz_or_ty_ty(&mut vars, node, loc)?;
                } else if let Some(node) = any.downcast_ref::<And<Type, Type, Var>>() {
                    self.analaysiz_and_ty_ty(&mut vars, node, loc)?;
                } else if let Some(node) = any.downcast_ref::<Mul<Type, Type, Var>>() {
                    self.analaysiz_mul_ty_ty(&mut vars, node, loc)?;
                } else if let Some(node) = any.downcast_ref::<Div<Type, Type, Var>>() {
                    self.analaysiz_div_ty_ty(&mut vars, node, loc)?;
                } else if let Some(node) = any.downcast_ref::<Rem<Type, Type, Var>>() {
                    self.analaysiz_rem_ty_ty(&mut vars, node, loc)?;
                } else if let Some(node) = any.downcast_ref::<Shl<Type, Type, Var>>() {
                    self.analaysiz_shl_ty_ty(&mut vars, node, loc)?;
                } else if let Some(node) = any.downcast_ref::<Shr<Type, Type, Var>>() {
                    self.analaysiz_shr_ty_ty(&mut vars, node, loc)?;
                } else if let Some(node) = any.downcast_ref::<Add<Var, Var, Var>>() {
                    self.analaysiz_add_var_var(&mut vars, node, loc)?;
                } else if let Some(node) = any.downcast_ref::<Sub<Var, Var, Var>>() {
                    self.analaysiz_sub_var_var(&mut vars, node, loc)?;
                } else if let Some(node) = any.downcast_ref::<Xor<Var, Var, Var>>() {
                    self.analaysiz_xor_var_var(&mut vars, node, loc)?;
                } else if let Some(node) = any.downcast_ref::<Or<Var, Var, Var>>() {
                    self.analaysiz_or_var_var(&mut vars, node, loc)?;
                } else if let Some(node) = any.downcast_ref::<And<Var, Var, Var>>() {
                    self.analaysiz_and_var_var(&mut vars, node, loc)?;
                } else if let Some(node) = any.downcast_ref::<Mul<Var, Var, Var>>() {
                    self.analaysiz_mul_var_var(&mut vars, node, loc)?;
                } else if let Some(node) = any.downcast_ref::<Div<Var, Var, Var>>() {
                    self.analaysiz_div_var_var(&mut vars, node, loc)?;
                } else if let Some(node) = any.downcast_ref::<Rem<Var, Var, Var>>() {
                    self.analaysiz_rem_var_var(&mut vars, node, loc)?;
                } else if let Some(node) = any.downcast_ref::<Shl<Var, Var, Var>>() {
                    self.analaysiz_shl_var_var(&mut vars, node, loc)?;
                } else if let Some(node) = any.downcast_ref::<Shr<Var, Var, Var>>() {
                    self.analaysiz_shr_var_var(&mut vars, node, loc)?;
                } else if let Some(node) = any.downcast_ref::<Cast<Var, TypeMetadata, Var>>() {
                    self.analaysiz_cast(&mut vars, node, loc)?;
                } else if let Some(node) = any.downcast_ref::<BrCond<Var, BlockId, BlockId>>() {
                    self.analaysiz_br_cond(func, &mut vars, node, loc)?;
                } else if let Some(node) = any.downcast_ref::<Cmp>() {
                    self.analaysiz_cmp(&mut vars, node, loc)?;
                } else if let Some(node) = any.downcast_ref::<Alloca<Var, TypeMetadata>>() {
                    self.analaysiz_alloca(&mut vars, node, loc)?;
                } else if let Some(node) = any.downcast_ref::<Store<Var, Var>>() {
                    self.analaysiz_store(&mut vars, node, loc)?;
                } else if let Some(node) = any.downcast_ref::<Store<Var, Type>>() {
                    self.analaysiz_store_ty(&mut vars, node, loc)?;
                } else if let Some(node) = any.downcast_ref::<Load<Var, Var, TypeMetadata>>() {
                    self.analaysiz_load(&mut vars, node, loc)?;
                } else if let Some(node) = any.downcast_ref::<Phi>() {
                    if vars.contains_key(&node.out.name) {
                        Err(IrError::DefinedTwice { loc: loc, name: node.out.name.to_owned() })?
                    } else {
                        vars.insert(node.out.name.to_owned(), node.typ);
                    }
                } else if let Some(node) = any.downcast_ref::<Switch>() {
                    self.analyze_switch(func, &mut vars, node, loc)?;
                } else if let Some(node) = any.downcast_ref::<Neg<Var, Var>>() {
                    self.analyze_neg(&mut vars, node, loc)?;
                } else if let Some(node) = any.downcast_ref::<Select<Var, Var>>() {
                    self.analyze_selectvv(&mut vars, node, loc)?;
                } else if let Some(node) = any.downcast_ref::<Select<Var, Type>>() {
                    self.analyze_selectvt(&mut vars, node, loc)?;
                } else if let Some(node) = any.downcast_ref::<Select<Type, Var>>() {
                    self.analyze_selecttv(&mut vars, node, loc)?;
                } else if let Some(node) = any.downcast_ref::<Select<Type, Type>>() {
                    self.analyze_selecttt(&mut vars, node, loc)?;
                } else if let Some(node) = any.downcast_ref::<GetElemPtr>() {
                    self.analyze_getelemptr(&mut vars, node, loc)?;
                } else {
                    todo!("implement node: {}", node.inst.dump());
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

    fn analyize_call(&mut self, vars: &mut HashMap<String, TypeMetadata>, node: &Call<FuncId, Vec<Var>, Var>, loc: Loc) -> Result<(), IrError> {
        let name = &node.inner1.name;
        let mut sig = node.inner1.ty.to_owned();

        if let Some((ty, _, _)) = self.func_sigs.get(&node.inner1.name) {
            sig = ty.to_owned();
        } else  {
            Err(IrError::Unkown {
                what: "function".to_owned(),
                loc: loc.to_owned(),
                name: name.to_owned(),
            })?
        }

        if sig.ret != node.inner1.ty.ret {
            Err(IrError::FuncWrongReturnTyoe { 
                expected: sig.ret, 
                found: node.inner1.ty.ret, 
                loc: loc.to_owned() 
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

    fn analaysiz_cast(&mut self, vars: &mut HashMap<String, TypeMetadata>, node: &Cast<Var, TypeMetadata, Var>, loc: Loc) -> Result<(), IrError> {
        if !vars.contains_key(&node.inner1.name) {
            Err(IrError::Unkown {
                loc: loc.to_owned(),
                name: node.inner1.name.to_owned(),

                what: "variable".into(),
            })?
        }

        vars.insert(node.inner3.name.to_owned(), node.inner2);
        
        Ok(())
    }

    fn analiyze_block(&mut self, func: &String, node: &Br<BlockId>, loc: Loc) -> Result<(), IrError> {
        let br_block = &node.inner1.name;

        let (_, _, blocks) = self.func_sigs.get(func).unwrap();

        if !blocks.contains(br_block) {
            Err(IrError::Unkown { 
                what: "block".to_owned(), 
                name: br_block.to_owned(), 
                loc: loc
            })?
        }

        Ok(())
    }

    fn analaysiz_br_cond(&mut self, func: &String, vars: &mut HashMap<String, TypeMetadata>, node: &BrCond<Var, BlockId, BlockId>, loc: Loc) -> Result<(), IrError> {
        let (_, _, blocks) = self.func_sigs.get(func).unwrap();

        if !blocks.contains(&node.inner2.name) {
            Err(IrError::Unkown { 
                what: "block".to_owned(), 
                name: node.inner2.name.to_owned(), 
                loc: loc.to_owned()
            })?
        }

        if !blocks.contains(&node.inner3.name) {
            Err(IrError::Unkown { 
                what: "block".to_owned(), 
                name: node.inner3.name.to_owned(), 
                loc: loc.to_owned()
            })?
        }

        if !vars.contains_key(&node.inner1.name) {
            Err(IrError::Unkown { 
                what: "variable".to_owned(), 
                name: node.inner1.name.to_owned(), 
                loc: loc
            })?
        }

        Ok(())
    }

    fn analaysiz_cmp(&mut self, vars: &mut HashMap<String, TypeMetadata>, node: &Cmp, loc: Loc) -> Result<(), IrError> {
        if !vars.contains_key(&node.ls.name) {
            Err(IrError::Unkown { 
                what: "variable".to_owned(), 
                name: node.ls.name.to_owned(), 
                loc: loc.clone()
            })?
        }

        if !vars.contains_key(&node.rs.name) {
            Err(IrError::Unkown { 
                what: "variable".to_owned(), 
                name: node.rs.name.to_owned(), 
                loc: loc.clone()
            })?
        }

        if vars.contains_key(&node.out.name) {
            Err(IrError::DefinedTwice {
                name: node.out.name.to_owned(), 
                loc: loc
            })?
        }

        vars.insert(node.out.name.to_owned(), node.out.ty);

        Ok(())
    }

    fn analaysiz_alloca(&mut self, vars: &mut HashMap<String, TypeMetadata>, node: &Alloca<Var, TypeMetadata>, loc: Loc) -> Result<(), IrError> {
        if vars.contains_key(&node.inner1.name) {
            Err(IrError::DefinedTwice { 
                loc: loc, 
                name: node.inner1.name.to_owned() 
            })?
        }

        vars.insert(node.inner1.name.to_owned(), node.inner1.ty);
        
        Ok(())
    }

    fn analaysiz_store(&mut self, vars: &mut HashMap<String, TypeMetadata>, node: &Store<Var, Var>, loc: Loc) -> Result<(), IrError> {
        if !vars.contains_key(&node.inner1.name) {
            Err(IrError::Unkown { 
                what: "var".to_owned(), 
                name: node.inner1.name.to_owned(), 
                loc: loc.to_owned()
            })?
        }

        if !vars.contains_key(&node.inner2.name) {
            Err(IrError::Unkown { 
                what: "var".to_owned(), 
                name: node.inner2.name.to_owned(), 
                loc: loc.to_owned()
            })?
        }
        
        Ok(())
    }
    
    fn analaysiz_store_ty(&mut self, vars: &mut HashMap<String, TypeMetadata>, node: &Store<Var, Type>, loc: Loc) -> Result<(), IrError> {
        if !vars.contains_key(&node.inner1.name) {
            Err(IrError::Unkown { 
                what: "var".to_owned(), 
                name: node.inner1.name.to_owned(), 
                loc: loc.to_owned()
            })?
        }
        
        Ok(())
    }

    fn analaysiz_load(&mut self, vars: &mut HashMap<String, TypeMetadata>, node: &Load<Var, Var, TypeMetadata>, loc: Loc) -> Result<(), IrError> {
        // TODO: maybe add checks that the pointer can only be a type of ptr and not i32
        
        if !vars.contains_key(&node.inner2.name) {
            Err(IrError::Unkown {
                what: "variable".to_owned(), 
                name: node.inner2.name.to_owned(), 
                loc: loc 
            })?
        }

        vars.insert(node.inner1.name.to_owned(), node.inner3);
        
        Ok(())
    }

    fn analyize_const(&mut self, _: &String, _: &Vec<u8>, _: &Loc, _: Linkage) -> Result<(), IrError> {
        Ok(()) // what can go wrong on constants?
    }

    fn analyze_switch(&mut self, func: &String, vars: &mut HashMap<String, TypeMetadata>, node: &Switch, loc: Loc) -> Result<(), IrError> {
        let (_, _, blocks) = self.func_sigs.get(func).unwrap();
        
        if !vars.contains_key(&node.to_switch.name) {
            Err(IrError::Unkown { 
                what: "variable".into(), 
                name: node.to_switch.name.to_owned(), 
                loc: loc.clone() 
            })?
        }

        if !blocks.contains(&node.default.name) {
            Err(IrError::Unkown { 
                what: "block".into(), 
                name: node.default.name.to_owned(), 
                loc: loc.clone() 
            })?
        }

        for (_, case) in &node.cases {
            if !blocks.contains(&case.name) {
                Err(IrError::Unkown { 
                    what: "block".into(), 
                    name: case.name.to_owned(), 
                    loc: loc.clone() 
                })?
            }
        }

        Ok(())
    }

    fn analyze_neg(&mut self, vars: &mut HashMap<String, TypeMetadata>, node: &Neg<Var, Var>, loc: Loc) -> Result<(), IrError> {
        if !vars.contains_key(&node.inner1.name) {
            Err(IrError::Unkown { 
                what: "variable".into(), 
                name: node.inner1.name.to_owned(), 
                loc: loc.clone() 
            })?
        }

        if vars.contains_key(&node.inner2.name) {
            Err(IrError::DefinedTwice { 
                name: node.inner1.name.to_owned(), 
                loc: loc.clone() 
            })?
        }

        vars.insert(node.inner2.name.to_owned(), node.inner2.ty);

        Ok(())
    }

    fn analyze_selectvv(&mut self, vars: &mut HashMap<String, TypeMetadata>, node: &Select<Var, Var>, loc: Loc) -> Result<(), IrError> {
        if !vars.contains_key(&node.yes.name) {
            Err(IrError::Unkown { 
                what: "variable".into(), 
                name: node.yes.name.to_owned(), 
                loc: loc.clone()
            })?
        }

        if !vars.contains_key(&node.no.name) {
            Err(IrError::Unkown {
                what: "variable".into(), 
                name: node.no.name.to_owned(), 
                loc: loc.clone()
            })?
        }

        if !vars.contains_key(&node.cond.name) {
            Err(IrError::Unkown {
                what: "variable".into(), 
                name: node.cond.name.to_owned(), 
                loc: loc.clone()
            })?
        }

        if vars.contains_key(&node.out.name) {
            Err(IrError::DefinedTwice {
                name: node.out.name.to_owned(), 
                loc: loc.clone()
            })?
        }

        vars.insert(node.out.name.clone(), node.out.ty);

        Ok(())
    }
    
    fn analyze_selecttv(&mut self, vars: &mut HashMap<String, TypeMetadata>, node: &Select<Type, Var>, loc: Loc) -> Result<(), IrError> {
        if !vars.contains_key(&node.no.name) {
            Err(IrError::Unkown {
                what: "variable".into(), 
                name: node.no.name.to_owned(), 
                loc: loc.clone()
            })?
        }

        if !vars.contains_key(&node.cond.name) {
            Err(IrError::Unkown {
                what: "variable".into(), 
                name: node.cond.name.to_owned(), 
                loc: loc.clone()
            })?
        }

        if vars.contains_key(&node.out.name) {
            Err(IrError::DefinedTwice {
                name: node.out.name.to_owned(), 
                loc: loc.clone()
            })?
        }

        vars.insert(node.out.name.clone(), node.out.ty);

        Ok(())
    }
    
    fn analyze_selectvt(&mut self, vars: &mut HashMap<String, TypeMetadata>, node: &Select<Var, Type>, loc: Loc) -> Result<(), IrError> {
        if !vars.contains_key(&node.yes.name) {
            Err(IrError::Unkown { 
                what: "variable".into(), 
                name: node.yes.name.to_owned(), 
                loc: loc.clone()
            })?
        }

        if !vars.contains_key(&node.cond.name) {
            Err(IrError::Unkown {
                what: "variable".into(), 
                name: node.cond.name.to_owned(), 
                loc: loc.clone()
            })?
        }

        if vars.contains_key(&node.out.name) {
            Err(IrError::DefinedTwice {
                name: node.out.name.to_owned(), 
                loc: loc.clone()
            })?
        }

        vars.insert(node.out.name.clone(), node.out.ty);

        Ok(())
    }

    fn analyze_selecttt(&mut self, vars: &mut HashMap<String, TypeMetadata>, node: &Select<Type, Type>, loc: Loc) -> Result<(), IrError> {
        if !vars.contains_key(&node.cond.name) {
            Err(IrError::Unkown {
                what: "variable".into(), 
                name: node.cond.name.to_owned(), 
                loc: loc.clone()
            })?
        }

        if vars.contains_key(&node.out.name) {
            Err(IrError::DefinedTwice {
                name: node.out.name.to_owned(), 
                loc: loc.clone()
            })?
        }

        vars.insert(node.out.name.clone(), node.out.ty);

        Ok(())
    }

    fn analyze_getelemptr(&mut self, vars: &mut HashMap<String, TypeMetadata>, node: &GetElemPtr, loc: Loc) -> Result<(), IrError> {
        if vars.contains_key(&node.out.name) {
            Err(IrError::DefinedTwice { 
                loc: loc.clone(), 
                name: node.out.name.to_owned()
            })?
        }

        if !vars.contains_key(&node.ptr.name) {
            Err(IrError::Unkown { 
                what: "variable".into(), 
                name: node.ptr.name.to_owned(), 
                loc: loc.clone() 
            })?
        }

        if !vars.contains_key(&node.index.name) {
            Err(IrError::Unkown { 
                what: "variable".into(), 
                name: node.index.name.to_owned(), 
                loc: loc.clone() 
            })?
        }

        vars.insert(node.out.name.to_owned(), node.ty);

        Ok(())
    }
}   

macro_rules! SemnaticImplMathVarTy {
    ($func:ident, $node:ident) => {
        impl<'a> IrSemnatic<'a> {
            fn $func(&mut self, vars: &mut HashMap<String, TypeMetadata>, node: &$node<Var, Type, Var>, loc: Loc) -> Result<(), IrError> {
                if !vars.contains_key(&node.inner1.name) {
                    Err(IrError::Unkown {
                        loc: loc.to_owned(),
                        name: node.inner1.name.to_owned(),

                        what: "variable".into(),
                    })?
                }

                if vars.contains_key(&node.inner3.name) {
                    Err(IrError::DefinedTwice {
                        loc: loc.to_owned(),
                        name: node.inner3.name.to_owned(),
                    })?
                }

                vars.insert(node.inner3.name.to_owned(), node.inner2.into());
                
                Ok(())
            }
        }
    };
}

SemnaticImplMathVarTy!(analaysiz_add_var_ty, Add);
SemnaticImplMathVarTy!(analaysiz_sub_var_ty, Sub);
SemnaticImplMathVarTy!(analaysiz_xor_var_ty, Xor);
SemnaticImplMathVarTy!(analaysiz_or_var_ty,  Or );
SemnaticImplMathVarTy!(analaysiz_and_var_ty, And);
SemnaticImplMathVarTy!(analaysiz_mul_var_ty, Mul);
SemnaticImplMathVarTy!(analaysiz_div_var_ty, Div);
SemnaticImplMathVarTy!(analaysiz_rem_var_ty, Rem);
SemnaticImplMathVarTy!(analaysiz_shl_var_ty, Shl);
SemnaticImplMathVarTy!(analaysiz_shr_var_ty, Shr);

macro_rules! SemnaticImplMathTyTy {
    ($func:ident, $node:ident) => {
        impl<'a> IrSemnatic<'a> {
            fn $func(&mut self, vars: &mut HashMap<String, TypeMetadata>, node: &$node<Type, Type, Var>, loc: Loc) -> Result<(), IrError> {
                if vars.contains_key(&node.inner3.name) {
                    Err(IrError::DefinedTwice {
                        loc: loc.to_owned(),
                        name: node.inner3.name.to_owned(),
                    })?
                }

                vars.insert(node.inner3.name.to_owned(), node.inner1.into());
                
                Ok(())
            }
        }
    };
}

SemnaticImplMathTyTy!(analaysiz_add_ty_ty, Add);
SemnaticImplMathTyTy!(analaysiz_sub_ty_ty, Sub);
SemnaticImplMathTyTy!(analaysiz_xor_ty_ty, Xor);
SemnaticImplMathTyTy!(analaysiz_or_ty_ty,  Or );
SemnaticImplMathTyTy!(analaysiz_and_ty_ty, And);
SemnaticImplMathTyTy!(analaysiz_mul_ty_ty, Mul);
SemnaticImplMathTyTy!(analaysiz_div_ty_ty, Div);
SemnaticImplMathTyTy!(analaysiz_rem_ty_ty, Rem);
SemnaticImplMathTyTy!(analaysiz_shl_ty_ty, Shl);
SemnaticImplMathTyTy!(analaysiz_shr_ty_ty, Shr);

macro_rules! SemnaticImplMathVarVar {
    ($func:ident, $node:ident) => {
        impl<'a> IrSemnatic<'a> {
            fn $func(&mut self, vars: &mut HashMap<String, TypeMetadata>, node: &$node<Var, Var, Var>, loc: Loc) -> Result<(), IrError> {
                if !vars.contains_key(&node.inner1.name) {
                    Err(IrError::Unkown {
                        loc: loc.to_owned(),
                        name: node.inner1.name.to_owned(),

                        what: "variable".into(),
                    })?
                }

                if !vars.contains_key(&node.inner2.name) {
                    Err(IrError::Unkown {
                        loc: loc.to_owned(),
                        name: node.inner2.name.to_owned(),

                        what: "variable".into(),
                    })?
                }

                if vars.contains_key(&node.inner3.name) {
                    Err(IrError::DefinedTwice {
                        loc: loc.to_owned(),
                        name: node.inner3.name.to_owned(),
                    })?
                }

                let ty = if let Some(var) = vars.get(&node.inner2.name) { var } else { unreachable!() };

                vars.insert(node.inner3.name.to_owned(), *ty);
                
                Ok(())
            }
        }
    };
}

SemnaticImplMathVarVar!(analaysiz_add_var_var, Add);
SemnaticImplMathVarVar!(analaysiz_sub_var_var, Sub);
SemnaticImplMathVarVar!(analaysiz_xor_var_var, Xor);
SemnaticImplMathVarVar!(analaysiz_or_var_var,  Or );
SemnaticImplMathVarVar!(analaysiz_and_var_var, And);
SemnaticImplMathVarVar!(analaysiz_mul_var_var, Mul);
SemnaticImplMathVarVar!(analaysiz_div_var_var, Div);
SemnaticImplMathVarVar!(analaysiz_rem_var_var, Rem);
SemnaticImplMathVarVar!(analaysiz_shl_var_var, Shl);
SemnaticImplMathVarVar!(analaysiz_shr_var_var, Shr);