use std::fmt::Display;

use crate::IR::TypeMetadata;

/// A x64 register
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum x64Reg {
    Rax, Eax, Ax, Al,
    Rbx, Ebx, Bx, Bl,
    Rcx, Ecx, Cx, Cl,
    Rdx, Edx, Dx, Dl,
    Rsi, Esi, Si, Sil,
    Rdi, Edi, Di, Dil,

    Rsp, Esp, Sp, Spl,
    Rbp, Ebp, Bp, Bpl,

    R8, R8d, R8w, R8b,
    R9, R9d, R9w, R9b,
    R10, R10d, R10w, R10b,
    R11, R11d, R11w, R11b,
    R12, R12d, R12w, R12b,
    R13, R13d, R13w, R13b,
    R14, R14d, R14w, R14b,
    R15, R15d, R15w, R15b,

    Xmm0, Xmm1, Xmm2, Xmm3,
    Xmm4, Xmm5, Xmm6, Xmm7,
    Xmm8, Xmm9, Xmm10, Xmm11,
    Xmm12, Xmm13, Xmm14, Xmm15,
}

impl x64Reg {
    /// Parses the string to an register (Returns none if it's invalid)
    pub fn parse(string: String) -> Option<Self> {
        use x64Reg::*;
        match string.to_ascii_lowercase().as_str() {
            "rax" => Some(Rax), "eax" => Some(Eax), "ax" => Some(Ax), "al" => Some(Al),
            "rbx" => Some(Rbx), "ebx" => Some(Ebx), "bx" => Some(Bx), "bl" => Some(Bl),
            "rcx" => Some(Rcx), "ecx" => Some(Ecx), "cx" => Some(Cx), "cl" => Some(Cl),
            "rdx" => Some(Rdx), "edx" => Some(Edx), "dx" => Some(Dx), "dl" => Some(Dl),
            "rsi" => Some(Rsi), "esi" => Some(Esi), "si" => Some(Si), "sil" => Some(Sil),
            "rdi" => Some(Rdi), "edi" => Some(Edi), "di" => Some(Di), "dil" => Some(Dil),

            "rsp" => Some(Rsp), "esp" => Some(Esp), "sp" => Some(Sp), "spl" => Some(Spl),
            "rbp" => Some(Rbp), "ebp" => Some(Ebp), "bp" => Some(Bp), "bpl" => Some(Bpl),

            "r8" => Some(R8), "r8d" => Some(R8d), "r8w" => Some(R8w), "r8b" => Some(R8b),
            "r9" => Some(R9), "r9d" => Some(R9d), "r9w" => Some(R9w), "r9b" => Some(R9b),
            "r10" => Some(R10), "r10d" => Some(R10d), "r10w" => Some(R10w), "r10b" => Some(R10b),
            "r11" => Some(R11), "r11d" => Some(R11d), "r11w" => Some(R11w), "r11b" => Some(R11b),
            "r12" => Some(R12), "r12d" => Some(R12d), "r12w" => Some(R12w), "r12b" => Some(R12b),
            "r13" => Some(R13), "r13d" => Some(R13d), "r13w" => Some(R13w), "r13b" => Some(R13b),
            "r14" => Some(R14), "r14d" => Some(R14d), "r14w" => Some(R14w), "r14b" => Some(R14b),
            "r15" => Some(R15), "r15d" => Some(R15d), "r15w" => Some(R15w), "r15b" => Some(R15b),

            "xmm0" => Some(Xmm0), "xmm1" => Some(Xmm1), "xmm2" => Some(Xmm2),
            "xmm3" => Some(Xmm3), "xmm4" => Some(Xmm4), "xmm5" => Some(Xmm5),
            "xmm6" => Some(Xmm6), "xmm7" => Some(Xmm7), "xmm8" => Some(Xmm8),
            "xmm9" => Some(Xmm9), "xmm10" => Some(Xmm10), "xmm11" => Some(Xmm11),
            "xmm12" => Some(Xmm12), "xmm13" => Some(Xmm13), "xmm14" => Some(Xmm14),
            "xmm15" => Some(Xmm15),
            
            _ => None,
        }
    }

    /// Returns if the reg is in the extendet region (r8->r15)
    pub fn extended(&self) -> bool {
        use x64Reg::*;
        match self {
            R8 | R8d | R8w | R8b |
            R9 | R9d | R9w | R9b |
            R10 | R10d | R10w | R10b |
            R11 | R11d | R11w | R11b |
            R12 | R12d | R12w | R12b |
            R13 | R13d | R13w | R13b |
            R14 | R14d | R14w | R14b |
            R15 | R15d | R15w | R15b  => true,

            Xmm8 | Xmm9 | Xmm10 | Xmm11 |
            Xmm12 | Xmm13 | Xmm14 | Xmm15  => true,
            _ => false,
        }
    }

