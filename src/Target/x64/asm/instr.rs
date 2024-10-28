use std::{fmt::Display, ops::{Add, Sub}, str::FromStr};
use iced_x86::{BlockEncoder, BlockEncoderOptions, Code, Instruction, InstructionBlock, MemoryOperand, Register};
use object::RelocationEncoding;

use crate::CodeGen::MCInstr;
use crate::Obj::Link;
use crate::Support::{ColorClass, ColorProfile};
use crate::Target::x64::X64Reg;

/// The target instruction
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct X64MCInstr {
    /// The mnemonic to use
    pub mnemonic: Mnemonic,
    /// First operand
    pub op1: Option<Operand>,
    /// Second operand
    pub op2: Option<Operand>,
    /// Third operand
    pub op3: Option<Operand>,

    // for far calls
    pub(crate) far: bool,
}

impl X64MCInstr {
    /// Creates the instruction with 0 operands
    pub fn with0(mne: Mnemonic) -> Self {
        Self {
            mnemonic: mne,
            op1: None,
            op2: None,
            op3: None,
            far: false,
        }
    }

    /// Makes the instruction use far calls (for call and so on)
    pub fn make_far(&self) -> Self {
        Self {
            mnemonic: self.mnemonic.to_owned(),
            op1: self.op1.to_owned(),
            op2: self.op2.to_owned(),
            op3: self.op3.to_owned(),
            far: true,
        }
    }

    /// Creates the instruction with 1 operand
    pub fn with1(mne: Mnemonic, op: Operand) -> Self {
        Self {
            mnemonic: mne,
            op1: Some(op),
            op2: None,
            op3: None,
            far: false,
        }
    }

    /// Creates the instruction with 2 operands
    pub fn with2(mne: Mnemonic, op1: Operand, op2: Operand) -> Self {
        Self {
            mnemonic: mne,
            op1: Some(op1),
            op2: Some(op2),
            op3: None,
            far: false,
        }
    }

    /// Creates the instruction with 3 operands
    pub fn with3(mne: Mnemonic, op1: Operand, op2: Operand, op3: Operand) -> Self {
        Self {
            mnemonic: mne,
            op1: Some(op1),
            op2: Some(op2),
            op3: Some(op3),
            far: false,
        }
    }

