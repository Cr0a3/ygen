use std::collections::{BTreeMap, HashMap};

use crate::Obj::Linkage;
use crate::IR::{BlockId, Const, FunctionType, TypeMetadata, Var};

use crate::prelude::ir::*;

use super::parser::{IrBlock, IrStmt};
use super::lexer::Loc;
use super::IrError;

/// semantic analyze for ir stmts
pub struct IrSemnatic<'a> {
    input: &'a mut Vec<IrStmt>,

    const_sigs: HashMap<String, Linkage>,
    func_sigs: HashMap<String, (FunctionType, Linkage, /*the blocks*/Vec<String>)>,
}

impl<'a> IrSemnatic<'a> {
    /// Creates an new ir semnatic analyzizer
    pub fn new(exprs: &'a mut Vec<IrStmt>) -> Self {
        Self {
            input: exprs,

            const_sigs: HashMap::new(),
            func_sigs: HashMap::new(),
        }
    }

    /// verifys the input
    pub fn verify(&mut self) -> Result<(), IrError> {
        for stmt in &self.input.clone() {
            match stmt {
                IrStmt::Func { name, ret, args, body, scope, location } => self.add_func(name, *ret, args, scope, body, location)?,
                IrStmt::Const { name, data: _, location, scope } => self.add_const(name, scope, location)?
            }
        }

        let mut stmts = self.input.to_vec();
        for stmt in &mut stmts {
            match stmt {
                IrStmt::Func { name, ret, args, body, scope, location } => self.analizye_func(name, *ret, args, body, *scope, location)?,
                IrStmt::Const { name, data, location, scope } => self.analyize_const(name, data, location, *scope)?,
            }
        }

        *self.input = stmts; 

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

        for (name, arg) in &args.0 {
            fun_args.push( (name.to_owned(), *arg) );
        }
        
        let mut ty = FunctionType {
            args: fun_args,
            ret: ret,
            any_args: false,
        };

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

    fn analizye_func(&mut self, name: &String, ret: TypeMetadata, args: &(BTreeMap<String, TypeMetadata>, bool), body: &mut Vec<(String, IrBlock)>, scope: Linkage, loc: &Loc) -> Result<(), IrError> {
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

        // analyise block dependence

        let mut branches_to = HashMap::new(); // HashMap<BlockName, Vec<Branches to these blocks>>

        for (name, block) in body.iter() {
            for node in &block.body {

                let mut branches = Vec::new();

                if let Some(br) = node.inst.as_any().downcast_ref::<Br>() {
                    branches.push(br.inner1.to_owned());
                } else if let Some(br) = node.inst.as_any().downcast_ref::<BrCond>() {
                    branches.push(br.inner2.to_owned());
                    branches.push(br.inner3.to_owned());
                } else if let Some(switch) = node.inst.as_any().downcast_ref::<Switch>() {
                    branches.push(switch.default.to_owned());   
                    for (_, case) in &switch.cases {
                        branches.push(case.to_owned());
                    }
                }

                branches_to.insert(name.to_owned(), branches);
            }
        }

        for (_, block) in body.iter() {
            for node in &block.body {
                if let Some(out) = node.inst.output() {
                    if vars.contains_key(&out.name) {
                        Err(IrError::DefinedTwice { 
                            loc: node.loc.to_owned(),
                            name: out.name.to_owned(),
                        })?
                    }

                    if node.inst.is_alloca() {
                        vars.insert(out.name, TypeMetadata::ptr);
                    } else {
                        vars.insert(out.name, out.ty);
                    }
                }
            }
        }

        for (name, block) in body {
            if blocks.contains(name) {
                Err(IrError::DefinedTwice {
                    loc: block.loc.to_owned(),
                    name: name.to_owned()
                })?
            }

            blocks.push(name.to_owned());

            for node in &mut block.body {
                let instr = &node.inst;
                let loc = node.loc.to_owned();

                let any = instr.as_any();

                if let Some(node) = any.downcast_ref::<Return>() {
                    self.analiyze_ret(&mut vars, node, ret, loc.to_owned())?;
                } else if let Some(node) = any.downcast_ref::<Assign<Var, Const>>() {
                    self.analiyze_assign_const(&mut vars, node, loc.to_owned())?;
                } else if let Some(node) = any.downcast_ref::<Call>() {
                    self.analyize_call(&mut vars, node, loc.to_owned())?;
                } else if let Some(node) = any.downcast_ref::<Br>() {
                    self.analiyze_block(func, node, loc.to_owned())?;
                } else if let Some(node) = any.downcast_ref::<Phi>() {
                    if vars.contains_key(&node.out.name) {
                    let mut handled = Vec::new();

                    for (branch, branches) in &branches_to {
                        if branches.contains(&BlockId(name.to_owned())) {
                            let mut recived = false;

                            for recive in &node.recive_from_blocks {
                                if recive.0.name == branch.to_owned() {
                                    handled.push(recive.0.name.clone());
                                    recived = true;
                                    break;
                                }
                            }

                            if !recived {
                                Err(IrError::PhiBranchNotHandled {
                                    loc: loc.to_owned(),
                                    branch: branch.to_owned(),
                                })?
                            }
                        }
                    }

                    for (recive, _) in &node.recive_from_blocks {
                        if !handled.contains(&recive.name) {
                            Err(IrError::Unkown { 
                                what: "block".into(), 
                                name: recive.name.to_owned(), 
                                loc: loc.to_owned() 
                            })?
                        }
                    }
                }} else if let Some(node) = any.downcast_ref::<Switch>() {
                    self.analyze_switch(func, &mut vars, node, loc.to_owned())?;
                } else if let Some(br) = any.downcast_ref::<Br>() {
                    self.analyze_br(func, &mut vars, br, loc.to_owned())?;
                } else if let Some(br) = any.downcast_ref::<BrCond>() {
                    self.analyze_brcond(func, &mut vars, br, loc.to_owned())?;
                }

                for input in node.inst.inputs_mut() {
                    let Some(ty) = vars.get(&input.name) else {
                        Err(IrError::Unkown { 
                            what: "variable".into(), 
                            name: input.name.to_owned(),
                            loc: loc.to_owned() 
                        })?
                    };

                    input.ty = *ty;
                }
            }
        }

        Ok(())
    }

    fn analiyze_ret(&mut self, vars: &mut HashMap<String, TypeMetadata>, node: &Return, fsig: TypeMetadata, loc: Loc) -> Result<(), IrError> {
        let ret: TypeMetadata = node.inner1.get_ty();

        if let IROperand::Var(var_to_return) = &node.inner1 {
            if let Some(var) = vars.get(&var_to_return.name) {
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
                    name: var_to_return.name.to_owned(), 
                    loc: loc.to_owned()
                })?
            }
        }

