mod builder;

/// X86 assembly optimization
pub mod opt;

use crate::Target::instr::McInstr;

use super::reg::{X64Reg, X64RegSize};

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
    Lea
}

/// A x64 assembly operand
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum X64Operand {
    Reg(X64Reg),
    Const(i64),
    MemDispl(X64MemDispl),
}

/// A x64 memory displacment
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct X64MemDispl {
    base: Option<X64Reg>,
    option: X64MemOption,
    index: Option<X64Reg>,
    displ: Option<i32>,
    scale: Option<i32>,
    size: X64RegSize,
}

/// What to do in the x64 displacment
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum X64MemOption {
    /// `base + ...`
    Plus,
    /// no operation (like `[rax]`)
    Nothing
}

impl std::fmt::Display for X64Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            X64Operand::Reg(x64) => write!(f, "{x64}")?,
            X64Operand::Const(imm) => write!(f, "{imm}")?,
            X64Operand::MemDispl(mem) => {
                write!(f, "[")?;
                if let Some(base) = mem.base {
                    write!(f, "{base} ")?;
                }

                if mem.option == X64MemOption::Plus {
                    let mut written = false;

                    if let Some(displ) = mem.displ { 
                        if displ.is_negative() { 
                            write!(f, "- ")?;
                            written = true;
                        }
                    }

                    if !written {
                        write!(f, "+ ")?;
                    }
                }

                if let Some(displ) = mem.displ {
                    write!(f, "{displ} ")?;
                }

                if let Some(index) = mem.index {
                    write!(f, "{index} ")?;
                }

                if let Some(scale) = mem.scale {
                    write!(f, "* {scale}")?;
                }

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
            X64Mnemonic::Lea => "lea",
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