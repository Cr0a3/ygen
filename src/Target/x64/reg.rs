use std::fmt::Display;

use crate::Target::registry::Reg;

/// A x64 register
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum x64Reg {
    Rax, Eax, Ax, Al,
    Rbx, Ebx, Bx, Bl,
    Rcx, Ecx, Cx, Cl,
    Rdx, Edx, Dx, Dl,
    Rsi, Esi, Si, Sil,
    Rdi, Edi, Di, Dil,

    R8, R8d, R8w, R8b,
    R9, R9d, R9w, R9b,
    R10, R10d, R10w, R10b,
    R11, R11d, R11w, R11b,
    R12, R12d, R12w, R12b,
    R13, R13d, R13w, R13b,
    R14, R14d, R14w, R14b,
    R15, R15d, R15w, R15b,
}

impl Display for x64Reg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

impl Reg for x64Reg {
    fn sub64(&self) -> String {
        use x64Reg::*;
        match self {
            Rax | Eax | Ax | Al => "rax",
            Rbx | Ebx | Bx | Bl => "rbx",
            Rcx | Ecx | Cx | Cl => "rcx",
            Rdx | Edx | Dx | Dl => "rdx",
            Rsi | Esi | Si | Sil => "rsi",
            Rdi | Edi | Di | Dil => "rdi",
        
            R8 | R8d | R8w | R8b => "r8",
            R9 | R9d | R9w | R9b => "r9",
            R10 | R10d | R10w | R10b => "r10",
            R11 | R11d | R11w | R11b => "r11",
            R12 | R12d | R12w | R12b => "r12",
            R13 | R13d | R13w | R13b => "r13",
            R14 | R14d | R14w | R14b => "r14",
            R15 | R15d | R15w | R15b => "r15",
        }.to_string()
    }

    fn sub32(&self) -> String {
        use x64Reg::*;
        match self {
            Rax | Eax | Ax | Al => "eax",
            Rbx | Ebx | Bx | Bl => "ebx",
            Rcx | Ecx | Cx | Cl => "ecx",
            Rdx | Edx | Dx | Dl => "edx",
            Rsi | Esi | Si | Sil => "esi",
            Rdi | Edi | Di | Dil => "edi",
        
            R8 | R8d | R8w | R8b => "r8d",
            R9 | R9d | R9w | R9b => "r9d",
            R10 | R10d | R10w | R10b => "r10d",
            R11 | R11d | R11w | R11b => "r11d",
            R12 | R12d | R12w | R12b => "r12d",
            R13 | R13d | R13w | R13b => "r13d",
            R14 | R14d | R14w | R14b => "r14d",
            R15 | R15d | R15w | R15b => "r15d",
        }.to_string()
    }

    fn sub16(&self) -> String {
        use x64Reg::*;
        match self {
            Rax | Eax | Ax | Al => "ax",
            Rbx | Ebx | Bx | Bl => "bx",
            Rcx | Ecx | Cx | Cl => "cx",
            Rdx | Edx | Dx | Dl => "dx",
            Rsi | Esi | Si | Sil => "si",
            Rdi | Edi | Di | Dil => "di",
        
            R8 | R8d | R8w | R8b => "r8w",
            R9 | R9d | R9w | R9b => "r9w",
            R10 | R10d | R10w | R10b => "r10w",
            R11 | R11d | R11w | R11b => "r11w",
            R12 | R12d | R12w | R12b => "r12w",
            R13 | R13d | R13w | R13b => "r13w",
            R14 | R14d | R14w | R14b => "r14w",
            R15 | R15d | R15w | R15b => "r15w",
        }.to_string()
    }

    fn sub8(&self) -> String {
        use x64Reg::*;
        match self {
            Rax | Eax | Ax | Al => "ax",
            Rbx | Ebx | Bx | Bl => "bx",
            Rcx | Ecx | Cx | Cl => "cx",
            Rdx | Edx | Dx | Dl => "dx",
            Rsi | Esi | Si | Sil => "sil",
            Rdi | Edi | Di | Dil => "dil",
        
            R8 | R8d | R8w | R8b => "r8b",
            R9 | R9d | R9w | R9b => "r9b",
            R10 | R10d | R10w | R10b => "r10b",
            R11 | R11d | R11w | R11b => "r11b",
            R12 | R12d | R12w | R12b => "r12b",
            R13 | R13d | R13w | R13b => "r13b",
            R14 | R14d | R14w | R14b => "r14b",
            R15 | R15d | R15w | R15b => "r15b",
        }.to_string()
    }

    fn boxed(&self) -> Box<dyn Reg> {
        Box::from(*self)
    }
    
    fn from(&self, string: String) -> Box<dyn Reg> {
        use x64Reg::*;
        Box::new(match string.as_str() {
            "rax" => Rax, "eax" => Eax, "ax" => Ax, "al" => Al,
            "rbx" => Rbx, "ebx" => Ebx, "bx" => Bx, "bl" => Bl,
            "rcx" => Rcx, "ecx" => Ecx, "cx" => Cx, "cl" => Cl,
            "rdx" => Rdx, "edx" => Edx, "dx" => Dx, "dl" => Dl,
            "rsi" => Rsi, "esi" => Esi, "si" => Si, "sil" => Sil,
            "rdi" => Rdi, "edi" => Edi, "di" => Di, "dil" => Dil,

            "r8" => R8, "r8d" => R8d, "r8w" => R8w, "r8b" => R8w,
            "r9" => R9, "r9d" => R9d, "r9w" => R9w, "r9b" => R9w,
            "r10" => R10, "r10d" => R10d, "r10w" => R10w, "r10b" => R10w,
            "r11" => R11, "r11d" => R11d, "r11w" => R11w, "r11b" => R11w,
            "r12" => R12, "r12d" => R12d, "r12w" => R12w, "r12b" => R12w,
            "r13" => R13, "r13d" => R13d, "r13w" => R13w, "r13b" => R13w,
            "r14" => R14, "r14d" => R14d, "r14w" => R14w, "r14b" => R14w,
            "r15" => R15, "r15d" => R15d, "r15w" => R15w, "r15b" => R15w,
            _ => todo!("unknown register"),
        })
    }
}
