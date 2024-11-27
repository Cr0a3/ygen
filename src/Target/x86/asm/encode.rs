use iced_x86::{BlockEncoder, BlockEncoderOptions, Instruction, InstructionBlock, Code};

use super::*;

impl X86Instr {
    /// Encodes the x86 instruction
    pub fn encode(&self) -> Vec<u8> {
        match self.mnemonic {
            X86Mnemonic::Mov => self.encode_mov(),
            X86Mnemonic::Movss => self.encode_movss(),
            X86Mnemonic::Movsd => self.encode_movsd(),
            X86Mnemonic::Movdqa =>self.encode_movdqa(),
            X86Mnemonic::Ret => self.encode_ret(),
            X86Mnemonic::Add => self.encode_add(),
            X86Mnemonic::Paddq => self.encode_paddq(),
            X86Mnemonic::Paddd => self.encode_paddd(),
            X86Mnemonic::Sub => self.encode_sub(),
            X86Mnemonic::Psubq => self.encode_psubq(),
            X86Mnemonic::Psubd => self.encode_psubd(),
            X86Mnemonic::Psubw => self.encode_psubw(),
            X86Mnemonic::Psubb => self.encode_psubb(),
            X86Mnemonic::Lea => self.encode_lea(),
            X86Mnemonic::Jmp => self.encode_jmp(),
            X86Mnemonic::Sete => self.encode_sete(),
            X86Mnemonic::Setne => self.encode_setne(),
            X86Mnemonic::Setl => self.encode_setl(),
            X86Mnemonic::Setle => self.encode_setle(),
            X86Mnemonic::Setg => self.encode_setg(),
            X86Mnemonic::Setge => self.encode_setge(),
            X86Mnemonic::Cmp => self.encode_cmp(),
            X86Mnemonic::Pinsrb => self.encode_pinsrb(),
            X86Mnemonic::Pinsrw => self.encode_pinsrw(),
            X86Mnemonic::Pinsrd => self.encode_pinsrd(),
            X86Mnemonic::Pinsrq => self.encode_pinsrq(),
            X86Mnemonic::Insertps => self.encode_insertps(),
        }
    }

    fn encode_mov(&self) -> Vec<u8> {
        let dst = self.op1.expect("expected dst");
        let src = self.op2.expect("expected src");
        
        let instr = match (dst, src) {
            (X86Operand::Reg(dst), X86Operand::Reg(src)) => {
                match dst.size {
                    X86RegSize::Byte => Instruction::with2::<iced_x86::Register, iced_x86::Register>(Code::Mov_r8_rm8, dst.into(), src.into()),
                    X86RegSize::Word => Instruction::with2::<iced_x86::Register, iced_x86::Register>(Code::Mov_r16_rm16, dst.into(), src.into()),
                    X86RegSize::Dword => Instruction::with2::<iced_x86::Register, iced_x86::Register>(Code::Mov_r32_rm32, dst.into(), src.into()),
                    X86RegSize::Qword => Instruction::with2::<iced_x86::Register, iced_x86::Register>(Code::Mov_r64_rm64, dst.into(), src.into()),
                    X86RegSize::SimdVec => panic!("mov deosn't support simd vecs"),
                }
            },
            (X86Operand::Reg(dst), X86Operand::Const(src)) => {
                match dst.size {
                    X86RegSize::Byte => Instruction::with2::<iced_x86::Register, i32>(Code::Mov_r8_imm8, dst.into(), src as i32),
                    X86RegSize::Word => Instruction::with2::<iced_x86::Register, i32>(Code::Mov_r16_imm16, dst.into(), src as i32),
                    X86RegSize::Dword => Instruction::with2::<iced_x86::Register, i32>(Code::Mov_r32_imm32, dst.into(), src as i32),
                    X86RegSize::Qword => Instruction::with2::<iced_x86::Register, i32>(Code::Mov_r64_imm64, dst.into(), src as i32),
                    X86RegSize::SimdVec => panic!("mov deosn't support simd vecs"),
                }
            },
            (X86Operand::Reg(dst), X86Operand::MemDispl(src)) => {
                match dst.size {
                    X86RegSize::Byte => Instruction::with2::<iced_x86::Register, iced_x86::MemoryOperand>(Code::Mov_r8_rm8, dst.into(), src.into()),
                    X86RegSize::Word => Instruction::with2::<iced_x86::Register, iced_x86::MemoryOperand>(Code::Mov_r16_rm16, dst.into(), src.into()),
                    X86RegSize::Dword => Instruction::with2::<iced_x86::Register, iced_x86::MemoryOperand>(Code::Mov_r32_rm32, dst.into(), src.into()),
                    X86RegSize::Qword => Instruction::with2::<iced_x86::Register, iced_x86::MemoryOperand>(Code::Mov_r64_rm64, dst.into(), src.into()),
                    X86RegSize::SimdVec => panic!("mov deosn't support simd vecs"),
                }
            },
            (X86Operand::MemDispl(dst), X86Operand::Reg(src)) => {
                match dst.size {
                    X86RegSize::Byte => Instruction::with2::<iced_x86::MemoryOperand, iced_x86::Register>(Code::Mov_rm8_r8, dst.into(), src.into()),
                    X86RegSize::Word => Instruction::with2::<iced_x86::MemoryOperand, iced_x86::Register>(Code::Mov_rm16_r16, dst.into(), src.into()),
                    X86RegSize::Dword => Instruction::with2::<iced_x86::MemoryOperand, iced_x86::Register>(Code::Mov_rm32_r32, dst.into(), src.into()),
                    X86RegSize::Qword => Instruction::with2::<iced_x86::MemoryOperand, iced_x86::Register>(Code::Mov_rm64_r64, dst.into(), src.into()),
                    X86RegSize::SimdVec => panic!("mov deosn't support simd vecs"),
                }
            },
            (X86Operand::MemDispl(dst), X86Operand::Const(src)) => {
                match dst.size {
                    X86RegSize::Byte => Instruction::with2::<iced_x86::MemoryOperand, i32>(Code::Mov_rm8_imm8, dst.into(), src as i32),
                    X86RegSize::Word => Instruction::with2::<iced_x86::MemoryOperand, i32>(Code::Mov_rm16_imm16, dst.into(), src as i32),
                    X86RegSize::Dword => Instruction::with2::<iced_x86::MemoryOperand, i32>(Code::Mov_rm32_imm32, dst.into(), src as i32),
                    X86RegSize::Qword => Instruction::with2::<iced_x86::MemoryOperand, i32>(Code::Mov_rm64_imm32, dst.into(), src as i32),
                    X86RegSize::SimdVec => panic!("mov deosn't support simd vecs"),
                }
            },

            _ => panic!("invalid variant: {self} (maybe unresolved tmps?)"),
        }.expect("invalid instruction");

        BlockEncoder::encode(
            64, 
            InstructionBlock::new(&[instr], 0), 
            BlockEncoderOptions::DONT_FIX_BRANCHES
        ).expect("encoding error").code_buffer
    }

