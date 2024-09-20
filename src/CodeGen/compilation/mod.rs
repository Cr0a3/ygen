use std::collections::HashMap;

use crate::{Target::{Arch, CallConv}, IR::{Function, TypeMetadata, Var}};

use super::{calling_convention::MachineCallingConvention, reg::Reg, reg_vec::RegVec, MCInstr, MachineInstr};

mod math;
mod cast;
mod call;
mod ret;
mod assign;
mod br;
mod cmp;
mod prolog;
mod alloca;
mod store;

/// helps with compilation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompilationHelper {
    pub(crate) regs: RegVec,
    pub(crate) arch: Arch,
    pub(crate) lower: Option<fn(CallConv, Vec<MachineInstr>) -> Vec<Box<dyn MCInstr>>>,

    pub(crate) call: MachineCallingConvention,

    pub(crate) vars: HashMap<String, VarLocation>,

    pub(crate) stack_off: i64,
}

impl CompilationHelper {
    pub(crate) fn new(arch: Arch, call: MachineCallingConvention) -> Self {
        Self {
            regs: RegVec::new(),
            arch: arch,
            vars: HashMap::new(),
            call: call,
            lower: None,
            stack_off: call.shadow(arch),
        }
    }

    /// frees the resources of the variable
    pub(crate) fn free(&mut self, var: &Var) {
        if let Some(location) = self.vars.get(&var.name) {
            match location {
                VarLocation::Reg(reg) => self.regs.push(reg.arch(), reg.clone()),
                _ => {}, // stack offsets cannot be subtracted
            }
        }
    }

    /// allocates resources for a var but on the stack
    pub(crate) fn alloc_stack(&mut self, var: &Var) -> VarLocation {
        let loc = if var.ty.byteSize() as i64 <= self.call.align(self.arch) {
            VarLocation::Mem(self.stack_off)
        } else {
            todo!()
        };

        self.vars.insert(var.name.to_owned(), loc);

        self.stack_off += self.call.align(self.arch);

        loc
    }

    /// allocates a variable on the stack
    /// 
    /// The difference to `alloc_stack` is that here you can use a custom stack space size
    /// 
    /// **NOTE:** it does not register the var
    pub(crate) fn alloc_custom_stack(&mut self, ty: &TypeMetadata) -> (VarLocation, i64) {
        let loc = if ty.byteSize() as i64 <= self.call.align(self.arch) {
            VarLocation::Mem(self.stack_off)
        } else {
            todo!()
        };

        let ret = (loc, self.stack_off);

        self.stack_off += self.call.align(self.arch);

        ret
    }

    /// allocates resources for a new variable
    pub(crate) fn alloc(&mut self, var: &Var) -> VarLocation {
        let location = if let Some(reg) = self.regs.pop(self.arch) {
            VarLocation::Reg(match reg {
                Reg::x64(x64) => Reg::x64(x64.sub_ty(var.ty)),
            })
        } else {
            return self.alloc_stack(var);
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
    Mem(i64),
}