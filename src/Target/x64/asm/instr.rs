use std::{fmt::Display, ops::{Add, Sub}, str::FromStr};
use iced_x86::{BlockEncoder, BlockEncoderOptions, Code, Instruction, InstructionBlock, MemoryOperand, Register};

use crate::CodeGen::MCInstr;
use crate::Obj::Link;
use crate::Support::{ColorClass, ColorProfile};
use crate::Target::x64::x64Reg;

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

        if Mnemonic::Debug == self.mnemonic {
            return Ok((vec![], None))
        }

        let instr = match self.mnemonic {
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
                        } else { todo!() }
                    } else if let Some(Operand::Mem(op2)) = &self.op2 {
                        if op1.is_gr8() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Add_r8_rm8, (*op1).into(), op2.into())?
                        } else if op1.is_gr16() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Add_r16_rm16, (*op1).into(), op2.into())?
                        } else if op1.is_gr32() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Add_r32_rm32, (*op1).into(), op2.into())?
                        } else if op1.is_gr64() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Add_r64_rm64, (*op1).into(), op2.into())?
                        } else { todo!() }
                    } else if let Some(Operand::Imm(imm)) = &self.op2 {
                        if op1.is_gr8() {
                            Instruction::with2::<Register, i32>(Code::Add_rm8_imm8, (*op1).into(), *imm as i32)?
                        } else if op1.is_gr16() {
                            Instruction::with2::<Register, i32>(Code::Add_rm16_imm16, (*op1).into(), *imm as i32)?
                        } else if op1.is_gr32() {
                            Instruction::with2::<Register, i32>(Code::Add_rm32_imm32, (*op1).into(), *imm as i32)?
                        } else if op1.is_gr64() {
                            Instruction::with2::<Register, i32>(Code::Add_rm64_imm32, (*op1).into(), *imm as i32)?
                        } else { todo!() }
                    } else { todo!() }
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
                        } else { todo!() }
                    } else { todo!() }
                } else { todo!() }
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
                        } else { todo!() }
                    } else if let Some(Operand::Mem(op2)) = &self.op2 {
                        if op1.is_gr8() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Adc_r8_rm8, (*op1).into(), op2.into())?
                        } else if op1.is_gr16() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Adc_r16_rm16, (*op1).into(), op2.into())?
                        } else if op1.is_gr32() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Adc_r32_rm32, (*op1).into(), op2.into())?
                        } else if op1.is_gr64() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Adc_r64_rm64, (*op1).into(), op2.into())?
                        } else { todo!() }
                    } else if let Some(Operand::Imm(imm)) = &self.op2 {
                        if op1.is_gr8() {
                            Instruction::with2::<Register, i32>(Code::Adc_rm8_imm8, (*op1).into(), *imm as i32)?
                        } else if op1.is_gr16() {
                            Instruction::with2::<Register, i32>(Code::Adc_rm16_imm16, (*op1).into(), *imm as i32)?
                        } else if op1.is_gr32() {
                            Instruction::with2::<Register, i32>(Code::Adc_rm32_imm32, (*op1).into(), *imm as i32)?
                        } else if op1.is_gr64() {
                            Instruction::with2::<Register, i32>(Code::Adc_rm64_imm32, (*op1).into(), *imm as i32)?
                        } else { todo!() }
                    } else { todo!() }
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
                        } else { todo!() }
                    } else { todo!() }
                } else { todo!() }
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
                        } else { todo!() }
                    } else if let Some(Operand::Mem(op2)) = &self.op2 {
                        if op1.is_gr8() {
                            Instruction::with2::<Register, MemoryOperand>(Code::And_r8_rm8, (*op1).into(), op2.into())?
                        } else if op1.is_gr16() {
                            Instruction::with2::<Register, MemoryOperand>(Code::And_r16_rm16, (*op1).into(), op2.into())?
                        } else if op1.is_gr32() {
                            Instruction::with2::<Register, MemoryOperand>(Code::And_r32_rm32, (*op1).into(), op2.into())?
                        } else if op1.is_gr64() {
                            Instruction::with2::<Register, MemoryOperand>(Code::And_r64_rm64, (*op1).into(), op2.into())?
                        } else { todo!() }
                    } else if let Some(Operand::Imm(imm)) = &self.op2 {
                        if op1.is_gr8() {
                            Instruction::with2::<Register, i32>(Code::And_rm8_imm8, (*op1).into(), *imm as i32)?
                        } else if op1.is_gr16() {
                            Instruction::with2::<Register, i32>(Code::And_rm16_imm16, (*op1).into(), *imm as i32)?
                        } else if op1.is_gr32() {
                            Instruction::with2::<Register, i32>(Code::And_rm32_imm32, (*op1).into(), *imm as i32)?
                        } else if op1.is_gr64() {
                            Instruction::with2::<Register, i32>(Code::And_rm64_imm32, (*op1).into(), *imm as i32)?
                        } else { todo!() }
                    } else { todo!() }
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
                        } else { todo!() }
                    } else { todo!() }
                } else { todo!() }
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
                        } else { todo!() }
                    } else if let Some(Operand::Mem(op2)) = &self.op2 {
                        if op1.is_gr8() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Or_r8_rm8, (*op1).into(), op2.into())?
                        } else if op1.is_gr16() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Or_r16_rm16, (*op1).into(), op2.into())?
                        } else if op1.is_gr32() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Or_r32_rm32, (*op1).into(), op2.into())?
                        } else if op1.is_gr64() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Or_r64_rm64, (*op1).into(), op2.into())?
                        } else { todo!() }
                    } else if let Some(Operand::Imm(imm)) = &self.op2 {
                        if op1.is_gr8() {
                            Instruction::with2::<Register, i32>(Code::Or_rm8_imm8, (*op1).into(), *imm as i32)?
                        } else if op1.is_gr16() {
                            Instruction::with2::<Register, i32>(Code::Or_rm16_imm16, (*op1).into(), *imm as i32)?
                        } else if op1.is_gr32() {
                            Instruction::with2::<Register, i32>(Code::Or_rm32_imm32, (*op1).into(), *imm as i32)?
                        } else if op1.is_gr64() {
                            Instruction::with2::<Register, i32>(Code::Or_rm64_imm32, (*op1).into(), *imm as i32)?
                        } else { todo!() }
                    } else { todo!() }
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
                        } else { todo!() }
                    } else { todo!() }
                } else { todo!() }
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
                        } else { todo!() }
                    } else if let Some(Operand::Mem(op2)) = &self.op2 {
                        if op1.is_gr8() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Xor_r8_rm8, (*op1).into(), op2.into())?
                        } else if op1.is_gr16() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Xor_r16_rm16, (*op1).into(), op2.into())?
                        } else if op1.is_gr32() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Xor_r32_rm32, (*op1).into(), op2.into())?
                        } else if op1.is_gr64() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Xor_r64_rm64, (*op1).into(), op2.into())?
                        } else { todo!() }
                    } else if let Some(Operand::Imm(imm)) = &self.op2 {
                        if op1.is_gr8() {
                            Instruction::with2::<Register, i32>(Code::Xor_rm8_imm8, (*op1).into(), *imm as i32)?
                        } else if op1.is_gr16() {
                            Instruction::with2::<Register, i32>(Code::Xor_rm16_imm16, (*op1).into(), *imm as i32)?
                        } else if op1.is_gr32() {
                            Instruction::with2::<Register, i32>(Code::Xor_rm32_imm32, (*op1).into(), *imm as i32)?
                        } else if op1.is_gr64() {
                            Instruction::with2::<Register, i32>(Code::Xor_rm64_imm32, (*op1).into(), *imm as i32)?
                        } else { todo!() }
                    } else { todo!() }
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
                        } else { todo!() }
                    } else { todo!() }
                } else { todo!() }
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
                        } else { todo!() }
                    } else if let Some(Operand::Mem(op2)) = &self.op2 {
                        if op1.is_gr8() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Sub_r8_rm8, (*op1).into(), op2.into())?
                        } else if op1.is_gr16() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Sub_r16_rm16, (*op1).into(), op2.into())?
                        } else if op1.is_gr32() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Sub_r32_rm32, (*op1).into(), op2.into())?
                        } else if op1.is_gr64() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Sub_r64_rm64, (*op1).into(), op2.into())?
                        } else { todo!() }
                    } else if let Some(Operand::Imm(imm)) = &self.op2 {
                        if op1.is_gr8() {
                            Instruction::with2::<Register, i32>(Code::Sub_rm8_imm8, (*op1).into(), *imm as i32)?
                        } else if op1.is_gr16() {
                            Instruction::with2::<Register, i32>(Code::Sub_rm16_imm16, (*op1).into(), *imm as i32)?
                        } else if op1.is_gr32() {
                            Instruction::with2::<Register, i32>(Code::Sub_rm32_imm32, (*op1).into(), *imm as i32)?
                        } else if op1.is_gr64() {
                            Instruction::with2::<Register, i32>(Code::Sub_rm64_imm32, (*op1).into(), *imm as i32)?
                        } else { todo!() }
                    } else { todo!() }
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
                        } else { todo!() }
                    } else { todo!() }
                } else { todo!() }
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
                    } else { todo!() }
                } else if let Some(Operand::Mem(op1)) = &self.op1 {
                    Instruction::with1::<MemoryOperand>(Code::Neg_rm64, op1.into())?
                } else { todo!() }
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
                        } else { todo!() }
                    } else if let Some(Operand::Mem(op2)) = &self.op2 {
                        if op1.is_gr8() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Cmp_r8_rm8, (*op1).into(), op2.into())?
                        } else if op1.is_gr16() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Cmp_r16_rm16, (*op1).into(), op2.into())?
                        } else if op1.is_gr32() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Cmp_r32_rm32, (*op1).into(), op2.into())?
                        } else if op1.is_gr64() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Cmp_r64_rm64, (*op1).into(), op2.into())?
                        } else { todo!() }
                    } else if let Some(Operand::Imm(imm)) = &self.op2 {
                        if op1.is_gr8() {
                            Instruction::with2::<Register, i32>(Code::Cmp_rm8_imm8, (*op1).into(), *imm as i32)?
                        } else if op1.is_gr16() {
                            Instruction::with2::<Register, i32>(Code::Cmp_rm16_imm16, (*op1).into(), *imm as i32)?
                        } else if op1.is_gr32() {
                            Instruction::with2::<Register, i32>(Code::Cmp_rm32_imm32, (*op1).into(), *imm as i32)?
                        } else if op1.is_gr64() {
                            Instruction::with2::<Register, i32>(Code::Cmp_rm64_imm32, (*op1).into(), *imm as i32)?
                        } else { todo!() }
                    } else { todo!() }
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
                        } else { todo!() }
                    } else { todo!() }
                } else { todo!() }
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
                        } else { todo!() }
                    } else { todo!() }
                } else { todo!() }
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
                        } else { todo!() }
                    } else if let Some(Operand::Mem(op2)) = &self.op2 {
                        if op1.is_gr8() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Mov_r8_rm8, (*op1).into(), op2.into())?
                        } else if op1.is_gr16() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Mov_r16_rm16, (*op1).into(), op2.into())?
                        } else if op1.is_gr32() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Mov_r32_rm32, (*op1).into(), op2.into())?
                        } else if op1.is_gr64() {
                            Instruction::with2::<Register, MemoryOperand>(Code::Mov_r64_rm64, (*op1).into(), op2.into())?
                        } else { todo!() }
                    } else if let Some(Operand::Imm(imm)) = &self.op2 {
                        if op1.is_gr8() {
                            Instruction::with2::<Register, i32>(Code::Mov_rm8_imm8, (*op1).into(), *imm as i32)?
                        } else if op1.is_gr16() {
                            Instruction::with2::<Register, i32>(Code::Mov_rm16_imm16, (*op1).into(), *imm as i32)?
                        } else if op1.is_gr32() {
                            Instruction::with2::<Register, i32>(Code::Mov_rm32_imm32, (*op1).into(), *imm as i32)?
                        } else if op1.is_gr64() {
                            Instruction::with2::<Register, i32>(Code::Mov_rm64_imm32, (*op1).into(), *imm as i32)?
                        } else { todo!() }
                    } else { todo!() }
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
                        } else { todo!() }
                    } else { todo!() }
                } else { todo!() }
            },
            Mnemonic::Movzx => todo!(),
            Mnemonic::Push => {
                if let Some(Operand::Reg(op1)) = &self.op1 {
                    if op1.is_gr64() {
                        Instruction::with1::<Register>(Code::Push_r64, (*op1).into())?
                    } else if op1.is_gr32() {
                        Instruction::with1::<Register>(Code::Push_r32, (*op1).into())?
                    } else if op1.is_gr16() {
                        Instruction::with1::<Register>(Code::Push_r16, (*op1).into())?
                    } else { todo!()}
                } else if let Some(Operand::Mem(op1)) = &self.op1 {
                    Instruction::with1::<MemoryOperand>(Code::Push_rm64, op1.into())?
                } else if let Some(Operand::Imm(imm)) = &self.op1 {
                    Instruction::with1(Code::Pushd_imm32, *imm as i32)?
                } else { todo!() }
            },
            Mnemonic::Pop => {
                if let Some(Operand::Reg(op1)) = &self.op1 {
                    if op1.is_gr64() {
                        Instruction::with1::<Register>(Code::Pop_r64, (*op1).into())?
                    } else if op1.is_gr32() {
                        Instruction::with1::<Register>(Code::Pop_r32, (*op1).into())?
                    } else if op1.is_gr16() {
                        Instruction::with1::<Register>(Code::Pop_r16, (*op1).into())?
                    } else { todo!()}
                } else if let Some(Operand::Mem(op1)) = &self.op1 {
                    Instruction::with1::<MemoryOperand>(Code::Pop_rm64, op1.into())?
                } else { todo!() }
            },
            Mnemonic::Ret => Instruction::with(Code::Retnw),
            Mnemonic::Imul => todo!(),
            Mnemonic::Mul => todo!(),
            Mnemonic::Idiv => todo!(),
            Mnemonic::Div => todo!(),
            Mnemonic::Call => todo!(),
            Mnemonic::Jmp => todo!(),
            Mnemonic::Jne => todo!(),
            Mnemonic::Je => todo!(),
            Mnemonic::Endbr64 => Instruction::with(Code::Endbr64),
            Mnemonic::Link => todo!(),
            Mnemonic::Debug => todo!(),
            Mnemonic::StartOptimization => todo!(),
            Mnemonic::EndOptimization => todo!(),
            Mnemonic::Sete => todo!(),
            Mnemonic::Setne => todo!(),
            Mnemonic::Setg => todo!(),
            Mnemonic::Setl => todo!(),
            Mnemonic::Setge => todo!(),
            Mnemonic::Setle => todo!(),
            Mnemonic::Cmove => todo!(),
            Mnemonic::Cmovne => todo!(),
            Mnemonic::Sal => todo!(),
            Mnemonic::Shr => todo!(),
            Mnemonic::Sar => todo!(),
            Mnemonic::Movq => todo!(),
            Mnemonic::Movd => todo!(),
            Mnemonic::Movss => todo!(),
            Mnemonic::Movsd => todo!(),
            Mnemonic::Movups => todo!(),
            Mnemonic::Movupd => todo!(),
            Mnemonic::Addss => todo!(),
            Mnemonic::Addsd => todo!(),
            Mnemonic::Divss => todo!(),
            Mnemonic::Divsd => todo!(),
            Mnemonic::Mulss => todo!(),
            Mnemonic::Mulsd => todo!(),
            Mnemonic::Subss => todo!(),
            Mnemonic::Subsd => todo!(),
            Mnemonic::Ucomiss => todo!(),
            Mnemonic::Ucomisd => todo!(),
        
        };
        
        let binding = [instr];
        let instr = InstructionBlock::new(&binding, 0);

        let encoder = BlockEncoder::encode(64, instr, BlockEncoderOptions::NONE)?;

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

        MemoryOperand::new(
            base, 
            index, 
            self.scale as u32, 
            self.displ as i64, 
            4, 
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

impl Into<Register> for x64Reg {
    fn into(self) -> Register {
        match self {
            x64Reg::Rax => Register::RAX,
            x64Reg::Eax => Register::EAX,
            x64Reg::Ax =>  Register::AX,
            x64Reg::Al =>  Register::AL,

            x64Reg::Rbx => Register::RBX,
            x64Reg::Ebx => Register::EBX,
            x64Reg::Bx =>  Register::BX,
            x64Reg::Bl =>  Register::BL,

            x64Reg::Rcx => Register::RCX,
            x64Reg::Ecx => Register::ECX,
            x64Reg::Cx =>  Register::CX,
            x64Reg::Cl =>  Register::CL,

            x64Reg::Rdx => Register::RDX,
            x64Reg::Edx => Register::EDX,
            x64Reg::Dx =>  Register::DX,
            x64Reg::Dl =>  Register::DL,

            x64Reg::Rsi => Register::RSI,
            x64Reg::Esi => Register::ESI,
            x64Reg::Si =>  Register::SI,
            x64Reg::Sil => Register::SIL,

            x64Reg::Rdi => Register::RDI,
            x64Reg::Edi => Register::EDI,
            x64Reg::Di =>  Register::DI,
            x64Reg::Dil => Register::DIL,

            x64Reg::Rsp => Register::RSP,
            x64Reg::Esp => Register::ESP,
            x64Reg::Sp =>  Register::SP,
            x64Reg::Spl => Register::SPL,

            x64Reg::Rbp => Register::RBP,
            x64Reg::Ebp => Register::EBP,
            x64Reg::Bp =>  Register::BP,
            x64Reg::Bpl => Register::BPL,

            x64Reg::R8 =>  Register::R8,
            x64Reg::R8d => Register::R8D,
            x64Reg::R8w => Register::R8W,
            x64Reg::R8b => Register::R8L,

            x64Reg::R9 =>  Register::R9,
            x64Reg::R9d => Register::R9D,
            x64Reg::R9w => Register::R9W,
            x64Reg::R9b => Register::R9L,

            x64Reg::R10 =>   Register::R10,
            x64Reg::R10d =>  Register::R10D,
            x64Reg::R10w =>  Register::R10W,
            x64Reg::R10b =>  Register::R10L,

            x64Reg::R11 =>  Register::R11,
            x64Reg::R11d => Register::R11D,
            x64Reg::R11w => Register::R11W,
            x64Reg::R11b => Register::R11L,

            x64Reg::R12 =>  Register::R12,
            x64Reg::R12d => Register::R12D,
            x64Reg::R12w => Register::R12W,
            x64Reg::R12b => Register::R12L,

            x64Reg::R13 =>  Register::R13,
            x64Reg::R13d => Register::R13D,
            x64Reg::R13w => Register::R13W,
            x64Reg::R13b => Register::R13L,

            x64Reg::R14 =>  Register::R14,
            x64Reg::R14d => Register::R14D,
            x64Reg::R14w => Register::R14W,
            x64Reg::R14b => Register::R14L,

            x64Reg::R15 =>  Register::R15,
            x64Reg::R15d => Register::R15D,
            x64Reg::R15w => Register::R15W,
            x64Reg::R15b => Register::R15L,

            x64Reg::Xmm0 => Register::XMM0,
            x64Reg::Xmm1 => Register::XMM1,
            x64Reg::Xmm2 => Register::XMM2,
            x64Reg::Xmm3 => Register::XMM3,
            x64Reg::Xmm4 => Register::XMM4,
            x64Reg::Xmm5 => Register::XMM5,
            x64Reg::Xmm6 => Register::XMM6,
            x64Reg::Xmm7 => Register::XMM7,
            x64Reg::Xmm8 => Register::XMM8,
            x64Reg::Xmm9 => Register::XMM9,
            x64Reg::Xmm10 => Register::XMM10,
            x64Reg::Xmm11 => Register::XMM11,
            x64Reg::Xmm12 => Register::XMM12,
            x64Reg::Xmm13 => Register::XMM13,
            x64Reg::Xmm14 => Register::XMM14,
            x64Reg::Xmm15 => Register::XMM15,
        }
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