    /// Encodes the instruction (some will say compiles)
    pub fn encode(&self) -> Result<(Vec<u8>, Option<Link>), Box<dyn std::error::Error>> {
        self.verify()?;


        if self.mnemonic == Mnemonic::Link {
            if let Some(Operand::LinkDestination(dst, addend)) = &self.op1 {
                return Ok((vec![], Some(Link { 
                    from: "".into(), 
                    to: dst.to_string(), 
                    at: 0, addend: *addend, 
                    special: false,
                    kind: RelocationEncoding::X86Branch,
                })));
            } else if let Some(Operand::BlockLinkDestination(dst, addend)) = &self.op1 {
                return Ok((vec![], Some(Link { 
                    from: "".into(), 
                    to: dst.to_string(), 
                    at: 0, addend: *addend, 
                    special: true,
                    kind: RelocationEncoding::Generic,
                 })));
            } else {
                return Ok((vec![], None));
            }
        }

        if Mnemonic::Debug == self.mnemonic || Mnemonic::StartOptimization == self.mnemonic || Mnemonic::EndOptimization == self.mnemonic {
            return Ok((vec![], None))
        }

        let /*mut*/ instr = match self.mnemonic {
            Mnemonic::Link | Mnemonic::Debug | Mnemonic::StartOptimization | Mnemonic::EndOptimization => unreachable!(),
            Mnemonic::Add => {
                if let Some(Operand::Reg(op1)) = &self.op1 {
                    if let Some(Operand::Reg(op2)) = &self.op2 {
                        if op1.is_gr8() {
                            Instruction::with2::<Register, Register>(Code::Add_rm8_r8, (*op1).into(), (*op2).into())?
                        } else if op1.is_gr16() {
                            Instruction::with2::<Register, Register>(Code::Add_rm16_r16, (*op1).into(), (*op2).into())?
                        } else if op1.is_gr32() {
                            Instruction::with2::<Register, Register>(Code::Add_rm32_r32, (*op1).into(), (*op2).into())?
                        } else if op1.is_gr64() {
                            Instruction::with2::<Register, Register>(Code::Add_rm64_r64, (*op1).into(), (*op2).into())?
                        } else { todo!("{}", self) }
                    } else if let Some(Operand::Mem(op2)) = &self.op2 {
                        if op1.is_gr8() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Add_r8_rm8, (*op1).into(), op2.into())?
                        } else if op1.is_gr16() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Add_r16_rm16, (*op1).into(), op2.into())?
                        } else if op1.is_gr32() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Add_r32_rm32, (*op1).into(), op2.into())?
                        } else if op1.is_gr64() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Add_r64_rm64, (*op1).into(), op2.into())?
                        } else { todo!("{}", self) }
                    } else if let Some(Operand::Imm(imm)) = &self.op2 {
                        if op1.is_gr8() {
                            Instruction::with2::<Register, i32>(Code::Add_rm8_imm8, (*op1).into(), *imm as i32)?
                        } else if op1.is_gr16() {
                            Instruction::with2::<Register, i32>(Code::Add_rm16_imm16, (*op1).into(), *imm as i32)?
                        } else if op1.is_gr32() {
                            Instruction::with2::<Register, i32>(Code::Add_rm32_imm32, (*op1).into(), *imm as i32)?
                        } else if op1.is_gr64() {
                            Instruction::with2::<Register, i32>(Code::Add_rm64_imm32, (*op1).into(), *imm as i32)?
                        } else { todo!("{}", self) }
                    } else { todo!("{}", self) }
                } else if let Some(Operand::Mem(op1)) = &self.op1 {
                    if let Some(Operand::Reg(op2)) = &self.op2 {
                        if op2.is_gr8() {
                            Instruction::with2::<MemoryOperand, Register>(Code::Add_rm8_r8, op1.into(), (*op2).into())?
                        } else if op2.is_gr16() {
                            Instruction::with2::<MemoryOperand, Register>(Code::Add_rm16_r16, op1.into(), (*op2).into())?
                        } else if op2.is_gr32() {
                            Instruction::with2::<MemoryOperand, Register>(Code::Add_rm32_r32, op1.into(), (*op2).into())?
                        } else if op2.is_gr64() {
                            Instruction::with2::<MemoryOperand, Register>(Code::Add_rm64_r64, op1.into(), (*op2).into())?
                        } else { todo!("{}", self) }
                    } else { todo!("{}", self) }
                } else { todo!("{}", self) }
            },
            Mnemonic::Adc => {
                if let Some(Operand::Reg(op1)) = &self.op1 {
                    if let Some(Operand::Reg(op2)) = &self.op2 {
                        if op1.is_gr8() {
                            Instruction::with2::<Register, Register>(Code::Adc_rm8_r8, (*op1).into(), (*op2).into())?
                        } else if op1.is_gr16() {
                            Instruction::with2::<Register, Register>(Code::Adc_rm16_r16, (*op1).into(), (*op2).into())?
                        } else if op1.is_gr32() {
                            Instruction::with2::<Register, Register>(Code::Adc_rm32_r32, (*op1).into(), (*op2).into())?
                        } else if op1.is_gr64() {
                            Instruction::with2::<Register, Register>(Code::Adc_rm64_r64, (*op1).into(), (*op2).into())?
                        } else { todo!("{}", self) }
                    } else if let Some(Operand::Mem(op2)) = &self.op2 {
                        if op1.is_gr8() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Adc_r8_rm8, (*op1).into(), op2.into())?
                        } else if op1.is_gr16() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Adc_r16_rm16, (*op1).into(), op2.into())?
                        } else if op1.is_gr32() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Adc_r32_rm32, (*op1).into(), op2.into())?
                        } else if op1.is_gr64() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Adc_r64_rm64, (*op1).into(), op2.into())?
                        } else { todo!("{}", self) }
                    } else if let Some(Operand::Imm(imm)) = &self.op2 {
                        if op1.is_gr8() {
                            Instruction::with2::<Register, i32>(Code::Adc_rm8_imm8, (*op1).into(), *imm as i32)?
                        } else if op1.is_gr16() {
                            Instruction::with2::<Register, i32>(Code::Adc_rm16_imm16, (*op1).into(), *imm as i32)?
                        } else if op1.is_gr32() {
                            Instruction::with2::<Register, i32>(Code::Adc_rm32_imm32, (*op1).into(), *imm as i32)?
                        } else if op1.is_gr64() {
                            Instruction::with2::<Register, i32>(Code::Adc_rm64_imm32, (*op1).into(), *imm as i32)?
                        } else { todo!("{}", self) }
                    } else { todo!("{}", self) }
                } else if let Some(Operand::Mem(op1)) = &self.op1 {
                    if let Some(Operand::Reg(op2)) = &self.op2 {
                        if op2.is_gr8() {
                            Instruction::with2::<MemoryOperand, Register>(Code::Adc_rm8_r8, op1.into(), (*op2).into())?
                        } else if op2.is_gr16() {
                            Instruction::with2::<MemoryOperand, Register>(Code::Adc_rm16_r16, op1.into(), (*op2).into())?
                        } else if op2.is_gr32() {
                            Instruction::with2::<MemoryOperand, Register>(Code::Adc_rm32_r32, op1.into(), (*op2).into())?
                        } else if op2.is_gr64() {
                            Instruction::with2::<MemoryOperand, Register>(Code::Adc_rm64_r64, op1.into(), (*op2).into())?
                        } else { todo!("{}", self) }
                    } else { todo!("{}", self) }
                } else { todo!("{}", self) }
            },
            Mnemonic::And => {
                if let Some(Operand::Reg(op1)) = &self.op1 {
                    if let Some(Operand::Reg(op2)) = &self.op2 {
                        if op1.is_gr8() {
                            Instruction::with2::<Register, Register>(Code::And_rm8_r8, (*op1).into(), (*op2).into())?
                        } else if op1.is_gr16() {
                            Instruction::with2::<Register, Register>(Code::And_rm16_r16, (*op1).into(), (*op2).into())?
                        } else if op1.is_gr32() {
                            Instruction::with2::<Register, Register>(Code::And_rm32_r32, (*op1).into(), (*op2).into())?
                        } else if op1.is_gr64() {
                            Instruction::with2::<Register, Register>(Code::And_rm64_r64, (*op1).into(), (*op2).into())?
                        } else { todo!("{}", self) }
                    } else if let Some(Operand::Mem(op2)) = &self.op2 {
                        if op1.is_gr8() {
                            Instruction::with2::<Register, MemoryOperand>(Code::And_r8_rm8, (*op1).into(), op2.into())?
                        } else if op1.is_gr16() {
                            Instruction::with2::<Register, MemoryOperand>(Code::And_r16_rm16, (*op1).into(), op2.into())?
                        } else if op1.is_gr32() {
                            Instruction::with2::<Register, MemoryOperand>(Code::And_r32_rm32, (*op1).into(), op2.into())?
                        } else if op1.is_gr64() {
                            Instruction::with2::<Register, MemoryOperand>(Code::And_r64_rm64, (*op1).into(), op2.into())?
                        } else { todo!("{}", self) }
                    } else if let Some(Operand::Imm(imm)) = &self.op2 {
                        if op1.is_gr8() {
                            Instruction::with2::<Register, i32>(Code::And_rm8_imm8, (*op1).into(), *imm as i32)?
                        } else if op1.is_gr16() {
                            Instruction::with2::<Register, i32>(Code::And_rm16_imm16, (*op1).into(), *imm as i32)?
                        } else if op1.is_gr32() {
                            Instruction::with2::<Register, i32>(Code::And_rm32_imm32, (*op1).into(), *imm as i32)?
                        } else if op1.is_gr64() {
                            Instruction::with2::<Register, i32>(Code::And_rm64_imm32, (*op1).into(), *imm as i32)?
                        } else { todo!("{}", self) }
                    } else { todo!("{}", self) }
                } else if let Some(Operand::Mem(op1)) = &self.op1 {
                    if let Some(Operand::Reg(op2)) = &self.op2 {
                        if op2.is_gr8() {
                            Instruction::with2::<MemoryOperand, Register>(Code::And_rm8_r8, op1.into(), (*op2).into())?
                        } else if op2.is_gr16() {
                            Instruction::with2::<MemoryOperand, Register>(Code::And_rm16_r16, op1.into(), (*op2).into())?
                        } else if op2.is_gr32() {
                            Instruction::with2::<MemoryOperand, Register>(Code::And_rm32_r32, op1.into(), (*op2).into())?
                        } else if op2.is_gr64() {
                            Instruction::with2::<MemoryOperand, Register>(Code::And_rm64_r64, op1.into(), (*op2).into())?
                        } else { todo!("{}", self) }
                    } else { todo!("{}", self) }
                } else { todo!("{}", self) }
            },
            Mnemonic::Or => {
                if let Some(Operand::Reg(op1)) = &self.op1 {
                    if let Some(Operand::Reg(op2)) = &self.op2 {
                        if op1.is_gr8() {
                            Instruction::with2::<Register, Register>(Code::Or_rm8_r8, (*op1).into(), (*op2).into())?
                        } else if op1.is_gr16() {
                            Instruction::with2::<Register, Register>(Code::Or_rm16_r16, (*op1).into(), (*op2).into())?
                        } else if op1.is_gr32() {
                            Instruction::with2::<Register, Register>(Code::Or_rm32_r32, (*op1).into(), (*op2).into())?
                        } else if op1.is_gr64() {
                            Instruction::with2::<Register, Register>(Code::Or_rm64_r64, (*op1).into(), (*op2).into())?
                        } else { todo!("{}", self) }
                    } else if let Some(Operand::Mem(op2)) = &self.op2 {
                        if op1.is_gr8() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Or_r8_rm8, (*op1).into(), op2.into())?
                        } else if op1.is_gr16() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Or_r16_rm16, (*op1).into(), op2.into())?
                        } else if op1.is_gr32() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Or_r32_rm32, (*op1).into(), op2.into())?
                        } else if op1.is_gr64() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Or_r64_rm64, (*op1).into(), op2.into())?
                        } else { todo!("{}", self) }
                    } else if let Some(Operand::Imm(imm)) = &self.op2 {
                        if op1.is_gr8() {
                            Instruction::with2::<Register, i32>(Code::Or_rm8_imm8, (*op1).into(), *imm as i32)?
                        } else if op1.is_gr16() {
                            Instruction::with2::<Register, i32>(Code::Or_rm16_imm16, (*op1).into(), *imm as i32)?
                        } else if op1.is_gr32() {
                            Instruction::with2::<Register, i32>(Code::Or_rm32_imm32, (*op1).into(), *imm as i32)?
                        } else if op1.is_gr64() {
                            Instruction::with2::<Register, i32>(Code::Or_rm64_imm32, (*op1).into(), *imm as i32)?
                        } else { todo!("{}", self) }
                    } else { todo!("{}", self) }
                } else if let Some(Operand::Mem(op1)) = &self.op1 {
                    if let Some(Operand::Reg(op2)) = &self.op2 {
                        if op2.is_gr8() {
                            Instruction::with2::<MemoryOperand, Register>(Code::Or_rm8_r8, op1.into(), (*op2).into())?
                        } else if op2.is_gr16() {
                            Instruction::with2::<MemoryOperand, Register>(Code::Or_rm16_r16, op1.into(), (*op2).into())?
                        } else if op2.is_gr32() {
                            Instruction::with2::<MemoryOperand, Register>(Code::Or_rm32_r32, op1.into(), (*op2).into())?
                        } else if op2.is_gr64() {
                            Instruction::with2::<MemoryOperand, Register>(Code::Or_rm64_r64, op1.into(), (*op2).into())?
                        } else { todo!("{}", self) }
                    } else { todo!("{}", self) }
                } else { todo!("{}", self) }
            },
            Mnemonic::Xor => {
                if let Some(Operand::Reg(op1)) = &self.op1 {
                    if let Some(Operand::Reg(op2)) = &self.op2 {
                        if op1.is_gr8() {
                            Instruction::with2::<Register, Register>(Code::Xor_rm8_r8, (*op1).into(), (*op2).into())?
                        } else if op1.is_gr16() {
                            Instruction::with2::<Register, Register>(Code::Xor_rm16_r16, (*op1).into(), (*op2).into())?
                        } else if op1.is_gr32() {
                            Instruction::with2::<Register, Register>(Code::Xor_rm32_r32, (*op1).into(), (*op2).into())?
                        } else if op1.is_gr64() {
                            Instruction::with2::<Register, Register>(Code::Xor_rm64_r64, (*op1).into(), (*op2).into())?
                        } else { todo!("{}", self) }
                    } else if let Some(Operand::Mem(op2)) = &self.op2 {
                        if op1.is_gr8() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Xor_r8_rm8, (*op1).into(), op2.into())?
                        } else if op1.is_gr16() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Xor_r16_rm16, (*op1).into(), op2.into())?
                        } else if op1.is_gr32() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Xor_r32_rm32, (*op1).into(), op2.into())?
                        } else if op1.is_gr64() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Xor_r64_rm64, (*op1).into(), op2.into())?
                        } else { todo!("{}", self) }
                    } else if let Some(Operand::Imm(imm)) = &self.op2 {
                        if op1.is_gr8() {
                            Instruction::with2::<Register, i32>(Code::Xor_rm8_imm8, (*op1).into(), *imm as i32)?
                        } else if op1.is_gr16() {
                            Instruction::with2::<Register, i32>(Code::Xor_rm16_imm16, (*op1).into(), *imm as i32)?
                        } else if op1.is_gr32() {
                            Instruction::with2::<Register, i32>(Code::Xor_rm32_imm32, (*op1).into(), *imm as i32)?
                        } else if op1.is_gr64() {
                            Instruction::with2::<Register, i32>(Code::Xor_rm64_imm32, (*op1).into(), *imm as i32)?
                        } else { todo!("{}", self) }
                    } else { todo!("{}", self) }
                } else if let Some(Operand::Mem(op1)) = &self.op1 {
                    if let Some(Operand::Reg(op2)) = &self.op2 {
                        if op2.is_gr8() {
                            Instruction::with2::<MemoryOperand, Register>(Code::Xor_rm8_r8, op1.into(), (*op2).into())?
                        } else if op2.is_gr16() {
                            Instruction::with2::<MemoryOperand, Register>(Code::Xor_rm16_r16, op1.into(), (*op2).into())?
                        } else if op2.is_gr32() {
                            Instruction::with2::<MemoryOperand, Register>(Code::Xor_rm32_r32, op1.into(), (*op2).into())?
                        } else if op2.is_gr64() {
                            Instruction::with2::<MemoryOperand, Register>(Code::Xor_rm64_r64, op1.into(), (*op2).into())?
                        } else { todo!("{}", self) }
                    } else { todo!("{}", self) }
                } else { todo!("{}", self) }
            },
            Mnemonic::Sub => {
                if let Some(Operand::Reg(op1)) = &self.op1 {
                    if let Some(Operand::Reg(op2)) = &self.op2 {
                        if op1.is_gr8() {
                            Instruction::with2::<Register, Register>(Code::Sub_rm8_r8, (*op1).into(), (*op2).into())?
                        } else if op1.is_gr16() {
                            Instruction::with2::<Register, Register>(Code::Sub_rm16_r16, (*op1).into(), (*op2).into())?
                        } else if op1.is_gr32() {
                            Instruction::with2::<Register, Register>(Code::Sub_rm32_r32, (*op1).into(), (*op2).into())?
                        } else if op1.is_gr64() {
                            Instruction::with2::<Register, Register>(Code::Sub_rm64_r64, (*op1).into(), (*op2).into())?
                        } else { todo!("{}", self) }
                    } else if let Some(Operand::Mem(op2)) = &self.op2 {
                        if op1.is_gr8() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Sub_r8_rm8, (*op1).into(), op2.into())?
                        } else if op1.is_gr16() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Sub_r16_rm16, (*op1).into(), op2.into())?
                        } else if op1.is_gr32() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Sub_r32_rm32, (*op1).into(), op2.into())?
                        } else if op1.is_gr64() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Sub_r64_rm64, (*op1).into(), op2.into())?
                        } else { todo!("{}", self) }
                    } else if let Some(Operand::Imm(imm)) = &self.op2 {
                        if op1.is_gr8() {
                            Instruction::with2::<Register, i32>(Code::Sub_rm8_imm8, (*op1).into(), *imm as i32)?
                        } else if op1.is_gr16() {
                            Instruction::with2::<Register, i32>(Code::Sub_rm16_imm16, (*op1).into(), *imm as i32)?
                        } else if op1.is_gr32() {
                            Instruction::with2::<Register, i32>(Code::Sub_rm32_imm32, (*op1).into(), *imm as i32)?
                        } else if op1.is_gr64() {
                            Instruction::with2::<Register, i32>(Code::Sub_rm64_imm32, (*op1).into(), *imm as i32)?
                        } else { todo!("{}", self) }
                    } else { todo!("{}", self) }
                } else if let Some(Operand::Mem(op1)) = &self.op1 {
                    if let Some(Operand::Reg(op2)) = &self.op2 {
                        if op2.is_gr8() {
                            Instruction::with2::<MemoryOperand, Register>(Code::Sub_rm8_r8, op1.into(), (*op2).into())?
                        } else if op2.is_gr16() {
                            Instruction::with2::<MemoryOperand, Register>(Code::Sub_rm16_r16, op1.into(), (*op2).into())?
                        } else if op2.is_gr32() {
                            Instruction::with2::<MemoryOperand, Register>(Code::Sub_rm32_r32, op1.into(), (*op2).into())?
                        } else if op2.is_gr64() {
                            Instruction::with2::<MemoryOperand, Register>(Code::Sub_rm64_r64, op1.into(), (*op2).into())?
                        } else { todo!("{}", self) }
                    } else { todo!("{}", self) }
                } else { todo!("{}", self) }
            },
            Mnemonic::Neg => {
                if let Some(Operand::Reg(op1)) = &self.op1 {
                    if op1.is_gr64() {
                        Instruction::with1::<Register>(Code::Neg_rm64, (*op1).into())?
                    } else if op1.is_gr32() {
                        Instruction::with1::<Register>(Code::Neg_rm32, (*op1).into())?
                    } else if op1.is_gr16() {
                        Instruction::with1::<Register>(Code::Neg_rm16, (*op1).into())?
                    } else if op1.is_gr8() {
                        Instruction::with1::<Register>(Code::Neg_rm8, (*op1).into())?
                    } else { todo!("{}", self) }
                } else if let Some(Operand::Mem(op1)) = &self.op1 {
                    Instruction::with1::<MemoryOperand>(Code::Neg_rm64, op1.into())?
                } else { todo!("{}", self) }
            },
            Mnemonic::Cmp => {
                if let Some(Operand::Reg(op1)) = &self.op1 {
                    if let Some(Operand::Reg(op2)) = &self.op2 {
                        if op1.is_gr8() {
                            Instruction::with2::<Register, Register>(Code::Cmp_rm8_r8, (*op1).into(), (*op2).into())?
                        } else if op1.is_gr16() {
                            Instruction::with2::<Register, Register>(Code::Cmp_rm16_r16, (*op1).into(), (*op2).into())?
                        } else if op1.is_gr32() {
                            Instruction::with2::<Register, Register>(Code::Cmp_rm32_r32, (*op1).into(), (*op2).into())?
                        } else if op1.is_gr64() {
                            Instruction::with2::<Register, Register>(Code::Cmp_rm64_r64, (*op1).into(), (*op2).into())?
                        } else { todo!("{}", self) }
                    } else if let Some(Operand::Mem(op2)) = &self.op2 {
                        if op1.is_gr8() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Cmp_r8_rm8, (*op1).into(), op2.into())?
                        } else if op1.is_gr16() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Cmp_r16_rm16, (*op1).into(), op2.into())?
                        } else if op1.is_gr32() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Cmp_r32_rm32, (*op1).into(), op2.into())?
                        } else if op1.is_gr64() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Cmp_r64_rm64, (*op1).into(), op2.into())?
                        } else { todo!("{}", self) }
                    } else if let Some(Operand::Imm(imm)) = &self.op2 {
                        if op1.is_gr8() {
                            Instruction::with2::<Register, i32>(Code::Cmp_rm8_imm8, (*op1).into(), *imm as i32)?
                        } else if op1.is_gr16() {
                            Instruction::with2::<Register, i32>(Code::Cmp_rm16_imm16, (*op1).into(), *imm as i32)?
                        } else if op1.is_gr32() {
                            Instruction::with2::<Register, i32>(Code::Cmp_rm32_imm32, (*op1).into(), *imm as i32)?
                        } else if op1.is_gr64() {
                            Instruction::with2::<Register, i32>(Code::Cmp_rm64_imm32, (*op1).into(), *imm as i32)?
                        } else { todo!("{}", self) }
                    } else { todo!("{}", self) }
                } else if let Some(Operand::Mem(op1)) = &self.op1 {
                    if let Some(Operand::Reg(op2)) = &self.op2 {
                        if op2.is_gr8() {
                            Instruction::with2::<MemoryOperand, Register>(Code::Cmp_rm8_r8, op1.into(), (*op2).into())?
                        } else if op2.is_gr16() {
                            Instruction::with2::<MemoryOperand, Register>(Code::Cmp_rm16_r16, op1.into(), (*op2).into())?
                        } else if op2.is_gr32() {
                            Instruction::with2::<MemoryOperand, Register>(Code::Cmp_rm32_r32, op1.into(), (*op2).into())?
                        } else if op2.is_gr64() {
                            Instruction::with2::<MemoryOperand, Register>(Code::Cmp_rm64_r64, op1.into(), (*op2).into())?
                        } else { todo!("{}", self) }
                    } else { todo!("{}", self) }
                } else { todo!("{}", self) }
            },
            Mnemonic::Lea => {
                if let Some(Operand::Reg(op1)) = &self.op1 {
                    if let Some(Operand::Mem(op2)) = &self.op2 {
                        if op1.is_gr16() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Lea_r16_m, (*op1).into(), op2.into())?
                        } else if op1.is_gr32() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Lea_r32_m, (*op1).into(), op2.into())?
                        } else if op1.is_gr64() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Lea_r64_m, (*op1).into(), op2.into())?
                        } else { todo!("{}", self) }
                    } else if let Some(Operand::RipRelative(_)) = &self.op2 {
                        Instruction::with2::<Register, MemoryOperand>(Code::Lea_r64_m, (*op1).into(), MemoryOperand::with_base_displ(Register::RIP, 7))?
                    } else { todo!("{}", self) }
                } else { todo!("{}", self) }
            },
            Mnemonic::Mov => {
                if let Some(Operand::Reg(op1)) = &self.op1 {
                    if let Some(Operand::Reg(op2)) = &self.op2 {
                        if op1.is_gr8() {
                            Instruction::with2::<Register, Register>(Code::Mov_rm8_r8, (*op1).into(), (*op2).into())?
                        } else if op1.is_gr16() {
                            Instruction::with2::<Register, Register>(Code::Mov_rm16_r16, (*op1).into(), (*op2).into())?
                        } else if op1.is_gr32() {
                            Instruction::with2::<Register, Register>(Code::Mov_rm32_r32, (*op1).into(), (*op2).into())?
                        } else if op1.is_gr64() {
                            Instruction::with2::<Register, Register>(Code::Mov_rm64_r64, (*op1).into(), (*op2).into())?
                        } else { todo!("{}", self) }
                    } else if let Some(Operand::Mem(op2)) = &self.op2 {
                        if op1.is_gr8() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Mov_r8_rm8, (*op1).into(), op2.into())?
                        } else if op1.is_gr16() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Mov_r16_rm16, (*op1).into(), op2.into())?
                        } else if op1.is_gr32() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Mov_r32_rm32, (*op1).into(), op2.into())?
                        } else if op1.is_gr64() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Mov_r64_rm64, (*op1).into(), op2.into())?
                        } else { todo!("{}", self) }
                    } else if let Some(Operand::Imm(imm)) = &self.op2 {
                        if op1.is_gr8() {
                            Instruction::with2::<Register, i32>(Code::Mov_rm8_imm8, (*op1).into(), *imm as i32)?
                        } else if op1.is_gr16() {
                            Instruction::with2::<Register, i32>(Code::Mov_rm16_imm16, (*op1).into(), *imm as i32)?
                        } else if op1.is_gr32() {
                            Instruction::with2::<Register, i32>(Code::Mov_rm32_imm32, (*op1).into(), *imm as i32)?
                        } else if op1.is_gr64() {
                            Instruction::with2::<Register, i32>(Code::Mov_rm64_imm32, (*op1).into(), *imm as i32)?
                        } else { todo!("{}", self) }
                    } else { todo!("{}", self) }
                } else if let Some(Operand::Mem(op1)) = &self.op1 {
                    if let Some(Operand::Reg(op2)) = &self.op2 {
                        if op2.is_gr8() {
                            Instruction::with2::<MemoryOperand, Register>(Code::Mov_rm8_r8, op1.into(), (*op2).into())?
                        } else if op2.is_gr16() {
                            Instruction::with2::<MemoryOperand, Register>(Code::Mov_rm16_r16, op1.into(), (*op2).into())?
                        } else if op2.is_gr32() {
                            Instruction::with2::<MemoryOperand, Register>(Code::Mov_rm32_r32, op1.into(), (*op2).into())?
                        } else if op2.is_gr64() {
                            Instruction::with2::<MemoryOperand, Register>(Code::Mov_rm64_r64, op1.into(), (*op2).into())?
                        } else { todo!("{}", self) }
                    } else { todo!("{}", self) }
                } else { todo!("{}", self) }
            },
            Mnemonic::Movzx => {
                if let Some(Operand::Reg(op1)) = &self.op1 {
                    if let Some(Operand::Reg(op2)) = &self.op2 {
                        if op1.is_gr64() {
                            if op2.is_gr16() {
                                Instruction::with2::<Register, Register>(Code::Movzx_r64_rm16, (*op1).into(), (*op2).into())?
                            } else if op2.is_gr8() {
                                Instruction::with2::<Register, Register>(Code::Movzx_r64_rm8, (*op1).into(), (*op2).into())?
                            } else { todo!("{}", self) }
                        } else if op1.is_gr32() {
                            if op2.is_gr16() {
                                Instruction::with2::<Register, Register>(Code::Movzx_r32_rm16, (*op1).into(), (*op2).into())?
                            } else if op2.is_gr8() {
                                Instruction::with2::<Register, Register>(Code::Movzx_r32_rm8, (*op1).into(), (*op2).into())?
                            } else { todo!("{}", self) }
                        } else if op1.is_gr16() {
                            if op2.is_gr16() {
                                Instruction::with2::<Register, Register>(Code::Movzx_r32_rm16, (*op1).into(), (*op2).into())?
                            } else if op2.is_gr8() {
                                Instruction::with2::<Register, Register>(Code::Movzx_r32_rm8, (*op1).into(), (*op2).into())?
                            } else { todo!("{}", self) }
                        } else { todo!("{}", self)}
                    } else if let Some(Operand::Mem(op2)) = &self.op2 {
                        if op1.is_gr64() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Movzx_r64_rm16, (*op1).into(), op2.into())?
                        } else if op1.is_gr32() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Movzx_r32_rm16, (*op1).into(), op2.into())?
                        } else if op1.is_gr16() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Movzx_r16_rm16, (*op1).into(), op2.into())?
                        } else {todo!("{}", self) } 
                    } else { todo!("{}", self)}
                } else { todo!("{}", self) }
            },
            Mnemonic::Push => {
                if let Some(Operand::Reg(op1)) = &self.op1 {
                    if op1.is_gr64() {
                        Instruction::with1::<Register>(Code::Push_r64, (*op1).into())?
                    } else if op1.is_gr16() {
                        Instruction::with1::<Register>(Code::Push_r16, (*op1).into())?
                    } else { todo!("{}", self)}
                } else if let Some(Operand::Mem(op1)) = &self.op1 {
                    Instruction::with1::<MemoryOperand>(Code::Push_rm64, op1.into())?
                } else if let Some(Operand::Imm(imm)) = &self.op1 {
                    Instruction::with1(Code::Pushd_imm32, *imm as i32)?
                } else { todo!("{}", self) }
            },
            Mnemonic::Pop => {
                if let Some(Operand::Reg(op1)) = &self.op1 {
                    if op1.is_gr64() {
                        Instruction::with1::<Register>(Code::Pop_r64, (*op1).into())?
                    } else if op1.is_gr32() {
                        Instruction::with1::<Register>(Code::Pop_r32, (*op1).into())?
                    } else if op1.is_gr16() {
                        Instruction::with1::<Register>(Code::Pop_r16, (*op1).into())?
                    } else { todo!("{}", self)}
                } else if let Some(Operand::Mem(op1)) = &self.op1 {
                    Instruction::with1::<MemoryOperand>(Code::Pop_rm64, op1.into())?
                } else { todo!("{}", self) }
            },
            Mnemonic::Ret => Instruction::with(Code::Retnq),
            Mnemonic::Imul => {
                if let Some(Operand::Reg(op1)) = &self.op1 {
                    if let Some(Operand::Reg(op2)) = &self.op2 {
                        if let Some(Operand::Imm(op3)) = &self.op3 {
                            if op1.is_gr16() {
                                Instruction::with3::<Register, Register, i32>(Code::Imul_r16_rm16_imm16, (*op1).into(), (*op2).into(), *op3 as i32)?
                            } else if op1.is_gr32() {
                                Instruction::with3::<Register, Register, i32>(Code::Imul_r32_rm32_imm32, (*op1).into(), (*op2).into(), *op3 as i32)?
                            } else if op1.is_gr64() {
                                Instruction::with3::<Register, Register, i32>(Code::Imul_r64_rm64_imm32, (*op1).into(), (*op2).into(), *op3 as i32)?
                            } else { todo!("{}", self)}
                        } else if op1.is_gr64() {
                            Instruction::with2::<Register, Register>(Code::Imul_r64_rm64, (*op1).into(), (*op2).into())?
                        } else if op1.is_gr32() {
                            Instruction::with2::<Register, Register>(Code::Imul_r32_rm32, (*op1).into(), (*op2).into())?
                        } else if op1.is_gr16() {
                            Instruction::with2::<Register, Register>(Code::Imul_r16_rm16, (*op1).into(), (*op2).into())?
                        } else { todo!("{}", self)}
                    } else if let Some(Operand::Mem(op2)) = &self.op2 {
                        if let Some(Operand::Imm(op3)) = &self.op3 {
                            if op1.is_gr16() {
                                Instruction::with3::<Register, MemoryOperand, i32>(Code::Imul_r16_rm16_imm16, (*op1).into(), op2.into(), *op3 as i32)?
                            } else if op1.is_gr32() {
                                Instruction::with3::<Register, MemoryOperand, i32>(Code::Imul_r32_rm32_imm32, (*op1).into(), op2.into(), *op3 as i32)?
                            } else if op1.is_gr64() {
                                Instruction::with3::<Register, MemoryOperand, i32>(Code::Imul_r64_rm64_imm32, (*op1).into(), op2.into(), *op3 as i32)?
                            } else { todo!("{}", self)}
                        } else if op1.is_gr64() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Imul_r64_rm64, (*op1).into(), op2.into())?
                        } else if op1.is_gr32() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Imul_r32_rm32, (*op1).into(), op2.into())?
                        } else if op1.is_gr16() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Imul_r16_rm16, (*op1).into(), op2.into())?
                        } else { todo!("{}", self)}
                    } else {
                        if op1.is_gr64() {
                            Instruction::with1::<Register>(Code::Imul_rm64, (*op1).into())?
                        } else if op1.is_gr32() {
                            Instruction::with1::<Register>(Code::Imul_rm32, (*op1).into())?
                        } else if op1.is_gr16() {
                            Instruction::with1::<Register>(Code::Imul_rm16, (*op1).into())?
                        } else if op1.is_gr8() {
                            Instruction::with1::<Register>(Code::Imul_rm8, (*op1).into())?
                        } else { todo!("{}", self)}
                    }
                } else if let Some(Operand::Mem(op1)) = &self.op1 {
                    Instruction::with1::<MemoryOperand>(Code::Imul_rm64, op1.into())?
                } else { todo!("{}", self) }
            },
            Mnemonic::Mul => {
                if let Some(Operand::Reg(op1)) = &self.op1 {
                    if op1.is_gr64() {
                        Instruction::with1::<Register>(Code::Mul_rm64, (*op1).into())?
                    } else if op1.is_gr32() {
                        Instruction::with1::<Register>(Code::Mul_rm32, (*op1).into())?
                    } else if op1.is_gr16() {
                        Instruction::with1::<Register>(Code::Mul_rm16, (*op1).into())?
                    } else if op1.is_gr8() {
                        Instruction::with1::<Register>(Code::Mul_rm8, (*op1).into())?
                    } else { todo!("{}", self)}
                } else if let Some(Operand::Mem(op1)) = &self.op1 {
                    Instruction::with1::<MemoryOperand>(Code::Mul_rm64, op1.into())?
                } else { todo!("{}", self) }
            },
            Mnemonic::Idiv => {
                if let Some(Operand::Reg(op1)) = &self.op1 {
                    if op1.is_gr64() {
                        Instruction::with1::<Register>(Code::Idiv_rm64, (*op1).into())?
                    } else if op1.is_gr32() {
                        Instruction::with1::<Register>(Code::Idiv_rm32, (*op1).into())?
                    } else if op1.is_gr16() {
                        Instruction::with1::<Register>(Code::Idiv_rm16, (*op1).into())?
                    } else if op1.is_gr8() {
                        Instruction::with1::<Register>(Code::Idiv_rm8, (*op1).into())?
                    } else { todo!("{}", self)}
                } else if let Some(Operand::Mem(op1)) = &self.op1 {
                    Instruction::with1::<MemoryOperand>(Code::Idiv_rm64, op1.into())?
                } else { todo!("{}", self) }
            },
            Mnemonic::Div => {
                if let Some(Operand::Reg(op1)) = &self.op1 {
                    if op1.is_gr64() {
                        Instruction::with1::<Register>(Code::Div_rm64, (*op1).into())?
                    } else if op1.is_gr32() {
                        Instruction::with1::<Register>(Code::Div_rm32, (*op1).into())?
                    } else if op1.is_gr16() {
                        Instruction::with1::<Register>(Code::Div_rm16, (*op1).into())?
                    } else if op1.is_gr8() {
                        Instruction::with1::<Register>(Code::Div_rm8, (*op1).into())?
                    } else { todo!("{}", self)}
                } else if let Some(Operand::Mem(op1)) = &self.op1 {
                    Instruction::with1::<MemoryOperand>(Code::Div_rm64, op1.into())?
                } else { todo!("{}", self) }
            },
            Mnemonic::Call => {
                if let Some(Operand::Reg(op1)) = &self.op1 {
                    if op1.is_gr64() {
                        Instruction::with1::<Register>(Code::Call_rm64, (*op1).into())?
                    } else if op1.is_gr32() {
                        Instruction::with1::<Register>(Code::Call_rm32, (*op1).into())?
                    } else if op1.is_gr16() {
                        Instruction::with1::<Register>(Code::Call_rm16, (*op1).into())?
                    } else { todo!("{}", self) }
                } else if let Some(Operand::Mem(op1)) = &self.op1 {
                    Instruction::with1::<MemoryOperand>(Code::Call_rm64, op1.into())?
                } else if let Some(Operand::Imm(op1)) = &self.op1 {
                    if self.far {
                        Instruction::with_far_branch(Code::Call_m1664, 0x33, *op1 as u32)?
                    } else {
                        Instruction::with_branch(Code::Call_rel32_64, *op1 as u64)?
                    }
                } else if let Some(Operand::LinkDestination(..)) = &self.op1 {
                    Instruction::with_branch(Code::Call_rel32_64, 5)?
                } else { todo!("{}", self) }
            },
            Mnemonic::Jmp => {
                if let Some(Operand::Reg(op1)) = &self.op1 {
                    if op1.is_gr64() {
                        Instruction::with1::<Register>(Code::Jmp_rm64, (*op1).into())?
                    } else if op1.is_gr32() {
                        Instruction::with1::<Register>(Code::Jmp_rm32, (*op1).into())?
                    } else if op1.is_gr16() {
                        Instruction::with1::<Register>(Code::Jmp_rm16, (*op1).into())?
                    } else { todo!("{}", self) }
                } else if let Some(Operand::Mem(op1)) = &self.op1 {
                    Instruction::with1::<MemoryOperand>(Code::Jmp_rm64, op1.into())?
                } else if let Some(Operand::Imm(op1)) = &self.op1 {
                    if self.far {
                        Instruction::with_far_branch(Code::Jmp_m1664, 0x33, *op1 as u32)?
                    } else {
                        Instruction::with_branch(Code::Jmp_rel32_64, *op1 as u64)?
                    }
                } else if let Some(Operand::LinkDestination(..)) = &self.op1 {
                    Instruction::with_branch(Code::Jmp_rel32_64, 0)?
                } else if let Some(Operand::BlockLinkDestination(..)) = &self.op1 {
                    Instruction::with_branch(Code::Jmp_rel32_64, 0)?
                } else { todo!("{}", self) }
            },
            Mnemonic::Jne => {
                if let Some(Operand::Imm(op1)) = &self.op1 {
                    Instruction::with_branch(Code::Jne_rel32_64, *op1 as u64)?
                } else if let Some(Operand::LinkDestination(..)) = &self.op1 {
                    Instruction::with_branch(Code::Jne_rel32_64, 0)?
                } else if let Some(Operand::BlockLinkDestination(..)) = &self.op1 {
                    Instruction::with_branch(Code::Jne_rel32_64, 0)?
                } else { todo!("{}", self) }
            },
            Mnemonic::Je => {
                if let Some(Operand::Imm(op1)) = &self.op1 {
                    Instruction::with_branch(Code::Je_rel32_64, *op1 as u64)?
                } else if let Some(Operand::LinkDestination(..)) = &self.op1 {
                    Instruction::with_branch(Code::Je_rel32_64, 0)?
                } else if let Some(Operand::BlockLinkDestination(..)) = &self.op1 {
                    Instruction::with_branch(Code::Je_rel32_64, 0)?
                } else { todo!("{}", self) }
            },
            Mnemonic::Jge => {
                if let Some(Operand::Imm(op1)) = &self.op1 {
                    Instruction::with_branch(Code::Jge_rel32_64, *op1 as u64)?
                } else if let Some(Operand::LinkDestination(..)) = &self.op1 {
                    Instruction::with_branch(Code::Jge_rel32_64, 0)?
                } else if let Some(Operand::BlockLinkDestination(..)) = &self.op1 {
                    Instruction::with_branch(Code::Jge_rel32_64, 0)?
                } else { todo!("{}", self) }
            },
            Mnemonic::Jl => {
                if let Some(Operand::Imm(op1)) = &self.op1 {
                    Instruction::with_branch(Code::Jl_rel32_64, *op1 as u64)?
                } else if let Some(Operand::LinkDestination(..)) = &self.op1 {
                    Instruction::with_branch(Code::Jl_rel32_64, 0)?
                } else if let Some(Operand::BlockLinkDestination(..)) = &self.op1 {
                    Instruction::with_branch(Code::Jl_rel32_64, 0)?
                } else { todo!("{}", self) }
            },
            Mnemonic::Jle => {
                if let Some(Operand::Imm(op1)) = &self.op1 {
                    Instruction::with_branch(Code::Jle_rel32_64, *op1 as u64)?
                } else if let Some(Operand::LinkDestination(..)) = &self.op1 {
                    Instruction::with_branch(Code::Jle_rel32_64, 0)?
                } else if let Some(Operand::BlockLinkDestination(..)) = &self.op1 {
                    Instruction::with_branch(Code::Jle_rel32_64, 0)?
                } else { todo!("{}", self) }
            },
            Mnemonic::Jg => {
                if let Some(Operand::Imm(op1)) = &self.op1 {
                    Instruction::with_branch(Code::Jg_rel32_64, *op1 as u64)?
                } else if let Some(Operand::LinkDestination(..)) = &self.op1 {
                    Instruction::with_branch(Code::Jg_rel32_64, 0)?
                } else if let Some(Operand::BlockLinkDestination(..)) = &self.op1 {
                    Instruction::with_branch(Code::Jg_rel32_64, 0)?
                } else { todo!("{}", self) }
            },
            Mnemonic::Endbr64 => Instruction::with(Code::Endbr64),
            Mnemonic::Sete => {
                if let Some(Operand::Reg(op1)) = &self.op1 {
                    Instruction::with1::<Register>(Code::Sete_rm8, (*op1).into())?
                } else if let Some(Operand::Mem(op1)) = &self.op1 {
                    Instruction::with1::<MemoryOperand>(Code::Sete_rm8, op1.into())?
                } else { todo!("{}", self) }
            },
            Mnemonic::Setne => {
                if let Some(Operand::Reg(op1)) = &self.op1 {
                    Instruction::with1::<Register>(Code::Setne_rm8, (*op1).into())?
                } else if let Some(Operand::Mem(op1)) = &self.op1 {
                    Instruction::with1::<MemoryOperand>(Code::Setne_rm8, op1.into())?
                } else { todo!("{}", self) }
            },
            Mnemonic::Setg => {
                if let Some(Operand::Reg(op1)) = &self.op1 {
                    Instruction::with1::<Register>(Code::Setg_rm8, (*op1).into())?
                } else if let Some(Operand::Mem(op1)) = &self.op1 {
                    Instruction::with1::<MemoryOperand>(Code::Setg_rm8, op1.into())?
                } else { todo!("{}", self) }
            },
            Mnemonic::Setl => {
                if let Some(Operand::Reg(op1)) = &self.op1 {
                    Instruction::with1::<Register>(Code::Setl_rm8, (*op1).into())?
                } else if let Some(Operand::Mem(op1)) = &self.op1 {
                    Instruction::with1::<MemoryOperand>(Code::Setl_rm8, op1.into())?
                } else { todo!("{}", self) }
            },
            Mnemonic::Setge => {
                if let Some(Operand::Reg(op1)) = &self.op1 {
                    Instruction::with1::<Register>(Code::Setge_rm8, (*op1).into())?
                } else if let Some(Operand::Mem(op1)) = &self.op1 {
                    Instruction::with1::<MemoryOperand>(Code::Setge_rm8, op1.into())?
                } else { todo!("{}", self) }
            },
            Mnemonic::Setle => {
                if let Some(Operand::Reg(op1)) = &self.op1 {
                    Instruction::with1::<Register>(Code::Setle_rm8, (*op1).into())?
                } else if let Some(Operand::Mem(op1)) = &self.op1 {
                    Instruction::with1::<MemoryOperand>(Code::Setle_rm8, op1.into())?
                } else { todo!("{}", self) }
            },
            Mnemonic::Cmove => {
                if let Some(Operand::Reg(op1)) = &self.op1 {
                    if let Some(Operand::Reg(op2)) = &self.op2 {
                        if op1.is_gr16() {
                            Instruction::with2::<Register, Register>(Code::Cmove_r16_rm16, (*op1).into(), (*op2).into())?
                        } else if op1.is_gr32() {
                            Instruction::with2::<Register, Register>(Code::Cmove_r32_rm32, (*op1).into(), (*op2).into())?
                        } else if op1.is_gr64() {
                            Instruction::with2::<Register, Register>(Code::Cmove_r64_rm64, (*op1).into(), (*op2).into())?
                        } else { todo!("{}", self) }
                    } else if let Some(Operand::Mem(op2)) = &self.op2 {
                        if op1.is_gr16() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Cmove_r16_rm16, (*op1).into(), op2.into())?
                        } else if op1.is_gr32() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Cmove_r32_rm32, (*op1).into(), op2.into())?
                        } else if op1.is_gr64() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Cmove_r64_rm64, (*op1).into(), op2.into())?
                        } else { todo!("{}", self) }
                    } else { todo!("{}", self) }
                } else { todo!("{}", self) }
            },
            Mnemonic::Cmovne => {
                if let Some(Operand::Reg(op1)) = &self.op1 {
                    if let Some(Operand::Reg(op2)) = &self.op2 {
                        if op1.is_gr16() {
                            Instruction::with2::<Register, Register>(Code::Cmovne_r16_rm16, (*op1).into(), (*op2).into())?
                        } else if op1.is_gr32() {
                            Instruction::with2::<Register, Register>(Code::Cmovne_r32_rm32, (*op1).into(), (*op2).into())?
                        } else if op1.is_gr64() {
                            Instruction::with2::<Register, Register>(Code::Cmovne_r64_rm64, (*op1).into(), (*op2).into())?
                        } else { todo!("{}", self) }
                    } else if let Some(Operand::Mem(op2)) = &self.op2 {
                        if op1.is_gr16() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Cmovne_r16_rm16, (*op1).into(), op2.into())?
                        } else if op1.is_gr32() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Cmovne_r32_rm32, (*op1).into(), op2.into())?
                        } else if op1.is_gr64() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Cmovne_r64_rm64, (*op1).into(), op2.into())?
                        } else { todo!("{}", self) }
                    } else { todo!("{}", self) }
                } else { todo!("{}", self) }
            },
            Mnemonic::Sal => {
                if let Some(Operand::Reg(op1)) = &self.op1 {
                    if op1.is_gr8() {
                        Instruction::with2::<Register, Register>(Code::Sal_rm8_CL, (*op1).into(), Register::CL)?
                    } else if op1.is_gr16() {
                        Instruction::with2::<Register, Register>(Code::Sal_rm16_CL, (*op1).into(), Register::CL)?
                    } else if op1.is_gr32() {
                        Instruction::with2::<Register, Register>(Code::Sal_rm32_CL, (*op1).into(), Register::CL)?
                    } else if op1.is_gr64() {
                        Instruction::with2::<Register, Register>(Code::Sal_rm64_CL, (*op1).into(), Register::CL)?
                    } else { todo!("{}", self) }
                } else if let Some(Operand::Mem(op1)) = &self.op1 {
                    Instruction::with2::<MemoryOperand, Register>(Code::Sal_rm64_CL, op1.into(), Register::CL)?
                } else { todo!("{}", self) }
            },
            Mnemonic::Shr => {
                if let Some(Operand::Reg(op1)) = &self.op1 {
                    if op1.is_gr8() {
                        Instruction::with2::<Register, Register>(Code::Shr_rm8_CL, (*op1).into(), Register::CL)?
                    } else if op1.is_gr16() {
                        Instruction::with2::<Register, Register>(Code::Shr_rm16_CL, (*op1).into(), Register::CL)?
                    } else if op1.is_gr32() {
                        Instruction::with2::<Register, Register>(Code::Shr_rm32_CL, (*op1).into(), Register::CL)?
                    } else if op1.is_gr64() {
                        Instruction::with2::<Register, Register>(Code::Shr_rm64_CL, (*op1).into(), Register::CL)?
                    } else { todo!("{}", self) }
                } else if let Some(Operand::Mem(op1)) = &self.op1 {
                    Instruction::with2::<MemoryOperand, Register>(Code::Shr_rm64_CL, op1.into(), Register::CL)?
                } else { todo!("{}", self) }
            },
            Mnemonic::Sar => {
                if let Some(Operand::Reg(op1)) = &self.op1 {
                    if op1.is_gr8() {
                        Instruction::with2::<Register, Register>(Code::Sar_rm8_CL, (*op1).into(), Register::CL)?
                    } else if op1.is_gr16() {
                        Instruction::with2::<Register, Register>(Code::Sar_rm16_CL, (*op1).into(), Register::CL)?
                    } else if op1.is_gr32() {
                        Instruction::with2::<Register, Register>(Code::Sar_rm32_CL, (*op1).into(), Register::CL)?
                    } else if op1.is_gr64() {
                        Instruction::with2::<Register, Register>(Code::Sar_rm64_CL, (*op1).into(), Register::CL)?
                    } else { todo!("{}", self) }
                } else if let Some(Operand::Mem(op1)) = &self.op1 {
                    Instruction::with2::<MemoryOperand, Register>(Code::Sar_rm64_CL, op1.into(), Register::CL)?
                } else { todo!("{}", self) }
            },
            Mnemonic::Movq => {
                if let Some(Operand::Reg(op1)) = &self.op1 {
                    if let Some(Operand::Mem(op2)) = &self.op2 {
                        if op1.is_xmm() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Movq_xmm_rm64, (*op1).into(), op2.into())?
                        } else {
                            Instruction::with2::<Register, MemoryOperand>(Code::Movq_rm64_xmm, (*op1).into(), op2.into())?
                        }
                    } else if let Some(Operand::Reg(op2)) = &self.op2 {
                        if op1.is_xmm() {
                            Instruction::with2::<Register, Register>(Code::Movq_xmm_rm64, (*op1).into(), (*op2).into())?
                        } else {
                            Instruction::with2::<Register, Register>(Code::Movq_rm64_xmm, (*op1).into(), (*op2).into())?
                        }
                    } else { todo!("{}", self) }
                } else if let Some(Operand::Mem(op1)) = &self.op1 {
                    if let Some(Operand::Reg(op2)) = &self.op2 {
                        Instruction::with2::<MemoryOperand, Register>(Code::Movq_rm64_xmm, op1.into(), (*op2).into())?
                    } else { todo!("{}", self) }
                } else { todo!("{}", self) }
            },
            Mnemonic::Movd => {
                if let Some(Operand::Reg(op1)) = &self.op1 {
                    if let Some(Operand::Mem(op2)) = &self.op2 {
                        if op1.is_xmm() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Movd_xmm_rm32, (*op1).into(), op2.into())?
                        } else {
                            Instruction::with2::<Register, MemoryOperand>(Code::Movd_rm32_xmm, (*op1).into(), op2.into())?
                        }
                    } else if let Some(Operand::Reg(op2)) = &self.op2 {
                        if op1.is_xmm() {
                            Instruction::with2::<Register, Register>(Code::Movd_xmm_rm32, (*op1).into(), (*op2).into())?
                        } else {
                            Instruction::with2::<Register, Register>(Code::Movd_rm32_xmm, (*op1).into(), (*op2).into())?
                        }
                    } else { todo!("{}", self) }
                } else if let Some(Operand::Mem(op1)) = &self.op1 {
                    if let Some(Operand::Reg(op2)) = &self.op2 {
                        Instruction::with2::<MemoryOperand, Register>(Code::Movd_rm32_xmm, op1.into(), (*op2).into())?
                    } else { todo!("{}", self) }
                } else { todo!("{}", self) }
            },
            Mnemonic::Movss => {
                if let Some(Operand::Reg(op1)) = &self.op1 {
                    if let Some(Operand::Reg(op2))  = &self.op2 {
                        Instruction::with2::<Register, Register>(Code::Movss_xmmm32_xmm, (*op1).into(), (*op2).into())?
                    } else if let Some(Operand::Mem(op2))  = &self.op2 {
                        Instruction::with2::<Register, MemoryOperand>(Code::Movss_xmm_xmmm32, (*op1).into(), op2.into())?
                    } else { todo!("{}", self) }
                } else if let Some(Operand::Mem(op1)) = &self.op1 {
                    if let Some(Operand::Reg(op2))  = &self.op2 {
                        Instruction::with2::<MemoryOperand, Register>(Code::Movss_xmmm32_xmm, op1.into(), (*op2).into())?
                    } else { todo!("{}", self) } 
                } else { todo!("{}", self) }
            },
            Mnemonic::Movsd => {
                if let Some(Operand::Reg(op1)) = &self.op1 {
                    if let Some(Operand::Reg(op2))  = &self.op2 {
                        Instruction::with2::<Register, Register>(Code::Movsd_xmmm64_xmm, (*op1).into(), (*op2).into())?
                    } else if let Some(Operand::Mem(op2))  = &self.op2 {
                        Instruction::with2::<Register, MemoryOperand>(Code::Movsd_xmm_xmmm64, (*op1).into(), op2.into())?
                    } else { todo!("{}", self) }
                } else if let Some(Operand::Mem(op1)) = &self.op1 {
                    if let Some(Operand::Reg(op2))  = &self.op2 {
                        Instruction::with2::<MemoryOperand, Register>(Code::Movsd_xmmm64_xmm, op1.into(), (*op2).into())?
                    } else { todo!("{}", self) } 
                } else { todo!("{}", self) }
            },
            Mnemonic::Movups => {
                if let Some(Operand::Reg(op1)) = &self.op1 {
                    if let Some(Operand::Reg(op2))  = &self.op2 {
                        Instruction::with2::<Register, Register>(Code::Movups_xmm_xmmm128, (*op1).into(), (*op2).into())?
                    } else if let Some(Operand::Mem(op2))  = &self.op2 {
                        Instruction::with2::<Register, MemoryOperand>(Code::Movups_xmm_xmmm128, (*op1).into(), op2.into())?
                    } else { todo!("{}", self) }
                } else if let Some(Operand::Mem(op1)) = &self.op1 {
                    if let Some(Operand::Reg(op2))  = &self.op2 {
                        Instruction::with2::<MemoryOperand, Register>(Code::Movups_xmmm128_xmm, op1.into(), (*op2).into())?
                    } else { todo!("{}", self) } 
                } else { todo!("{}", self) }
            },
            Mnemonic::Movupd => {
                if let Some(Operand::Reg(op1)) = &self.op1 {
                    if let Some(Operand::Reg(op2))  = &self.op2 {
                        Instruction::with2::<Register, Register>(Code::Movupd_xmm_xmmm128, (*op1).into(), (*op2).into())?
                    } else if let Some(Operand::Mem(op2))  = &self.op2 {
                        Instruction::with2::<Register, MemoryOperand>(Code::Movupd_xmm_xmmm128, (*op1).into(), op2.into())?
                    } else { todo!("{}", self) }
                } else if let Some(Operand::Mem(op1)) = &self.op1 {
                    if let Some(Operand::Reg(op2))  = &self.op2 {
                        Instruction::with2::<MemoryOperand, Register>(Code::Movupd_xmmm128_xmm, op1.into(), (*op2).into())?
                    } else { todo!("{}", self) } 
                } else { todo!("{}", self) }
            },
            Mnemonic::Addss => {
                if let Some(Operand::Reg(op1)) = &self.op1 {
                    if let Some(Operand::Reg(op2))  = &self.op2 {
                        Instruction::with2::<Register, Register>(Code::Addss_xmm_xmmm32, (*op1).into(), (*op2).into())?
                    } else if let Some(Operand::Mem(op2))  = &self.op2 {
                        Instruction::with2::<Register, MemoryOperand>(Code::Addss_xmm_xmmm32, (*op1).into(), op2.into())?
                    } else { todo!("{}", self) }
                } else { todo!("{}", self) }
            },
            Mnemonic::Addsd => {
                if let Some(Operand::Reg(op1)) = &self.op1 {
                    if let Some(Operand::Reg(op2))  = &self.op2 {
                        Instruction::with2::<Register, Register>(Code::Addsd_xmm_xmmm64, (*op1).into(), (*op2).into())?
                    } else if let Some(Operand::Mem(op2))  = &self.op2 {
                        Instruction::with2::<Register, MemoryOperand>(Code::Addsd_xmm_xmmm64, (*op1).into(), op2.into())?
                    } else { todo!("{}", self) }
                } else { todo!("{}", self) }
            },
            Mnemonic::Divss => {
                if let Some(Operand::Reg(op1)) = &self.op1 {
                    if let Some(Operand::Reg(op2))  = &self.op2 {
                        Instruction::with2::<Register, Register>(Code::Divss_xmm_xmmm32, (*op1).into(), (*op2).into())?
                    } else if let Some(Operand::Mem(op2))  = &self.op2 {
                        Instruction::with2::<Register, MemoryOperand>(Code::Divss_xmm_xmmm32, (*op1).into(), op2.into())?
                    } else { todo!("{}", self) }
                } else { todo!("{}", self) }
            },
            Mnemonic::Divsd => {
                if let Some(Operand::Reg(op1)) = &self.op1 {
                    if let Some(Operand::Reg(op2))  = &self.op2 {
                        Instruction::with2::<Register, Register>(Code::Divsd_xmm_xmmm64, (*op1).into(), (*op2).into())?
                    } else if let Some(Operand::Mem(op2))  = &self.op2 {
                        Instruction::with2::<Register, MemoryOperand>(Code::Divsd_xmm_xmmm64, (*op1).into(), op2.into())?
                    } else { todo!("{}", self) }
                } else { todo!("{}", self) }
            },
            Mnemonic::Mulss => {
                if let Some(Operand::Reg(op1)) = &self.op1 {
                    if let Some(Operand::Reg(op2))  = &self.op2 {
                        Instruction::with2::<Register, Register>(Code::Mulss_xmm_xmmm32, (*op1).into(), (*op2).into())?
                    } else if let Some(Operand::Mem(op2))  = &self.op2 {
                        Instruction::with2::<Register, MemoryOperand>(Code::Mulss_xmm_xmmm32, (*op1).into(), op2.into())?
                    } else { todo!("{}", self) }
                } else { todo!("{}", self) }
            },
            Mnemonic::Mulsd => {
                if let Some(Operand::Reg(op1)) = &self.op1 {
                    if let Some(Operand::Reg(op2))  = &self.op2 {
                        Instruction::with2::<Register, Register>(Code::Mulsd_xmm_xmmm64, (*op1).into(), (*op2).into())?
                    } else if let Some(Operand::Mem(op2))  = &self.op2 {
                        Instruction::with2::<Register, MemoryOperand>(Code::Mulsd_xmm_xmmm64, (*op1).into(), op2.into())?
                    } else { todo!("{}", self) }
                } else { todo!("{}", self) }
            },
            Mnemonic::Subss => {
                if let Some(Operand::Reg(op1)) = &self.op1 {
                    if let Some(Operand::Reg(op2))  = &self.op2 {
                        Instruction::with2::<Register, Register>(Code::Subss_xmm_xmmm32, (*op1).into(), (*op2).into())?
                    } else if let Some(Operand::Mem(op2))  = &self.op2 {
                        Instruction::with2::<Register, MemoryOperand>(Code::Subss_xmm_xmmm32, (*op1).into(), op2.into())?
                    } else { todo!("{}", self) }
                } else { todo!("{}", self) }
            },
            Mnemonic::Subsd => {
                if let Some(Operand::Reg(op1)) = &self.op1 {
                    if let Some(Operand::Reg(op2))  = &self.op2 {
                        Instruction::with2::<Register, Register>(Code::Subsd_xmm_xmmm64, (*op1).into(), (*op2).into())?
                    } else if let Some(Operand::Mem(op2))  = &self.op2 {
                        Instruction::with2::<Register, MemoryOperand>(Code::Subsd_xmm_xmmm64, (*op1).into(), op2.into())?
                    } else { todo!("{}", self) }
                } else { todo!("{}", self) }
            },
            Mnemonic::Ucomiss => {
                if let Some(Operand::Reg(op1)) = &self.op1 {
                    if let Some(Operand::Reg(op2)) = &self.op2 {
                        if op1.is_xmm() && op2.is_xmm() {
                            Instruction::with2::<Register, Register>(Code::Ucomiss_xmm_xmmm32, (*op1).into(), (*op2).into())?
                        } else { todo!("{}", self) }
                    } else { todo!("{}", self) }
                } else { todo!("{}", self) }
            },
            Mnemonic::Ucomisd => {
                if let Some(Operand::Reg(op1)) = &self.op1 {
                    if let Some(Operand::Reg(op2)) = &self.op2 {
                        if op1.is_xmm() && op2.is_xmm() {
                            Instruction::with2::<Register, Register>(Code::Ucomisd_xmm_xmmm64, (*op1).into(), (*op2).into())?
                        } else { todo!("{}", self) }
                    } else { todo!("{}", self) }
                } else { todo!("{}", self) }
            },
            Mnemonic::Cvtss2si => {
                if let Some(Operand::Reg(op1)) = &self.op1 {
                    if let Some(Operand::Reg(op2)) = &self.op2 {
                        if op1.is_gr32() {
                            Instruction::with2::<Register, Register>(Code::Cvtss2si_r32_xmmm32, (*op1).into(), (*op2).into())?
                        } else if op1.is_gr64() {
                            Instruction::with2::<Register, Register>(Code::Cvtss2si_r64_xmmm32, (*op1).into(), (*op2).into())?
                        } else { todo!("{}", self) }
                    } else if let Some(Operand::Mem(op2)) = &self.op2 {
                        if op1.is_gr32() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Cvtss2si_r32_xmmm32, (*op1).into(), op2.into())?
                        } else if op1.is_gr64() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Cvtss2si_r64_xmmm32, (*op1).into(), op2.into())?
                        } else { todo!("{}", self) }
                    } else { todo!("{}", self) }
                } else { todo!("{}", self) }
            },
            Mnemonic::Cvtsd2si => {
                if let Some(Operand::Reg(op1)) = &self.op1 {
                    if let Some(Operand::Reg(op2)) = &self.op2 {
                        if op1.is_gr32() {
                            Instruction::with2::<Register, Register>(Code::Cvtsd2si_r32_xmmm64, (*op1).into(), (*op2).into())?
                        } else if op1.is_gr64() {
                            Instruction::with2::<Register, Register>(Code::Cvtsd2si_r64_xmmm64, (*op1).into(), (*op2).into())?
                        } else { todo!("{}", self) }
                    } else if let Some(Operand::Mem(op2)) = &self.op2 {
                        if op1.is_gr32() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Cvtsd2si_r32_xmmm64, (*op1).into(), op2.into())?
                        } else if op1.is_gr64() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Cvtsd2si_r64_xmmm64, (*op1).into(), op2.into())?
                        } else { todo!("{}", self) }
                    } else { todo!("{}", self) }
                } else { todo!("{}", self) }
            },
            Mnemonic::Cvtss2sd => {
                if let Some(Operand::Reg(op1)) = &self.op1 {
                    if let Some(Operand::Reg(op2)) = &self.op2 {
                        if op1.is_xmm() {
                            Instruction::with2::<Register, Register>(Code::Cvtss2sd_xmm_xmmm32, (*op1).into(), (*op2).into())?
                        } else { todo!("{}", self) }
                    } else if let Some(Operand::Mem(op2)) = &self.op2 {
                        if op1.is_xmm() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Cvtss2sd_xmm_xmmm32, (*op1).into(), op2.into())?
                        } else { todo!("{}", self) }
                    } else { todo!("{}", self) }
                } else { todo!("{}", self) }
            },
            Mnemonic::Cvtsd2ss => {
                if let Some(Operand::Reg(op1)) = &self.op1 {
                    if let Some(Operand::Reg(op2)) = &self.op2 {
                        if op1.is_xmm() {
                            Instruction::with2::<Register, Register>(Code::Cvtsd2ss_xmm_xmmm64, (*op1).into(), (*op2).into())?
                        } else { todo!("{}", self) }
                    } else if let Some(Operand::Mem(op2)) = &self.op2 {
                        if op1.is_xmm() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Cvtsd2ss_xmm_xmmm64, (*op1).into(), op2.into())?
                        } else { todo!("{}", self) }
                    } else { todo!("{}", self) }
                } else { todo!("{}", self) }
            },
            Mnemonic::Cvtsi2ss => {
                if let Some(Operand::Reg(op1)) = &self.op1 {
                    if let Some(Operand::Reg(op2)) = &self.op2 {
                        if op1.is_xmm() {
                            Instruction::with2::<Register, Register>(Code::Cvtsi2ss_xmm_rm32, (*op1).into(), (*op2).into())?
                        } else { todo!("{}", self) }
                    } else if let Some(Operand::Mem(op2)) = &self.op2 {
                        if op1.is_xmm() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Cvtsi2ss_xmm_rm32, (*op1).into(), op2.into())?
                        } else { todo!("{}", self) }
                    } else { todo!("{}", self) }
                } else { todo!("{}", self) }
            },
            Mnemonic::Cvtsi2sd => {
                if let Some(Operand::Reg(op1)) = &self.op1 {
                    if let Some(Operand::Reg(op2)) = &self.op2 {
                        if op1.is_xmm() {
                            Instruction::with2::<Register, Register>(Code::Cvtsi2sd_xmm_rm64, (*op1).into(), (*op2).into())?
                        } else { todo!("{}", self) }
                    } else if let Some(Operand::Mem(op2)) = &self.op2 {
                        if op1.is_xmm() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Cvtsi2sd_xmm_rm64, (*op1).into(), op2.into())?
                        } else { todo!("{}", self) }
                    } else { todo!("{}", self) }
                } else { todo!("{}", self) }
            },
            Mnemonic::Cbw => Instruction::with(Code::Cbw),
            Mnemonic::Cwd => Instruction::with(Code::Cwd),
            Mnemonic::Cdq => Instruction::with(Code::Cdq),
            Mnemonic::Cqo => Instruction::with(Code::Cqo),
        };
        
        //instr.as_near_branch();
        //println!("instr: {}", instr);

        let binding = [instr];
        let instr = InstructionBlock::new(&binding, 0);

        let encoder = BlockEncoder::encode(64, instr, BlockEncoderOptions::DONT_FIX_BRANCHES)?;

        let mut links = None;

        if let Some(Operand::LinkDestination(target, addend)) = &self.op1 {
            links = Some(Link {
                from: "".into(),
                to: target.to_owned(),
                at: 0,
                addend: *addend,
                special: false,
                kind: RelocationEncoding::X86Branch,
            })
        } else if let Some(Operand::BlockLinkDestination(target, addend)) = &self.op1 {
            links = Some(Link {
                from: "".into(),
                to: target.to_owned(),
                at: 0,
                addend: *addend,
                special: true,
                kind: RelocationEncoding::Generic,
            })
        } else if let Some(Operand::RipRelative(target)) = &self.op2 {
            links = Some(Link {
                from: "".into(),
                to: target.to_owned(),
                at: 0,
                addend: -4,
                special: false,
                kind: RelocationEncoding::X86RipRelative,
            })
        }

        Ok((encoder.code_buffer, links))        
    }

    /// Verifys the instruction (like checking the right opcodes etc.)
    pub fn verify(&self) -> Result<(), InstrEncodingError> {
        Ok(())
    }

    /// Does the same as the encode function just for naming pourpuses
    pub fn compile(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
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
                Operand::RipRelative(rip) => profile.markup(&format!("[rel {}]", rip), ColorClass::Var),
                Operand::LinkDestination(_, _) => "".to_string(),
                Operand::BlockLinkDestination(_, _) => "".to_string(),
                Operand::Debug(s) => s.to_string(),
            }));
            if let Some(op2) = &self.op2 {
                string.push_str(&format!(", {}", match op2 {
                    Operand::Imm(num) => profile.markup(&format!("{}", num.to_string()), ColorClass::Value),
                    Operand::Reg(reg) => profile.markup(&format!(", {}", reg.to_string()), ColorClass::Var),
                    Operand::RipRelative(rip) => profile.markup(&format!("[rel {}]", rip), ColorClass::Var),
                    Operand::Mem(mem) => profile.markup(&format!("{}", mem), ColorClass::Var),
                    Operand::LinkDestination(_, _) => "".to_string(),
                    Operand::BlockLinkDestination(_, _) => "".to_string(),
                    Operand::Debug(s) => s.to_string(),
                }));
            }
        }

        string
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
                if let Some(op3) = &self.op3 {
                    string.push_str(&format!(", {}", op3));
                }
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

    Cvtss2si,
    Cvtsd2si,
    Cvtss2sd,
    Cvtsd2ss,
    Cvtsi2ss,
    Cvtsi2sd,

    Jg,
    Jl,
    Jge,
    Jle,

    Cbw,
    Cwd,
    Cdq,
    Cqo,
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
            "cvtss2si"  => Ok(Mnemonic::Cvtss2si),
            "cvtsd2si"  => Ok(Mnemonic::Cvtsd2si),
            "cvtss2sd"  => Ok(Mnemonic::Cvtss2sd),
            "cvtsd2ss"  => Ok(Mnemonic::Cvtsd2ss),
            "cvtsi2ss" => Ok(Mnemonic::Cvtsi2ss),
            "cvtsi2sd" => Ok(Mnemonic::Cvtsi2sd),
            "jg" => Ok(Mnemonic::Jg),
            "jl" => Ok(Mnemonic::Jl),
            "jge" => Ok(Mnemonic::Jge),
            "jle" => Ok(Mnemonic::Jle),
            "cbw" => Ok(Mnemonic::Cbw),
            "cwd" => Ok(Mnemonic::Cwd),
            "cdq" => Ok(Mnemonic::Cdq),
            "cqo" => Ok(Mnemonic::Cqo),
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
            Mnemonic::Cvtss2si => "cvtss2si",
            Mnemonic::Cvtsd2si => "cvtsd2si",
            Mnemonic::Cvtss2sd => "cvtss2sd",
            Mnemonic::Cvtsd2ss => "cvtsd2ss",
            Mnemonic::Cvtsi2ss => "cvtsi2ss",
            Mnemonic::Cvtsi2sd => "cvtsi2sd",
            Mnemonic::Jg => "jg",
            Mnemonic::Jl => "jl",
            Mnemonic::Jge => "jge",
            Mnemonic::Jle => "jle",
            Mnemonic::Cbw => "cbw",
            Mnemonic::Cwd => "cwd",
            Mnemonic::Cdq => "cdq",
            Mnemonic::Cqo => "cqo",
        })
    }
}

