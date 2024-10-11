use std::{fmt::Display, ops::{Add, Sub}, str::FromStr};
use crate::CodeGen::MCInstr;
use crate::Obj::Link;
use crate::Support::{ColorClass, ColorProfile};
use crate::Target::x64::isa::{buildOpcode, MandatoryPrefix, RexPrefix};
use crate::Target::x64::x64Reg;

use super::isa::ModRm;

/// The target instruction
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct X64MCInstr {
    /// The mnemonic to use
    pub mnemonic: Mnemonic,
    /// First operand
    pub op1: Option<Operand>,
    /// Second operand
    pub op2: Option<Operand>,
}

impl X64MCInstr {
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
            Mnemonic::Add | Mnemonic::Adc | Mnemonic::And | Mnemonic::Or | Mnemonic::Sub | Mnemonic::Xor | Mnemonic::Mov | Mnemonic::Cmp => {
                let mandatory = if let Some(Operand::Reg(reg)) = &self.op1 {
                    if reg.is_gr16() { Some(MandatoryPrefix::t16BitOps)}
                    else { None }
                } else { None };

                let (mut r, mut m, i, ibase, ibase8) = match self.mnemonic {
                    Mnemonic::Add => (0x01, 0x03, 0, 0x81, 0x80),
                    Mnemonic::Adc => (0x11, 0x03, 2, 0x81, 0x80),
                    Mnemonic::Sub => (0x29, 0x2B, 5, 0x81, 0x80),
                    Mnemonic::And => (0x21, 0x23, 4, 0x81, 0x80),
                    Mnemonic::Or => (0x09, 0x0B, 1, 0x81, 0x80),
                    Mnemonic::Xor => (0x31, 0x33, 6, 0x81, 0x80),
                    Mnemonic::Mov => (0x89, 0x8B, 0, 0xC7, 0xC6),

                    Mnemonic::Cmp => (0x39, 0x3B, 7, 0x81, 0x80),
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
                            rex.sync(mem.rex(true));
                            let enc = &mem.encode(Some(*reg));
                            op.extend_from_slice(&enc.1);
                        } else { todo!() }

                        buildOpcode(mandatory, rex.option(), op)
                    },
                    Operand::Mem(mem) => {
                        let mut op = vec![];
                        let mut rex = None;

                        if let Some(Operand::Reg(op0)) = &self.op1 {
                            let op0 = op0.as_any().downcast_ref::<x64Reg>().expect("expected x64 registers and not the ones from other archs");

                            if op0.extended() || op0.is_gr64() { 
                                rex = RexPrefix { w: op0.is_gr64(), r: op0.extended(), x: false, b: false }.option();
                            }
                            op.push(m);

                            if !mem.rex(false).empty() {
                                if let Some(rext) = rex {
                                    rex = Some(rext.sync(mem.rex(false)));
                                } else {rex = Some(mem.rex(false))}
                            }

                            let enc = &mem.encode(Some(*op0));
                            op.extend_from_slice(&enc.1);

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
                    if !mem.rex(false).empty() {
                        if let Some(rext) = rex {
                            rex = Some(rext.sync(mem.rex(false)));
                        } else {rex = Some(mem.rex(false))}
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
                    /*if imm < i8::MAX as i64 && imm > i8::MIN as i64 {
                        op.pop(); op.push(0xEB);
                        op.push(bytes[7]);
                    } else {*/
                        op.push(bytes[7]);
                        op.push(bytes[6]);
                        op.push(bytes[5]);
                        op.push(bytes[4]);
                    //}
                } else { todo!() }

                (buildOpcode(None, None, op), None)
            }
            Mnemonic::Jne | Mnemonic::Je => {
                let mut op = match self.mnemonic {
                    Mnemonic::Jne => vec![0x0F, 0x85],
                    Mnemonic::Je => vec![0x0F, 0x84],
                    _ => unreachable!()
                };

                if let Some(Operand::Imm(num)) = self.op1 {
                    let bytes = num.to_be_bytes();
                    op.push( bytes[7] );
                    op.push( bytes[6] );
                    op.push( bytes[5] );
                    op.push( bytes[4] );
                } else { unreachable!() }

                (buildOpcode(None, None, op), None)
            }
            Mnemonic::Link => {
                if let Some(Operand::LinkDestination(dst, addend)) = &self.op1 {
                    (vec![], Some(Link { from: "".into(), to: dst.to_string(), at: 0, addend: *addend, special: false }))
                } else if let Some(Operand::BlockLinkDestination(dst, addend)) = &self.op1 {
                    (vec![], Some(Link { from: "".into(), to: dst.to_string(), at: 0, addend: *addend, special: true }))
                } else {
                    (vec![], None)
                }
            }
            Mnemonic::Debug | Mnemonic::StartOptimization | Mnemonic::EndOptimization => (vec![], None),
            Mnemonic::Imul | Mnemonic::Mul | Mnemonic::Div | Mnemonic::Idiv => {
                let i = match self.mnemonic {
                    Mnemonic::Imul => 5,
                    Mnemonic::Mul => 4,
                    Mnemonic::Idiv => 7,
                    Mnemonic::Div => 6,
                    _ => unreachable!(),
                };

                let mut r = 0xF7;

                let mut mandatory = None;

                let mut rex = RexPrefix::none();

                let mut op = vec![];

                if let Some(Operand::Reg(reg)) = self.op1 {
                    if reg.is_gr8() {
                        r -= 1;
                    }

                    if reg.is_gr16() {
                        mandatory = Some(MandatoryPrefix::t16BitOps);
                    }

                    if reg.is_gr64() || reg.extended() {
                        rex.w = reg.is_gr64();
                        rex.r = reg.is_gr8();
                    }

                    op.push(r);

                    op.extend_from_slice(&ModRm::regWimm(i, reg));
                } else if let Some(Operand::Mem(mem)) = &self.op1 {
                    rex = mem.rex(false);
                    op.push(r);
                    let (mod_, rm) = mem.encode(None);

                    if mem.index.is_some() {
                        op.push(mod_ << 6 | i << 3 | 0b100);
                    }
                    op.extend_from_slice(&rm);
                } else { todo!() }

                (buildOpcode(mandatory, rex.option(), op), None)
            }
            Mnemonic::Setg | Mnemonic::Setge | Mnemonic::Setl | Mnemonic::Setle | Mnemonic::Sete | Mnemonic::Setne => {
                let mut op = vec![];
                let rex;

                let instr = match self.mnemonic {
                    Mnemonic::Setg => 0x9F,
                    Mnemonic::Setge => 0x9D,
                    Mnemonic::Setl => 0x9C,
                    Mnemonic::Setle => 0x9E,
                    Mnemonic::Sete => 0x94,
                    Mnemonic::Setne => 0x95,
                    _ => unreachable!(),
                };

                op.push(0x0f);
                op.push(instr);

                if let Some(Operand::Reg(reg)) = self.op1 {

                    rex = {
                        let mut rex = RexPrefix::none();
                        rex.b = reg.extended();
                        if reg.sub64() != x64Reg::Rax &&
                           reg.sub64() != x64Reg::Rbx &&
                           reg.sub64() != x64Reg::Rcx &&
                           reg.sub64() != x64Reg::Rdx {
                            Some(rex)
                        } else { None }
                    };

                    op.extend_from_slice( &ModRm::regWimm(0, reg) );
                } else if let Some(Operand::Mem(mem)) = &self.op1 {
                    rex = mem.rex(true).option();
                    let (mod_, encoded) = mem.encode(None);
                    
                    if mem.index.is_some() {
                        op.push(mod_ << 6 | 0b100);
                    }
                    op.extend_from_slice( &encoded );
                } else { unreachable!() }

                (buildOpcode(None, rex, op), None)
            }
            Mnemonic::Neg => {
                let mut mandatory = None;
                let mut rex = RexPrefix::none();
                let mut op = vec![];

                if let Some(Operand::Reg(reg)) = &self.op1 {
                    if reg.is_gr8() { op.push(0xF6); }
                    else { op.push(0xF7); }

                    rex.b = reg.extended();

                    if reg.is_gr16() { mandatory = Some(MandatoryPrefix::t16BitOps); }

                    op.extend_from_slice(&ModRm::regWimm(3, *reg));
                } else if let Some(Operand::Mem(mem)) = &self.op1 {
                    op.push(0xF7);

                    rex = mem.rex(true);
                    let (mod_, encoded) = mem.encode(None);
                    
                    if mem.index.is_some() {
                        op.push(mod_ << 6 | 0b100);
                    }
                    op.extend_from_slice( &encoded );
                } else { todo!() }

                (buildOpcode(mandatory, rex.option(), op), None)
            }
            Mnemonic::Cmove | Mnemonic::Cmovne => {
                let mut op = match self.mnemonic {
                    Mnemonic::Cmove => vec![0x0F, 0x44],
                    Mnemonic::Cmovne => vec![0x0F, 0x45],
                    _ => unreachable!(),
                };

                let mut mandatory = None;
                let mut rex = RexPrefix::none();

                let op1 = match self.op1.clone().unwrap() { Operand::Reg(reg) => reg, _ => unreachable!() };

                rex.w = op1.is_gr64();
                rex.r = op1.extended();

                if let Some(Operand::Reg(op2)) = &self.op2 {
                    rex.x = op2.extended();

                    if op2.is_gr16() { mandatory = Some(MandatoryPrefix::t16BitOps); }
                    
                    op.extend_from_slice(&ModRm::reg2(*op2, op1));
                } else if let Some(Operand::Mem(op2)) = &self.op2 {
                    rex = rex.sync(op2.rex(false));
                    
                    op.extend_from_slice(&ModRm::regM(op1, op2.clone()));
                } else { todo!() }

                (buildOpcode(mandatory, rex.option(), op), None)
            }
            Mnemonic::Sal | Mnemonic::Shr | Mnemonic::Sar => {
                let (mut op, i) = match self.mnemonic {
                    Mnemonic::Sal => (0xD3, 4),
                    Mnemonic::Shr => (0xD3, 5),
                    Mnemonic::Sar => (0xD2, 7),
                    _ => unreachable!(),
                };

                let mut encoded = Vec::new();

                let mut mandatory = None;
                let mut rex = RexPrefix::none();

                if let Some(Operand::Reg(reg)) = &self.op1 {
                    rex.w = reg.is_gr64();
                    rex.r = reg.extended();
                    if reg.is_gr16() { mandatory = Some(MandatoryPrefix::t16BitOps); }
                    if reg.is_gr8() { op -= 1; }

                    encoded.push(op);
                    encoded.extend_from_slice(&ModRm::regWimm(i, *reg));
                } else if let Some(Operand::Mem(mem)) = &self.op1 {
                    rex = mem.rex(true);

                    encoded.push(op);
                    encoded.extend_from_slice(&ModRm::imMem(i, mem.clone()));
                }

                (buildOpcode(mandatory, rex.option(), encoded), None)
            }
            _ => todo!(),
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
                    if let Some(Operand::Reg(_)) = self.op2 {} else {
                        Err(InstrEncodingError::InvalidVariant(self.clone(), "mov is only allowed: `mov rm/8, r` but something other was found".into()))?
                    }
                }
            },
            Mnemonic::Add | Mnemonic::Adc | Mnemonic::Sub | Mnemonic::And | Mnemonic::Or | Mnemonic::Xor | Mnemonic::Cmp => {
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
                        Err(InstrEncodingError::InvalidVariant(self.clone(), "call/jmp can needs to have num/mem operand".into()))?
                    }
                }
            }
            Mnemonic::Link | Mnemonic::Debug | Mnemonic::StartOptimization | Mnemonic::EndOptimization => {},
            Mnemonic::Endbr64 => {
                if self.op1.is_some() || self.op2.is_some() {
                    Err(InstrEncodingError::InvalidVariant(self.clone(), "endbr64 can't have operands".to_string()))?
                }
            }
            Mnemonic::Mul | Mnemonic::Imul | Mnemonic::Div | Mnemonic::Idiv => {
                if !(self.op1 != None && self.op2 == None) {
                    Err(InstrEncodingError::InvalidVariant(self.clone(), "mul/imul/div/idiv need on operand of type r/m".into()))?
                }

                if let Some(Operand::Imm(_)) = self.op1  {
                    Err(InstrEncodingError::InvalidVariant(self.clone(), 
                        "mul/imul/div/idiv need one operand of type r/m".into()
                    ))?
                }
            }
            Mnemonic::Jne | Mnemonic::Je => {
                if let Some(Operand::Imm(_)) = self.op1 {} else {
                    Err(InstrEncodingError::InvalidVariant(self.to_owned(), "j.. expects one imm as its ops".to_owned()))?
                }
            }
            Mnemonic::Setg | Mnemonic::Setge | Mnemonic::Setl | Mnemonic::Setle | Mnemonic::Sete | Mnemonic::Setne => {
                if self.op2.is_some() || self.op1.is_none() {
                    Err(InstrEncodingError::InvalidVariant(self.to_owned(), "set.. expects one operand".to_owned()))?
                }

                if let Some(Operand::Imm(_)) = self.op1 {
                    Err(InstrEncodingError::InvalidVariant(self.to_owned(), "set.. requires one operand of either register or memory".to_owned()))?
                }
            }
            Mnemonic::Neg => {
                if !(self.op1.is_some() && self.op2.is_none()) {
                    Err(InstrEncodingError::InvalidVariant(self.clone(), "neg r/m.. is required for neg".into()))?
                }
                if let Some(Operand::Imm(_)) = self.op1 {
                    Err(InstrEncodingError::InvalidVariant(self.clone(), "neg r/m.. is required for neg".into()))?
                }
            }
            Mnemonic::Cmove | Mnemonic::Cmovne  => {
                if let Some(Operand::Reg(_)) = &self.op1 {
                    if let Some(Operand::Imm(_)) = &self.op2 {
                        Err(InstrEncodingError::InvalidVariant(self.clone(), "cmov expects r, r/m".into()))?
                    } else if self.op2.is_none() {
                        Err(InstrEncodingError::InvalidVariant(self.clone(), "cmov expects r, r/m".into()))?
                    }
                } else {
                    Err(InstrEncodingError::InvalidVariant(self.clone(), "cmov expects r, r/m".into()))?
                }
            }
            Mnemonic::Sal | Mnemonic::Shr | Mnemonic::Sar => {
                if self.op1.is_none() || self.op2.is_none() {
                    Err(InstrEncodingError::InvalidVariant(self.clone(), "sal/shr/sar expects r/m, cl".into()))?
                }

                if let Some(Operand::Reg(reg)) = &self.op2 {
                    if x64Reg::Cl != *reg {
                        Err(InstrEncodingError::InvalidVariant(self.clone(), "sal/shr/sar expects r/m, cl".into()))?
                    }
                }
            }
            Mnemonic::Movq   =>  todo!(),
            Mnemonic::Movd   =>  todo!(),
            Mnemonic::Movss  =>  todo!(),
            Mnemonic::Movsd  =>  todo!(),
            Mnemonic::Movups =>  todo!(),
            Mnemonic::Movupd =>  todo!(),
            Mnemonic::Addss  =>  todo!(),
            Mnemonic::Addsd  =>  todo!(),
            Mnemonic::Divss  =>  todo!(),
            Mnemonic::Divsd  =>  todo!(),
            Mnemonic::Mulss  =>  todo!(),
            Mnemonic::Mulsd  =>  todo!(),
            Mnemonic::Subss  =>  todo!(),
            Mnemonic::Subsd  =>  todo!(),
            Mnemonic::Ucomiss  =>  todo!(),
            Mnemonic::Ucomisd  =>  todo!(),
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
                Operand::BlockLinkDestination(_, _) => "".to_string(),
                Operand::Debug(s) => s.to_string(),
            }));
            if let Some(op2) = &self.op2 {
                string.push_str(&format!(", {}", match op2 {
                    Operand::Imm(num) => profile.markup(&format!("{}", num.to_string()), ColorClass::Value),
                    Operand::Reg(reg) => profile.markup(&format!(", {}", reg.to_string()), ColorClass::Var),
                    Operand::Mem(mem) => profile.markup(&format!("{}", mem), ColorClass::Var),
                    Operand::LinkDestination(_, _) => "".to_string(),
                    Operand::BlockLinkDestination(_, _) => "".to_string(),
                    Operand::Debug(s) => s.to_string(),
                }));
            }
        }

        string
    }

    /// Returns if the current instruction is the other instruction but inverted
    pub fn invert_of(&self, other: &X64MCInstr) -> bool {
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

    /// returns if the instruction is empty like mov rsi, rsi
    pub fn empty(&self) -> bool {
        if self.op1 == self.op2 && self.mnemonic == Mnemonic::Mov {
            true
        } else {
            false
        }
    }
}