    fn encode_movss(&self) -> Vec<u8> {
        let dst = self.op1.expect("expected dst");
        let src = self.op2.expect("expected src");

        let instr = match (dst, src) {
            (X86Operand::Reg(dst), X86Operand::Reg(src)) => Instruction::with2::<iced_x86::Register, iced_x86::Register>(Code::Movss_xmm_xmmm32, dst.into(), src.into()),
            (X86Operand::Reg(dst), X86Operand::MemDispl(src)) => Instruction::with2::<iced_x86::Register, iced_x86::MemoryOperand>(Code::Movss_xmm_xmmm32, dst.into(), src.into()),
            (X86Operand::MemDispl(dst), X86Operand::Reg(src)) => Instruction::with2::<iced_x86::MemoryOperand, iced_x86::Register>(Code::Movss_xmmm32_xmm, dst.into(), src.into()),

            _ => panic!("illegal variant: {self} (maybe unsresolved tmps?)"),
        }.expect("invalid instruction");

        BlockEncoder::encode(
            64, 
            InstructionBlock::new(&[instr], 0), 
            BlockEncoderOptions::DONT_FIX_BRANCHES
        ).expect("encoding error").code_buffer
    }

    fn encode_movsd(&self) -> Vec<u8> {
        let dst = self.op1.expect("expected dst");
        let src = self.op2.expect("expected src");

        let instr = match (dst, src) {
            (X86Operand::Reg(dst), X86Operand::Reg(src)) => Instruction::with2::<iced_x86::Register, iced_x86::Register>(Code::Movsd_xmm_xmmm64, dst.into(), src.into()),
            (X86Operand::Reg(dst), X86Operand::MemDispl(src)) => Instruction::with2::<iced_x86::Register, iced_x86::MemoryOperand>(Code::Movsd_xmm_xmmm64, dst.into(), src.into()),
            (X86Operand::MemDispl(dst), X86Operand::Reg(src)) => Instruction::with2::<iced_x86::MemoryOperand, iced_x86::Register>(Code::Movsd_xmmm64_xmm, dst.into(), src.into()),

            _ => panic!("illegal variant: {self} (maybe unsresolved tmps?)"),
        }.expect("invalid instruction");

        BlockEncoder::encode(
            64, 
            InstructionBlock::new(&[instr], 0), 
            BlockEncoderOptions::DONT_FIX_BRANCHES
        ).expect("encoding error").code_buffer
    }

    fn encode_movdqa(&self) -> Vec<u8> {
        let dst = self.op1.expect("expected dst");
        let src = self.op2.expect("expected src");

        let instr = match (dst, src) {
            (X86Operand::Reg(dst), X86Operand::Reg(src)) => Instruction::with2::<iced_x86::Register, iced_x86::Register>(Code::Movdqa_xmm_xmmm128, dst.into(), src.into()),
            (X86Operand::Reg(dst), X86Operand::MemDispl(src)) => Instruction::with2::<iced_x86::Register, iced_x86::MemoryOperand>(Code::Movdqa_xmm_xmmm128, dst.into(), src.into()),
            (X86Operand::MemDispl(dst), X86Operand::Reg(src)) => Instruction::with2::<iced_x86::MemoryOperand, iced_x86::Register>(Code::Movdqa_xmmm128_xmm, dst.into(), src.into()),

            _ => panic!("illegal variant: {self} (maybe unsresolved tmps?)"),
        }.expect("invalid instruction");

        BlockEncoder::encode(
            64, 
            InstructionBlock::new(&[instr], 0), 
            BlockEncoderOptions::DONT_FIX_BRANCHES
        ).expect("encoding error").code_buffer
    }

    fn encode_ret(&self) -> Vec<u8> {
        vec![0xC3]
    }