/// The operand type and value to use
#[derive(Debug, Clone, Eq)]
pub enum Operand {
    /// A number operand
    Imm(i64),
    /// A register operand
    Reg(X64Reg),
    /// A memory displacement
    Mem(MemOp),
    /// A link destination
    LinkDestination(String, i64),
    /// A link destination to a block
    BlockLinkDestination(String, i64),
    /// For debugging
    Debug(String),
    /// A rip relative
    RipRelative(String),
}

impl Operand {
    /// Returns if the operand is a register
    pub fn is_reg(&self) -> bool {
        matches!(self, Operand::Reg(_))
    }

    /// Returns if the operand is a imm
    pub fn is_imm(&self) -> bool {
        matches!(self, Operand::Imm(_))
    }

    /// Returns if the operand is a memory displacmenet or rip relative
    pub fn is_mem(&self) -> bool {
        matches!(self, Operand::Mem(_) | Operand::RipRelative(_))
    }
}

impl PartialEq for Operand {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Imm(l0), Self::Imm(r0)) => l0 == r0,
            (Self::Reg(l0), Self::Reg(r0)) => l0 == r0,
            (Self::Mem(l0), Self::Mem(r0)) => l0 == r0,
            (Self::LinkDestination(l0, l1), Self::LinkDestination(r0, r1)) => l0 == r0 && l1 == r1,
            (Self::Debug(l0), Self::Debug(r0)) => l0 == r0,
            (Self::RipRelative(l0), Self::RipRelative(r0)) => l0 == r0,
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
            Operand::LinkDestination(target, _) => target.to_string(),
            Operand::BlockLinkDestination(target, _) =>target.to_string(),
            Operand::Debug(s) => s.to_string(),
            Operand::RipRelative(target) => format!("[rel {}]", target),
        })
    }
}

