mod builder;

/// X86 assembly optimization
pub mod opt;

mod encode;

use crate::Target::instr::McInstr;

use super::reg::{X86Reg, X86RegSize};

/// A X86 assembly instruction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct X86Instr {
    pub(crate) mnemonic: X86Mnemonic,
    pub(crate) op1: Option<X86Operand>,
    pub(crate) op2: Option<X86Operand>,
    pub(crate) op3: Option<X86Operand>,
}

/// A X86 assembly mnemonic
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum X86Mnemonic {
    Mov,
    Movss,
    Movsd,
    Movdqa,

    Ret,

    Add,
    Addss,
    Paddq,
    Paddd,

    Sub,
    Psubq,
    Psubd,
    Psubw,
    Psubb,

    Lea,

    Jmp,
    Je,

    Sete,
    Setne,
    Setl,
    Setle,
    Setg,
    Setge,
    Cmp,

    Pinsrb,
    Pinsrw,
    Pinsrd,
    Pinsrq,
    Insertps,

    Imul,

    And,
}

/// A X86 assembly operand
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum X86Operand {
    Reg(X86Reg),
    Const(i64),
    MemDispl(X86MemDispl),
    Tmp(usize),
    BlockRel(i64),
}

/// A X86 memory displacment
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct X86MemDispl {
    base: Option<X86Reg>,
    option: X86MemOption,
    index: Option<X86Reg>,
    displ: Option<i32>,
    scale: Option<i32>,
    size: X86RegSize,
}

/// What to do in the X86 displacment
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum X86MemOption {
    /// `base + ...`
    Plus,
    /// no operation (like `[rax]`)
    Nothing
}

impl std::fmt::Display for X86Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            X86Operand::Reg(X86) => write!(f, "{X86}")?,
            X86Operand::Const(imm) => write!(f, "{imm}")?,
            X86Operand::MemDispl(mem) => {
                write!(f, "[")?;
                if let Some(base) = mem.base {
                    write!(f, "{base} ")?;
                }

                if mem.option == X86MemOption::Plus {
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
                    write!(f, "{} ", displ.abs())?;
                }

                if let Some(index) = mem.index {
                    write!(f, "{index} ")?;
                }

                if let Some(scale) = mem.scale {
                    write!(f, "* {scale}")?;
                }

                write!(f, "]")?;
            },
            X86Operand::Tmp(t) => write!(f, "tmps.{t}")?,
            X86Operand::BlockRel(block) => {
                let block = crate::Target::x86::get_block_rel(*block);
                write!(f, ".{block}")?;
            },
        };

        std::fmt::Result::Ok(())
    }
}

impl std::fmt::Display for X86Instr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self.mnemonic {
            X86Mnemonic::Mov => "mov",
            X86Mnemonic::Ret => "ret",
            X86Mnemonic::Lea => "lea",
            X86Mnemonic::Add => "add",
            X86Mnemonic::Movss => "movss",
            X86Mnemonic::Movsd => "movsd",
            X86Mnemonic::Movdqa => "movdqa",
            X86Mnemonic::Paddq => "paddq",
            X86Mnemonic::Paddd => "paddd",
            X86Mnemonic::Sub => "sub",
            X86Mnemonic::Psubb => "psubq",
            X86Mnemonic::Psubw => "psubw",
            X86Mnemonic::Psubd => "psubd",
            X86Mnemonic::Psubq => "psubq",
            X86Mnemonic::Jmp => "jmp",
            X86Mnemonic::Je => "je",
            X86Mnemonic::Cmp => "cmp",
            X86Mnemonic::Sete => "sete",
            X86Mnemonic::Setne => "setne",
            X86Mnemonic::Setl => "setl",
            X86Mnemonic::Setle => "setle",
            X86Mnemonic::Setg => "setg",
            X86Mnemonic::Setge => "setge",
            X86Mnemonic::Pinsrb => "pinsrb",
            X86Mnemonic::Pinsrw => "pinsrw",
            X86Mnemonic::Pinsrd => "pinsrd",
            X86Mnemonic::Pinsrq => "pinsrq",
            X86Mnemonic::Insertps => "insertps",
            X86Mnemonic::Addss => "addss",
            X86Mnemonic::Imul => "imul",
            X86Mnemonic::And => "and",
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

impl McInstr for X86Instr {
    fn asm(&self) -> String {
        format!("{self}")
    }

    fn encode(&self) -> Vec<u8> {
        self.encode()
    }

    fn branch_to_block(&self) -> Option<crate::Obj::Link> {
        if let Some(X86Operand::BlockRel(branch)) = &self.op1 {
            return Some(crate::Obj::Link {
                from: String::new(),
                to: crate::Target::x86::get_block_rel(*branch),
                at: 0,
                addend: -4,
                special: true,
                kind: object::RelocationEncoding::X86Branch,
            });
        }

        None
    }

    fn relocation(&self) -> Option<crate::Obj::Link> {
        // TODO
        
        None
    }
}