    fn encode_add(&self) -> Vec<u8> {
        let dst = self.op1.expect("expected dst");
        let src = self.op2.expect("expected src");
        
        let instr = match (dst, src) {
            (X86Operand::Reg(dst), X86Operand::Reg(src)) => {
                match dst.size {
                    X86RegSize::Byte => Instruction::with2::<iced_x86::Register, iced_x86::Register>(Code::Add_r8_rm8, dst.into(), src.into()),
                    X86RegSize::Word => Instruction::with2::<iced_x86::Register, iced_x86::Register>(Code::Add_r16_rm16, dst.into(), src.into()),
                    X86RegSize::Dword => Instruction::with2::<iced_x86::Register, iced_x86::Register>(Code::Add_r32_rm32, dst.into(), src.into()),
                    X86RegSize::Qword => Instruction::with2::<iced_x86::Register, iced_x86::Register>(Code::Add_r64_rm64, dst.into(), src.into()),
                    X86RegSize::SimdVec => panic!("add deosn't support simd vecs"),
                }
            },
            (X86Operand::Reg(dst), X86Operand::Const(src)) => {
                match dst.size {
                    X86RegSize::Byte => Instruction::with2::<iced_x86::Register, i32>(Code::Add_rm8_imm8, dst.into(), src as i32),
                    X86RegSize::Word => Instruction::with2::<iced_x86::Register, i32>(Code::Add_rm16_imm16, dst.into(), src as i32),
                    X86RegSize::Dword => Instruction::with2::<iced_x86::Register, i32>(Code::Add_rm32_imm32, dst.into(), src as i32),
                    X86RegSize::Qword => Instruction::with2::<iced_x86::Register, i32>(Code::Add_rm64_imm32, dst.into(), src as i32),
                    X86RegSize::SimdVec => panic!("add deosn't support simd vecs"),
                }
            },
            (X86Operand::Reg(dst), X86Operand::MemDispl(src)) => {
                match dst.size {
                    X86RegSize::Byte => Instruction::with2::<iced_x86::Register, iced_x86::MemoryOperand>(Code::Add_r8_rm8, dst.into(), src.into()),
                    X86RegSize::Word => Instruction::with2::<iced_x86::Register, iced_x86::MemoryOperand>(Code::Add_r16_rm16, dst.into(), src.into()),
                    X86RegSize::Dword => Instruction::with2::<iced_x86::Register, iced_x86::MemoryOperand>(Code::Add_r32_rm32, dst.into(), src.into()),
                    X86RegSize::Qword => Instruction::with2::<iced_x86::Register, iced_x86::MemoryOperand>(Code::Add_r64_rm64, dst.into(), src.into()),
                    X86RegSize::SimdVec => panic!("add deosn't support simd vecs"),
                }
            },
            (X86Operand::MemDispl(dst), X86Operand::Reg(src)) => {
                match dst.size {
                    X86RegSize::Byte => Instruction::with2::<iced_x86::MemoryOperand, iced_x86::Register>(Code::Add_rm8_r8, dst.into(), src.into()),
                    X86RegSize::Word => Instruction::with2::<iced_x86::MemoryOperand, iced_x86::Register>(Code::Add_rm16_r16, dst.into(), src.into()),
                    X86RegSize::Dword => Instruction::with2::<iced_x86::MemoryOperand, iced_x86::Register>(Code::Add_rm32_r32, dst.into(), src.into()),
                    X86RegSize::Qword => Instruction::with2::<iced_x86::MemoryOperand, iced_x86::Register>(Code::Add_rm64_r64, dst.into(), src.into()),
                    X86RegSize::SimdVec => panic!("add deosn't support simd vecs"),
                }
            },
            (X86Operand::MemDispl(dst), X86Operand::Const(src)) => {
                match dst.size {
                    X86RegSize::Byte => Instruction::with2::<iced_x86::MemoryOperand, i32>(Code::Add_rm8_imm8, dst.into(), src as i32),
                    X86RegSize::Word => Instruction::with2::<iced_x86::MemoryOperand, i32>(Code::Add_rm16_imm16, dst.into(), src as i32),
                    X86RegSize::Dword => Instruction::with2::<iced_x86::MemoryOperand, i32>(Code::Add_rm32_imm32, dst.into(), src as i32),
                    X86RegSize::Qword => Instruction::with2::<iced_x86::MemoryOperand, i32>(Code::Add_rm64_imm32, dst.into(), src as i32),
                    X86RegSize::SimdVec => panic!("add deosn't support simd vecs"),
                }
            },

            _ => panic!("invalid variant: {self} (maybe unresolved tmps?)"),
        }.expect("invalid instruction");

        BlockEncoder::encode(
            64, 
            InstructionBlock::new(&[instr], 0), 
            BlockEncoderOptions::DONT_FIX_BRANCHES
        ).expect("encoding error").code_buffer
    }

    fn encode_paddq(&self) -> Vec<u8> {
        todo!()
    }

    fn encode_paddd(&self) -> Vec<u8> {
        todo!()
    }