/// A memory displacement
#[derive(Eq)]
pub struct MemOp {
    /// The base register
    pub base: Option<X64Reg>,
    /// The index register
    pub index: Option<X64Reg>,
    /// The scale
    pub scale: isize,
    /// The displacement
    pub displ: isize,
    /// rip relativ
    pub rip: bool,
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

impl Into<MemoryOperand> for MemOp {
    fn into(self) -> MemoryOperand {
        let mut base = Register::None;
        let mut index = Register::None;

        if let Some(base_reg) = &self.base {
            base = (*base_reg).into();
        }
        if let Some(index_reg) = &self.index {
            index = (*index_reg).into();
        }

        let mut displ_size = 0;

        if self.displ != 0 { // TRUE: there's an displacement
            displ_size = 1;
        }

        if self.rip {
            base = Register::RIP;
        }

        MemoryOperand::new(
            base, 
            index, 
            self.scale as u32, 
            self.displ as i64, 
            displ_size, 
            false, 
            Register::None
        )
    }
}

impl Into<MemoryOperand> for &MemOp {
    fn into(self) -> MemoryOperand {
        self.clone().into()
    }
}

impl Into<Register> for X64Reg {
    fn into(self) -> Register {
        match self {
            X64Reg::Rax => Register::RAX,
            X64Reg::Eax => Register::EAX,
            X64Reg::Ax =>  Register::AX,
            X64Reg::Al =>  Register::AL,

            X64Reg::Rbx => Register::RBX,
            X64Reg::Ebx => Register::EBX,
            X64Reg::Bx =>  Register::BX,
            X64Reg::Bl =>  Register::BL,

            X64Reg::Rcx => Register::RCX,
            X64Reg::Ecx => Register::ECX,
            X64Reg::Cx =>  Register::CX,
            X64Reg::Cl =>  Register::CL,

            X64Reg::Rdx => Register::RDX,
            X64Reg::Edx => Register::EDX,
            X64Reg::Dx =>  Register::DX,
            X64Reg::Dl =>  Register::DL,

            X64Reg::Rsi => Register::RSI,
            X64Reg::Esi => Register::ESI,
            X64Reg::Si =>  Register::SI,
            X64Reg::Sil => Register::SIL,

            X64Reg::Rdi => Register::RDI,
            X64Reg::Edi => Register::EDI,
            X64Reg::Di =>  Register::DI,
            X64Reg::Dil => Register::DIL,

            X64Reg::Rsp => Register::RSP,
            X64Reg::Esp => Register::ESP,
            X64Reg::Sp =>  Register::SP,
            X64Reg::Spl => Register::SPL,

            X64Reg::Rbp => Register::RBP,
            X64Reg::Ebp => Register::EBP,
            X64Reg::Bp =>  Register::BP,
            X64Reg::Bpl => Register::BPL,

            X64Reg::R8 =>  Register::R8,
            X64Reg::R8d => Register::R8D,
            X64Reg::R8w => Register::R8W,
            X64Reg::R8b => Register::R8L,

            X64Reg::R9 =>  Register::R9,
            X64Reg::R9d => Register::R9D,
            X64Reg::R9w => Register::R9W,
            X64Reg::R9b => Register::R9L,

            X64Reg::R10 =>   Register::R10,
            X64Reg::R10d =>  Register::R10D,
            X64Reg::R10w =>  Register::R10W,
            X64Reg::R10b =>  Register::R10L,

            X64Reg::R11 =>  Register::R11,
            X64Reg::R11d => Register::R11D,
            X64Reg::R11w => Register::R11W,
            X64Reg::R11b => Register::R11L,

            X64Reg::R12 =>  Register::R12,
            X64Reg::R12d => Register::R12D,
            X64Reg::R12w => Register::R12W,
            X64Reg::R12b => Register::R12L,

            X64Reg::R13 =>  Register::R13,
            X64Reg::R13d => Register::R13D,
            X64Reg::R13w => Register::R13W,
            X64Reg::R13b => Register::R13L,

            X64Reg::R14 =>  Register::R14,
            X64Reg::R14d => Register::R14D,
            X64Reg::R14w => Register::R14W,
            X64Reg::R14b => Register::R14L,

            X64Reg::R15 =>  Register::R15,
            X64Reg::R15d => Register::R15D,
            X64Reg::R15w => Register::R15W,
            X64Reg::R15b => Register::R15L,

            X64Reg::Xmm0 => Register::XMM0,
            X64Reg::Xmm1 => Register::XMM1,
            X64Reg::Xmm2 => Register::XMM2,
            X64Reg::Xmm3 => Register::XMM3,
            X64Reg::Xmm4 => Register::XMM4,
            X64Reg::Xmm5 => Register::XMM5,
            X64Reg::Xmm6 => Register::XMM6,
            X64Reg::Xmm7 => Register::XMM7,
            X64Reg::Xmm8 => Register::XMM8,
            X64Reg::Xmm9 => Register::XMM9,
            X64Reg::Xmm10 => Register::XMM10,
            X64Reg::Xmm11 => Register::XMM11,
            X64Reg::Xmm12 => Register::XMM12,
            X64Reg::Xmm13 => Register::XMM13,
            X64Reg::Xmm14 => Register::XMM14,
            X64Reg::Xmm15 => Register::XMM15,
        }
    }
}

impl Add<u32> for X64Reg {
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

impl Add<X64Reg> for X64Reg {
    type Output = MemOp;

