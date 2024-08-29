use std::collections::HashMap;

use crate::{Target::Arch, IR::{Block, Var}};

use super::{reg::Reg, reg_vec::RegVec};

mod math;

pub(crate) struct CompilationHelper {
    pub(crate) regs: RegVec,
    pub(crate) arch: Arch,

    pub(crate) vars: HashMap<Var, VarLocation>,

    pub(crate) block: Block,
}

impl CompilationHelper {
    /// frees the resources of the variable
    pub fn free(&mut self, var: &Var) {
        if let Some(location) = self.vars.get(var) {
            match location {
                VarLocation::Reg(reg) => self.regs.push(reg.arch(), reg.clone()),
                _ => todo!()
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum VarLocation {
    Reg(Reg),
    //Mem(Mem),
}