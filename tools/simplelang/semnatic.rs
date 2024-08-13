use std::collections::VecDeque;

use crate::ast::*;
//use crate::macros::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Semnatic {
    stmts: VecDeque<Statement>,

    funcs: Vec<FnStmt>,
    vars: Vec<Expr>,

    error: bool,
}

impl Semnatic {
    pub fn new(stmts: Vec<Statement>) -> Self {
        Self {
            stmts: stmts.into(),

            funcs: vec![],
            vars: vec![],
            error: false,
        }
    }

    pub fn analyze(&mut self) {
        while let Some(stmt) = self.stmts.pop_front() {
            self.analyze_stmt(&stmt);
        }
    }

    fn analyze_stmt(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Fn(func) => self.analyze_func(func),
            Statement::Expr(expr) => self.analyze_expr(expr),
            Statement::Ret(ret) => self.analyze_ret(ret),
        }
    }

    fn analyze_func(&mut self, func: &FnStmt) {
        todo!("anaylizing: {:?}", func)
    }

    fn analyze_expr(&mut self, expr: &Expr) {
        todo!("anaylizing: {:?}", expr)
    }

    fn analyze_ret(&mut self, ret: &RetStmt) {
        todo!("anaylizing: {:?}", ret)
    }

    pub fn had_errors(&self) -> bool {
        self.error
    }
}