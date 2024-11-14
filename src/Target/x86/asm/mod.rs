mod builder;

use crate::Target::instr::McInstr;

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
}

impl std::fmt::Display for X64Instr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self.mnemonic {
            X64Mnemonic::Mov => "mov",
            X64Mnemonic::Ret => "ret",
        })?;
        
        if let Some(op) = &self.op1 {
            write!(f, " {}", match op {
                X64Operand::Reg(x64) => x64.to_string(),
                X64Operand::Const(imm) => imm.to_string(),
            })?;
        }

        if let Some(op) = &self.op2 {
            write!(f, ", {}", match op {
                X64Operand::Reg(x64) => x64.to_string(),
                X64Operand::Const(imm) => imm.to_string(),
            })?;
        }

        if let Some(op) = &self.op3 {
            write!(f, ", {}", match op {
                X64Operand::Reg(x64) => x64.to_string(),
                X64Operand::Const(imm) => imm.to_string(),
            })?;
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