    fn add(self, rhs: X64Reg) -> Self::Output {
        MemOp {
            base: Some(self),
            index: Some(rhs),
            scale: 1,
            displ: 0,
            rip: false,
        }
    }
}

impl Sub<u32> for X64Reg {
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

macro_rules! IsCheckerOps0 {
    ($func:tt, $mnemonic:expr) => {
        impl X64MCInstr {
            /// [AUTO GENERATED] checks if the instruction is the same as the other
            pub fn $func(&self) -> bool {
                self.mnemonic == $mnemonic
            }
        }
    };
}

IsCheckerOps0!(is_add, Mnemonic::Add);
IsCheckerOps0!(is_adc, Mnemonic::Adc);
IsCheckerOps0!(is_and, Mnemonic::And);
IsCheckerOps0!(is_or, Mnemonic::Or);
IsCheckerOps0!(is_xor, Mnemonic::Xor);
IsCheckerOps0!(is_sub, Mnemonic::Sub);
IsCheckerOps0!(is_neg, Mnemonic::Neg);
IsCheckerOps0!(is_cmp, Mnemonic::Cmp);
IsCheckerOps0!(is_lea, Mnemonic::Lea);
IsCheckerOps0!(is_mov, Mnemonic::Mov);
IsCheckerOps0!(is_movzx, Mnemonic::Movzx);
IsCheckerOps0!(is_push, Mnemonic::Push);
IsCheckerOps0!(is_pop, Mnemonic::Pop);
IsCheckerOps0!(is_ret, Mnemonic::Ret);
IsCheckerOps0!(is_imul, Mnemonic::Imul);
IsCheckerOps0!(is_mul, Mnemonic::Mul);
IsCheckerOps0!(is_idiv, Mnemonic::Idiv);
IsCheckerOps0!(is_div, Mnemonic::Div);
IsCheckerOps0!(is_call, Mnemonic::Call);
IsCheckerOps0!(is_jmp, Mnemonic::Jmp);
IsCheckerOps0!(is_jne, Mnemonic::Jne);
IsCheckerOps0!(is_je, Mnemonic::Je);
IsCheckerOps0!(is_jg, Mnemonic::Jg);
IsCheckerOps0!(is_jl, Mnemonic::Jl);
IsCheckerOps0!(is_jge, Mnemonic::Jge);
IsCheckerOps0!(is_jle, Mnemonic::Jle);
IsCheckerOps0!(is_endbr64, Mnemonic::Endbr64);
IsCheckerOps0!(is_sete, Mnemonic::Sete);
IsCheckerOps0!(is_setne, Mnemonic::Setne);
IsCheckerOps0!(is_setg, Mnemonic::Setg);
IsCheckerOps0!(is_setl, Mnemonic::Setl);
IsCheckerOps0!(is_setge, Mnemonic::Setge);
IsCheckerOps0!(is_setle, Mnemonic::Setle);
IsCheckerOps0!(is_cmove, Mnemonic::Cmove);
IsCheckerOps0!(is_cmovne, Mnemonic::Cmovne);
IsCheckerOps0!(is_sal, Mnemonic::Sal);
IsCheckerOps0!(is_shr, Mnemonic::Shr);
IsCheckerOps0!(is_sar, Mnemonic::Sar);
IsCheckerOps0!(is_movq, Mnemonic::Movq);
IsCheckerOps0!(is_movd, Mnemonic::Movd);
IsCheckerOps0!(is_movss, Mnemonic::Movss);
IsCheckerOps0!(is_movsd, Mnemonic::Movsd);
IsCheckerOps0!(is_movups, Mnemonic::Movups);
IsCheckerOps0!(is_movupd, Mnemonic::Movupd);
IsCheckerOps0!(is_addss, Mnemonic::Addss);
IsCheckerOps0!(is_addsd, Mnemonic::Addsd);
IsCheckerOps0!(is_divss, Mnemonic::Divss);
IsCheckerOps0!(is_divsd, Mnemonic::Divsd);
IsCheckerOps0!(is_mulss, Mnemonic::Mulss);
IsCheckerOps0!(is_mulsd, Mnemonic::Mulsd);
IsCheckerOps0!(is_subss, Mnemonic::Subss);
IsCheckerOps0!(is_subsd, Mnemonic::Subsd);
IsCheckerOps0!(is_ucomiss, Mnemonic::Ucomiss);
IsCheckerOps0!(is_ucomisd, Mnemonic::Ucomisd);
IsCheckerOps0!(is_cvtss2si, Mnemonic::Cvtss2si);
IsCheckerOps0!(is_cvtsd2si, Mnemonic::Cvtsd2si);
IsCheckerOps0!(is_cvtss2sd, Mnemonic::Cvtss2sd);
IsCheckerOps0!(is_cvtsd2ss, Mnemonic::Cvtsd2ss);
IsCheckerOps0!(is_cvtsi2ss, Mnemonic::Cvtsi2ss);
IsCheckerOps0!(is_cvtsi2sd, Mnemonic::Cvtsi2sd);

macro_rules! IsCheckerOps1 {
    ($func:tt, $mnemonic:expr) => {
        impl X64MCInstr {
            /// [AUTO GENERATED] checks if the instruction is the same as the other
            pub fn $func(&self, op1: &Operand) -> bool {
                self.mnemonic == $mnemonic && self.op1 == Some(op1.to_owned())
            }
        }
    };
}

IsCheckerOps1!(is_add1, Mnemonic::Add);
IsCheckerOps1!(is_adc1, Mnemonic::Adc);
IsCheckerOps1!(is_and1, Mnemonic::And);
IsCheckerOps1!(is_or1, Mnemonic::Or);
IsCheckerOps1!(is_xor1, Mnemonic::Xor);
IsCheckerOps1!(is_sub1, Mnemonic::Sub);
IsCheckerOps1!(is_neg1, Mnemonic::Neg);
IsCheckerOps1!(is_cmp1, Mnemonic::Cmp);
IsCheckerOps1!(is_lea1, Mnemonic::Lea);
IsCheckerOps1!(is_mov1, Mnemonic::Mov);
IsCheckerOps1!(is_movzx1, Mnemonic::Movzx);
IsCheckerOps1!(is_push1, Mnemonic::Push);
IsCheckerOps1!(is_pop1, Mnemonic::Pop);
IsCheckerOps1!(is_imul1, Mnemonic::Imul);
IsCheckerOps1!(is_mul1, Mnemonic::Mul);
IsCheckerOps1!(is_idiv1, Mnemonic::Idiv);
IsCheckerOps1!(is_sete1, Mnemonic::Sete);
IsCheckerOps1!(is_setne1, Mnemonic::Setne);
IsCheckerOps1!(is_setg1, Mnemonic::Setg);
IsCheckerOps1!(is_setl1, Mnemonic::Setl);
IsCheckerOps1!(is_setge1, Mnemonic::Setge);
IsCheckerOps1!(is_setle1, Mnemonic::Setle);
IsCheckerOps1!(is_cmove1, Mnemonic::Cmove);
IsCheckerOps1!(is_cmovne1, Mnemonic::Cmovne);
IsCheckerOps1!(is_sal1, Mnemonic::Sal);
IsCheckerOps1!(is_shr1, Mnemonic::Shr);
IsCheckerOps1!(is_movq1, Mnemonic::Movq);
IsCheckerOps1!(is_movd1, Mnemonic::Movd);
IsCheckerOps1!(is_movss1, Mnemonic::Movd);
IsCheckerOps1!(is_movsd1, Mnemonic::Movd);
IsCheckerOps1!(is_movups1, Mnemonic::Movups);
IsCheckerOps1!(is_movupd1, Mnemonic::Movupd);
IsCheckerOps1!(is_addss1, Mnemonic::Addss);
IsCheckerOps1!(is_addsd1, Mnemonic::Addsd);
IsCheckerOps1!(is_divss1, Mnemonic::Divss);
IsCheckerOps1!(is_divsd1, Mnemonic::Divsd);
IsCheckerOps1!(is_mulss1, Mnemonic::Mulss);
IsCheckerOps1!(is_mulsd1, Mnemonic::Mulsd);
IsCheckerOps1!(is_subss1, Mnemonic::Subss);
IsCheckerOps1!(is_subsd1, Mnemonic::Subsd);
IsCheckerOps1!(is_ucomiss1, Mnemonic::Ucomiss);
IsCheckerOps1!(is_ucomisd1, Mnemonic::Ucomisd);
IsCheckerOps1!(is_cvtss2si1, Mnemonic::Cvtss2si);
IsCheckerOps1!(is_cvtsd2si1, Mnemonic::Cvtsd2si);
IsCheckerOps1!(is_cvtss2sd1, Mnemonic::Cvtss2sd);
IsCheckerOps1!(is_cvtsd2ss1, Mnemonic::Cvtsd2ss);
IsCheckerOps1!(is_cvtsi2ss1, Mnemonic::Cvtsi2ss);
IsCheckerOps1!(is_cvtsi2sd1, Mnemonic::Cvtsi2sd);

macro_rules! IsCheckerOps2 {
    ($func:tt, $mnemonic:expr) => {
        impl X64MCInstr {
            /// [AUTO GENERATED] checks if the instruction is the same as the other
            pub fn $func(&self, op1: &Operand, op2: &Operand) -> bool {
                self.mnemonic == $mnemonic && self.op1 == Some(op1.to_owned()) && self.op2 == Some(op2.to_owned())
            }
        }
    };
}

IsCheckerOps2!(is_add2, Mnemonic::Add);
IsCheckerOps2!(is_adc2, Mnemonic::Adc);
IsCheckerOps2!(is_and2, Mnemonic::And);
IsCheckerOps2!(is_or2, Mnemonic::Or);
IsCheckerOps2!(is_xor2, Mnemonic::Xor);
IsCheckerOps2!(is_sub2, Mnemonic::Sub);
IsCheckerOps2!(is_cmp2, Mnemonic::Cmp);
IsCheckerOps2!(is_lea2, Mnemonic::Lea);
IsCheckerOps2!(is_mov2, Mnemonic::Mov);
IsCheckerOps2!(is_movzx2, Mnemonic::Movzx);
IsCheckerOps2!(is_cmove2, Mnemonic::Cmove);
IsCheckerOps2!(is_cmovne2, Mnemonic::Cmovne);
IsCheckerOps2!(is_sal2, Mnemonic::Sal);
IsCheckerOps2!(is_shr2, Mnemonic::Shr);
IsCheckerOps2!(is_movq2, Mnemonic::Movq);
IsCheckerOps2!(is_movd2, Mnemonic::Movd);
IsCheckerOps2!(is_movss2, Mnemonic::Movd);
IsCheckerOps2!(is_movsd2, Mnemonic::Movd);
IsCheckerOps2!(is_movups2, Mnemonic::Movups);
IsCheckerOps2!(is_movupd2, Mnemonic::Movupd);
IsCheckerOps2!(is_addss2, Mnemonic::Addss);
IsCheckerOps2!(is_addsd2, Mnemonic::Addsd);
IsCheckerOps2!(is_divss2, Mnemonic::Divss);
IsCheckerOps2!(is_divsd2, Mnemonic::Divsd);
IsCheckerOps2!(is_mulss2, Mnemonic::Mulss);
IsCheckerOps2!(is_mulsd2, Mnemonic::Mulsd);
IsCheckerOps2!(is_subss2, Mnemonic::Subss);
IsCheckerOps2!(is_subsd2, Mnemonic::Subsd);
IsCheckerOps2!(is_ucomiss2, Mnemonic::Ucomiss);
IsCheckerOps2!(is_ucomisd2, Mnemonic::Ucomisd);
IsCheckerOps2!(is_cvtss2si2, Mnemonic::Cvtss2si);
IsCheckerOps2!(is_cvtsd2si2, Mnemonic::Cvtsd2si);
IsCheckerOps2!(is_cvtss2sd2, Mnemonic::Cvtss2sd);
IsCheckerOps2!(is_cvtsd2ss2, Mnemonic::Cvtsd2ss);
IsCheckerOps2!(is_cvtsi2ss2, Mnemonic::Cvtsi2ss);
IsCheckerOps2!(is_cvtsi2sd2, Mnemonic::Cvtsi2sd);

impl X64MCInstr {
    /// Checks if the first operand is a register
    pub fn is_op1_reg(&self) -> bool {
        matches!(self.op1, Some(Operand::Reg(_)))
    }

    /// Checks if the first operand is a memory displacment
    pub fn is_op1_mem(&self) -> bool {
        matches!(self.op1, Some(Operand::Mem(_)) | Some(Operand::RipRelative(_)))
    }

    /// Checks if the first operand is an imm
    pub fn is_op1_imm(&self) -> bool {
        matches!(self.op1, Some(Operand::Imm(_)))
    }
    
    /// Checks if the second operand is a register
    pub fn is_op2_reg(&self) -> bool {
        matches!(self.op2, Some(Operand::Reg(_)))
    }

    /// Checks if the second operand is a memory displacment
    pub fn is_op2_mem(&self) -> bool {
        matches!(self.op2, Some(Operand::Mem(_)) | Some(Operand::RipRelative(_)))
    }

    /// Checks if the second operand is a rip relative
    pub fn is_op2_rip(&self) -> bool {
        matches!(self.op2, Some(Operand::RipRelative(_)))
    }

    /// Checks if the second operand is an imm
    pub fn is_op2_imm(&self) -> bool {
        matches!(self.op2, Some(Operand::Imm(_)))
    }
}