    fn encode_sub(&self) -> Vec<u8> {
        let dst = self.op1.expect("expected dst");
        let src = self.op2.expect("expected src");
        
        let instr = match (dst, src) {
            (X86Operand::Reg(dst), X86Operand::Reg(src)) => {
                match dst.size {
                    X86RegSize::Byte => Instruction::with2::<iced_x86::Register, iced_x86::Register>(Code::Sub_r8_rm8, dst.into(), src.into()),
                    X86RegSize::Word => Instruction::with2::<iced_x86::Register, iced_x86::Register>(Code::Sub_r16_rm16, dst.into(), src.into()),
                    X86RegSize::Dword => Instruction::with2::<iced_x86::Register, iced_x86::Register>(Code::Sub_r32_rm32, dst.into(), src.into()),
                    X86RegSize::Qword => Instruction::with2::<iced_x86::Register, iced_x86::Register>(Code::Sub_r64_rm64, dst.into(), src.into()),
                    X86RegSize::SimdVec => panic!("sub deosn't support simd vecs"),
                }
            },
            (X86Operand::Reg(dst), X86Operand::Const(src)) => {
                match dst.size {
                    X86RegSize::Byte => Instruction::with2::<iced_x86::Register, i32>(Code::Sub_rm8_imm8, dst.into(), src as i32),
                    X86RegSize::Word => Instruction::with2::<iced_x86::Register, i32>(Code::Sub_rm16_imm16, dst.into(), src as i32),
                    X86RegSize::Dword => Instruction::with2::<iced_x86::Register, i32>(Code::Sub_rm32_imm32, dst.into(), src as i32),
                    X86RegSize::Qword => Instruction::with2::<iced_x86::Register, i32>(Code::Sub_rm64_imm32, dst.into(), src as i32),
                    X86RegSize::SimdVec => panic!("sub deosn't support simd vecs"),
                }
            },
            (X86Operand::Reg(dst), X86Operand::MemDispl(src)) => {
                match dst.size {
                    X86RegSize::Byte => Instruction::with2::<iced_x86::Register, iced_x86::MemoryOperand>(Code::Sub_r8_rm8, dst.into(), src.into()),
                    X86RegSize::Word => Instruction::with2::<iced_x86::Register, iced_x86::MemoryOperand>(Code::Sub_r16_rm16, dst.into(), src.into()),
                    X86RegSize::Dword => Instruction::with2::<iced_x86::Register, iced_x86::MemoryOperand>(Code::Sub_r32_rm32, dst.into(), src.into()),
                    X86RegSize::Qword => Instruction::with2::<iced_x86::Register, iced_x86::MemoryOperand>(Code::Sub_r64_rm64, dst.into(), src.into()),
                    X86RegSize::SimdVec => panic!("sub deosn't support simd vecs"),
                }
            },
            (X86Operand::MemDispl(dst), X86Operand::Reg(src)) => {
                match dst.size {
                    X86RegSize::Byte => Instruction::with2::<iced_x86::MemoryOperand, iced_x86::Register>(Code::Sub_rm8_r8, dst.into(), src.into()),
                    X86RegSize::Word => Instruction::with2::<iced_x86::MemoryOperand, iced_x86::Register>(Code::Sub_rm16_r16, dst.into(), src.into()),
                    X86RegSize::Dword => Instruction::with2::<iced_x86::MemoryOperand, iced_x86::Register>(Code::Sub_rm32_r32, dst.into(), src.into()),
                    X86RegSize::Qword => Instruction::with2::<iced_x86::MemoryOperand, iced_x86::Register>(Code::Sub_rm64_r64, dst.into(), src.into()),
                    X86RegSize::SimdVec => panic!("sub deosn't support simd vecs"),
                }
            },
            (X86Operand::MemDispl(dst), X86Operand::Const(src)) => {
                match dst.size {
                    X86RegSize::Byte => Instruction::with2::<iced_x86::MemoryOperand, i32>(Code::Sub_rm8_imm8, dst.into(), src as i32),
                    X86RegSize::Word => Instruction::with2::<iced_x86::MemoryOperand, i32>(Code::Sub_rm16_imm16, dst.into(), src as i32),
                    X86RegSize::Dword => Instruction::with2::<iced_x86::MemoryOperand, i32>(Code::Sub_rm32_imm32, dst.into(), src as i32),
                    X86RegSize::Qword => Instruction::with2::<iced_x86::MemoryOperand, i32>(Code::Sub_rm64_imm32, dst.into(), src as i32),
                    X86RegSize::SimdVec => panic!("szb deosn't support simd vecs"),
                }
            },

            _ => panic!("invalid variant: {self} (maybe unresolved tmps?)"),
        }.expect("invalid instruction");

        BlockEncoder::encode(
            64, 
            InstructionBlock::new(&[instr], 0), 
            BlockEncoderOptions::DONT_FIX_BRANCHES
        ).expect("encoding error").code_buffer
    }

    fn encode_psubq(&self) -> Vec<u8> {
        todo!()
    }

    fn encode_psubd(&self) -> Vec<u8> {
        todo!()
    }

    fn encode_psubw(&self) -> Vec<u8> {
        todo!()
    }

    fn encode_psubb(&self) -> Vec<u8> {
        todo!()
    }

    fn encode_lea(&self) -> Vec<u8> {
        let Some(X86Operand::Reg(dst)) = self.op1 else { panic!("invalid variant: {self}") };
        let Some(X86Operand::MemDispl(src)) = self.op2 else { panic!("invalid variant: {self}") };

        let instr = match dst.size {
            X86RegSize::Byte => Instruction::with2::<iced_x86::Register, iced_x86::MemoryOperand>(Code::Lea_r16_m, {
                // we need to modify the size of dst to be 16bits
                let mut dst = dst;
                dst.size = X86RegSize::Word;
                dst.into()
            }, src.into()),
            X86RegSize::Word => Instruction::with2::<iced_x86::Register, iced_x86::MemoryOperand>(Code::Lea_r16_m, dst.into(), src.into()),
            X86RegSize::Dword => Instruction::with2::<iced_x86::Register, iced_x86::MemoryOperand>(Code::Lea_r32_m, dst.into(), src.into()),
            X86RegSize::Qword => Instruction::with2::<iced_x86::Register, iced_x86::MemoryOperand>(Code::Lea_r64_m, dst.into(), src.into()),
            X86RegSize::SimdVec => panic!("invalid size for lea: {:?}", dst.size),
        }.expect("invalid instruction");

        BlockEncoder::encode(
            64, 
            InstructionBlock::new(&[instr], 0), 
            BlockEncoderOptions::DONT_FIX_BRANCHES
        ).expect("encoding error").code_buffer
    }

    fn encode_jmp(&self) -> Vec<u8> {
        let target = self.op1.expect("expected jump target");

        let instr = match target {
            X86Operand::Reg(reg) => Instruction::with1::<iced_x86::Register>(Code::Jmp_rm64, reg.into()),
            X86Operand::Const(imm) => Instruction::with1(Code::Jmp_rel32_64, imm as i32),
            X86Operand::MemDispl(mem) => Instruction::with1::<iced_x86::MemoryOperand>(Code::Jmp_rm64, mem.into()),
            X86Operand::BlockRel(_) => Instruction::with1(Code::Jmp_rel32_64, 0), // branch will be resolved later
            _ => panic!("invalid variant: {self}"),
        }.expect("invalid instruction");

        BlockEncoder::encode(
            64, 
            InstructionBlock::new(&[instr], 0), 
            BlockEncoderOptions::DONT_FIX_BRANCHES
        ).expect("encoding error").code_buffer
    }

