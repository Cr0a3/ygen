use std::{fmt::Display, ops::{Add, Sub}, str::FromStr};

use crate::Target::{isa::{buildOpcode, MandatoryPrefix, RexPrefix}, x64Reg, Reg};

use super::isa::ModRm;

/// The target instruction
#[derive(Debug, Clone, PartialEq, Eq)]
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
        self.verify()?;
        
        Ok(match self.mnemonic {
            Mnemonic::Add | Mnemonic::Adc | Mnemonic::And | Mnemonic::Or | Mnemonic::Sub | Mnemonic::Xor => {

                let mandatory = if let Some(Operand::Reg(reg)) = &self.op1 {
                    if reg.is_gr16() { Some(MandatoryPrefix::t16BitOps)}
                    else { None }
                } else { None };

                let rex: Option<RexPrefix> = if let Some(Operand::Reg(reg)) = &self.op1 {
                    if reg.is_gr64() { Some(RexPrefix { w: true, r: false, x: false, b: false })}
                    else { None }
                } else { None };

                let (mut r, mut m, _) = match self.mnemonic {
                    Mnemonic::Add => (0x01, 0x03, 0),
                    Mnemonic::Adc => (0x11, 0x03, 2),
                    Mnemonic::Sub => (0x29, 0x2B, 5),
                    Mnemonic::And => (0x21, 0x23, 4),
                    Mnemonic::Or => (0x09, 0x0B, 1),
                    Mnemonic::Xor => (0x31, 0x33, 6),
                    _ => unreachable!(),
                };

                if let Some(Operand::Reg(reg)) = &self.op1 {
                    if reg.is_gr8() { r -= 1; m -= 1; }
                }

                match self.op2.as_ref().expect("verifycation failed") {
                    Operand::Reg(reg) => {
                        let reg = reg.as_any().downcast_ref::<x64Reg>().expect("expected x64 registers and not the ones from other archs");

                        let mut rex = if reg.extended() {
                            if let Some(mut rex) = rex {
                                rex.r = true;
                                Some(rex)
                            }  else {
                                Some(RexPrefix { w: false, r: true, x: false, b: false })
                            }
                        } else { rex };
                        let mut op = vec![];

                        if let Some(Operand::Reg(op0)) = &self.op1 {
                            let op0 = op0.as_any().downcast_ref::<x64Reg>().expect("expected x64 registers and not the ones from other archs");

                            rex = if op0.extended() {
                                if let Some(mut rex) = rex {
                                    rex.b = true;
                                    Some(rex)
                                }  else {
                                    Some(RexPrefix { w: false, r: false, x: false, b: true })
                                }
                            } else { rex };

                            op.push(m);
                            op.extend_from_slice(&ModRm::reg2(
                                *reg, 
                                *op0
                            ));

                        } else if let Some(Operand::Mem(mem)) = &self.op1 {
                            op.push(m);
                            op.extend_from_slice(&ModRm::regM(
                                *reg.as_any().downcast_ref::<x64Reg>().expect("expected x64 registers and not the ones from other archs"), 
                                mem.clone(),
                            ))
                        } else { todo!() }

                        buildOpcode(mandatory, rex, op)
                    },
                    Operand::Mem(mem) => {
                        let mut op = vec![];
                        let mut rex = None;

                        if let Some(Operand::Reg(op0)) = &self.op1 {
                            let op0 = op0.as_any().downcast_ref::<x64Reg>().expect("expected x64 registers and not the ones from other archs");

                            if op0.extended() { 
                                rex = Some(RexPrefix { w: false, r: false, x: false, b: true }); 
                            }

                            op.push(r);
                            op.extend_from_slice(&ModRm::memR(
                                mem.clone(), 
                                *op0
                            ));

                        } else { todo!() }

                        buildOpcode(mandatory, rex, op)
                    },
                    
                    _ => todo!(),
                }
            },
            Mnemonic::Lea => todo!(),
            Mnemonic::Mov => todo!(),
            Mnemonic::Push => todo!(),
            Mnemonic::Pop => todo!(),
            Mnemonic::Ret => vec![0xC3],
        })
    }

    /// Verifys the instruction (like checking the right opcodes etc.)
    pub fn verify(&self) -> Result<(), InstrEncodingError> {
        match self.mnemonic {
            Mnemonic::Lea => {
                if self.op2 == None || self.op1 == None {
                    Err(InstrEncodingError::InvalidVariant(self.clone(), "lea needs to have two operand".into()))?
                }
                if let Some(Operand::Reg(_)) = self.op1 {} else {
                    Err(InstrEncodingError::InvalidVariant(self.clone(), "leas first operand needs to be an register".into()))?
                }
                if let Some(Operand::Mem(_)) = self.op2 {} else {
                    Err(InstrEncodingError::InvalidVariant(self.clone(), "leas secound operand needs to be an memop".into()))?
                }
            },
            Mnemonic::Mov => {
                if self.op2 == None || self.op1 == None {
                    Err(InstrEncodingError::InvalidVariant(self.clone(), "mov needs to have two operand".into()))?
                }
                if let Some(Operand::Imm(_)) = self.op1 {
                    Err(InstrEncodingError::InvalidVariant(self.clone(), "the mov instructions requires that the first operand is either a reg or a memop".into()))?
                }

                if let Some(Operand::Mem(_)) = self.op1 {
                    if let Some(Operand::Mem(_)) = self.op2 {
                        Err(InstrEncodingError::InvalidVariant(self.clone(), "add/sub/or/xor can't have two mem operands".into()))?
                    }
                }
            },
            Mnemonic::Add | Mnemonic::Adc | Mnemonic::Sub | Mnemonic::And | Mnemonic::Or | Mnemonic::Xor => {
                if self.op2 == None || self.op1 == None {
                    Err(InstrEncodingError::InvalidVariant(self.clone(), "add/sub/and/or/xor needs to have two operand".into()))?
                }
                if let Some(Operand::Imm(_)) = self.op1 {
                    Err(InstrEncodingError::InvalidVariant(self.clone(), "the add/sub/and/or/xor instructions requires that the first operand is either a reg or a memop".into()))?
                }

                if let Some(Operand::Mem(_)) = self.op1 {
                    if let Some(Operand::Mem(_)) = self.op2 {
                        Err(InstrEncodingError::InvalidVariant(self.clone(), "add/sub/or/xor can't have two mem operands".into()))?
                    }
                }
            },
            Mnemonic::Push => {
                if let Some(Operand::Imm(_)) = self.op1 {
                    Err(InstrEncodingError::InvalidVariant(self.clone(), "the push instruction needs to have an op1 of either reg or mem".into()))?
                }

                if self.op2 != None || self.op1 == None {
                    Err(InstrEncodingError::InvalidVariant(self.clone(), "push needs to have one operand".into()))?
                }
            },
            Mnemonic::Pop => {
                if let Some(Operand::Imm(_)) = self.op1 {
                    Err(InstrEncodingError::InvalidVariant(self.clone(), "the pop instruction needs to have an op1 of either reg or mem".into()))?
                }

                if self.op2 != None || self.op1 == None {
                    Err(InstrEncodingError::InvalidVariant(self.clone(), "pop needs to have one operand".into()))?
                }
            },
            Mnemonic::Ret => {
                if self.op1 != None || self.op2 != None {
                    Err(InstrEncodingError::InvalidVariant(self.clone(), "ret isn't allowed to have operands".into()))?
                }
            },
        };

        Ok(())
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
#[derive(Debug, Clone, PartialEq, Eq)]
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
    Add,
    Adc,
    And,
    Or,
    Xor,
    Sub,

    Lea,
    Mov,
    Push,
    Pop,
    Ret,
}

