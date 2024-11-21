mod builder;

/// X86 assembly optimization
pub mod opt;

use crate::Target::instr::McInstr;

use super::reg::{X64Reg, X64RegSize};

/// A x64 assembly instruction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct X64Instr {
    pub(crate) mnemonic: X64Mnemonic,
    pub(crate) op1: Option<X64Operand>,
    pub(crate) op2: Option<X64Operand>,
    pub(crate) op3: Option<X64Operand>,
}

/// A x64 assembly mnemonic
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum X64Mnemonic {
    Mov,
    Movss,
    Movsd,
    Movdqa,

    Ret,

    Add,
    Paddq,
    Paddd,

    Sub,
    Psubq,
    Psubd,
    Psubw,
    Psubb,

    Lea,

    Jmp,
}

/// A x64 assembly operand
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum X64Operand {
    Reg(X64Reg),
    Const(i64),
    MemDispl(X64MemDispl),
    Tmp(usize),
    BlockRel(i64),
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
            X64Operand::Tmp(t) => write!(f, "tmps.{t}")?,
            X64Operand::BlockRel(block) => {
                let block = crate::Target::x86::get_block_rel(*block);
                write!(f, ".{block}")?;
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
            X64Mnemonic::Add => "add",
            X64Mnemonic::Movss => "movss",
            X64Mnemonic::Movsd => "movsd",
            X64Mnemonic::Movdqa => "movdqa",
            X64Mnemonic::Paddq => "paddq",
            X64Mnemonic::Paddd => "paddd",
            X64Mnemonic::Sub => "sub",
            X64Mnemonic::Psubb => "psubq",
            X64Mnemonic::Psubw => "psubw",
            X64Mnemonic::Psubd => "psubd",
            X64Mnemonic::Psubq => "psubq",
            X64Mnemonic::Jmp => "jmp",
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
        format!("{self}")
    }

    fn encode(&self) -> Vec<u8> {
        todo!()
    }

    fn branch_to_block(&self) -> Option<crate::Obj::Link> {
        todo!()
    }

    fn relocation(&self) -> Option<crate::Obj::Link> {
        todo!()
    }
}