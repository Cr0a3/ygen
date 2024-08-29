use crate::Target::{x64Reg, Arch, CallConv};

use super::Reg;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineCallingConvention {
    pub(crate) call_conv: CallConv,
}

impl MachineCallingConvention {
    pub fn return_reg(&self, arch: Arch) -> Reg {
        match self.call_conv {
            CallConv::WindowsFastCall => {
                match arch {
                    Arch::X86_64 => Reg::x64(x64Reg::Rax),
                    _ => todo!()
                }
            },
            CallConv::SystemV => {
                match arch {
                    Arch::X86_64 => Reg::x64(x64Reg::Rax),
                    _ => todo!()
                }
            },
            CallConv::AppleAarch64 => todo!(),
            CallConv::WasmBasicCAbi => todo!(),
        }
    }
}