use crate::{Target::{x64Reg, Arch, CallConv}, IR::TypeMetadata};

use super::Reg;

/// A more machine specifc calling convention
/// (Just a wrapper around the normal calling convention but with some pretty handy functions)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineCallingConvention {
    pub(crate) call_conv: CallConv,
}

impl MachineCallingConvention {
    /// returns the return register for the specific architecture and ty
    pub fn return_reg(&self, arch: Arch, ty: TypeMetadata) -> Reg {
        match self.call_conv {
            CallConv::WindowsFastCall => {
                match arch {
                    Arch::X86_64 => Reg::x64(x64Reg::Rax.sub_ty(ty)),
                    _ => todo!()
                }
            },
            CallConv::SystemV => {
                match arch {
                    Arch::X86_64 => Reg::x64(x64Reg::Rax.sub_ty(ty)),
                    _ => todo!()
                }
            },
            CallConv::AppleAarch64 => todo!(),
            CallConv::WasmBasicCAbi => todo!(),
        }
    }
    
    /// returns the args for the specifc architecture
    pub fn args(&self, arch: Arch) -> Vec<Reg> {
        match self.call_conv {
            CallConv::WindowsFastCall => {
                match arch {
                    Arch::X86_64 => vec![
                        Reg::x64(x64Reg::Rcx), Reg::x64(x64Reg::Rdx), 
                        Reg::x64(x64Reg::R8), Reg::x64(x64Reg::R9)
                    ],
                    _ => todo!()
                }
            },
            CallConv::SystemV => {
                match arch {
                    Arch::X86_64 => vec![
                        Reg::x64(x64Reg::Rdi), Reg::x64(x64Reg::Rsi), 
                        Reg::x64(x64Reg::Rcx), Reg::x64(x64Reg::Rdx), 
                        Reg::x64(x64Reg::R8), Reg::x64(x64Reg::R9)
                    ],
                    _ => todo!()
                }
            },
            CallConv::AppleAarch64 => todo!(),
            CallConv::WasmBasicCAbi => todo!(),
        }
    }
    
    /// returns how many arguments are stored in registers
    pub fn num_reg_args(&self, arch: Arch) -> usize {
        self.args(arch).len()
    }

    /// returns the stack shadow space
    pub fn shadow(&self, _: Arch) -> i64 {
        match self.call_conv {
            CallConv::WindowsFastCall => 32,
            _ => 8,
        }
    }

    /// Returns the alignment
    pub fn align(&self, _: Arch) -> i64 {
        8 // ygen only supports 64bit architectures
    }
}