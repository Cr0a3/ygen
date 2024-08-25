use std::{fmt::Display, ops::{Add, Sub}, str::FromStr};

use crate::{Obj::Link, Support::{ColorClass, ColorProfile}, Target::{isa::{buildOpcode, MandatoryPrefix, RexPrefix}, x64Reg, Reg}};

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
    pub fn encode(&self) -> Result<(Vec<u8>, Option<Link>), InstrEncodingError> {
        self.verify()?;
        
        Ok(match self.mnemonic {
            Mnemonic::Add | Mnemonic::Adc | Mnemonic::And | Mnemonic::Or | Mnemonic::Sub | Mnemonic::Xor | Mnemonic::Mov => {
                let mandatory = if let Some(Operand::Reg(reg)) = &self.op1 {
                    if reg.is_gr16() { Some(MandatoryPrefix::t16BitOps)}
                    else { None }
                } else { None };

                let (mut r, mut m, i, ibase, ibase8) = match self.mnemonic {
                    Mnemonic::Add => (0x01, 0x03, 0, 0x81, 0x80),
                    Mnemonic::Adc => (0x11, 0x03, 2, 0x81, 0x80),
                    Mnemonic::Sub => (0x29, 0x2B, 5, 0x81, 0x80),
                    Mnemonic::And => (0x21, 0x23, 3, 0x81, 0x80),
                    Mnemonic::Or => (0x09, 0x0B, 1, 0x81, 0x80),
                    Mnemonic::Xor => (0x31, 0x33, 6, 0x81, 0x80),
                    Mnemonic::Mov => (0x89, 0x8B, 0, 0xC7, 0xC6),
                    _ => unreachable!(),
                };

                if let Some(Operand::Reg(reg)) = &self.op1 {
                    if reg.is_gr8() { 
                        r -= 1; m -= 1;
                    }
                }

                (match self.op2.as_ref().expect("verifycation failed") {
                    Operand::Reg(reg) => {
                        let reg = reg.as_any().downcast_ref::<x64Reg>().expect("expected x64 registers and not the ones from other archs");

                        let mut rex = RexPrefix::none();
                        rex.w = reg.is_gr64();
                        rex.r = reg.extended();

                        let mut op = vec![];

                        if let Some(Operand::Reg(op0)) = &self.op1 {
                            let op0 = op0.as_any().downcast_ref::<x64Reg>().expect("expected x64 registers and not the ones from other archs");
                            
                            if op0.extended() {
                                rex.b = true;
                            }

                            if reg.extended() || op0.extended() { 
                                op.push(r);
                                op.extend_from_slice(&ModRm::reg2(
                                    *op0, 
                                    *reg
                                ));
                            }
                            else {
                                op.push(m);
                                op.extend_from_slice(&ModRm::reg2(
                                    *reg, 
                                    *op0
                                ));
                            }

                        } else if let Some(Operand::Mem(mem)) = &self.op1 {
                            op.push(r);
                            rex.sync(mem.rex());
                            op.extend_from_slice(&ModRm::memR(
                                mem.clone(),
                                *reg.as_any().downcast_ref::<x64Reg>().expect("expected x64 registers and not the ones from other archs")
                            ))
                        } else { todo!() }

                        buildOpcode(mandatory, rex.option(), op)
                    },
                    Operand::Mem(mem) => {
                        let mut op = vec![];
                        let mut rex = None;

                        if let Some(Operand::Reg(op0)) = &self.op1 {
                            let op0 = op0.as_any().downcast_ref::<x64Reg>().expect("expected x64 registers and not the ones from other archs");

                            if op0.extended() { 
                                if op0.is_gr64() { rex = Some(RexPrefix { w: true, r: false, x: false, b: true });  }
                                else { rex = Some(RexPrefix { w: false, r: false, x: false, b: true });  }
                            } else if op0.is_gr64() { rex = Some(RexPrefix { w: true, r: false, x: false, b: false });  }
                            op.push(m);

                            if !mem.rex().empty() {
                                if let Some(rext) = rex {
                                    rex = Some(rext.sync(mem.rex()));
                                } else {rex = Some(mem.rex())}
                            }

                            op.extend_from_slice(&ModRm::regM(
                                *op0,
                                mem.clone()
                            ));

                        } else { todo!() }

                        buildOpcode(mandatory, rex, op)
                    },
                    Operand::Imm(num) => {
                        let mut mandatory = None;
                        let mut rex = None;
                        let mut op = vec![];

                        if let Some(Operand::Reg(op0)) = &self.op1 {
                            let op0 = op0.as_any().downcast_ref::<x64Reg>().expect("expected x64 registers and not the ones from other archs");

                            if op0.is_gr64() || op0.extended() {
                                rex = Some(RexPrefix { w: op0.is_gr64(), r: false, x: false, b: op0.extended() });
                            }

                            if op0.is_gr16() {
                                mandatory = Some(MandatoryPrefix::t16BitOps);
                            }

                            if !op0.is_gr8() {
                                op.push(ibase);
                            } else {
                                op.push(ibase8);
                            }

                            op.extend_from_slice(&ModRm::regWimm(i, *op0));

                            let bytes = (*num).to_le_bytes();

                            if op0.is_gr64() || op0.is_gr32() {
                                op.push(bytes[0]); op.push(bytes[1]);
                                op.push(bytes[2]); op.push(bytes[3]);
                            } else if op0.is_gr16() {
                                op.push(bytes[0]); op.push(bytes[1]);
                            } else if op0.is_gr8() {
                                op.push(bytes[0]);
                            }
                        } else { todo!() }

                        buildOpcode(mandatory, rex, op)
                    }
                    _ => todo!(),
                }, None)
            },
            Mnemonic::Lea => {
                let op0 =  if let Operand::Reg(reg) = &self.op1.clone().expect("verify faild") {
                    *reg.as_any().downcast_ref::<x64Reg>().unwrap()
                } else { panic!() };

                let mandatory = if op0.is_gr16() { Some(MandatoryPrefix::t16BitOps) } else { None };
                let mut rex = {
                    if op0.is_gr64() {
                        Some(RexPrefix { w: true, r: op0.extended(), x: false, b: false })
                    } else if op0.extended() {
                        Some(RexPrefix { w: false, r: true, x: false, b: false })
                    } else { None }
                };

                let mut op = vec![];

                if let Some(Operand::Mem(mem)) = &self.op2 {
                    if !mem.rex().empty() {
                        if let Some(rext) = rex {
                            rex = Some(rext.sync(mem.rex()));
                        } else {rex = Some(mem.rex())}
                    }

                    op.push(0x8D);
                    if !mem.rip {
                        op.extend_from_slice(&ModRm::regM(op0, mem.clone()));
                    } else {
                        op.extend_from_slice(&mem.rip(op0));
                    }
                } else { todo!() }

                (buildOpcode(mandatory, rex, op), None)
            },
            Mnemonic::Push | Mnemonic::Pop => {
                let mut mandatory = None;
                let mut rex = None;
                let mut op = vec![];

                let (r, m, i) = match self.mnemonic {
                    Mnemonic::Push => (0x50, 0xff, 6),
                    Mnemonic::Pop => (0x58, 0xff, 0),
                    _ => unreachable!()
                };

                if let Some(Operand::Reg(op0)) = &self.op1 {
                    let op0 = op0.as_any().downcast_ref::<x64Reg>().expect("expected x64 registers and not the ones from other archs");

                    rex = RexPrefix { w: false, r: false, x: false, b: op0.extended() }.option();

                    if op0.is_gr16() {
                        mandatory = Some(MandatoryPrefix::t16BitOps);
                    }
                    
                    op.push(r + op0.enc());

                } else if let Some(Operand::Mem(mem)) = &self.op1 {
                    op.push(m);
                    let enc = mem.encode(None);
                    op.push(enc.0 | i << 3 | 0b100);
                    op.extend_from_slice(&enc.1);
                } else if self.mnemonic == Mnemonic::Push {
                    if let Some(Operand::Imm(num)) = &self.op1 {
                        let bytes = (*num).to_be_bytes();
                        if *num >= i8::MIN as i64 && *num <= i8::MAX as i64 {
                            op.push(0x6A);
                            op.push(bytes[7]);
                        } else if *num >= i16::MIN as i64 && *num <= i16::MAX as i64 {
                            op.push(0x68);
                            op.push(bytes[7]);
                            op.push(bytes[6]);
                        } else if *num >= i32::MIN as i64 && *num <= i32::MAX as i64 {
                            op.push(0x68);
                            op.push(bytes[7]);
                            op.push(bytes[6]);
                            op.push(bytes[5]);
                            op.push(bytes[4]);
                        } else { todo!("you can't push 64bit ints")}
                    } else { todo!()}
                } else { todo!() }

                (buildOpcode(mandatory, rex, op), None)
            },
            Mnemonic::Ret => (vec![0xC3], None),
            Mnemonic::Endbr64 => (vec![0xF3, 0x0F, 0x1E, 0xFA], None),
            Mnemonic::Movzx => todo!(),
            Mnemonic::Call => {
                let (i, m, r) = (0xE8, 0xFF, 2);

                let mut op = vec![];
                if let Some(Operand::Reg(reg)) = &self.op1 {
                    op.push(m);
                    op.extend_from_slice(&ModRm::regWimm(r, *reg.as_any().downcast_ref::<x64Reg>().unwrap()));
                } else if let Some(Operand::Mem(mem)) = &self.op1 {
                    op.push(m);
                    op.extend_from_slice(&ModRm::imMem(r, mem.clone()));
                } else if let Some(Operand::Imm(imm)) = self.op1 {
                    op.push(i);
                    let bytes = imm.to_be_bytes();
                    op.push(bytes[7]);
                    op.push(bytes[6]);
                    op.push(bytes[5]);
                    op.push(bytes[4]);
                } else { todo!() }

                (buildOpcode(None, None, op), None)
            }
            Mnemonic::Jmp => {
                let (m, r) = (0xFF, 4);

                let mut op = vec![];
                if let Some(Operand::Reg(reg)) = &self.op1 {
                    op.push(m);
                    op.extend_from_slice(&ModRm::regWimm(r, *reg.as_any().downcast_ref::<x64Reg>().unwrap()));
                } else if let Some(Operand::Mem(mem)) = &self.op1 {
                    op.push(m);
                    op.extend_from_slice(&ModRm::imMem(r, mem.clone()));
                } else if let Some(Operand::Imm(imm)) = self.op1 {
                    op.push(0xE9);
                    let bytes = imm.to_be_bytes();
                    if imm < i8::MAX as i64 && imm > i8::MIN as i64 {
                        op.pop(); op.push(0xEB);
                        op.push(bytes[7]);
                    } else {
                        op.push(bytes[7]);
                        op.push(bytes[6]);
                        op.push(bytes[5]);
                        op.push(bytes[4]);
                    }
                } else { todo!() }

                (buildOpcode(None, None, op), None)
            }
            Mnemonic::Link => {
                if let Some(Operand::LinkDestination(dst, addend)) = &self.op1 {
                    (vec![], Some(Link { from: "".into(), to: dst.to_string(), at: 0, addend: *addend }))
                } else {
                    (vec![], None)
                }
            }
            Mnemonic::Debug => (vec![], None),
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
                    if let Some(Operand::Imm(_)) = self.op2 {
                        Err(InstrEncodingError::InvalidVariant(self.clone(), "add/sub/or/xor can't have mem, num (idk why)".into()))?
                    }
                }
            },
            Mnemonic::Push => {
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
            Mnemonic::Movzx => todo!(),
            Mnemonic::Call | Mnemonic::Jmp => {
                if self.op2 != None {
                    Err(InstrEncodingError::InvalidVariant(self.clone(), "call/jmp only needs one operand".into()))?
                }

                if let Some(Operand::Imm(_)) = self.op1 {} else {
                    if let Some(Operand::Mem(_)) = self.op1 {} else {
                        Err(InstrEncodingError::InvalidVariant(self.clone(), "call/jmp can only have num/mem operand".into()))?
                    }
                }
            }
            Mnemonic::Link | Mnemonic::Debug => {},
            Mnemonic::Endbr64 => {
                if self.op1.is_some() || self.op2.is_some() {
                    Err(InstrEncodingError::InvalidVariant(self.clone(), "endbr64 can't have operands".to_string()))?
                }
            }
        };

        Ok(())
    }

    /// Does the same as the encode function just for naming pourpuses
    pub fn compile(&self) -> Result<Vec<u8>, InstrEncodingError> {
        Ok(self.encode()?.0)
    }

    /// Returns the instruction as assembly representation
    pub fn to_string(&self) -> String {
        format!("{}", self)
    }

    /// emits the instruction as one colored string
    pub fn color(&self, profile: ColorProfile) -> String {
        let mut string = profile.markup(&format!("{}", self.mnemonic), ColorClass::Instr);

        if let Some(op1) = &self.op1 {
            string.push_str(&format!(" {}", match op1 {
                Operand::Imm(num) => profile.markup(&num.to_string(), ColorClass::Value),
                Operand::Reg(reg) => profile.markup(&reg.to_string(), ColorClass::Var),
                Operand::Mem(mem) => profile.markup(&format!("{}", mem), ColorClass::Var),
                Operand::LinkDestination(_, _) => "".to_string(),
                Operand::Debug(s) => s.to_string(),
            }));
            if let Some(op2) = &self.op2 {
                string.push_str(&format!(", {}", match op2 {
                    Operand::Imm(num) => profile.markup(&format!("{}", num.to_string()), ColorClass::Value),
                    Operand::Reg(reg) => profile.markup(&format!(", {}", reg.to_string()), ColorClass::Var),
                    Operand::Mem(mem) => profile.markup(&format!("{}", mem), ColorClass::Var),
                    Operand::LinkDestination(_, _) => "".to_string(),
                    Operand::Debug(s) => s.to_string(),
                }));
            }
        }

        string
    }

    /// Returns if the current instruction is the other instruction but inverted
    pub fn invert_of(&self, other: &Instr) -> bool {
        let mut out = false;

        if self.mnemonic == Mnemonic::Mov && other.mnemonic == Mnemonic::Mov {
            if self.op1 == other.op2 {
                if self.op2 == other.op1 {
                    out = true;
                }
            }

            if self.op2 == other.op1 {
                if self.op1 == other.op2 {
                    out = true;
                }
            }
        }

        if self.mnemonic == Mnemonic::Add && other.mnemonic == Mnemonic::Sub {
            if self.op1 == other.op2 {
                if self.op2 == other.op1 {
                    out = true;
                }
            }
        }

        if self.mnemonic == Mnemonic::Sub && other.mnemonic == Mnemonic::Add {
            if self.op1 == other.op2 {
                if self.op2 == other.op1 {
                    out = true;
                }
            }
        }

        out
    }

    /// returns true if it overrides the given operand
    pub fn uses_mut(&self, op: &Option<Operand>) -> bool {
        match self.mnemonic {
            Mnemonic::Add | Mnemonic::Adc | Mnemonic::And | 
            Mnemonic::Or | Mnemonic::Xor |Mnemonic::Sub | 
            Mnemonic::Mov | Mnemonic::Movzx | Mnemonic::Lea => {
              if self.op1 == *op {
                true
              } else { false }
            },
            
            _ => false,
        }
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
    Movzx,
    Push,
    Pop,
    Ret,

    Call,
    Jmp,

    Endbr64,

    /// here's a link placed
    Link,
    /// for debugging pourpusis
    Debug,
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
            "movzx" => Ok(Mnemonic::Movzx),
            "push" => Ok(Mnemonic::Push),
            "pop" => Ok(Mnemonic::Pop),
            "ret" => Ok(Mnemonic::Ret),
            "call" => Ok(Mnemonic::Call),
            "jmp" => Ok(Mnemonic::Jmp),
            "endbr64" => Ok(Mnemonic::Endbr64),
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
            Mnemonic::Movzx => "movzx",
            Mnemonic::Push => "push",
            Mnemonic::Pop => "pop",
            Mnemonic::Ret => "ret",
            Mnemonic::Call => "call",
            Mnemonic::Jmp => "jmp",
            Mnemonic::Endbr64 => "endbr64",
            Mnemonic::Link => "",
            Mnemonic::Debug => "#",
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
    /// The link destination
    LinkDestination(String, i64),
    /// For debugging
    Debug(String),
}

