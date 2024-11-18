use std::fmt::Display;

use crate::{Target::x86::get_call, IR::TypeMetadata};

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
    SimdVec,
}

impl From<usize> for X64RegSize {
    fn from(value: usize) -> Self {
        match value {
            1 => X64RegSize::Byte,
            2 => X64RegSize::Word,
            4 => X64RegSize::Dword,
            8 => X64RegSize::Qword,
            16 => X64RegSize::SimdVec,
            _ => todo!("invalid size for a register: {value}")
        }
    }
}

/// An x64 register
#[derive(Debug, Clone, Copy)]
#[allow(missing_docs)]
pub struct X64Reg {
    pub size: X64RegSize,
    pub variant: X64RegVariant,
}

impl PartialEq for X64Reg {
    fn eq(&self, other: &Self) -> bool {
        self.variant == other.variant // && self.size == other.size
    }
}

impl Eq for X64Reg {}

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

    /// Returns if the register is a fp register
    pub fn is_fp(&self) -> bool {
        match self.variant {
            X64RegVariant::Xmm0 |
            X64RegVariant::Xmm1 |
            X64RegVariant::Xmm2 |
            X64RegVariant::Xmm3 |
            X64RegVariant::Xmm4 |
            X64RegVariant::Xmm5 |
            X64RegVariant::Xmm6 |
            X64RegVariant::Xmm7 |
            X64RegVariant::Xmm8 |
            X64RegVariant::Xmm9 |
            X64RegVariant::Xmm10 |
            X64RegVariant::Xmm11 |
            X64RegVariant::Xmm12 |
            X64RegVariant::Xmm13 |
            X64RegVariant::Xmm14 |
            X64RegVariant::Xmm15 => true,
            _ => false,
        }
    }

    /// Returns a score of the register
    /// 
    /// Rules:
    /// 1. Starts at 4
    /// 2. `-1` if it is requires a rex prefix
    /// 3. `-2` if it is callee saved
    pub fn score(&self) -> usize {
        let mut score = 4;

        use X64RegVariant::*;
        match self.variant {
            Rax | Rbx | Rcx | Rdx |
            Rsi | Rdi | Xmm0 | Xmm1 | 
            Xmm2 | Xmm3 | Xmm4 | Xmm5 | 
            Xmm6 | Xmm7 => {},
            _ => score -= 1,
        }

        if get_call().x86_is_callee_saved(self.variant) {
            score -= 2;
        }

        score
    }

    /// Returns if the register is a simd reg (supporting the given vector)
    pub fn is_simd(&self, vec: &crate::IR::VecTy) -> bool {
        // x86 makes simd using xmm.. registers
        // so we should first check if it is a fp register
        // cuz xmm.. registers are fp regs
        
        if !self.is_fp() { return false; }

        // then we should check if ygen even supports the vector size
        // we currently use sse so ygen should support:
        //   2x64 (fp/int)
        //   4x32 (fp/int)
        //   8x16 (int)
        //   16x8 (int)

        let ty: TypeMetadata = vec.ty.into();
        match (vec.size, ty.toSigned()) {
            (2, TypeMetadata::i64) | (2, TypeMetadata::f64) |
            (4, TypeMetadata::i32) | (4, TypeMetadata::f32) |
            (8, TypeMetadata::i16) |
            (16, TypeMetadata::i8) => {}, // allowed
            _ => todo!("(currently) unsupported vector size in ygen: {vec}"),
        }

        // now after all checks we safely know that we have a 128bit ygen x86
        // captible register
        true
    }
}

impl Display for X64Reg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use X64RegSize::*;
        write!(f, "{}", match self.variant {
            X64RegVariant::Rax => match self.size { Dword => "eax", Word => "ax", Byte => "al", Qword | _ => "rax"},
            X64RegVariant::Rbx => match self.size { Dword => "ebx", Word => "bx", Byte => "bl", Qword | _ => "rbx"},
            X64RegVariant::Rcx => match self.size { Dword => "ecx", Word => "cx", Byte => "cl", Qword | _ => "rcx"},
            X64RegVariant::Rdx => match self.size { Dword => "edx", Word => "dx", Byte => "dl", Qword | _ => "rdx"},
            X64RegVariant::Rdi => match self.size { Dword => "edi", Word => "di", Byte => "dil", Qword | _ => "rdi"},
            X64RegVariant::Rsi => match self.size { Dword => "esi", Word => "si", Byte => "sil", Qword | _ => "rsi"},
            X64RegVariant::Rbp => match self.size { Dword => "ebp", Word => "bp", Byte => "bpl", Qword | _ => "rbp"},
            X64RegVariant::Rsp => match self.size { Dword => "esp", Word => "sp", Byte => "spl", Qword | _ => "rsp"},
            X64RegVariant::R8 => match self.size { Dword => "r8d", Word => "r8w", Byte => "r8l", Qword | _ => "r8"},
            X64RegVariant::R9 => match self.size { Dword => "r9d", Word => "r9w", Byte => "r9l", Qword | _ => "r9"},
            X64RegVariant::R10 => match self.size { Dword => "r10d", Word => "r10w", Byte => "r10l", Qword | _ => "r10"},
            X64RegVariant::R11 => match self.size { Dword => "r11d", Word => "r11w", Byte => "r11l", Qword | _ => "r11"},
            X64RegVariant::R12 => match self.size { Dword => "r12d", Word => "r12w", Byte => "r12l", Qword | _ => "r12"},
            X64RegVariant::R13 => match self.size { Dword => "r13d", Word => "r13w", Byte => "r13l", Qword | _ => "r13"},
            X64RegVariant::R14 => match self.size { Dword => "r14d", Word => "r14w", Byte => "r14l", Qword | _ => "r14"},
            X64RegVariant::R15 => match self.size { Dword => "r15d", Word => "r15w", Byte => "r15l", Qword | _ => "r15"},
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