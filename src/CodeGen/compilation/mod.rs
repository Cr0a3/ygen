use std::collections::HashMap;

use crate::{Target::Arch, IR::{Function, Var}};

use super::{calling_convention::MachineCallingConvention, reg::Reg, reg_vec::RegVec, MCInstr, MachineInstr};

mod math;
mod cast;
mod call;
mod ret;
mod assign;

/// helps with compilation
pub struct CompilationHelper {
    pub(crate) regs: RegVec,
    pub(crate) arch: Arch,
    pub(crate) lower: Option<fn(Vec<MachineInstr>) -> Vec<Box<dyn MCInstr>>>,

    pub(crate) call: MachineCallingConvention,

    pub(crate) vars: HashMap<Var, VarLocation>,
}

impl CompilationHelper {
    pub(crate) fn new(arch: Arch, call: MachineCallingConvention) -> Self {
        Self {
            regs: RegVec::new(),
            arch: arch,
            vars: HashMap::new(),
            call: call,
            lower: None,
        }
    }

    /// frees the resources of the variable
    pub(crate) fn free(&mut self, var: &Var) {
        if let Some(location) = self.vars.get(var) {
            match location {
                VarLocation::Reg(reg) => self.regs.push(reg.arch(), reg.clone()),
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

    /// passes the arguments into the right register
    pub(crate) fn build_argument_preprocessing(&mut self, func: &Function) {
        let func = &func.ty;

        for (num, ty) in &func.args {

            let location = {
                if let Some(reg) = self.call.args(self.arch).get(*num) {
                    VarLocation::Reg(*reg)
                } else {
                    todo!("The new system currently doesn't support memory")
                }
            };

            self.vars.insert(
                Var { name: format!("%{}", num), ty: *ty }, 
                location
            );
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum VarLocation {
    Reg(Reg),
    //Mem(Mem),
}