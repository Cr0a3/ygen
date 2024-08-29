use std::collections::HashMap;

use crate::{Target::Arch, IR::{Block, Var}};

use super::{reg::Reg, reg_vec::RegVec};

mod math;

pub(crate) struct CompilationHelper {
    pub(crate) regs: RegVec,
    pub(crate) arch: Arch,

    pub(crate) vars: HashMap<Var, VarLocation>,
}

impl CompilationHelper {
    pub(crate) fn new(arch: Arch) -> Self {
        Self {
            regs: RegVec::new(),
            arch: arch,
            vars: HashMap::new(),
        }
    }

    /// frees the resources of the variable
    pub(crate) fn free(&mut self, var: &Var) {
        if let Some(location) = self.vars.get(var) {
            match location {
                VarLocation::Reg(reg) => self.regs.push(reg.arch(), reg.clone()),
                _ => todo!()
            }
        }
    }

    /// allocates resources for a new variable
    pub(crate) fn alloc(&mut self, var: &Var) -> VarLocation {
        let location = if let Some(reg) = self.regs.pop(self.arch) {
            VarLocation::Reg(reg)
        } else {
            todo!()
        };

        self.vars.insert(var.clone(), location);

        location
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum VarLocation {
    Reg(Reg),
    //Mem(Mem),
}