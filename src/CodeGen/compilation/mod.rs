use std::collections::HashMap;

use crate::Target::{Arch, CallConv};
use crate::IR::{Function, TypeMetadata};

use super::{calling_convention::MachineCallingConvention, reg::Reg, reg_alloc::RegAlloc, MCInstr, MachineInstr};

mod call;
mod ret;
mod br;

mod assign;
mod cmp;
mod math;
mod cast;

mod prolog;

mod alloca;
mod store;
mod load;

mod switch;

mod neg;

mod select;

/// helps with compilation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompilationHelper {
    pub(crate) arch: Arch,
    pub(crate) lower: Option<fn(CallConv, Vec<MachineInstr>) -> Vec<Box<dyn MCInstr>>>,

    pub(crate) call: MachineCallingConvention,

    pub(crate) vars: HashMap<String, VarLocation>,
    pub(crate) var_types: HashMap<String, TypeMetadata>,
    pub(crate) allocated_vars: Vec<String>,

    pub(crate) tmp_reg: Reg,

    pub(crate) alloc: RegAlloc,
}

impl CompilationHelper {
    pub(crate) fn new(arch: Arch, call: MachineCallingConvention, alloc: RegAlloc, tmp: Reg) -> Self {
        Self {
            arch: arch,
            allocated_vars: Vec::new(),
            vars: HashMap::new(),
            var_types: HashMap::new(),
            call: call,
            lower: None,
            alloc: alloc,
            tmp_reg: tmp,
        }
    }

    /// runs the register allocator
    pub fn run_alloc(&mut self, func: &Function) {
        self.alloc.run_alloc(func);
        self.vars = self.alloc.vars.to_owned();
        self.var_types = self.alloc.var_types.to_owned();
    }

    fn get_vars_to_save_for_call(&self, node: &crate::prelude::Call<crate::prelude::FuncId, Vec<crate::prelude::Var>, crate::prelude::Var>) -> Vec<(String, VarLocation)> {
        let vars = self.alloc.scoped_vars_before_node(Box::new( node.clone() ));
    
        let mut with_name = vec![];
        for (var, location) in vars {
            with_name.push((var.name, location));
        }

        with_name
    }

    #[inline]
    pub(crate) fn epilog(&self) -> bool {
        if self.alloc.stack_off != self.call.shadow(self.arch) {
            true
        } else { false }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum VarLocation {
    Reg(Reg),
    Mem(i64),
}