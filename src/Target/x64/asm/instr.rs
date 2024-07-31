use std::{fmt::Display, ops::{Add, Sub}};

use crate::Target::{x64Reg, Reg};

/// The target instruction
#[derive(Debug)]
pub struct Instr {
    /// The mnemonic to use
    pub mnemonic: Mnemonic,
    /// First operand
    pub op1: Option<Operand>,
    /// Second operand
    pub op2: Option<Operand>,
}

impl Instr {
    /// Creates the instruction with 0 operands
    pub fn with0(mne: Mnemonic) -> Self {
        Self {
            mnemonic: mne,
            op1: None,
            op2: None,
        }
    }

    /// Creates the instruction with 1 operand
    pub fn with1(mne: Mnemonic, op: Operand) -> Self {
        Self {
            mnemonic: mne,
            op1: Some(op),
            op2: None,
        }
    }

    /// Creates the instruction with 2 operands
    pub fn with2(mne: Mnemonic, op1: Operand, op2: Operand) -> Self {
        Self {
            mnemonic: mne,
            op1: Some(op1),
            op2: Some(op2),
        }
    }

    /// Encodes the instruction (some will say compiles)
    pub fn encode(&self) -> Result<Vec<u8>, InstrEncodingError> {
        // let out = vec![];
        // Ok(out)
        todo!()
    }

    /// Does the same as the encode function just for naming pourpuses
    pub fn compile(&self) -> Result<Vec<u8>, InstrEncodingError> {
        self.encode()
    }

    /// Returns the instruction as assembly representation
    pub fn to_string(&self) -> String {
        format!("{}", self)
    }
}

impl Display for Instr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = format!("{}", self.mnemonic);

        if let Some(op1) = &self.op1 {
            string.push_str(&format!(" {}", op1));
            if let Some(op2) = &self.op2 {
                string.push_str(&format!(", {}", op2));
            }
        }

        write!(f, "{}", string)
    }
}

/// An error which can occure during encoding instructions
#[derive(Debug)]
pub enum InstrEncodingError {
    /// The given instruction has an invalid variant
    InvalidVariant(Instr, String),
}

impl Display for InstrEncodingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            InstrEncodingError::InvalidVariant(instr, msg) => 
                format!("Your given instruction has an invalid variant '{}': {}", instr, msg),
        })
    }
}

impl std::error::Error for InstrEncodingError {}

/// The instructions mnemonic
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum Mnemonic {
    Lea,
    Mov,
    Add,
    Sub,
    Push,
    Pop,
    Ret,
}

impl Display for Mnemonic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Mnemonic::Lea => "lea",
            Mnemonic::Mov => "mov",
            Mnemonic::Add => "add",
            Mnemonic::Sub => "sub",
            Mnemonic::Push => "push",
            Mnemonic::Pop => "pop",
            Mnemonic::Ret => "ret",
        })
    }
}

/// The operand type and value to use
#[derive(Debug)]
pub enum Operand {
    /// A number operand
    Imm(i64),
    /// A register operand
    Reg(Box<dyn Reg>),
    /// A memory displacement
    Mem(MemOp),
}

impl Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Operand::Imm(num) => num.to_string(),
            Operand::Reg(reg) => reg.to_string(),
            Operand::Mem(mem) => format!("{}", mem),
        })
    }
}

/// A memory displacement
pub struct MemOp {
    /// The base register
    pub base: Box<dyn Reg>,
    /// The index register
    pub index: Option<Box<dyn Reg>>,
    /// The scale
    pub scale: isize,
    /// The displacement
    pub displ: isize,
    /// The operation (true -> +, false -> -)
    pub add: bool,
}

impl core::fmt::Debug for MemOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MemOp").field("base", &self.base).field("index", &self.index).field("scale", &self.scale).field("displ", &self.displ).field("add", &self.add).finish()
    }
}

impl Clone for MemOp {
    fn clone(&self) -> Self {
        Self { 
            base: self.base.boxed(), 
            index: {
                if let Some(index) = &self.index { Some(index.boxed()) }
                else { None }
            },
            scale: self.scale.clone(), 
            displ: self.displ.clone(), 
            add: self.add.clone() 
        }
    }
}

impl Display for MemOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::from("[");

        string.push(']');

        write!(f, "{}", string)
    }
}

impl Add<u32> for x64Reg {
    type Output = MemOp;

    fn add(self, rhs: u32) -> Self::Output {
        MemOp {
            base: self.boxed(),
            index: None,
            scale: 1,
            displ: rhs as isize,
            add: true,
        }
    }
}

impl Add<x64Reg> for x64Reg {
    type Output = MemOp;

    fn add(self, rhs: x64Reg) -> Self::Output {
        MemOp {
            base: self.boxed(),
            index: Some(rhs.boxed()),
            scale: 0,
            displ: 0,
            add: true,
        }
    }
}

impl Sub<u32> for x64Reg {
    type Output = MemOp;

    fn sub(self, rhs: u32) -> Self::Output {
        MemOp {
            base: self.boxed(),
            index: None,
            scale: 1,
            displ: rhs as isize,
            add: false,
        }
    }
}

impl Sub<x64Reg> for x64Reg {
    type Output = MemOp;

    fn sub(self, rhs: x64Reg) -> Self::Output {
        MemOp {
            base: self.boxed(),
            index: Some(rhs.boxed()),
            scale: 0,
            displ: 0,
            add: false,
        }
    }
}