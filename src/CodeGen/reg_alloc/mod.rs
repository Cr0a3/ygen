use std::collections::HashMap;

use crate::{prelude::{Ir, Phi}, Target::{Arch, CallConv}, IR::{Function, TypeMetadata, Var}};

use super::{MachineCallingConvention, VMem, VReg};

/// Ygens register allocator
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegAlloc {
    pub arch: Arch,
    pub call: MachineCallingConvention,

    /// The vreg index
    pub index: i32,

    pub stack_off: i32,

    pub(crate) vars: HashMap<String, VReg>,
    pub(crate) var_types: HashMap<String, TypeMetadata>,
    pub(crate) phi_vars: HashMap<String, VReg>,

    pub(crate) scopes: HashMap<String, Vec<(Var, VReg)>>,

    pub(crate) processed_args: bool,
}

impl RegAlloc {
    /// Creates an new register allocator
    pub fn new(arch: Arch, call: CallConv) -> Self {
        let call = MachineCallingConvention { 
            call_conv: call 
        };

        Self {
            arch: arch,
            call: call,

            vars: HashMap::new(),
            var_types: HashMap::new(),
            phi_vars: HashMap::new(),

            scopes: HashMap::new(),

            processed_args: false,

            index: 0,

            stack_off: call.align(arch) as i32,
        }
    }

    fn arg_prep(&mut self, func: &Function) {
        let func = &func.ty;

        let mut num = 0;

        for ty in &func.args {
            let location = VReg(self.index);
            
            self.index += 1;

            let name = || func.arg(num).name;

            self.vars.insert(
                name(), 
                location
            );

            self.var_types.insert(name(), *ty);

            num += 1;
        }

        self.processed_args = true;
    }

    /// runs all variable allocations for the function
    pub fn run_alloc(&mut self, func: &Function) {
        if !self.processed_args {
            self.arg_prep(func);
        }

        // run phis
        for block in &func.blocks {
            for node in &block.nodes {
                if let Some(phi) = node.as_any().downcast_ref::<Phi>() {
                    self.phi_prep( phi );
                }
            }
        }

        for block in &func.blocks {
            for node in &block.nodes {
                self.node_prep(node);
            }
        }
    }

    fn node_prep(&mut self, node: &Box<dyn Ir>) {
        /*let out = node.output();

        if let Some(out) = out {
            let location = self.alloc_rv(out.ty);
            self.vars.insert(out.name.to_owned(), location);
        }*/

        let mut scopes = Vec::new();

        for (name, location) in &self.vars {
            scopes.push( (Var {
                name: name.to_owned(),
                ty: *self.var_types.get(name).unwrap(),
            }, *location) );
        }

        self.scopes.insert(node.dump(), scopes);
    }

    pub(crate) fn alloc_rv(&mut self, _: TypeMetadata) -> VReg {
        let reg = VReg(self.index);
        self.index += 1;

        reg
    }

    pub(crate) fn alloc_stack(&mut self, ty: TypeMetadata) -> VMem {
        let mem = VMem(self.stack_off);
        self.stack_off += ty.byteSize() as i32;

        mem
    }


    #[inline]
    pub(crate) fn scoped_vars_before_node(&self, node: Box<dyn Ir>) -> Vec< (Var, VReg) > {
        let got = self.scopes.get(&node.dump()).expect("expected valid node");

        got.to_owned()
    }

    pub(crate) fn phi_prep(&mut self, phi: &Phi) {
        let out = self.alloc_rv(phi.typ);

        for (_, var) in &phi.recive_from_blocks {
            self.phi_vars.insert(var.name.to_owned(), out);
        }

        self.vars.insert(phi.out.name.to_owned(), out);
        self.var_types.insert(phi.out.name.to_owned(), phi.typ);
    }
}