impl PartialEq for Operand {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Imm(l0), Self::Imm(r0)) => l0 == r0,
            (Self::Reg(l0), Self::Reg(r0)) => l0 == r0,
            (Self::Mem(l0), Self::Mem(r0)) => l0 == r0,
            (Self::LinkDestination(l0, l1), Self::LinkDestination(r0, r1)) => l0 == r0 && l1 == r1,
            (Self::Debug(l0), Self::Debug(r0)) => l0 == r0,
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
            Operand::LinkDestination(_, _) => "".to_string(),
            Operand::Debug(s) => s.to_string(),
        })
    }
}

/// A memory displacement
#[derive(Eq)]
pub struct MemOp {
    /// The base register
    pub base: Option<Box<dyn Reg>>,
    /// The index register
    pub index: Option<Box<dyn Reg>>,
    /// The scale
    pub scale: isize,
    /// The displacement
    pub displ: isize,
    /// rip relativ
    pub rip: bool,
}

impl MemOp {
    #[doc(hidden)]
    pub fn encode(&self, basis: Option<Box<dyn Reg>>) -> (/*modrm mod*/u8, Vec<u8>) {
        let mut scale = match self.scale {
            0 => 0,
            1 => 0,
            2 => 1,
            4 => 2,
            8 => 3,
            _ => todo!("scale needs to be either 1/2/4/8")
        };

        let mut displ = vec![];
        let mut modrm = 0;

        if self.displ == 0 {
            modrm |= 0b00 << 6;
        } else if self.displ >= -128 && self.displ <= 127 {
            modrm |= 0b01 << 6;
            scale = 1;
            displ.push(self.displ as u8);
        } else {
            modrm |= 0b10 << 6;
            scale = 4;
            displ.extend_from_slice(&(self.displ as i32).to_le_bytes());
        }

        let mut sib = 0;

        if let Some(index) = &self.index {
            if let Some(base) = &self.base {
                sib |= (scale << 6) | (index.enc() << 3) | base.enc();
            } else {
                if let Some(base) = &basis {
                    sib |= (scale << 6) | (index.enc() << 3) | base.enc();
                } else {
                    sib |= (scale << 6) | (index.enc() << 3);
                }
            }
        } else {
            if let Some(base) = &self.base {
                if let Some(basis) = &basis {
                    sib |= (scale << 6) | basis.enc() << 3 | base.enc();
                } else {
                    sib |= (scale << 6) | 0b100 << 3 | base.enc();
                }
            } else {
                if let Some(base) = &basis {
                    sib |= (scale << 6) | 0b100 << 3 | base.enc();
                } else {
                    sib |= (scale << 6) | 0b100 << 3 | 0b100;
                }
            }
        }

        let mut encoding = vec![sib];
        encoding.extend_from_slice(&displ);

        (modrm, encoding)
    }