impl From<X64MCInstr> for Box<dyn MCInstr> {
    fn from(value: X64MCInstr) -> Self {
        Box::new( value )
    }
}

impl MCInstr for X64MCInstr {
    fn dump(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        Ok(vec![format!("{}", self)])
    }

    fn encode(&self) -> Result<(Vec<u8>, Option<Link>), Box<dyn std::error::Error>> {
        Ok(self.encode()?)
    }
    
    fn clone_box(&self) -> Box<dyn MCInstr> {
        Box::from( self.clone() )
    }
}

impl Display for X64MCInstr {
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
    InvalidVariant(X64MCInstr, String),
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

    Neg,

    Cmp,

    Lea,
    Mov,
    Movzx,
    Push,
    Pop,
    Ret,

    Imul,
    Mul,

    Idiv,
    Div,

    Call,
    Jmp,

    Jne,
    Je,

    Endbr64,

    /// here's a link placed
    Link,
    /// for debugging pourpusis
    Debug,
    /// start optimization again
    StartOptimization,
    /// stop optimization
    EndOptimization,

    Sete,
    Setne,
    Setg,
    Setl,
    Setge,
    Setle,

    Cmove,
    Cmovne,

    Sal,
    Shr,
    Sar,

    Movq,
    Movd,
    Movss,
    Movsd,
    Movups,
    Movupd,

