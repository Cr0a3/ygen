use std::collections::HashMap;

use crate::{Target::{Arch, CallConv}, IR::{Function, Var}};

use super::{calling_convention::MachineCallingConvention, reg::Reg, reg_vec::RegVec, MCInstr, MachineInstr};

mod math;
mod cast;
mod call;
mod ret;
mod assign;
mod br;

/// helps with compilation
pub struct CompilationHelper {
    pub(crate) regs: RegVec,
    pub(crate) arch: Arch,
    pub(crate) lower: Option<fn(CallConv, Vec<MachineInstr>) -> Vec<Box<dyn MCInstr>>>,

    pub(crate) call: MachineCallingConvention,

    pub(crate) vars: HashMap<String, VarLocation>,
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
        if let Some(location) = self.vars.get(&var.name) {
            match location {
                VarLocation::Reg(reg) => self.regs.push(reg.arch(), reg.clone()),
            }
        }
    }

    /// allocates resources for a new variable
    pub(crate) fn alloc(&mut self, var: &Var) -> VarLocation {
        let location = if let Some(reg) = self.regs.pop(self.arch) {
            VarLocation::Reg(match reg {
                Reg::x64(x64) => Reg::x64(x64.sub_ty(var.ty)),
            })
        } else {
            todo!("Registers ran out. And memory variables are currently not implemented")
        };

        self.vars.insert(var.name.to_owned(), location);

        location
    }

    /// passes the arguments into the right register
    pub(crate) fn build_argument_preprocessing(&mut self, func: &Function) {
        let func = &func.ty;

        let mut num = 0;

        for ty in &func.args {
            let location = {
                if let Some(reg) = self.call.args(self.arch).get(num) {
                    VarLocation::Reg(match reg {
                        Reg::x64(x64) => Reg::x64(x64.sub_ty(*ty)),
                    })
                } else {
                    todo!("The new system currently doesn't support memory")
                }
            };

            self.vars.insert(
                func.arg(num).name, 
                location
            );

            num += 1;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum VarLocation {
    Reg(Reg),
    //Mem(Mem),
}