    /// Returns the used rex prefix for the memory displacment
    pub fn rex(&self) -> RexPrefix {
        let mut rex = RexPrefix::none();
        if let Some(base) = &self.base {
            rex.b = base.as_any().downcast_ref::<x64Reg>().unwrap().extended();
        }
        
        if let Some(index) = &self.index {
            rex.x = index.as_any().downcast_ref::<x64Reg>().unwrap().extended();
        }

        rex
    }

    #[doc(hidden)]
    pub fn rip(&self, basis: x64Reg) -> Vec<u8> {
        if !self.rip {
            todo!()
        }

        ModRm::regRipImm(basis, self.displ as i32)
    }
}

impl PartialEq for MemOp {
    fn eq(&self, other: &Self) -> bool {
        self.base == other.base.clone() && 
        self.index == other.index && 
        self.scale == other.scale && 
        self.displ == other.displ
    }
}

impl core::fmt::Debug for MemOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MemOp")
            .field("base", &self.base)
            .field("index", &self.index)
            .field("scale", &self.scale)
            .field("displ", &self.displ)
        .finish()
    }
}

impl Clone for MemOp {
    fn clone(&self) -> Self {
        Self { 
            base: self.base.clone(), 
            index: {
                if let Some(index) = &self.index { Some(index.boxed()) }
                else { None }
            },
            scale: self.scale.clone(), 
            displ: self.displ.clone(),
            rip: self.rip, 
        }
    }
}