    fn encode_sete(&self) -> Vec<u8> {
        let dst = self.op1.expect("expected op1");

        let instr = match dst {
            X86Operand::Reg(reg) => Instruction::with1::<iced_x86::Register>(Code::Sete_rm8, reg.into()),
            X86Operand::MemDispl(mem) => Instruction::with1::<iced_x86::MemoryOperand>(Code::Sete_rm8, mem.into()),
            _ => panic!("invalid variant: {self}"),
        }.expect("invalid instruction");

        BlockEncoder::encode(
            64, 
            InstructionBlock::new(&[instr], 0), 
            BlockEncoderOptions::DONT_FIX_BRANCHES
        ).expect("encoding error").code_buffer
    }

    fn encode_setne(&self) -> Vec<u8> {
        let dst = self.op1.expect("expected op1");

        let instr = match dst {
            X86Operand::Reg(reg) => Instruction::with1::<iced_x86::Register>(Code::Setne_rm8, reg.into()),
            X86Operand::MemDispl(mem) => Instruction::with1::<iced_x86::MemoryOperand>(Code::Setne_rm8, mem.into()),
            _ => panic!("invalid variant: {self}"),
        }.expect("invalid instruction");

        BlockEncoder::encode(
            64, 
            InstructionBlock::new(&[instr], 0), 
            BlockEncoderOptions::DONT_FIX_BRANCHES
        ).expect("encoding error").code_buffer
    }

    fn encode_setl(&self) -> Vec<u8> {
        let dst = self.op1.expect("expected op1");

        let instr = match dst {
            X86Operand::Reg(reg) => Instruction::with1::<iced_x86::Register>(Code::Setl_rm8, reg.into()),
            X86Operand::MemDispl(mem) => Instruction::with1::<iced_x86::MemoryOperand>(Code::Setl_rm8, mem.into()),
            _ => panic!("invalid variant: {self}"),
        }.expect("invalid instruction");

        BlockEncoder::encode(
            64, 
            InstructionBlock::new(&[instr], 0), 
            BlockEncoderOptions::DONT_FIX_BRANCHES
        ).expect("encoding error").code_buffer
    }

    fn encode_setle(&self) -> Vec<u8> {
        let dst = self.op1.expect("expected op1");

        let instr = match dst {
            X86Operand::Reg(reg) => Instruction::with1::<iced_x86::Register>(Code::Setle_rm8, reg.into()),
            X86Operand::MemDispl(mem) => Instruction::with1::<iced_x86::MemoryOperand>(Code::Setle_rm8, mem.into()),
            _ => panic!("invalid variant: {self}"),
        }.expect("invalid instruction");

        BlockEncoder::encode(
            64, 
            InstructionBlock::new(&[instr], 0), 
            BlockEncoderOptions::DONT_FIX_BRANCHES
        ).expect("encoding error").code_buffer
    }

    fn encode_setg(&self) -> Vec<u8> {
        let dst = self.op1.expect("expected op1");

        let instr = match dst {
            X86Operand::Reg(reg) => Instruction::with1::<iced_x86::Register>(Code::Setg_rm8, reg.into()),
            X86Operand::MemDispl(mem) => Instruction::with1::<iced_x86::MemoryOperand>(Code::Setg_rm8, mem.into()),
            _ => panic!("invalid variant: {self}"),
        }.expect("invalid instruction");

        BlockEncoder::encode(
            64, 
            InstructionBlock::new(&[instr], 0), 
            BlockEncoderOptions::DONT_FIX_BRANCHES
        ).expect("encoding error").code_buffer
    }

    fn encode_setge(&self) -> Vec<u8> {
        let dst = self.op1.expect("expected op1");

        let instr = match dst {
            X86Operand::Reg(reg) => Instruction::with1::<iced_x86::Register>(Code::Setge_rm8, reg.into()),
            X86Operand::MemDispl(mem) => Instruction::with1::<iced_x86::MemoryOperand>(Code::Setge_rm8, mem.into()),
            _ => panic!("invalid variant: {self}"),
        }.expect("invalid instruction");

        BlockEncoder::encode(
            64, 
            InstructionBlock::new(&[instr], 0), 
            BlockEncoderOptions::DONT_FIX_BRANCHES
        ).expect("encoding error").code_buffer
    }

