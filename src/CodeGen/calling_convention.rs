use crate::{Target::{x64::X64Reg, Arch, CallConv}, IR::TypeMetadata};

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
                    Arch::X86_64 => match ty {
                        TypeMetadata::f32 | TypeMetadata::f64 => Reg::x64(X64Reg::Xmm0),
                        _ => Reg::x64(X64Reg::Rax.sub_ty(ty))
                    },
                    _ => todo!()
                }
            },
            CallConv::SystemV => {
                match arch {
                    Arch::X86_64 => match ty {
                        TypeMetadata::f32 | TypeMetadata::f64 => Reg::x64(X64Reg::Xmm0),
                        _ => Reg::x64(X64Reg::Rax.sub_ty(ty))
                    },
                    _ => todo!()
                }
            },
            CallConv::AppleAarch64 => todo!(),
            CallConv::WasmBasicCAbi => Reg::wasm(0, TypeMetadata::i32),
        }
    }
    
    /// returns the args for the specifc architecture
    pub fn arg(&self, arch: Arch, ty: TypeMetadata, idx: usize) -> Option<Reg> {
        match self.call_conv {
            CallConv::WindowsFastCall => {
                if TypeMetadata::f32 == ty || TypeMetadata::f64 == ty {
                    return match arch {
                        Arch::X86_64 => {
                            let args = vec![
                                Reg::x64(X64Reg::Xmm0), Reg::x64(X64Reg::Xmm1), Reg::x64(X64Reg::Xmm2),
                                Reg::x64(X64Reg::Xmm3), Reg::x64(X64Reg::Xmm4), Reg::x64(X64Reg::Xmm5),
                                Reg::x64(X64Reg::Xmm6), Reg::x64(X64Reg::Xmm7)
                            ];
                            let arg = args.get(idx).cloned();
                            arg
                        },

                        _ => todo!(),
                    }
                }

                match arch {
                    Arch::X86_64 => {
                        let args = vec![
                            Reg::x64(X64Reg::Rcx.sub_ty(ty)), Reg::x64(X64Reg::Rdx.sub_ty(ty)), 
                            Reg::x64(X64Reg::R8.sub_ty(ty)), Reg::x64(X64Reg::R9.sub_ty(ty))
                        ];
                        let arg = args.get(idx).cloned();
                        arg
                    },
                    _ => todo!()
                }
            },
            CallConv::SystemV => {
                if TypeMetadata::f32 == ty || TypeMetadata::f64 == ty {
                    return match arch {
                        Arch::X86_64 => {
                            let args = vec![
                                Reg::x64(X64Reg::Xmm0), Reg::x64(X64Reg::Xmm1), Reg::x64(X64Reg::Xmm2),
                                Reg::x64(X64Reg::Xmm3), Reg::x64(X64Reg::Xmm4), Reg::x64(X64Reg::Xmm5),
                                Reg::x64(X64Reg::Xmm6), Reg::x64(X64Reg::Xmm7)
                            ];
                            let arg = args.get(idx).cloned();
                            arg
                        },

                        _ => todo!(),
                    }
                }

                match arch {
                    Arch::X86_64 => {
                        let args = vec![
                            Reg::x64(X64Reg::Rdi.sub_ty(ty)), Reg::x64(X64Reg::Rsi.sub_ty(ty)), 
                            Reg::x64(X64Reg::Rcx.sub_ty(ty)), Reg::x64(X64Reg::Rdx.sub_ty(ty)), 
                            Reg::x64(X64Reg::R8.sub_ty(ty)), Reg::x64(X64Reg::R9.sub_ty(ty))
                        ];
                        let arg = args.get(idx).cloned();
                        arg
                    },
                    _ => todo!()
                }
            },
            CallConv::AppleAarch64 => todo!(),
            CallConv::WasmBasicCAbi => Some(Reg::wasm(idx as i32, ty)),
        }
    }

    
    /// returns the args for the specifc architecture
    pub fn args(&self, arch: Arch, ty: TypeMetadata) -> Vec<Reg> {
        match self.call_conv {
            CallConv::WindowsFastCall => {
                if TypeMetadata::f32 == ty || TypeMetadata::f64 == ty {
                    return match arch {
                        Arch::X86_64 => vec![
                            Reg::x64(X64Reg::Xmm0), Reg::x64(X64Reg::Xmm1), Reg::x64(X64Reg::Xmm2),
                            Reg::x64(X64Reg::Xmm3)
                        ],

                        _ => todo!(),
                    }
                }

                match arch {
                    Arch::X86_64 => vec![
                        Reg::x64(X64Reg::Rcx), Reg::x64(X64Reg::Rdx), 
                        Reg::x64(X64Reg::R8), Reg::x64(X64Reg::R9)
                    ],
                    _ => todo!()
                }
            },
            CallConv::SystemV => {
                if TypeMetadata::f32 == ty || TypeMetadata::f64 == ty {
                    return match arch {
                        Arch::X86_64 => vec![
                            Reg::x64(X64Reg::Xmm0), Reg::x64(X64Reg::Xmm1), Reg::x64(X64Reg::Xmm2),
                            Reg::x64(X64Reg::Xmm3), Reg::x64(X64Reg::Xmm4), Reg::x64(X64Reg::Xmm5),
                            Reg::x64(X64Reg::Xmm6), Reg::x64(X64Reg::Xmm7)
                        ],

                        _ => todo!(),
                    }
                }

                match arch {
                    Arch::X86_64 => vec![
                        Reg::x64(X64Reg::Rdi), Reg::x64(X64Reg::Rsi), 
                        Reg::x64(X64Reg::Rcx), Reg::x64(X64Reg::Rdx), 
                        Reg::x64(X64Reg::R8), Reg::x64(X64Reg::R9)
                    ],
                    _ => todo!()
                }
            },
            CallConv::AppleAarch64 => todo!(),
            CallConv::WasmBasicCAbi => Vec::new(),
        }
    }
    
    /// returns how many arguments are stored in registers
    pub fn num_reg_args(&self, arch: Arch, ty: TypeMetadata) -> usize {
        match arch {
            Arch::X86_64 => match self.call_conv {
                CallConv::SystemV => if ty.float() { 6 } else { 7 },
                CallConv::WindowsFastCall => if ty.float() { 4 } else { 7 },
                _ => panic!("unsuported calling convention for x86-64"),
            },

            Arch::Wasm64 => usize::MAX, // all arguments are stored in registers
            _ => panic!("unsuported arch: {:?}", arch),
        }
    }



    /// returns the stack shadow space
    pub fn shadow(&self, _: Arch) -> i64 {
        match self.call_conv {
            CallConv::WindowsFastCall => 32,
            CallConv::SystemV => 16,
            _ => 8,
        }
    }

    /// Returns the alignment
    pub fn align(&self, _: Arch) -> i64 {
        8 // ygen (currently) only supports x64 which has an alignment of 8
    }

    /// Returns the fp registers which are saved by the caller
    pub fn caller_saved_fps(&self, arch: Arch) -> Vec<Reg> {
        match (arch, self.call_conv) {
            (Arch::X86_64, CallConv::SystemV) => Vec::new(),
            (Arch::X86_64, CallConv::WindowsFastCall) => vec![Reg::x64(X64Reg::Xmm0), Reg::x64(X64Reg::Xmm1), Reg::x64(X64Reg::Xmm2), Reg::x64(X64Reg::Xmm3), Reg::x64(X64Reg::Xmm4), Reg::x64(X64Reg::Xmm5)],
            _ => Vec::new(),
        }
    }

    /// Returns the gr registers which are saved by the caller
    pub fn caller_saved_grs(&self, arch: Arch) -> Vec<Reg> {
        match (arch, self.call_conv) {
            (Arch::X86_64, CallConv::SystemV) => vec![Reg::x64(X64Reg::Rax), Reg::x64(X64Reg::Rcx), Reg::x64(X64Reg::Rdx), Reg::x64(X64Reg::Rsi), Reg::x64(X64Reg::Rdi), Reg::x64(X64Reg::R8), Reg::x64(X64Reg::R9), Reg::x64(X64Reg::R10), Reg::x64(X64Reg::R11)],
            (Arch::X86_64, CallConv::WindowsFastCall) => vec![Reg::x64(X64Reg::Rax), Reg::x64(X64Reg::Rcx), Reg::x64(X64Reg::Rdx), Reg::x64(X64Reg::R8), Reg::x64(X64Reg::R9), Reg::x64(X64Reg::R10), Reg::x64(X64Reg::R11)],
            _ => Vec::new(),
        }
    }
}