    Addss,
    Addsd,
    Divss,
    Divsd,
    Mulss,
    Mulsd,
    Subss,
    Subsd,

    Ucomiss,
    Ucomisd,
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
            "imul" => Ok(Mnemonic::Imul),
            "mul" => Ok(Mnemonic::Mul),
            "jne" => Ok(Mnemonic::Jne),
            "je" => Ok(Mnemonic::Je),
            "cmp" => Ok(Mnemonic::Cmp),
            "sete" => Ok(Mnemonic::Sete),
            "setne" => Ok(Mnemonic::Setne),
            "setg" => Ok(Mnemonic::Setg),
            "setl" => Ok(Mnemonic::Setl),
            "setge" => Ok(Mnemonic::Setge),
            "setle" => Ok(Mnemonic::Setle),
            "neg" => Ok(Mnemonic::Neg),
            "cmove" => Ok(Mnemonic::Cmove),
            "cmovne" => Ok(Mnemonic::Cmovne),
            "div" => Ok(Mnemonic::Div),
            "idiv" => Ok(Mnemonic::Idiv),
            "sal" => Ok(Mnemonic::Sal),
            "shr" => Ok(Mnemonic::Shr),
            "sar" => Ok(Mnemonic::Sar),
            "movq" => Ok(Mnemonic::Movq),
            "movd" => Ok(Mnemonic::Movd),
            "movss" => Ok(Mnemonic::Movss),
            "movsd" => Ok(Mnemonic::Movsd),
            "movups" => Ok(Mnemonic::Movups),
            "movupd" => Ok(Mnemonic::Movupd),
            "addss" => Ok(Mnemonic::Addss),
            "addsd" => Ok(Mnemonic::Addsd),
            "divss" => Ok(Mnemonic::Divss),
            "divsd" => Ok(Mnemonic::Divsd),
            "mulss" => Ok(Mnemonic::Mulss),
            "mulsd" => Ok(Mnemonic::Mulsd),
            "subss" => Ok(Mnemonic::Subss),
            "subsd" => Ok(Mnemonic::Subsd),
            "ucomiss" => Ok(Mnemonic::Ucomiss),
            "ucomisd" => Ok(Mnemonic::Ucomisd),
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
            Mnemonic::Mul => "mul",
            Mnemonic::Imul => "imul",
            Mnemonic::Link => "",
            Mnemonic::StartOptimization => "",
            Mnemonic::EndOptimization => "",
            Mnemonic::Debug => "#",
            Mnemonic::Jne => "jne",
            Mnemonic::Je => "je",
            Mnemonic::Cmp => "cmp",
            Mnemonic::Sete => "sete",
            Mnemonic::Setg => "setg",
            Mnemonic::Setl => "setl",
            Mnemonic::Setge => "setge",
            Mnemonic::Setle => "setle",
            Mnemonic::Setne => "setne",
            Mnemonic::Neg => "neg",
            Mnemonic::Cmove => "cmove",
            Mnemonic::Cmovne => "cmovne",
            Mnemonic::Div => "div",
            Mnemonic::Idiv => "idiv",
            Mnemonic::Sal => "sal",
            Mnemonic::Shr => "shr",
            Mnemonic::Sar => "sar",
            Mnemonic::Movq => "movq",
            Mnemonic::Movd => "movd",
            Mnemonic::Movss=> "movss",
            Mnemonic::Movsd=> "movss",
            Mnemonic::Movups=> "movups",
            Mnemonic::Movupd => "movupd",
            Mnemonic::Addss => "addss",
            Mnemonic::Addsd => "addsd",
            Mnemonic::Divss => "divss",
            Mnemonic::Divsd => "divsd",
            Mnemonic::Mulss => "mulss",
            Mnemonic::Mulsd => "mulsd",
            Mnemonic::Subss => "subss",
            Mnemonic::Subsd => "subsd",
            Mnemonic::Ucomiss => "ucomiss",
            Mnemonic::Ucomisd => "ucomisd",
        })
    }
}