    fn encode_cmp(&self) -> Vec<u8> {
        let dst = self.op1.expect("expected dst");
        let src = self.op2.expect("expected src");
        
        let instr = match (dst, src) {
            (X86Operand::Reg(dst), X86Operand::Reg(src)) => {
                match dst.size {
                    X86RegSize::Byte => Instruction::with2::<iced_x86::Register, iced_x86::Register>(Code::Cmp_r8_rm8, dst.into(), src.into()),
                    X86RegSize::Word => Instruction::with2::<iced_x86::Register, iced_x86::Register>(Code::Cmp_r16_rm16, dst.into(), src.into()),
                    X86RegSize::Dword => Instruction::with2::<iced_x86::Register, iced_x86::Register>(Code::Cmp_r32_rm32, dst.into(), src.into()),
                    X86RegSize::Qword => Instruction::with2::<iced_x86::Register, iced_x86::Register>(Code::Cmp_r64_rm64, dst.into(), src.into()),
                    X86RegSize::SimdVec => panic!("cmp deosn't support simd vecs"),
                }
            },
            (X86Operand::Reg(dst), X86Operand::Const(src)) => {
                match dst.size {
                    X86RegSize::Byte => Instruction::with2::<iced_x86::Register, i32>(Code::Cmp_rm8_imm8, dst.into(), src as i32),
                    X86RegSize::Word => Instruction::with2::<iced_x86::Register, i32>(Code::Cmp_rm16_imm16, dst.into(), src as i32),
                    X86RegSize::Dword => Instruction::with2::<iced_x86::Register, i32>(Code::Cmp_rm32_imm32, dst.into(), src as i32),
                    X86RegSize::Qword => Instruction::with2::<iced_x86::Register, i32>(Code::Cmp_rm64_imm32, dst.into(), src as i32),
                    X86RegSize::SimdVec => panic!("cmp deosn't support simd vecs"),
                }
            },
            (X86Operand::Reg(dst), X86Operand::MemDispl(src)) => {
                match dst.size {
                    X86RegSize::Byte => Instruction::with2::<iced_x86::Register, iced_x86::MemoryOperand>(Code::Cmp_r8_rm8, dst.into(), src.into()),
                    X86RegSize::Word => Instruction::with2::<iced_x86::Register, iced_x86::MemoryOperand>(Code::Cmp_r16_rm16, dst.into(), src.into()),
                    X86RegSize::Dword => Instruction::with2::<iced_x86::Register, iced_x86::MemoryOperand>(Code::Cmp_r32_rm32, dst.into(), src.into()),
                    X86RegSize::Qword => Instruction::with2::<iced_x86::Register, iced_x86::MemoryOperand>(Code::Cmp_r64_rm64, dst.into(), src.into()),
                    X86RegSize::SimdVec => panic!("cmp deosn't support simd vecs"),
                }
            },
            (X86Operand::MemDispl(dst), X86Operand::Reg(src)) => {
                match dst.size {
                    X86RegSize::Byte => Instruction::with2::<iced_x86::MemoryOperand, iced_x86::Register>(Code::Cmp_rm8_r8, dst.into(), src.into()),
                    X86RegSize::Word => Instruction::with2::<iced_x86::MemoryOperand, iced_x86::Register>(Code::Cmp_rm16_r16, dst.into(), src.into()),
                    X86RegSize::Dword => Instruction::with2::<iced_x86::MemoryOperand, iced_x86::Register>(Code::Cmp_rm32_r32, dst.into(), src.into()),
                    X86RegSize::Qword => Instruction::with2::<iced_x86::MemoryOperand, iced_x86::Register>(Code::Cmp_rm64_r64, dst.into(), src.into()),
                    X86RegSize::SimdVec => panic!("cmp deosn't support simd vecs"),
                }
            },
            (X86Operand::MemDispl(dst), X86Operand::Const(src)) => {
                match dst.size {
                    X86RegSize::Byte => Instruction::with2::<iced_x86::MemoryOperand, i32>(Code::Cmp_rm8_imm8, dst.into(), src as i32),
                    X86RegSize::Word => Instruction::with2::<iced_x86::MemoryOperand, i32>(Code::Cmp_rm16_imm16, dst.into(), src as i32),
                    X86RegSize::Dword => Instruction::with2::<iced_x86::MemoryOperand, i32>(Code::Cmp_rm32_imm32, dst.into(), src as i32),
                    X86RegSize::Qword => Instruction::with2::<iced_x86::MemoryOperand, i32>(Code::Cmp_rm64_imm32, dst.into(), src as i32),
                    X86RegSize::SimdVec => panic!("cmp deosn't support simd vecs"),
                }
            },

            _ => panic!("invalid variant: {self} (maybe unresolved tmps?)"),
        }.expect("invalid instruction");

        BlockEncoder::encode(
            64, 
            InstructionBlock::new(&[instr], 0), 
            BlockEncoderOptions::DONT_FIX_BRANCHES
        ).expect("encoding error").code_buffer
    }

    fn encode_pinsrb(&self) -> Vec<u8> {
        let Some(X86Operand::Reg(dst)) = self.op1 else { panic!("invalid variant"); };
        let Some(src) = self.op2 else { panic!("invalid variant"); };
        let Some(X86Operand::Const(pos)) = self.op3 else { panic!("invalid variant"); };

        let instr = match src {
            X86Operand::Reg(src) => {
                match src.size {
                    X86RegSize::Dword => Instruction::with3::<iced_x86::Register, iced_x86::Register, i32>(Code::Pinsrb_xmm_r32m8_imm8, dst.into(), src.into(), pos as i32),
                    X86RegSize::Qword => Instruction::with3::<iced_x86::Register, iced_x86::Register, i32>(Code::Pinsrb_xmm_r64m8_imm8, dst.into(), src.into(), pos as i32),
                    _ => panic!("invalid variant: {self}"),
                }
            },
            X86Operand::MemDispl(mem) => Instruction::with3::<iced_x86::Register, iced_x86::MemoryOperand, i32>(Code::Pinsrb_xmm_r64m8_imm8, dst.into(), mem.into(), pos as i32),
            _ => panic!("invalid variant: {self}"),
        }.expect("invalid instruction");

        BlockEncoder::encode(
            64, 
            InstructionBlock::new(&[instr], 0), 
            BlockEncoderOptions::DONT_FIX_BRANCHES
        ).expect("encoding error").code_buffer
    }

    fn encode_pinsrw(&self) -> Vec<u8> {
        let Some(X86Operand::Reg(dst)) = self.op1 else { panic!("invalid variant"); };
        let Some(src) = self.op2 else { panic!("invalid variant"); };
        let Some(X86Operand::Const(pos)) = self.op3 else { panic!("invalid variant"); };

        let instr = match src {
            X86Operand::Reg(src) => {
                match src.size {
                    X86RegSize::Dword => Instruction::with3::<iced_x86::Register, iced_x86::Register, i32>(Code::Pinsrw_xmm_r32m16_imm8, dst.into(), src.into(), pos as i32),
                    X86RegSize::Qword => Instruction::with3::<iced_x86::Register, iced_x86::Register, i32>(Code::Pinsrw_xmm_r64m16_imm8, dst.into(), src.into(), pos as i32),
                    _ => panic!("invalid variant: {self}"),
                }
            },
            X86Operand::MemDispl(mem) => Instruction::with3::<iced_x86::Register, iced_x86::MemoryOperand, i32>(Code::Pinsrw_xmm_r64m16_imm8, dst.into(), mem.into(), pos as i32),
            _ => panic!("invalid variant: {self}"),
        }.expect("invalid instruction");

        BlockEncoder::encode(
            64, 
            InstructionBlock::new(&[instr], 0), 
            BlockEncoderOptions::DONT_FIX_BRANCHES
        ).expect("encoding error").code_buffer
    }

