use std::collections::{HashMap, VecDeque};

use ygen::IR::TypeMetadata;

use crate::{ast::*, err, warn};
//use crate::macros::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Semnatic {
    stmts: VecDeque<Statement>,

    funcs: HashMap<String, (/*args*/Vec<Expr>, /*body*/Vec<Expr>, /*dynamic amount of args*/bool)>,

    in_binary: bool,

    error: bool,
}

impl Semnatic {
    pub fn new(stmts: &Vec<Statement>) -> Self {
        Self {
            stmts: VecDeque::from(stmts.clone()),

            funcs: HashMap::new(),
            error: false,

            in_binary: false,
        }
    }

    pub fn analyze(&mut self) {
        for stmt in self.stmts.clone() {
            if let Statement::Fn(func) = stmt {
                self.add_func(&func);
            } else {
                err!(self.error, "expected function statement found {:?}", stmt);
            }
        }

        while let Some(stmt) = self.stmts.pop_front() {
            if let Statement::Fn(func) = stmt {
                self.analyze_func(&func);
            } else {
                err!(self.error, "expected function statement found {:?}", stmt);
            }
        }
    }

    fn analyze_stmt(&mut self, stmt: &Statement, vars: &mut HashMap<String, Option<TypeMetadata>>) {
        match stmt {
            Statement::Expr(expr) => self.analyze_expr(expr, vars),
            Statement::Ret(ret) => self.analyze_ret(ret, vars),
            _ => {
                err!(self.error, "inner functions aren't allowed ");
            }
        }
    }

    fn analyze_func(&mut self, func: &FnStmt) {
        let mut vars = HashMap::new();

        for arg in &func.args {
            match arg {
                Expr::Var(var) => {
                    vars.insert(var.0.to_string(), var.1);
                },
                _ => {
                    err!(self.error, "expected variables as function args not terms/calls");
                    return;
                }
            }
        }

        let mut returned = func.import;

        for stmt in &func.body {
            if returned {
                warn!("unreachable code after return statemant");
            };

            if let Statement::Ret(_) = stmt {
                returned = true;
            }

            self.analyze_stmt(stmt, &mut vars);
        }

        if !(returned) && func.ret != TypeMetadata::Void {
            err!(self.error, "function {:?} needs to return {:?} but found nothing", func.name, func.ret);
        }
        
    }

    fn add_func(&mut self, func: &FnStmt) {
        if self.funcs.contains_key(&func.name) {
            err!(self.error, "func {} defined twice", func.name);
            return;
        }

        let mut args = vec![];

        if func.import && func.body.len() > 0 {
            err!(self.error, "imported functions aren't allowed to have a body but {} has", func.name);
            return;
        }

        if func.dynamic_args && !func.import {
            err!(self.error, "only imported functions are allowed to have a variable amount of arguments. But func {} has", func.name);
            return;
        }

        for arg in &func.args {
            match arg {
                Expr::Var(var) => {    
                    args.push(Expr::Var(var.clone()));
                },
                _ => {
                    err!(self.error, "expected variables as function args not terms/calls");
                    return;
                }
            }
        }

        if func.import {
            if func.body.len() > 0 {
                err!(self.error, "imported functions can't have an body");
                return;
            }

            self.funcs.insert(func.name.to_string(), (args, vec![], func.dynamic_args));
            return;
        }

        self.funcs.insert(func.name.to_string(), (args, vec![], false));
    }

    fn analyze_expr(&mut self, expr: &Expr, vars: &mut HashMap<String, Option<TypeMetadata>>) {
        match expr {
            Expr::Var((var, _)) => {
                if !vars.contains_key(var) {
                    err!(self.error, "unknown variable: {}", var);
                }
            },
            Expr::LiteralInt(_) => {},
            Expr::Binary(bin) => self.analyze_bin(bin, vars),
            Expr::Call(call) => self.analyze_call(call, vars),
            Expr::LiteralString(str) => {
                if self.in_binary {
                    err!(self.error, "unexpected string: \"{}\"", str);
                }
            }
        }
    }

    fn analyze_ret(&mut self, ret: &RetStmt, vars: &mut HashMap<String, Option<TypeMetadata>>) {
        if let Some(expr) = &ret.var {
            self.analyze_expr(&expr, vars);
        } else {
            //err!(self.error, "expected return value");
        }
    }

    fn analyze_bin(&mut self, bin: &(Operator, Option<Box<Expr>>, Option<Box<Expr>>), vars: &mut HashMap<String, Option<TypeMetadata>>) {
        self.in_binary = true;
        
        let left = if let Some(left) = &bin.1 {
            left.clone()
        } else {
            err!(self.error, "expected lhs found nothing");
            return;
        };

        let right = if let Some(right) = &bin.2 {
            right.clone()
        } else {
            err!(self.error, "expected rhs found nothing");
            return;
        };

        if bin.0 == Operator::Assign {
            self.in_binary = false;
            match *left.clone() {
                Expr::Var((name, ty)) => {
                    if !vars.contains_key(&name) {
                        if ty.is_none() {
                            err!(self.error, "you can't declare a variable without a type");
                            return;
                        }
                    }
                    vars.insert(name, ty)
                },
                _ => unreachable!(),
            };

            self.analyze_expr(&right, vars);
            return;
        }

        self.analyze_expr(&left, vars);
        self.analyze_expr(&right, vars);

        self.in_binary = false;
    }

    fn analyze_call(&mut self, call: &CallStmt, vars: &mut HashMap<String, Option<TypeMetadata>>) {
        if !self.funcs.contains_key(&call.name) {
            err!(self.error, "unknown function {:?}", call.name);
            return;
        }

        let (args, _, dynamic) = self.funcs.get(&call.name).unwrap();

        let args = args.len();
        let given = call.args.len();

        if args != given && !dynamic {
            err!(self.error, "expected {} argument(s) found {}", args, given);
            return;
        }

        if args > given {
            err!(self.error, "too few arguments were supplyed (expected {} found {})", args, given);
            return;
        }

        for arg in &call.args {
            self.analyze_expr(arg, vars);
        }
    }

    pub fn had_errors(&self) -> bool {
        self.error
    }
}