/// The operand type and value to use
#[derive(Debug, Clone, Eq)]
pub enum Operand {
    /// A number operand
    Imm(i64),
    /// A register operand
    Reg(x64Reg),
    /// A memory displacement
    Mem(MemOp),
    /// A link destination
    LinkDestination(String, i64),
    /// A link destination to a block
    BlockLinkDestination(String, i64),
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
            Operand::BlockLinkDestination(_, _) => "".to_string(),
            Operand::Debug(s) => s.to_string(),
        })
    }
}

/// A memory displacement
#[derive(Eq)]
pub struct MemOp {
    /// The base register
    pub base: Option<x64Reg>,
    /// The index register
    pub index: Option<x64Reg>,
    /// The scale
    pub scale: isize,
    /// The displacement
    pub displ: isize,
    /// rip relativ
    pub rip: bool,
}

impl MemOp {
    #[doc(hidden)]
    pub fn encode(&self, basis: Option<x64Reg>) -> (/*modrm mod*/u8, Vec<u8>) {
        let mut scale = match self.scale {
            0 => 0,
            1 => 0,
            2 => 1,
            4 => 2,
            8 => 3,
            _ => todo!("scale needs to be either 1/2/4/8")
        };

        let mut displ = vec![];

        if self.displ == 0 {
        } else if self.displ >= -128 && self.displ <= 127 {
            scale = 0b01;
            displ.push(self.displ as u8);
        } else {
            scale = 0b10;
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

        (0, encoding)
    }

    /// Returns the used rex prefix for the memory displacment
    pub fn rex(&self, front: bool) -> RexPrefix {
        let mut rex = RexPrefix::none();
        if let Some(base) = &self.base {
            let base = base.as_any().downcast_ref::<x64Reg>().unwrap();
            if front {
                rex.r = base.extended();
            } else {
                rex.b = base.extended();
            }
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
                if let Some(index) = &self.index { Some(*index) }
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
            base: Some(self),
            index: None,
            scale: 1,
            displ: rhs as isize,
            rip: false,
        }
    }
}

impl Add<x64Reg> for x64Reg {
    type Output = MemOp;

    fn add(self, rhs: x64Reg) -> Self::Output {
        MemOp {
            base: Some(self),
            index: Some(rhs),
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
            base: Some(self),
            index: None,
            scale: 1,
            displ: -(rhs as isize),
            rip: false,
        }
    }
}