        if ret != fsig {
            Err(IrError::FuncWrongReturnTyoe {
                expected: fsig,
                found: ret,
                loc: loc,
            })?
        }

        Ok(())
    }

    fn analiyze_assign_const(&mut self, _vars: &mut HashMap<String, TypeMetadata>, node: &Assign<Var, Const>, loc: Loc) -> Result<(), IrError> {
        let name = &node.inner2.name;
        if !self.const_sigs.contains_key(name) {
            Err(IrError::Unkown {
                what: "const".to_owned(),
                loc: loc.to_owned(),
                name: name.to_owned(),
            })?
        }

        Ok(())
    }

    fn analyize_call(&mut self, vars: &mut HashMap<String, TypeMetadata>, node: &Call, loc: Loc) -> Result<(), IrError> {
        let name = &node.func.name;
        let mut sig = node.func.ty.to_owned();

        if let Some((ty, _, _)) = self.func_sigs.get(&node.func.name) {
            sig = ty.to_owned();
        } else if let Some(intrinsic) = &node.instric {
            sig = intrinsic.ty.to_owned();
        } else {
            Err(IrError::Unkown {
                what: "function".to_owned(),
                loc: loc.to_owned(),
                name: name.to_owned(),
            })?
        }

        if sig.ret != node.func.ty.ret {
            Err(IrError::FuncWrongReturnTyoe { 
                expected: sig.ret, 
                found: node.func.ty.ret, 
                loc: loc.to_owned() 
            })?
        }

        let mut index = 0;

        for arg in &node.args {
            let arg = if let IROperand::Var(arg) = arg {
                    if let Some(var) = vars.get(&arg.name) {
                    var
                } else {
                    Err(IrError::Unkown { 
                        what: "variable".to_owned(), 
                        name: arg.name.to_owned(), 
                        loc: loc.to_owned(), 
                    })?
                }
            } else { &arg.get_ty() };

            if let Some((_, expected)) = sig.args.get(index) {
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

    fn analiyze_block(&mut self, func: &String, node: &Br, loc: Loc) -> Result<(), IrError> {
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
    fn analyze_br(&mut self, func: &String, _: &mut HashMap<String, TypeMetadata>, node: &Br, loc: Loc) -> Result<(), IrError> {
        let (_, _, blocks) = self.func_sigs.get(func).unwrap();

        if !blocks.contains(&node.inner1.name) {
            Err(IrError::Unkown { 
                what: "block".to_owned(), 
                name: node.inner1.name.to_owned(), 
                loc: loc.to_owned()
            })?
        }

        Ok(())
    }
    fn analyze_brcond(&mut self, func: &String, vars: &mut HashMap<String, TypeMetadata>, node: &BrCond, loc: Loc) -> Result<(), IrError> {
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
}   