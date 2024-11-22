use std::fmt::Display;

use crate::{Target::x86::get_call, IR::TypeMetadata};

/// The register variants for X86
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum X86RegVariant {
    Rax, Rbx, Rcx, Rdx,
    Rdi, Rsi, Rbp, Rsp, R8,
    R9, R10, R11, R12, R13, R14,
    R15,

    Xmm0, Xmm1, Xmm2, Xmm3, Xmm4,
    Xmm5, Xmm6, Xmm7, Xmm8, Xmm9,
    Xmm10, Xmm11, Xmm12, Xmm13,
    Xmm14, Xmm15
}

/// The size for a register or memory displacment in the X86 backend
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum X86RegSize {
    Byte,
    Word,
    Dword,
    Qword,
    SimdVec,
}

impl From<usize> for X86RegSize {
    fn from(value: usize) -> Self {
        match value {
            1 => X86RegSize::Byte,
            2 => X86RegSize::Word,
            4 => X86RegSize::Dword,
            8 => X86RegSize::Qword,
            16 => X86RegSize::SimdVec,
            _ => todo!("invalid size for a register: {value}")
        }
    }
}

/// An X86 register
#[derive(Debug, Clone, Copy, Hash)]
#[allow(missing_docs)]
pub struct X86Reg {
    pub size: X86RegSize,
    pub variant: X86RegVariant,
}

impl PartialEq for X86Reg {
    fn eq(&self, other: &Self) -> bool {
        self.variant == other.variant // && self.size == other.size
    }
}

impl Eq for X86Reg {}

macro_rules! reg_creator {
    ($reg_name:ident, $doc:expr) => {
        #[doc = $doc]
        pub fn $reg_name() -> Self {
            Self {
                size: X86RegSize::Qword,
                variant: X86RegVariant::$reg_name
            }
        }
    };
}

impl X86Reg {
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
            X86RegVariant::Rax |
            X86RegVariant::Rbx |
            X86RegVariant::Rcx |
            X86RegVariant::Rdx |
            X86RegVariant::Rdi |
            X86RegVariant::Rsi |
            X86RegVariant::Rbp |
            X86RegVariant::Rsp |
            X86RegVariant::R8  |
            X86RegVariant::R9  |
            X86RegVariant::R10 |
            X86RegVariant::R11 |
            X86RegVariant::R12 |
            X86RegVariant::R13 |
            X86RegVariant::R14 |
            X86RegVariant::R15 => true,
            _ => false,
        }
    }

    /// Returns if the register is a fp register
    pub fn is_fp(&self) -> bool {
        match self.variant {
            X86RegVariant::Xmm0 |
            X86RegVariant::Xmm1 |
            X86RegVariant::Xmm2 |
            X86RegVariant::Xmm3 |
            X86RegVariant::Xmm4 |
            X86RegVariant::Xmm5 |
            X86RegVariant::Xmm6 |
            X86RegVariant::Xmm7 |
            X86RegVariant::Xmm8 |
            X86RegVariant::Xmm9 |
            X86RegVariant::Xmm10 |
            X86RegVariant::Xmm11 |
            X86RegVariant::Xmm12 |
            X86RegVariant::Xmm13 |
            X86RegVariant::Xmm14 |
            X86RegVariant::Xmm15 => true,
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

        use X86RegVariant::*;
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

impl Display for X86Reg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use X86RegSize::*;
        write!(f, "{}", match self.variant {
            X86RegVariant::Rax => match self.size { Dword => "eax", Word => "ax", Byte => "al", Qword | _ => "rax"},
            X86RegVariant::Rbx => match self.size { Dword => "ebx", Word => "bx", Byte => "bl", Qword | _ => "rbx"},
            X86RegVariant::Rcx => match self.size { Dword => "ecx", Word => "cx", Byte => "cl", Qword | _ => "rcx"},
            X86RegVariant::Rdx => match self.size { Dword => "edx", Word => "dx", Byte => "dl", Qword | _ => "rdx"},
            X86RegVariant::Rdi => match self.size { Dword => "edi", Word => "di", Byte => "dil", Qword | _ => "rdi"},
            X86RegVariant::Rsi => match self.size { Dword => "esi", Word => "si", Byte => "sil", Qword | _ => "rsi"},
            X86RegVariant::Rbp => match self.size { Dword => "ebp", Word => "bp", Byte => "bpl", Qword | _ => "rbp"},
            X86RegVariant::Rsp => match self.size { Dword => "esp", Word => "sp", Byte => "spl", Qword | _ => "rsp"},
            X86RegVariant::R8 => match self.size { Dword => "r8d", Word => "r8w", Byte => "r8l", Qword | _ => "r8"},
            X86RegVariant::R9 => match self.size { Dword => "r9d", Word => "r9w", Byte => "r9l", Qword | _ => "r9"},
            X86RegVariant::R10 => match self.size { Dword => "r10d", Word => "r10w", Byte => "r10l", Qword | _ => "r10"},
            X86RegVariant::R11 => match self.size { Dword => "r11d", Word => "r11w", Byte => "r11l", Qword | _ => "r11"},
            X86RegVariant::R12 => match self.size { Dword => "r12d", Word => "r12w", Byte => "r12l", Qword | _ => "r12"},
            X86RegVariant::R13 => match self.size { Dword => "r13d", Word => "r13w", Byte => "r13l", Qword | _ => "r13"},
            X86RegVariant::R14 => match self.size { Dword => "r14d", Word => "r14w", Byte => "r14l", Qword | _ => "r14"},
            X86RegVariant::R15 => match self.size { Dword => "r15d", Word => "r15w", Byte => "r15l", Qword | _ => "r15"},
            X86RegVariant::Xmm0 => "xmm0",
            X86RegVariant::Xmm1 => "xmm1",
            X86RegVariant::Xmm2 => "xmm2",
            X86RegVariant::Xmm3 => "xmm3",
            X86RegVariant::Xmm4 => "xmm4",
            X86RegVariant::Xmm5 => "xmm5",
            X86RegVariant::Xmm6 => "xmm6",
            X86RegVariant::Xmm7 => "xmm7",
            X86RegVariant::Xmm8 => "xmm8",
            X86RegVariant::Xmm9 => "xmm9",
            X86RegVariant::Xmm10 => "xmm10",
            X86RegVariant::Xmm11 => "xmm11",
            X86RegVariant::Xmm12 => "xmm12",
            X86RegVariant::Xmm13 => "xmm13",
            X86RegVariant::Xmm14 => "xmm14",
            X86RegVariant::Xmm15 => "xmm15",
        })
    }
}