    /// The sub 64 bit wide variant of the register
    pub fn sub64(&self) -> x64Reg {
        use x64Reg::*;
        match self {
            Rax | Eax | Ax | Al => Rax,
            Rbx | Ebx | Bx | Bl => Rbx,
            Rcx | Ecx | Cx | Cl => Rcx,
            Rdx | Edx | Dx | Dl => Rdx,
            Rsi | Esi | Si | Sil => Rsi,
            Rdi | Edi | Di | Dil => Rdi,

            Rsp | Esp | Sp | Spl => Rsp,
            Rbp | Ebp | Bp | Bpl => Rbp,
        
            R8 | R8d | R8w | R8b => R8,
            R9 | R9d | R9w | R9b => R9,
            R10 | R10d | R10w | R10b => R10,
            R11 | R11d | R11w | R11b => R11,
            R12 | R12d | R12w | R12b => R12,
            R13 | R13d | R13w | R13b => R13,
            R14 | R14d | R14w | R14b => R14,
            R15 | R15d | R15w | R15b => R15,

            _ => *self,
        }
    }

    /// The sub 32 bit wide variant of the register
    pub fn sub32(&self) -> x64Reg {
        use x64Reg::*;
        match self {
            Rax | Eax | Ax | Al => Eax,
            Rbx | Ebx | Bx | Bl => Ebx,
            Rcx | Ecx | Cx | Cl => Ecx,
            Rdx | Edx | Dx | Dl => Edx,
            Rsi | Esi | Si | Sil => Esi,
            Rdi | Edi | Di | Dil => Edi,

            Rsp | Esp | Sp | Spl => Esp,
            Rbp | Ebp | Bp | Bpl => Ebp,
        
            R8 | R8d | R8w | R8b => R8d,
            R9 | R9d | R9w | R9b => R9d,
            R10 | R10d | R10w | R10b => R10d,
            R11 | R11d | R11w | R11b => R11d,
            R12 | R12d | R12w | R12b => R12d,
            R13 | R13d | R13w | R13b => R13d,
            R14 | R14d | R14w | R14b => R14d,
            R15 | R15d | R15w | R15b => R15d,

            _ => *self,
        }
    }

    /// The sub 16 bit wide variant of the register
    pub fn sub16(&self) -> x64Reg {
        use x64Reg::*;
        match self {
            Rax | Eax | Ax | Al => Ax,
            Rbx | Ebx | Bx | Bl => Bx,
            Rcx | Ecx | Cx | Cl => Cx,
            Rdx | Edx | Dx | Dl => Dx,
            Rsi | Esi | Si | Sil => Si,
            Rdi | Edi | Di | Dil => Di,

            Rsp | Esp | Sp | Spl => Sp,
            Rbp | Ebp | Bp | Bpl => Bp,
        
            R8 | R8d | R8w | R8b => R8w,
            R9 | R9d | R9w | R9b => R9w,
            R10 | R10d | R10w | R10b => R10w,
            R11 | R11d | R11w | R11b => R11w,
            R12 | R12d | R12w | R12b => R12w,
            R13 | R13d | R13w | R13b => R13w,
            R14 | R14d | R14w | R14b => R14w,
            R15 | R15d | R15w | R15b => R15w,

            _ => *self,
        }
    }

    /// The sub8 bit wide variant of the register
    pub fn sub8(&self) -> x64Reg {
        use x64Reg::*;
        match self {
            Rax | Eax | Ax | Al => Al,
            Rbx | Ebx | Bx | Bl => Bl,
            Rcx | Ecx | Cx | Cl => Cl,
            Rdx | Edx | Dx | Dl => Dl,
            Rsi | Esi | Si | Sil => Sil,
            Rdi | Edi | Di | Dil => Dil,

            Rsp | Esp | Sp | Spl => Spl,
            Rbp | Ebp | Bp | Bpl => Bpl,
        
            R8 | R8d | R8w | R8b => R8b,
            R9 | R9d | R9w | R9b => R9b,
            R10 | R10d | R10w | R10b => R10b,
            R11 | R11d | R11w | R11b => R11b,
            R12 | R12d | R12w | R12b => R12b,
            R13 | R13d | R13w | R13b => R13b,
            R14 | R14d | R14w | R14b => R14b,
            R15 | R15d | R15w | R15b => R15b,

            _ => *self,
        }
    }
    
    /// gets the subvariant based on the type
    pub fn sub_ty(&self, ty: TypeMetadata) -> x64Reg {
        match ty.byteSize() {
            8 => self.sub64(),
            4 => self.sub32(),
            2 => self.sub16(),
            1  | 0 => self.sub8(),

            _ => todo!("the type {} is to big/small for a single register", ty),
        }
    }

