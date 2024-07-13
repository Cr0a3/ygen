use crate::Target::CallConv;

impl CallConv {
    /// Returns the number of register arguments in the calling convention
    pub fn regArgs(&self) -> usize {
        match self {
            CallConv::SystemV => 6,
            CallConv::WindowsFastCall => 4,
        }
    }

    /// Returns the 16Bit intenger argument registers as a vec
    pub fn args16(&self) -> Vec<String> {
        match self {
            CallConv::SystemV => vec!["si".into(), "di".into(), "dx".into(), "cx".into(), "r8w".into(), "r9w".into()],
            CallConv::WindowsFastCall => vec!["dx".into(), "cx".into(), "r8w".into(), "r9w".into()],
        }
    }

    /// Returns the 32Bit intenger argument registers as a vec
    pub fn args32(&self) -> Vec<String> {
        match self {
            CallConv::SystemV => vec!["esi".into(), "edi".into(), "edx".into(), "ecx".into(), "r8d".into(), "r9d".into()],
            CallConv::WindowsFastCall => vec!["edx".into(), "ecx".into(), "r8d".into(), "r9d".into()],
        }
    }

    /// Returns the 16Bit intenger argument registers as a vec
    pub fn args64(&self) -> Vec<String> {
        match self {
            CallConv::SystemV => vec!["rsi".into(), "rdi".into(), "rdx".into(), "rcx".into(), "r8".into(), "r9".into()],
            CallConv::WindowsFastCall => vec!["rdx".into(), "rcx".into(), "r8".into(), "r9".into()],
        }
    }
}