impl Display for MemOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::from("[ ");

        if let Some(base) = &self.base {
            string.push_str(&format!("{} ", base));
        }

        if let Some(index) = &self.index {
            string.push_str(&format!("+ {}", index))
        } else if self.displ != 0 {
            if self.rip {
                string.push_str("rip")
            }

            if self.displ > 0 { string.push_str("+ ") }
            else { string.push_str("- ") }
            string.push_str(&format!("{}", self.displ.abs()))
        }

        

        string.push_str(" ]");

        write!(f, "{}", string)
    }
}

impl Add<u32> for x64Reg {
    type Output = MemOp;

    fn add(self, rhs: u32) -> Self::Output {
        MemOp {
            base: Some(self.boxed()),
            index: None,
            scale: 2,
            displ: rhs as isize,
            rip: false,
        }
    }
}

impl Add<x64Reg> for x64Reg {
    type Output = MemOp;

    fn add(self, rhs: x64Reg) -> Self::Output {
        MemOp {
            base: Some(self.boxed()),
            index: Some(rhs.boxed()),
            scale: 1,
            displ: 0,
            rip: false,
        }
    }
}

impl Sub<u32> for x64Reg {
    type Output = MemOp;

    fn sub(self, rhs: u32) -> Self::Output {
        MemOp {
            base: Some(self.boxed()),
            index: None,
            scale: 2,
            displ: -(rhs as isize),
            rip: false,
        }
    }
}