    /// Is the register (or better the subvariant) 64 bit wide?
    pub fn is_gr64(&self) -> bool {
        use x64Reg::*;
        match self {
            Rax | Rbx | Rcx | Rdx | Rsi | Rdi |
            Rsp | Rbp | R8 | R9 | R10 | R11 |
            R12 | R13 | R14 | R15 => true,

            _ => false,
        }
    }
    
    /// Is the register (or better the subvariant) 32 bit wide?
    pub fn is_gr32(&self) -> bool {
        use x64Reg::*;
        match self {
            Eax | Ebx | Ecx | Edx | Esi | Edi |
            Esp | Ebp | R8d | R9d | R10d | R11d |
            R12d | R13d | R14d | R15d => true,

            _ => false,
        }
    }
    
    /// Is the register (or better the subvariant) 16 bit wide?
    pub fn is_gr16(&self) -> bool {
        use x64Reg::*;
        match self {
            Ax | Bx | Cx | Dx | Si | Di |
            Sp | Bp | R8w | R9w | R10w | R11w |
            R12w | R13w | R14w | R15w => true,

            _ => false,
        }
    }
    
    /// Is the register (or better the subvariant) 8 bit wide?
    pub fn is_gr8(&self) -> bool {
        use x64Reg::*;
        match self {
            Al | Bl | Cl | Dl | Sil | Dil |
            Spl | Bpl | R8b | R9b | R10b |
            R11b | R12b | R13b | R14b | R15b => true,

            _ => false,
        }
    }
    
    /// Is the register a xmm register?
    pub fn is_xmm(&self) -> bool {
        use x64Reg::*;
        match self {
            Xmm0    | Xmm1  | Xmm2  | Xmm3  | Xmm4  |
            Xmm5    | Xmm6  | Xmm7  | Xmm8  | Xmm9  |
            Xmm10   | Xmm11 | Xmm12 | Xmm13 | Xmm14 |
            Xmm15 => true,
            _ => false,
        }
    }

    #[doc(hidden)]
    pub fn enc(&self) -> u8 {
        match self {
            // GR
            x64Reg::Rax | x64Reg::Eax | x64Reg::Ax | x64Reg::Al => 0,
            x64Reg::Rcx | x64Reg::Ecx | x64Reg::Cx | x64Reg::Cl => 1,
            x64Reg::Rdx | x64Reg::Edx | x64Reg::Dx | x64Reg::Dl => 2,
            x64Reg::Rbx | x64Reg::Ebx | x64Reg::Bx | x64Reg::Bl => 3,
            x64Reg::Rsi | x64Reg::Esi | x64Reg::Si | x64Reg::Sil => 6,
            x64Reg::Rbp | x64Reg::Ebp | x64Reg::Bp | x64Reg::Bpl => 5,
            x64Reg::Rsp | x64Reg::Esp | x64Reg::Sp | x64Reg::Spl => 4,
            x64Reg::Rdi | x64Reg::Edi | x64Reg::Di | x64Reg::Dil => 7,

            // here use a rex prefix
            x64Reg::R8 | x64Reg::R8d | x64Reg::R8w | x64Reg::R8b => 0,
            x64Reg::R9 | x64Reg::R9d | x64Reg::R9w | x64Reg::R9b => 1,
            x64Reg::R10 | x64Reg::R10d | x64Reg::R10w | x64Reg::R10b => 2,
            x64Reg::R11 | x64Reg::R11d | x64Reg::R11w | x64Reg::R11b => 3,
            x64Reg::R12 | x64Reg::R12d | x64Reg::R12w | x64Reg::R12b => 4,
            x64Reg::R13 | x64Reg::R13d | x64Reg::R13w | x64Reg::R13b => 5,
            x64Reg::R14 | x64Reg::R14d | x64Reg::R14w | x64Reg::R14b => 6,
            x64Reg::R15 | x64Reg::R15d | x64Reg::R15w | x64Reg::R15b => 7,

            // Xmm
            x64Reg::Xmm0 => 0,
            x64Reg::Xmm1 => 1,
            x64Reg::Xmm2 => 2,
            x64Reg::Xmm3 => 3,
            x64Reg::Xmm4 => 4,
            x64Reg::Xmm5 => 5,
            x64Reg::Xmm6 => 6,
            x64Reg::Xmm7 => 7,

            // here use a rex prefix
            x64Reg::Xmm8 => 0,
            x64Reg::Xmm9 => 1,
            x64Reg::Xmm10 => 2,
            x64Reg::Xmm11 => 3,
            x64Reg::Xmm12 => 4,
            x64Reg::Xmm13 => 5,
            x64Reg::Xmm14 => 6,
            x64Reg::Xmm15 => 7,

        }
    }
    
    #[doc(hidden)]
    pub fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Display for x64Reg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}