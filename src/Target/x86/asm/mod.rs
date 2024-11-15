mod builder;

use crate::{CodeGen::memory::Memory, Target::instr::McInstr};

use super::reg::X64Reg;

/// A x64 assembly instruction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct X64Instr {
    mnemonic: X64Mnemonic,
    op1: Option<X64Operand>,
    op2: Option<X64Operand>,
    op3: Option<X64Operand>,
}

/// A x64 assembly mnemonic
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum X64Mnemonic {
    Mov,
    Ret,
}

/// A x64 assembly operand
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum X64Operand {
    Reg(X64Reg),
    Const(i64),
    MemDispl(Memory),
}

impl std::fmt::Display for X64Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            X64Operand::Reg(x64) => write!(f, "{x64}")?,
            X64Operand::Const(imm) => write!(f, "{imm}")?,
            X64Operand::MemDispl(mem) => {
                write!(f, "[")?;
                if mem.fp_relativ {
                    write!(f, "rbp ")?;
                } else if mem.sp_relativ {
                    write!(f, "rsp ")?;
                }
                
                if mem.offset.is_negative() {
                    write!(f, "- ")?;
                } else {
                    write!(f, "+ ")?;
                }
                
                write!(f, "{}", mem.offset.abs())?;
                write!(f, "]")?;
            },
        };

        std::fmt::Result::Ok(())
    }
}

impl std::fmt::Display for X64Instr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self.mnemonic {
            X64Mnemonic::Mov => "mov",
            X64Mnemonic::Ret => "ret",
        })?;
        
        if let Some(op) = &self.op1 {
            write!(f, " {op}")?;
        }

        if let Some(op) = &self.op2 {
            write!(f, ", {op}")?;
        }

        if let Some(op) = &self.op3 {
            write!(f, ", {op}")?;
        }

        std::fmt::Result::Ok(())
    }
}

impl McInstr for X64Instr {
    fn asm(&self) -> String {
        format!("{}", self)
    }

    fn encode(&self) -> Vec<u8> {
        todo!("x64 instructions do not support encoding yet")
    }
}