    fn encode_pinsrd(&self) -> Vec<u8> {
        let Some(X86Operand::Reg(dst)) = self.op1 else { panic!("invalid variant"); };
        let Some(src) = self.op2 else { panic!("invalid variant"); };
        let Some(X86Operand::Const(pos)) = self.op3 else { panic!("invalid variant"); };

        let instr = match src {
            X86Operand::Reg(src) => {
                match src.size {
                    X86RegSize::Dword => Instruction::with3::<iced_x86::Register, iced_x86::Register, i32>(Code::Pinsrd_xmm_rm32_imm8, dst.into(), src.into(), pos as i32),
                    _ => panic!("invalid variant: {self}"),
                }
            },
            X86Operand::MemDispl(mem) => Instruction::with3::<iced_x86::Register, iced_x86::MemoryOperand, i32>(Code::Pinsrd_xmm_rm32_imm8, dst.into(), mem.into(), pos as i32),
            _ => panic!("invalid variant: {self}"),
        }.expect("invalid instruction");

        BlockEncoder::encode(
            64, 
            InstructionBlock::new(&[instr], 0), 
            BlockEncoderOptions::DONT_FIX_BRANCHES
        ).expect("encoding error").code_buffer
    }

    fn encode_pinsrq(&self) -> Vec<u8> {
        let Some(X86Operand::Reg(dst)) = self.op1 else { panic!("invalid variant"); };
        let Some(src) = self.op2 else { panic!("invalid variant"); };
        let Some(X86Operand::Const(pos)) = self.op3 else { panic!("invalid variant"); };

        let instr = match src {
            X86Operand::Reg(src) => {
                match src.size {
                    X86RegSize::Qword => Instruction::with3::<iced_x86::Register, iced_x86::Register, i32>(Code::Pinsrq_xmm_rm64_imm8, dst.into(), src.into(), pos as i32),
                    _ => panic!("invalid variant: {self}"),
                }
            },
            X86Operand::MemDispl(mem) => Instruction::with3::<iced_x86::Register, iced_x86::MemoryOperand, i32>(Code::Pinsrq_xmm_rm64_imm8, dst.into(), mem.into(), pos as i32),
            _ => panic!("invalid variant: {self}"),
        }.expect("invalid instruction");

        BlockEncoder::encode(
            64, 
            InstructionBlock::new(&[instr], 0), 
            BlockEncoderOptions::DONT_FIX_BRANCHES
        ).expect("encoding error").code_buffer
    }

    fn encode_insertps(&self) -> Vec<u8> {
        let Some(X86Operand::Reg(dst)) = self.op1 else { panic!("invalid variant"); };
        let Some(src) = self.op2 else { panic!("invalid variant"); };
        let Some(X86Operand::Const(pos)) = self.op3 else { panic!("invalid variant"); };

        let instr = match src {
            X86Operand::Reg(src) => Instruction::with3::<iced_x86::Register, iced_x86::Register, i32>(Code::Insertps_xmm_xmmm32_imm8, dst.into(), src.into(), pos as i32),
            X86Operand::MemDispl(mem) => Instruction::with3::<iced_x86::Register, iced_x86::MemoryOperand, i32>(Code::Insertps_xmm_xmmm32_imm8, dst.into(), mem.into(), pos as i32),
            _ => panic!("invalid variant: {self}"),
        }.expect("invalid instruction");

        BlockEncoder::encode(
            64, 
            InstructionBlock::new(&[instr], 0), 
            BlockEncoderOptions::DONT_FIX_BRANCHES
        ).expect("encoding error").code_buffer
    }
}

