use std::collections::HashMap;

use crate::prelude::Ir;
use crate::Target::{Arch, CallConv};
use crate::IR::{Function, TypeMetadata, Var};

use super::{calling_convention::MachineCallingConvention, reg::Reg, MCInstr, MachineInstr};

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
mod getelemptr;

/// handeles how constant imms are handeled (wether creating a const or just an instr op)
/// 
/// Default: `ConstImmRules::InstrOp`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstImmRules {
    /// uses the imm as the instr operand
    InstrOp,
    /// creates the value as an const and loads the adress
    CreateConst,
}


/// The allocation functions for the `CompilationHelper`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Allocator {
    pub(crate) alloc: Option<fn(compiler: &mut Allocator, func: &Function)>,
    pub(crate) alloc_rv: Option<fn(compiler: &mut Allocator, TypeMetadata) -> VarLocation>,
    pub(crate) alloc_stack: Option<fn(compiler: &mut Allocator, TypeMetadata) -> VarLocation>,
    pub(crate) free: Option<fn(compiler: &mut Allocator, loc: VarLocation)>,
    pub(crate) after_alloc: Option<fn(&CompilationHelper)>,

    pub(crate) vars: HashMap<String, VarLocation>,
    pub(crate) var_types: HashMap<String, TypeMetadata>,
    pub(crate) allocated_vars: Vec<String>,
    pub(crate) epilog: bool,
    pub(crate) scopes: HashMap<String, Vec<(Var, VarLocation)>>,
    pub(crate) phi_vars: HashMap<String, VarLocation>,

    pub(crate) stack_off: i64,
    pub(crate) fregs: Vec<Reg>,
    pub(crate) ffpregs: Vec<Reg>,

    pub(crate) call: MachineCallingConvention,
}
/// helps with compilation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompilationHelper {
    pub(crate) arch: Arch,
    pub(crate) lower: Option<fn(CallConv, Vec<MachineInstr>) -> Vec<Box<dyn MCInstr>>>,
    
    pub(crate) alloc: Allocator,

    pub(crate) call: MachineCallingConvention,

    pub(crate) vars: HashMap<String, VarLocation>,
    pub(crate) var_types: HashMap<String, TypeMetadata>,
    pub(crate) allocated_vars: Vec<String>,
    pub(crate) scopes: HashMap<String, Vec<(Var, VarLocation)>>,
    pub(crate) phi_vars: HashMap<String, VarLocation>,
    pub(crate) epilog: bool,

    pub(crate) tmp_reg: Reg,

    pub(crate) fp_imm: ConstImmRules, 
}

impl CompilationHelper {
    pub(crate) fn new(arch: Arch, call: MachineCallingConvention, alloc: Allocator, tmp: Reg) -> Self {
        Self {
            arch: arch,
            allocated_vars: Vec::new(),
            vars: HashMap::new(),
            var_types: HashMap::new(),
            call: call,
            lower: None,
            tmp_reg: tmp,
            fp_imm: ConstImmRules::InstrOp,
            alloc: alloc,
            epilog: false,
            scopes: HashMap::new(),
            phi_vars: HashMap::new(),
        }
    }

    /// runs the register allocator
    pub fn run_alloc(&mut self, func: &Function) {
        if let Some(alloc) = self.alloc.alloc {
            alloc(&mut self.alloc, func);
        } else { panic!("no registered allocator for {:?}", self.arch) }
        if let Some(func) = self.alloc.after_alloc {
            func(self);
        }

        self.vars           = self.alloc.vars.to_owned();
        self.phi_vars       = self.alloc.phi_vars.to_owned();
        self.scopes         = self.alloc.scopes.to_owned();
        self.allocated_vars = self.alloc.allocated_vars.to_owned();
        self.var_types      = self.alloc.var_types.to_owned();
        self.epilog         = self.alloc.epilog.to_owned();
    }

    pub(crate) fn alloc_stack(&mut self, ty: TypeMetadata) -> VarLocation {
        if let Some(alloc_stack) = self.alloc.alloc_stack {
            alloc_stack(&mut self.alloc, ty)
        } else { panic!("no registered stack allocation function for {:?}", self.arch) }
    }

    pub(crate) fn alloc_rv(&mut self, ty: TypeMetadata) -> VarLocation {
        if let Some(alloc) = self.alloc.alloc_rv {
            alloc(&mut self.alloc, ty)
        }  else { panic!("no registered allocation function for {:?}", self.arch) }
    }

    pub(crate) fn free(&mut self, loc: VarLocation) {
        if let Some(free) = self.alloc.free {
            free(&mut self.alloc, loc);
        } 
    }

    
    #[inline]
    fn scoped_vars_before_node(&self, node: Box<dyn Ir>) -> Vec< (Var, VarLocation) > {
        let got = self.scopes.get(&node.dump()).expect("expected valid node");

        got.to_owned()
    }

    fn get_vars_to_save_for_call(&self, node: &crate::prelude::Call<crate::prelude::FuncId, Vec<crate::prelude::Var>, crate::prelude::Var>) -> Vec<(String, VarLocation)> {
        let vars = self.scoped_vars_before_node(Box::new( node.clone() ));
    
        let mut with_name = vec![];
        for (var, location) in vars {
            with_name.push((var.name, location));
        }

        with_name
    }

    #[inline]
    pub(crate) fn epilog(&self) -> bool {
        self.epilog
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum VarLocation {
    Reg(Reg),
    Mem(i64, TypeMetadata),
}