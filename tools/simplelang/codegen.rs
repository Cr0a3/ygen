use std::collections::{HashMap, VecDeque};

use Ygen::prelude::*;

use crate::ast::*;

#[derive(Debug, Clone)]
pub struct CodeGenerator {
    input: VecDeque<Statement>,

    module: Module,
}

impl CodeGenerator {
    pub fn new(stmts: Vec<Statement>) -> Self {
        Self {
            input: stmts.into(),
            module: Module(),
        }
    }
    
    pub fn gen(&mut self) {
        while let Some(stmt) = self.input.pop_front() {
            if let Statement::Fn(func) = stmt {
                self.gen_func(&func);
            } else { unreachable!() }
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

        let fun = self.module.add(&func.name, &func_ty);

        if func.extrn {
            fun.extrn();
        }

        if func.import {
            fun.import();
            return;
        }

        let mut block = Block("entry", &fun);
        builder.positionAtStart(&mut block);

        for stmt in &func.body {
            self.gen_stmt(stmt, &mut builder, &mut vars);
        }
    }

    fn gen_stmt(&mut self, stmt: &Statement, builder: &mut IRBuilder, vars: &mut HashMap<String, Var>) {
        match stmt {
            Statement::Expr(expr) => { self.gen_expr(expr, builder, vars); },
            Statement::Ret(ret) => { self.gen_ret(ret, builder, vars); },
            _ => unreachable!()
        }
    }

    fn gen_expr(&mut self, expr: &Expr, builder: &mut IRBuilder, vars: &mut HashMap<String, Var>) -> Var {
        todo!()
    }

    fn gen_ret(&mut self, ret: &RetStmt, builder: &mut IRBuilder, vars: &mut HashMap<String, Var>) {
        
        let ret = if let Some(ret) = &ret.var { ret} else {
            builder.BuildRet(Type::Void);
            return;
        };

        let out = self.gen_expr(ret, builder, vars);

        builder.BuildRet(out);

    }

    pub fn module(&mut self) -> &mut Module {
        &mut self.module
    }
}