use std::collections::HashMap;

use crate::{ydbg, IR::Function};

use super::{dag::{self, DagOpTarget}, reg::Reg};

/// Performes register allocation using iterated register coalescing
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItRegCoalAlloc<'a> {
    /// The free registers which are possibiy usable
    pub regs: Vec<Reg>,
    /// The current function
    pub curr_func: Option<&'a Function>,
    /// The target specific argument processor
    pub arg_processor: Option<fn(&mut ItRegCoalAlloc)>,
    /// The allocated vars
    pub vars: HashMap<String, DagOpTarget>,
    /// The current stack off
    pub stack: i32,
}

impl<'a> ItRegCoalAlloc<'a> {
    /// Creates a new iterated register coalescing register allocator
    pub fn new(regs: Vec<Reg>, arg_proc: fn(&mut ItRegCoalAlloc)) -> Self {
        Self {
            regs: regs,
            curr_func: None,
            arg_processor: Some(arg_proc),
            vars: HashMap::new(),
            stack: 0,
        }
    }

    /// Runs the ircy allocator on the given input function
    pub fn init(&mut self, func: &'a Function) {
        ydbg!("[INIT] Iterated Register Coalescing Allocator for {}", func.name);

        self.curr_func = Some(func);

        let Some(arg_proc) = self.arg_processor else {
            panic!("no current arg processor");
        };

        arg_proc(self);
    }

    /// Applys the var locations to the dag
    pub fn apply(&self, dag: &mut dag::DagNode) {
        if let Some(out) = &mut dag.out {
            if let DagOpTarget::UnallocatedVar(var) = &out.target {
                if let Some(target) = self.vars.get(&var.name) {
                    out.allocated = true;
                    out.target = target.to_owned();
                }
            }
        }

        for operand in &mut dag.ops {
            if let DagOpTarget::UnallocatedVar(var) = &operand.target {
                if let Some(target) = self.vars.get(&var.name) {
                    operand.allocated = true;
                    operand.target = target.to_owned();
                }
            }
        }
    }
}

/// A base for performing register allocation using iterated register coalescing
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItRegCoalAllocBase {
    /// The free registers which are possibiy usable
    pub regs: Vec<Reg>,
    /// The target specific argument processor
    pub arg_processor: Option<fn(&mut ItRegCoalAlloc)>
}

impl ItRegCoalAllocBase {
    /// Creates a new irc reg allocator with the given function
    /// and runs register allocation for the arguments
    pub fn fork<'a>(&self, func: &'a Function) -> ItRegCoalAlloc<'a> {
        let mut alloc = ItRegCoalAlloc::new(self.regs.to_owned(), self.arg_processor.to_owned().unwrap());
    
        alloc.init(func);

        alloc
    }
}