use std::fmt::Display;

/// The register variants for x64
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum X64RegVariant {
    Rax, Rbx, Rcx, Rdx,
    Rdi, Rsi, Rbp, Rsp, R8,
    R9, R10, R11, R12, R13, R14,
    R15,

    Xmm0, Xmm1, Xmm2, Xmm3, Xmm4,
    Xmm5, Xmm6, Xmm7, Xmm8, Xmm9,
    Xmm10, Xmm11, Xmm12, Xmm13,
    Xmm14, Xmm15
}

/// The size for a register or memory displacment in the x64 backend
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum X64RegSize {
    Byte,
    Word,
    Dword,
    Qword,
}

impl From<usize> for X64RegSize {
    fn from(value: usize) -> Self {
        match value {
            1 => X64RegSize::Byte,
            2 => X64RegSize::Word,
            3 => X64RegSize::Dword,
            4 => X64RegSize::Qword,
            _ => panic!("invalid size for a register: {value}")
        }
    }
}

/// An x64 register
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub struct X64Reg {
    pub size: X64RegSize,
    pub variant: X64RegVariant,
}

macro_rules! reg_creator {
    ($reg_name:ident, $doc:expr) => {
        #[doc = $doc]
        pub fn $reg_name() -> Self {
            Self {
                size: X64RegSize::Qword,
                variant: X64RegVariant::$reg_name
            }
        }
    };
}

impl X64Reg {
    reg_creator!(Rax, "creates a new rax register");
    reg_creator!(Rbx, "creates a new rbx register");
    reg_creator!(Rcx, "creates a new rcx register");
    reg_creator!(Rdx, "creates a new rdx register");
    reg_creator!(Rdi, "creates a new rdi register");
    reg_creator!(Rsi, "creates a new rsi register");
    reg_creator!(Rbp, "creates a new rbp register");
    reg_creator!(Rsp, "creates a new rsp register");
    reg_creator!(R8, "creates a new r8 register");
    reg_creator!(R9, "creates a new r9 register");
    reg_creator!(R10, "creates a new r10 register");
    reg_creator!(R11, "creates a new r11 register");
    reg_creator!(R12, "creates a new r12 register");
    reg_creator!(R13, "creates a new r13 register");
    reg_creator!(R14, "creates a new r14 register");
    reg_creator!(R15, "creates a new r15 register");
    reg_creator!(Xmm0, "creates a new xmm0 register");
    reg_creator!(Xmm1, "creates a new xmm1 register");
    reg_creator!(Xmm2, "creates a new xmm2 register");
    reg_creator!(Xmm3, "creates a new xmm3 register");
    reg_creator!(Xmm4, "creates a new xmm4 register");
    reg_creator!(Xmm5, "creates a new xmm5 register");
    reg_creator!(Xmm6, "creates a new xmm6 register");
    reg_creator!(Xmm7, "creates a new xmm7 register");
    reg_creator!(Xmm8, "creates a new xmm8 register");
    reg_creator!(Xmm9, "creates a new xmm9 register");
    reg_creator!(Xmm10, "creates a new xmm10 register");
    reg_creator!(Xmm11, "creates a new xmm11 register");
    reg_creator!(Xmm12, "creates a new xmm12 register");
    reg_creator!(Xmm13, "creates a new xmm13 register");
    reg_creator!(Xmm14, "creates a new xmm14 register");
    reg_creator!(Xmm15, "creates a new xmm15 register");

    /// Returns if the register is a gr register
    pub fn is_gr(&self) -> bool {
        match self.variant {
            X64RegVariant::Rax |
            X64RegVariant::Rbx |
            X64RegVariant::Rcx |
            X64RegVariant::Rdx |
            X64RegVariant::Rdi |
            X64RegVariant::Rsi |
            X64RegVariant::Rbp |
            X64RegVariant::Rsp |
            X64RegVariant::R8  |
            X64RegVariant::R9  |
            X64RegVariant::R10 |
            X64RegVariant::R11 |
            X64RegVariant::R12 |
            X64RegVariant::R13 |
            X64RegVariant::R14 |
            X64RegVariant::R15 => true,
            _ => false,
        }
    }
}

impl Display for X64Reg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use X64RegSize::*;
        write!(f, "{}", match self.variant {
            X64RegVariant::Rax => match self.size { Qword => "rax", Dword => "eax", Word => "ax", Byte => "al"},
            X64RegVariant::Rbx => match self.size { Qword => "rbx", Dword => "ebx", Word => "bx", Byte => "bl"},
            X64RegVariant::Rcx => match self.size { Qword => "rcx", Dword => "ecx", Word => "cx", Byte => "cl"},
            X64RegVariant::Rdx => match self.size { Qword => "rdx", Dword => "edx", Word => "dx", Byte => "dl"},
            X64RegVariant::Rdi => match self.size { Qword => "rdi", Dword => "edi", Word => "di", Byte => "dil"},
            X64RegVariant::Rsi => match self.size { Qword => "rsi", Dword => "esi", Word => "si", Byte => "sil"},
            X64RegVariant::Rbp => match self.size { Qword => "rbp", Dword => "ebp", Word => "bp", Byte => "bpl"},
            X64RegVariant::Rsp => match self.size { Qword => "rsp", Dword => "esp", Word => "sp", Byte => "spl"},
            X64RegVariant::R8 => match self.size { Qword => "r8", Dword => "r8d", Word => "r8w", Byte => "r8l"},
            X64RegVariant::R9 => match self.size { Qword => "r9", Dword => "r9d", Word => "r9w", Byte => "r9l"},
            X64RegVariant::R10 => match self.size { Qword => "r10", Dword => "r10d", Word => "r10w", Byte => "r10l"},
            X64RegVariant::R11 => match self.size { Qword => "r11", Dword => "r11d", Word => "r11w", Byte => "r11l"},
            X64RegVariant::R12 => match self.size { Qword => "r12", Dword => "r12d", Word => "r12w", Byte => "r12l"},
            X64RegVariant::R13 => match self.size { Qword => "r13", Dword => "r13d", Word => "r13w", Byte => "r13l"},
            X64RegVariant::R14 => match self.size { Qword => "r14", Dword => "r14d", Word => "r14w", Byte => "r14l"},
            X64RegVariant::R15 => match self.size { Qword => "r15", Dword => "r15d", Word => "r15w", Byte => "r15l"},
            X64RegVariant::Xmm0 => "xmm0",
            X64RegVariant::Xmm1 => "xmm1",
            X64RegVariant::Xmm2 => "xmm2",
            X64RegVariant::Xmm3 => "xmm3",
            X64RegVariant::Xmm4 => "xmm4",
            X64RegVariant::Xmm5 => "xmm5",
            X64RegVariant::Xmm6 => "xmm6",
            X64RegVariant::Xmm7 => "xmm7",
            X64RegVariant::Xmm8 => "xmm8",
            X64RegVariant::Xmm9 => "xmm9",
            X64RegVariant::Xmm10 => "xmm10",
            X64RegVariant::Xmm11 => "xmm11",
            X64RegVariant::Xmm12 => "xmm12",
            X64RegVariant::Xmm13 => "xmm13",
            X64RegVariant::Xmm14 => "xmm14",
            X64RegVariant::Xmm15 => "xmm15",
        })
    }
}