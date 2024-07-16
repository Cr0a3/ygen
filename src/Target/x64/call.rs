use crate::Target::CallConv;

use super::x64Reg;

impl CallConv {
    /// Returns the number of register arguments in the calling convention
    pub fn regArgs(&self) -> usize {
        match self {
            CallConv::SystemV => 6,
            CallConv::WindowsFastCall => 4,
        }
    }

    /// Returns the 16Bit intenger argument registers as a vec
    pub fn args16(&self) -> Vec<x64Reg> {
        match self {
            CallConv::SystemV => vec![x64Reg::Si, x64Reg::Di, x64Reg::Dx, x64Reg::Cx, x64Reg::R8w, x64Reg::R9w],
            CallConv::WindowsFastCall => vec![x64Reg::Dx, x64Reg::Cx, x64Reg::R8w, x64Reg::R9w],
        }
    }

    /// Returns the 32Bit intenger argument registers as a vec
    pub fn args32(&self) -> Vec<x64Reg> {
        match self {
            CallConv::SystemV => vec![x64Reg::Esi, x64Reg::Edi, x64Reg::Edx, x64Reg::Ecx, x64Reg::R8d, x64Reg::R9d],
            CallConv::WindowsFastCall => vec![x64Reg::Edx, x64Reg::Ecx, x64Reg::R8d, x64Reg::R9d],
        }
    }

    /// Returns the 16Bit intenger argument registers as a vec
    pub fn args64(&self) -> Vec<x64Reg> {
        match self {
            CallConv::SystemV => vec![x64Reg::Rsi, x64Reg::Rdi, x64Reg::Rdx, x64Reg::Rcx, x64Reg::R8, x64Reg::R9],
            CallConv::WindowsFastCall => vec![x64Reg::Rdx, x64Reg::Rcx, x64Reg::R8, x64Reg::R9],
        }
    }

    /// Returns the return register
    pub fn ret16(&self) -> x64Reg {
        match  self {
            CallConv::WindowsFastCall => x64Reg::Ax,
            CallConv::SystemV => x64Reg::Ax,
        }
    }

    /// Returns the return register
    pub fn ret32(&self) -> x64Reg {
        match  self {
            CallConv::WindowsFastCall =>x64Reg::Eax,
            CallConv::SystemV => x64Reg::Eax,
        }
    }

    /// Returns the return register
    pub fn ret64(&self) -> x64Reg {
        match  self {
            CallConv::WindowsFastCall => x64Reg::Rax,
            CallConv::SystemV => x64Reg::Rax,
        }
    }
}