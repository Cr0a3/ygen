use std::collections::{HashMap, VecDeque};

use Ygen::prelude::*;

use crate::ast::*;

#[derive(Debug, Clone)]
pub struct CodeGenerator {
    input: VecDeque<Statement>,

    module: Module,

    functions: HashMap<String, Function>,

    const_index: usize,
}

impl CodeGenerator {
    pub fn new(stmts: Vec<Statement>) -> Self {
        Self {
            input: stmts.into(),
            module: Module(),
            functions: HashMap::new(),
            const_index: 0,
        }
    }
    
    pub fn gen(&mut self) {
        while let Some(stmt) = self.input.pop_front() {
            if let Statement::Fn(func) = stmt {
                self.gen_func(&func);
            } else { unreachable!() }
        }

        for (_, fun) in &self.functions {
            self.module.add_raw(fun.clone())
        }
    }

    fn gen_func(&mut self, func: &FnStmt) {
        let mut vars = HashMap::new();

        let mut args = vec![];

        let mut index = 0;

        let ret = TypeMetadata::i32;//todo!();

        let mut func_ty = FnTy(vec![], ret);

        for arg in &func.args {
            let (name, ty) = if let Expr::Var(var) = arg { var } else { unreachable!() };  

            let ty = ty.unwrap();
            let name = name.to_string();

            args.push(ty);

            func_ty = FnTy(args.clone(), ret);

            vars.insert(name, func_ty.arg(index));

            index += 1;
        }
        
        let mut builder = IRBuilder();

        let mut fun = Func(func.name.to_string(), func_ty);

        if func.extrn {
            fun.extrn();
        }

        if func.import {
            fun.import();
            self.functions.insert(func.name.to_string(), fun.clone());
            return;
        }

        let block = fun.addBlock("entry");
        builder.positionAtStart(block);

        for stmt in &func.body {
            self.gen_stmt(stmt, &mut builder, &mut vars);
        }

        self.functions.insert(func.name.to_string(), fun);
    }

    fn gen_stmt(&mut self, stmt: &Statement, builder: &mut IRBuilder, vars: &mut HashMap<String, Var>) {
        match stmt {
            Statement::Expr(expr) => { self.gen_expr(expr, builder, vars); },
            Statement::Ret(ret) => { self.gen_ret(ret, builder, vars); },
            _ => unreachable!()
        }
    }

    fn gen_expr(&mut self, expr: &Expr, builder: &mut IRBuilder, vars: &mut HashMap<String, Var>) -> Var {
        match expr {
            Expr::Var((name, _)) => vars.get(name).unwrap().clone(),
            Expr::Binary(bin) => self.gen_bin(bin, builder, vars),
            Expr::LiteralInt(int) => builder.BuildAssign(Type::i64(*int)),
            Expr::Call(call) => self.gen_call(call, builder, vars),
            Expr::LiteralString(str) => self.gen_string(builder, str),
        }
    }

    fn gen_ret(&mut self, ret: &RetStmt, builder: &mut IRBuilder, vars: &mut HashMap<String, Var>) {
        let ret = if let Some(ret) = &ret.var { 
            ret
        } else {
            builder.BuildRet(Type::Void);
            return;
        };

        let out = self.gen_expr(ret, builder, vars);

        builder.BuildRet(out);

    }

    fn gen_bin(&mut self, bin: &(Operator, Option<Box<Expr>>, Option<Box<Expr>>), builder: &mut IRBuilder, vars: &mut HashMap<String, Var>) -> Var {
        let left = bin.1.as_ref().unwrap();
        let right = bin.2.as_ref().unwrap();

        if bin.0 == Operator::Assign {
            let right = self.gen_expr(&right, builder, vars);
            let out = builder.BuildAssign(right);
            vars.insert(match *bin.1.as_ref().unwrap().clone() {
                Expr::Var((name, _)) => name.to_string(),
                _ => unreachable!(),
            }, out.clone());
            return out;
        }

        let left = self.gen_expr(&left, builder, vars);
        let right = self.gen_expr(&right, builder, vars);

        match bin.0 {
            Operator::Sub => builder.BuildSub(left, right),
            Operator::Add => builder.BuildAdd(left, right),
            Operator::Mul => todo!("add mul support to ygen"),
            Operator::Div => todo!("add div support to ygen"),
            _ => todo!(),
        }
    }

    fn gen_call(&mut self, call: &CallStmt, builder: &mut IRBuilder, vars: &mut HashMap<String, Var>) -> Var {
        let fun = &self.functions.get(&call.name).unwrap().clone();

        let mut args = vec![];

        for arg in &call.args {
            let var = self.gen_expr(arg, builder, vars);
            args.push(var);
        }

        builder.BuildCall(&fun, args)
    }

    fn gen_string(&mut self, builder: &mut IRBuilder, string: &String) -> Var {
        let constant = self.module.addConst(&format!(".const{}", self.const_index));

        let mut string = string.clone();
        string.push('\0');

        constant.set(string.as_bytes().to_vec());

        builder.BuildAssign(&constant.clone())
    }

    pub fn module(&mut self) -> &mut Module {
        &mut self.module
    }
}