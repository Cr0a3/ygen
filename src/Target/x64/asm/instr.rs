use std::{fmt::Display, ops::{Add, Sub}, str::FromStr};
use iced_x86::{BlockEncoder, BlockEncoderOptions, Code, Instruction, InstructionBlock, MemoryOperand, Register};

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
    pub fn encode(&self) -> Result<(Vec<u8>, Option<Link>), Box<dyn std::error::Error>> {
        self.verify()?;


        if self.mnemonic == Mnemonic::Link {
            if let Some(Operand::LinkDestination(dst, addend)) = &self.op1 {
                return Ok((vec![], Some(Link { from: "".into(), to: dst.to_string(), at: 0, addend: *addend, special: false })));
            } else if let Some(Operand::BlockLinkDestination(dst, addend)) = &self.op1 {
                return Ok((vec![], Some(Link { from: "".into(), to: dst.to_string(), at: 0, addend: *addend, special: true })));
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
                    if op1.is_gr64() {
                        Instruction::with1::<Register>(Code::Imul_rm64, (*op1).into())?
                    } else if op1.is_gr32() {
                        Instruction::with1::<Register>(Code::Imul_rm32, (*op1).into())?
                    } else if op1.is_gr16() {
                        Instruction::with1::<Register>(Code::Imul_rm16, (*op1).into())?
                    } else if op1.is_gr8() {
                        Instruction::with1::<Register>(Code::Imul_rm8, (*op1).into())?
                    } else { todo!("{}", self)}
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
                    Instruction::with_branch(Code::Call_rel32_64, *op1 as u64)?
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
                    Instruction::with_branch(Code::Jmp_rel32_64, *op1 as u64)?
                } else { todo!("{}", self) }
            },
            Mnemonic::Jne => {
                if let Some(Operand::Imm(op1)) = &self.op1 {
                    Instruction::with_branch(Code::Jne_rel32_64, *op1 as u64)?
                } else { todo!("{}", self) }
            },
            Mnemonic::Je => {
                if let Some(Operand::Imm(op1)) = &self.op1 {
                    Instruction::with_branch(Code::Je_rel32_64, *op1 as u64)?
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
        
        };
        
        //instr.as_near_branch();
        //println!("instr: {}", instr);

        let binding = [instr];
        let instr = InstructionBlock::new(&binding, 0);

        let encoder = BlockEncoder::encode(64, instr, BlockEncoderOptions::DONT_FIX_BRANCHES)?;

        Ok((encoder.code_buffer, None))        
    }

    /// Verifys the instruction (like checking the right opcodes etc.)
    pub fn verify(&self) -> Result<(), InstrEncodingError> {
        /*match self.mnemonic {
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
            Mnemonic::Movzx => todo!("{}", self),
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
            Mnemonic::Movq   =>  todo!("{}", self),
            Mnemonic::Movd   =>  todo!("{}", self),
            Mnemonic::Movss  =>  todo!("{}", self),
            Mnemonic::Movsd  =>  todo!("{}", self),
            Mnemonic::Movups =>  todo!("{}", self),
            Mnemonic::Movupd =>  todo!("{}", self),
            Mnemonic::Addss  =>  todo!("{}", self),
            Mnemonic::Addsd  =>  todo!("{}", self),
            Mnemonic::Divss  =>  todo!("{}", self),
            Mnemonic::Divsd  =>  todo!("{}", self),
            Mnemonic::Mulss  =>  todo!("{}", self),
            Mnemonic::Mulsd  =>  todo!("{}", self),
            Mnemonic::Subss  =>  todo!("{}", self),
            Mnemonic::Subsd  =>  todo!("{}", self),
            Mnemonic::Ucomiss  =>  todo!("{}", self),
            Mnemonic::Ucomisd  =>  todo!("{}", self),
        };*/

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

    Cvtss2si,
    Cvtsd2si,
    Cvtss2sd,
    Cvtsd2ss,
    Cvtsi2ss,
    Cvtsi2sd,
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