impl Into<iced_x86::Register> for X86Reg {
    fn into(self) -> iced_x86::Register {
        match self.variant {
            crate::Target::x86::reg::X86RegVariant::Rax => match self.size { X86RegSize::Byte => iced_x86::Register::AL, X86RegSize::Word => iced_x86::Register::AX, X86RegSize::Dword => iced_x86::Register::EAX, X86RegSize::Qword => iced_x86::Register::RAX, _ => panic!()},
            crate::Target::x86::reg::X86RegVariant::Rbx => match self.size { X86RegSize::Byte => iced_x86::Register::BL, X86RegSize::Word => iced_x86::Register::BX, X86RegSize::Dword => iced_x86::Register::EBX, X86RegSize::Qword => iced_x86::Register::RBX, _ => panic!()},
            crate::Target::x86::reg::X86RegVariant::Rcx => match self.size { X86RegSize::Byte => iced_x86::Register::CL, X86RegSize::Word => iced_x86::Register::CX, X86RegSize::Dword => iced_x86::Register::ECX, X86RegSize::Qword => iced_x86::Register::RCX, _ => panic!()},
            crate::Target::x86::reg::X86RegVariant::Rdx => match self.size { X86RegSize::Byte => iced_x86::Register::DL, X86RegSize::Word => iced_x86::Register::DX, X86RegSize::Dword => iced_x86::Register::EDX, X86RegSize::Qword => iced_x86::Register::RDX, _ => panic!()},
            crate::Target::x86::reg::X86RegVariant::Rdi => match self.size { X86RegSize::Byte => iced_x86::Register::DIL, X86RegSize::Word => iced_x86::Register::DI, X86RegSize::Dword => iced_x86::Register::EDI, X86RegSize::Qword => iced_x86::Register::RDI, _ => panic!()},
            crate::Target::x86::reg::X86RegVariant::Rsi => match self.size { X86RegSize::Byte => iced_x86::Register::SIL, X86RegSize::Word => iced_x86::Register::SI, X86RegSize::Dword => iced_x86::Register::ESI, X86RegSize::Qword => iced_x86::Register::RSI, _ => panic!()},
            crate::Target::x86::reg::X86RegVariant::Rbp => match self.size { X86RegSize::Byte => iced_x86::Register::BPL, X86RegSize::Word => iced_x86::Register::BP, X86RegSize::Dword => iced_x86::Register::EBP, X86RegSize::Qword => iced_x86::Register::RBP, _ => panic!()},
            crate::Target::x86::reg::X86RegVariant::Rsp => match self.size { X86RegSize::Byte => iced_x86::Register::SPL, X86RegSize::Word => iced_x86::Register::SP, X86RegSize::Dword => iced_x86::Register::ESP, X86RegSize::Qword => iced_x86::Register::RSP, _ => panic!()},
            crate::Target::x86::reg::X86RegVariant::R8 => match self.size { X86RegSize::Byte => iced_x86::Register::R8L, X86RegSize::Word => iced_x86::Register::R8W, X86RegSize::Dword => iced_x86::Register::R8D, X86RegSize::Qword => iced_x86::Register::R8, _ => panic!()},
            crate::Target::x86::reg::X86RegVariant::R9 => match self.size { X86RegSize::Byte => iced_x86::Register::R9L, X86RegSize::Word => iced_x86::Register::R9W, X86RegSize::Dword => iced_x86::Register::R9D, X86RegSize::Qword => iced_x86::Register::R9, _ => panic!()},
            crate::Target::x86::reg::X86RegVariant::R10 => match self.size { X86RegSize::Byte => iced_x86::Register::R10L, X86RegSize::Word => iced_x86::Register::R10W, X86RegSize::Dword => iced_x86::Register::R10D, X86RegSize::Qword => iced_x86::Register::R10, _ => panic!()},
            crate::Target::x86::reg::X86RegVariant::R11 => match self.size { X86RegSize::Byte => iced_x86::Register::R11L, X86RegSize::Word => iced_x86::Register::R11W, X86RegSize::Dword => iced_x86::Register::R11D, X86RegSize::Qword => iced_x86::Register::R11, _ => panic!()},
            crate::Target::x86::reg::X86RegVariant::R12 => match self.size { X86RegSize::Byte => iced_x86::Register::R12L, X86RegSize::Word => iced_x86::Register::R12W, X86RegSize::Dword => iced_x86::Register::R12D, X86RegSize::Qword => iced_x86::Register::R12, _ => panic!()},
            crate::Target::x86::reg::X86RegVariant::R13 => match self.size { X86RegSize::Byte => iced_x86::Register::R13L, X86RegSize::Word => iced_x86::Register::R13W, X86RegSize::Dword => iced_x86::Register::R13D, X86RegSize::Qword => iced_x86::Register::R13, _ => panic!()},
            crate::Target::x86::reg::X86RegVariant::R14 => match self.size { X86RegSize::Byte => iced_x86::Register::R14L, X86RegSize::Word => iced_x86::Register::R14W, X86RegSize::Dword => iced_x86::Register::R14D, X86RegSize::Qword => iced_x86::Register::R14, _ => panic!()},
            crate::Target::x86::reg::X86RegVariant::R15 => match self.size { X86RegSize::Byte => iced_x86::Register::R15L, X86RegSize::Word => iced_x86::Register::R15W, X86RegSize::Dword => iced_x86::Register::R15D, X86RegSize::Qword => iced_x86::Register::R15, _ => panic!()},
            crate::Target::x86::reg::X86RegVariant::Xmm0 => iced_x86::Register::XMM0,
            crate::Target::x86::reg::X86RegVariant::Xmm1 => iced_x86::Register::XMM1,
            crate::Target::x86::reg::X86RegVariant::Xmm2 => iced_x86::Register::XMM2,
            crate::Target::x86::reg::X86RegVariant::Xmm3 => iced_x86::Register::XMM3,
            crate::Target::x86::reg::X86RegVariant::Xmm4 => iced_x86::Register::XMM4,
            crate::Target::x86::reg::X86RegVariant::Xmm5 => iced_x86::Register::XMM5,
            crate::Target::x86::reg::X86RegVariant::Xmm6 => iced_x86::Register::XMM6,
            crate::Target::x86::reg::X86RegVariant::Xmm7 => iced_x86::Register::XMM7,
            crate::Target::x86::reg::X86RegVariant::Xmm8 => iced_x86::Register::XMM8,
            crate::Target::x86::reg::X86RegVariant::Xmm9 => iced_x86::Register::XMM9,
            crate::Target::x86::reg::X86RegVariant::Xmm10 => iced_x86::Register::XMM10,
            crate::Target::x86::reg::X86RegVariant::Xmm11 => iced_x86::Register::XMM11,
            crate::Target::x86::reg::X86RegVariant::Xmm12 => iced_x86::Register::XMM12,
            crate::Target::x86::reg::X86RegVariant::Xmm13 => iced_x86::Register::XMM13,
            crate::Target::x86::reg::X86RegVariant::Xmm14 => iced_x86::Register::XMM14,
            crate::Target::x86::reg::X86RegVariant::Xmm15 => iced_x86::Register::XMM15,
        }
    }
}

impl Into<iced_x86::MemoryOperand> for X86MemDispl {
    fn into(self) -> iced_x86::MemoryOperand {
        let mut mem = iced_x86::MemoryOperand::default();

        if let Some(base) = self.base {
            mem.base = base.into();
        }

        if let Some(index) = self.index {
            mem.index = index.into();
        }

        if let Some(displ) = self.displ {
            mem.displacement = displ as i64;
            mem.displ_size = 1;
        }

        if let Some(scale) = self.scale {
            mem.scale = scale as u32;
        }

        mem
    }
}