impl FromStr for Mnemonic {
    type Err = ();

    fn from_str(s: &str) -> Result<Mnemonic, Self::Err> {
        match s {
            "add" => Ok(Mnemonic::Add),
            "adc" => Ok(Mnemonic::Adc),
            "and" => Ok(Mnemonic::And),
            "or" => Ok(Mnemonic::Or),
            "xor" => Ok(Mnemonic::Xor),
            "sub" => Ok(Mnemonic::Sub),
            "lea" => Ok(Mnemonic::Lea),
            "mov" => Ok(Mnemonic::Mov),
            "push" => Ok(Mnemonic::Push),
            "pop" => Ok(Mnemonic::Pop),
            "ret" => Ok(Mnemonic::Ret),
            _ => Err(()),
        }
    }
}

impl Display for Mnemonic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {    
            Mnemonic::Add => "add",
            Mnemonic::Adc => "adc",
            Mnemonic::And => "and",
            Mnemonic::Or => "or",
            Mnemonic::Xor => "xor",
            Mnemonic::Sub => "sub",
            Mnemonic::Lea => "lea",
            Mnemonic::Mov => "mov",
            Mnemonic::Push => "push",
            Mnemonic::Pop => "pop",
            Mnemonic::Ret => "ret",
        })
    }
}

/// The operand type and value to use
#[derive(Debug, Clone, Eq)]
pub enum Operand {
    /// A number operand
    Imm(i64),
    /// A register operand
    Reg(Box<dyn Reg>),
    /// A memory displacement
    Mem(MemOp),
}

impl PartialEq for Operand {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Imm(l0), Self::Imm(r0)) => l0 == r0,
            (Self::Reg(l0), Self::Reg(r0)) => l0 == r0,
            (Self::Mem(l0), Self::Mem(r0)) => l0 == r0,
            _ => false,
        }
    }
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
#[derive(Eq)]
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

impl MemOp {
    #[doc(hidden)]
    pub fn encode(&self) -> Vec<u8> {
        todo!()
    }

    #[doc(hidden)]
    pub fn _mod(&self) -> u8 {
        if self.displ == 0 { 0 }
        else if self.displ <= u8::MAX as isize { 0b01 }
        else  { 0b10 }
    }
}

impl PartialEq for MemOp {
    fn eq(&self, other: &Self) -> bool {
        self.base == other.base.clone() && self.index == other.index && self.scale == other.scale && self.displ == other.displ && self.add == other.add
    }
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