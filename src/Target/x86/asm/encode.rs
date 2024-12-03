use iced_x86::{BlockEncoder, BlockEncoderOptions, Instruction, InstructionBlock, Code};

use super::*;

impl X86Instr {
    /// Encodes the x86 instruction
    pub fn encode(&self) -> Vec<u8> {
        let instr = self.fix_sizing();

        match instr.mnemonic {
            X86Mnemonic::Mov => instr.encode_mov(),
            X86Mnemonic::Movss => instr.encode_movss(),
            X86Mnemonic::Movsd => instr.encode_movsd(),
            X86Mnemonic::Movdqa =>instr.encode_movdqa(),
            X86Mnemonic::Ret => instr.encode_ret(),
            X86Mnemonic::Add => instr.encode_add(),
            X86Mnemonic::Addss => instr.encode_addss(),
            X86Mnemonic::Paddq => instr.encode_paddq(),
            X86Mnemonic::Paddd => instr.encode_paddd(),
            X86Mnemonic::Sub => instr.encode_sub(),
            X86Mnemonic::Psubq => instr.encode_psubq(),
            X86Mnemonic::Psubd => instr.encode_psubd(),
            X86Mnemonic::Psubw => instr.encode_psubw(),
            X86Mnemonic::Psubb => instr.encode_psubb(),
            X86Mnemonic::Lea => instr.encode_lea(),
            X86Mnemonic::Jmp => instr.encode_jmp(),
            X86Mnemonic::Je => instr.encode_je(),
            X86Mnemonic::Sete => instr.encode_sete(),
            X86Mnemonic::Setne => instr.encode_setne(),
            X86Mnemonic::Setl => instr.encode_setl(),
            X86Mnemonic::Setle => instr.encode_setle(),
            X86Mnemonic::Setg => instr.encode_setg(),
            X86Mnemonic::Setge => instr.encode_setge(),
            X86Mnemonic::Cmp => instr.encode_cmp(),
            X86Mnemonic::Pinsrb => instr.encode_pinsrb(),
            X86Mnemonic::Pinsrw => instr.encode_pinsrw(),
            X86Mnemonic::Pinsrd => instr.encode_pinsrd(),
            X86Mnemonic::Pinsrq => instr.encode_pinsrq(),
            X86Mnemonic::Insertps => instr.encode_insertps(),
            X86Mnemonic::Imul => instr.encode_imul(),
            X86Mnemonic::And => instr.encode_and(),
            X86Mnemonic::Or => instr.encode_or(),
            X86Mnemonic::Xor => instr.encode_xor(),
            X86Mnemonic::Sar => instr.encode_sar(),
            X86Mnemonic::Shr => instr.encode_shr(),
            X86Mnemonic::Shl => instr.encode_shl(),
            X86Mnemonic::Sal => instr.encode_sal(),
            X86Mnemonic::Neg => instr.encode_neg(),
            X86Mnemonic::Movsx => instr.encode_movsx(),
            X86Mnemonic::Movsxd => instr.encode_movsxd(),
            X86Mnemonic::Cvtsi2sd => instr.encode_cvtsi2sd(),
            X86Mnemonic::Cvtsi2ss => instr.encode_cvtsi2ss(),
            X86Mnemonic::Cvtss2si => instr.encode_cvtss2si(),
            X86Mnemonic::Cvtsd2si => instr.encode_cvtsd2si(),
            X86Mnemonic::Cvtss2sd => instr.encode_cvtss2sd(),
            X86Mnemonic::Cvtsd2ss => instr.encode_cvtsd2ss(),
            X86Mnemonic::Idiv => instr.encode_idiv(),
            X86Mnemonic::Div => instr.encode_div(),
            X86Mnemonic::Cbw => instr.encode_cbw(),
            X86Mnemonic::Cwd => instr.encode_cwd(),
            X86Mnemonic::Cdq => instr.encode_cdq(),
            X86Mnemonic::Cqo => instr.encode_cqo(),
            X86Mnemonic::Call => instr.encode_call(),
            X86Mnemonic::Movq => instr.encode_movq(),
            X86Mnemonic::Push => instr.encode_push(),
            X86Mnemonic::Pop => instr.encode_pop(),
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

    fn encode_addss(&self) -> Vec<u8> {
        let dst = self.op1.expect("expected dst");
        let src = self.op2.expect("expected src");

        let instr = match (dst, src) {
            (X86Operand::Reg(dst), X86Operand::Reg(src)) => Instruction::with2::<iced_x86::Register, iced_x86::Register>(Code::Addss_xmm_xmmm32, dst.into(), src.into()),
            (X86Operand::Reg(dst), X86Operand::MemDispl(src)) => Instruction::with2::<iced_x86::Register, iced_x86::MemoryOperand>(Code::Addss_xmm_xmmm32, dst.into(), src.into()),
            _ => panic!("invalid variant: {self}"),
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
            X86Operand::Rel(..) => Instruction::with_branch(Code::Jmp_rel32_64, 0), // branch will be resolved later
            _ => panic!("invalid variant: {self}"),
        }.expect("invalid instruction");

        BlockEncoder::encode(
            64, 
            InstructionBlock::new(&[instr], 0), 
            BlockEncoderOptions::DONT_FIX_BRANCHES
        ).expect("encoding error").code_buffer
    }

    fn encode_je(&self) -> Vec<u8> {
        let target = self.op1.expect("expected je target");

        let instr = match target {
            X86Operand::Const(imm) => Instruction::with1(Code::Je_rel32_64, imm as i32),
            X86Operand::Rel(..) => Instruction::with_branch(Code::Je_rel32_64, 0), // branch will be resolved later
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

    fn encode_imul(&self) -> Vec<u8> {
        let instr = {
            if self.op1.is_some() && self.op2.is_none() && self.op3.is_none() { // maybe its imul r/m
                let Some(op1) = self.op1 else { unreachable!() };
                match op1 {
                    X86Operand::Reg(reg) => match reg.size {
                        X86RegSize::Byte => Instruction::with1::<iced_x86::Register>(Code::Imul_rm8, reg.into()),
                        X86RegSize::Word => Instruction::with1::<iced_x86::Register>(Code::Imul_rm16, reg.into()),
                        X86RegSize::Dword => Instruction::with1::<iced_x86::Register>(Code::Imul_rm32, reg.into()),
                        X86RegSize::Qword => Instruction::with1::<iced_x86::Register>(Code::Imul_rm64, reg.into()),
                        X86RegSize::SimdVec => panic!("imul doesn't support simd vectors")
                    },
                    X86Operand::MemDispl(mem) => match mem.size {
                        X86RegSize::Byte => Instruction::with1::<iced_x86::MemoryOperand>(Code::Imul_rm8, mem.into()),
                        X86RegSize::Word => Instruction::with1::<iced_x86::MemoryOperand>(Code::Imul_rm16, mem.into()),
                        X86RegSize::Dword => Instruction::with1::<iced_x86::MemoryOperand>(Code::Imul_rm32, mem.into()),
                        X86RegSize::Qword => Instruction::with1::<iced_x86::MemoryOperand>(Code::Imul_rm64, mem.into()),
                        X86RegSize::SimdVec => panic!("imul doesn't support simd vectors")
                    },
                    _ => panic!("invalid variant: {self}"),
                }
            } else if self.op1.is_some() && self.op2.is_some() && self.op3.is_none() { // maybe its imul r/m, r/m
                let Some(op1) = self.op1 else { unreachable!() };
                let Some(op2) = self.op2 else { unreachable!() };

                match (op1, op2) {
                    (X86Operand::Reg(op1), X86Operand::Reg(op2)) => match op1.size {
                        X86RegSize::Byte => panic!("imul doesn't support rm8"),
                        X86RegSize::Word => Instruction::with2::<iced_x86::Register, iced_x86::Register>(Code::Imul_r16_rm16, op1.into(), op2.into()),
                        X86RegSize::Dword => Instruction::with2::<iced_x86::Register, iced_x86::Register>(Code::Imul_r32_rm32, op1.into(), op2.into()),
                        X86RegSize::Qword => Instruction::with2::<iced_x86::Register, iced_x86::Register>(Code::Imul_r64_rm64, op1.into(), op2.into()),
                        X86RegSize::SimdVec => panic!("imul doesn't support simd vectors")
                    },
                    (X86Operand::Reg(op1), X86Operand::MemDispl(op2)) => match op1.size {
                        X86RegSize::Byte => panic!("imul doesn't support rm8"),
                        X86RegSize::Word => Instruction::with2::<iced_x86::Register, iced_x86::MemoryOperand>(Code::Imul_r16_rm16, op1.into(), op2.into()),
                        X86RegSize::Dword => Instruction::with2::<iced_x86::Register, iced_x86::MemoryOperand>(Code::Imul_r32_rm32, op1.into(), op2.into()),
                        X86RegSize::Qword => Instruction::with2::<iced_x86::Register, iced_x86::MemoryOperand>(Code::Imul_r64_rm64, op1.into(), op2.into()),
                        X86RegSize::SimdVec => panic!("imul doesn't support simd vectors")
                    },
                    _ => panic!("invalid variant: {self}"),
                }
            } else {
                let Some(X86Operand::Reg(op1)) = self.op1 else { panic!("expected op1") };
                let Some(op2) = self.op2 else { panic!("expected op2") };
                let Some(X86Operand::Const(op3)) = self.op3 else { panic!("expected op3") };

                match op2 {
                    X86Operand::Reg(reg) => match reg.size {
                        X86RegSize::Byte => panic!("imul doesn't support rm8"),
                        X86RegSize::Word => Instruction::with3::<iced_x86::Register, iced_x86::Register, i32>(Code::Imul_r16_rm16_imm16, op1.into(), reg.into(), op3 as i32),
                        X86RegSize::Dword => Instruction::with3::<iced_x86::Register, iced_x86::Register, i32>(Code::Imul_r32_rm32_imm32, op1.into(), reg.into(), op3 as i32),
                        X86RegSize::Qword => Instruction::with3::<iced_x86::Register, iced_x86::Register, i32>(Code::Imul_r64_rm64_imm32, op1.into(), reg.into(), op3 as i32),
                        X86RegSize::SimdVec => panic!("imul doesn't support simd vectors")
                    },
                    X86Operand::MemDispl(mem) => match mem.size {
                        X86RegSize::Byte => panic!("imul doesn't support rm8"),
                        X86RegSize::Word => Instruction::with3::<iced_x86::Register, iced_x86::MemoryOperand, i32>(Code::Imul_r16_rm16_imm16, op1.into(), mem.into(), op3 as i32),
                        X86RegSize::Dword => Instruction::with3::<iced_x86::Register, iced_x86::MemoryOperand, i32>(Code::Imul_r32_rm32_imm32, op1.into(), mem.into(), op3 as i32),
                        X86RegSize::Qword => Instruction::with3::<iced_x86::Register, iced_x86::MemoryOperand, i32>(Code::Imul_r64_rm64_imm32, op1.into(), mem.into(), op3 as i32),
                        X86RegSize::SimdVec => panic!("imul doesn't support simd vectors")
                    },
                    _ => panic!("invalid variant: {self}"),
                }
            }
        }.expect("invalid instruction");

        BlockEncoder::encode(
            64, 
            InstructionBlock::new(&[instr], 0), 
            BlockEncoderOptions::DONT_FIX_BRANCHES
        ).expect("encoding error").code_buffer
    }
    
    fn encode_and(&self) -> Vec<u8> {
        let dst = self.op1.expect("expected dst");
        let src = self.op2.expect("expected src");
        
        let instr = match (dst, src) {
            (X86Operand::Reg(dst), X86Operand::Reg(src)) => {
                match dst.size {
                    X86RegSize::Byte => Instruction::with2::<iced_x86::Register, iced_x86::Register>(Code::And_r8_rm8, dst.into(), src.into()),
                    X86RegSize::Word => Instruction::with2::<iced_x86::Register, iced_x86::Register>(Code::And_r16_rm16, dst.into(), src.into()),
                    X86RegSize::Dword => Instruction::with2::<iced_x86::Register, iced_x86::Register>(Code::And_r32_rm32, dst.into(), src.into()),
                    X86RegSize::Qword => Instruction::with2::<iced_x86::Register, iced_x86::Register>(Code::And_r64_rm64, dst.into(), src.into()),
                    X86RegSize::SimdVec => panic!("and deosn't support simd vecs"),
                }
            },
            (X86Operand::Reg(dst), X86Operand::Const(src)) => {
                match dst.size {
                    X86RegSize::Byte => Instruction::with2::<iced_x86::Register, i32>(Code::And_rm8_imm8, dst.into(), src as i32),
                    X86RegSize::Word => Instruction::with2::<iced_x86::Register, i32>(Code::And_rm16_imm16, dst.into(), src as i32),
                    X86RegSize::Dword => Instruction::with2::<iced_x86::Register, i32>(Code::And_rm32_imm32, dst.into(), src as i32),
                    X86RegSize::Qword => Instruction::with2::<iced_x86::Register, i32>(Code::And_rm64_imm32, dst.into(), src as i32),
                    X86RegSize::SimdVec => panic!("and deosn't support simd vecs"),
                }
            },
            (X86Operand::Reg(dst), X86Operand::MemDispl(src)) => {
                match dst.size {
                    X86RegSize::Byte => Instruction::with2::<iced_x86::Register, iced_x86::MemoryOperand>(Code::And_r8_rm8, dst.into(), src.into()),
                    X86RegSize::Word => Instruction::with2::<iced_x86::Register, iced_x86::MemoryOperand>(Code::And_r16_rm16, dst.into(), src.into()),
                    X86RegSize::Dword => Instruction::with2::<iced_x86::Register, iced_x86::MemoryOperand>(Code::And_r32_rm32, dst.into(), src.into()),
                    X86RegSize::Qword => Instruction::with2::<iced_x86::Register, iced_x86::MemoryOperand>(Code::And_r64_rm64, dst.into(), src.into()),
                    X86RegSize::SimdVec => panic!("and deosn't support simd vecs"),
                }
            },
            (X86Operand::MemDispl(dst), X86Operand::Reg(src)) => {
                match dst.size {
                    X86RegSize::Byte => Instruction::with2::<iced_x86::MemoryOperand, iced_x86::Register>(Code::And_rm8_r8, dst.into(), src.into()),
                    X86RegSize::Word => Instruction::with2::<iced_x86::MemoryOperand, iced_x86::Register>(Code::And_rm16_r16, dst.into(), src.into()),
                    X86RegSize::Dword => Instruction::with2::<iced_x86::MemoryOperand, iced_x86::Register>(Code::And_rm32_r32, dst.into(), src.into()),
                    X86RegSize::Qword => Instruction::with2::<iced_x86::MemoryOperand, iced_x86::Register>(Code::And_rm64_r64, dst.into(), src.into()),
                    X86RegSize::SimdVec => panic!("and deosn't support simd vecs"),
                }
            },
            (X86Operand::MemDispl(dst), X86Operand::Const(src)) => {
                match dst.size {
                    X86RegSize::Byte => Instruction::with2::<iced_x86::MemoryOperand, i32>(Code::And_rm8_imm8, dst.into(), src as i32),
                    X86RegSize::Word => Instruction::with2::<iced_x86::MemoryOperand, i32>(Code::And_rm16_imm16, dst.into(), src as i32),
                    X86RegSize::Dword => Instruction::with2::<iced_x86::MemoryOperand, i32>(Code::And_rm32_imm32, dst.into(), src as i32),
                    X86RegSize::Qword => Instruction::with2::<iced_x86::MemoryOperand, i32>(Code::And_rm64_imm32, dst.into(), src as i32),
                    X86RegSize::SimdVec => panic!("and deosn't support simd vecs"),
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

    fn encode_or(&self) -> Vec<u8> {
        let dst = self.op1.expect("expected dst");
        let src = self.op2.expect("expected src");
        
        let instr = match (dst, src) {
            (X86Operand::Reg(dst), X86Operand::Reg(src)) => {
                match dst.size {
                    X86RegSize::Byte => Instruction::with2::<iced_x86::Register, iced_x86::Register>(Code::Or_r8_rm8, dst.into(), src.into()),
                    X86RegSize::Word => Instruction::with2::<iced_x86::Register, iced_x86::Register>(Code::Or_r16_rm16, dst.into(), src.into()),
                    X86RegSize::Dword => Instruction::with2::<iced_x86::Register, iced_x86::Register>(Code::Or_r32_rm32, dst.into(), src.into()),
                    X86RegSize::Qword => Instruction::with2::<iced_x86::Register, iced_x86::Register>(Code::Or_r64_rm64, dst.into(), src.into()),
                    X86RegSize::SimdVec => panic!("or deosn't support simd vecs"),
                }
            },
            (X86Operand::Reg(dst), X86Operand::Const(src)) => {
                match dst.size {
                    X86RegSize::Byte => Instruction::with2::<iced_x86::Register, i32>(Code::Or_rm8_imm8, dst.into(), src as i32),
                    X86RegSize::Word => Instruction::with2::<iced_x86::Register, i32>(Code::Or_rm16_imm16, dst.into(), src as i32),
                    X86RegSize::Dword => Instruction::with2::<iced_x86::Register, i32>(Code::Or_rm32_imm32, dst.into(), src as i32),
                    X86RegSize::Qword => Instruction::with2::<iced_x86::Register, i32>(Code::Or_rm64_imm32, dst.into(), src as i32),
                    X86RegSize::SimdVec => panic!("Or deosn't support simd vecs"),
                }
            },
            (X86Operand::Reg(dst), X86Operand::MemDispl(src)) => {
                match dst.size {
                    X86RegSize::Byte => Instruction::with2::<iced_x86::Register, iced_x86::MemoryOperand>(Code::Or_r8_rm8, dst.into(), src.into()),
                    X86RegSize::Word => Instruction::with2::<iced_x86::Register, iced_x86::MemoryOperand>(Code::Or_r16_rm16, dst.into(), src.into()),
                    X86RegSize::Dword => Instruction::with2::<iced_x86::Register, iced_x86::MemoryOperand>(Code::Or_r32_rm32, dst.into(), src.into()),
                    X86RegSize::Qword => Instruction::with2::<iced_x86::Register, iced_x86::MemoryOperand>(Code::Or_r64_rm64, dst.into(), src.into()),
                    X86RegSize::SimdVec => panic!("Or deosn't support simd vecs"),
                }
            },
            (X86Operand::MemDispl(dst), X86Operand::Reg(src)) => {
                match dst.size {
                    X86RegSize::Byte => Instruction::with2::<iced_x86::MemoryOperand, iced_x86::Register>(Code::Or_rm8_r8, dst.into(), src.into()),
                    X86RegSize::Word => Instruction::with2::<iced_x86::MemoryOperand, iced_x86::Register>(Code::Or_rm16_r16, dst.into(), src.into()),
                    X86RegSize::Dword => Instruction::with2::<iced_x86::MemoryOperand, iced_x86::Register>(Code::Or_rm32_r32, dst.into(), src.into()),
                    X86RegSize::Qword => Instruction::with2::<iced_x86::MemoryOperand, iced_x86::Register>(Code::Or_rm64_r64, dst.into(), src.into()),
                    X86RegSize::SimdVec => panic!("Or deosn't support simd vecs"),
                }
            },
            (X86Operand::MemDispl(dst), X86Operand::Const(src)) => {
                match dst.size {
                    X86RegSize::Byte => Instruction::with2::<iced_x86::MemoryOperand, i32>(Code::Or_rm8_imm8, dst.into(), src as i32),
                    X86RegSize::Word => Instruction::with2::<iced_x86::MemoryOperand, i32>(Code::Or_rm16_imm16, dst.into(), src as i32),
                    X86RegSize::Dword => Instruction::with2::<iced_x86::MemoryOperand, i32>(Code::Or_rm32_imm32, dst.into(), src as i32),
                    X86RegSize::Qword => Instruction::with2::<iced_x86::MemoryOperand, i32>(Code::Or_rm64_imm32, dst.into(), src as i32),
                    X86RegSize::SimdVec => panic!("Or deosn't support simd vecs"),
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
    
    fn encode_xor(&self) -> Vec<u8> {
        let dst = self.op1.expect("expected dst");
        let src = self.op2.expect("expected src");
        
        let instr = match (dst, src) {
            (X86Operand::Reg(dst), X86Operand::Reg(src)) => {
                match dst.size {
                    X86RegSize::Byte => Instruction::with2::<iced_x86::Register, iced_x86::Register>(Code::Xor_r8_rm8, dst.into(), src.into()),
                    X86RegSize::Word => Instruction::with2::<iced_x86::Register, iced_x86::Register>(Code::Xor_r16_rm16, dst.into(), src.into()),
                    X86RegSize::Dword => Instruction::with2::<iced_x86::Register, iced_x86::Register>(Code::Xor_r32_rm32, dst.into(), src.into()),
                    X86RegSize::Qword => Instruction::with2::<iced_x86::Register, iced_x86::Register>(Code::Xor_r64_rm64, dst.into(), src.into()),
                    X86RegSize::SimdVec => panic!("Xor deosn't support simd vecs"),
                }
            },
            (X86Operand::Reg(dst), X86Operand::Const(src)) => {
                match dst.size {
                    X86RegSize::Byte => Instruction::with2::<iced_x86::Register, i32>(Code::Xor_rm8_imm8, dst.into(), src as i32),
                    X86RegSize::Word => Instruction::with2::<iced_x86::Register, i32>(Code::Xor_rm16_imm16, dst.into(), src as i32),
                    X86RegSize::Dword => Instruction::with2::<iced_x86::Register, i32>(Code::Xor_rm32_imm32, dst.into(), src as i32),
                    X86RegSize::Qword => Instruction::with2::<iced_x86::Register, i32>(Code::Xor_rm64_imm32, dst.into(), src as i32),
                    X86RegSize::SimdVec => panic!("Xor deosn't support simd vecs"),
                }
            },
            (X86Operand::Reg(dst), X86Operand::MemDispl(src)) => {
                match dst.size {
                    X86RegSize::Byte => Instruction::with2::<iced_x86::Register, iced_x86::MemoryOperand>(Code::Xor_r8_rm8, dst.into(), src.into()),
                    X86RegSize::Word => Instruction::with2::<iced_x86::Register, iced_x86::MemoryOperand>(Code::Xor_r16_rm16, dst.into(), src.into()),
                    X86RegSize::Dword => Instruction::with2::<iced_x86::Register, iced_x86::MemoryOperand>(Code::Xor_r32_rm32, dst.into(), src.into()),
                    X86RegSize::Qword => Instruction::with2::<iced_x86::Register, iced_x86::MemoryOperand>(Code::Xor_r64_rm64, dst.into(), src.into()),
                    X86RegSize::SimdVec => panic!("Xor deosn't support simd vecs"),
                }
            },
            (X86Operand::MemDispl(dst), X86Operand::Reg(src)) => {
                match dst.size {
                    X86RegSize::Byte => Instruction::with2::<iced_x86::MemoryOperand, iced_x86::Register>(Code::Xor_rm8_r8, dst.into(), src.into()),
                    X86RegSize::Word => Instruction::with2::<iced_x86::MemoryOperand, iced_x86::Register>(Code::Xor_rm16_r16, dst.into(), src.into()),
                    X86RegSize::Dword => Instruction::with2::<iced_x86::MemoryOperand, iced_x86::Register>(Code::Xor_rm32_r32, dst.into(), src.into()),
                    X86RegSize::Qword => Instruction::with2::<iced_x86::MemoryOperand, iced_x86::Register>(Code::Xor_rm64_r64, dst.into(), src.into()),
                    X86RegSize::SimdVec => panic!("Xor deosn't support simd vecs"),
                }
            },
            (X86Operand::MemDispl(dst), X86Operand::Const(src)) => {
                match dst.size {
                    X86RegSize::Byte => Instruction::with2::<iced_x86::MemoryOperand, i32>(Code::Xor_rm8_imm8, dst.into(), src as i32),
                    X86RegSize::Word => Instruction::with2::<iced_x86::MemoryOperand, i32>(Code::Xor_rm16_imm16, dst.into(), src as i32),
                    X86RegSize::Dword => Instruction::with2::<iced_x86::MemoryOperand, i32>(Code::Xor_rm32_imm32, dst.into(), src as i32),
                    X86RegSize::Qword => Instruction::with2::<iced_x86::MemoryOperand, i32>(Code::Xor_rm64_imm32, dst.into(), src as i32),
                    X86RegSize::SimdVec => panic!("Xor deosn't support simd vecs"),
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

    fn encode_sar(&self) -> Vec<u8> {
        let dst = self.op1.expect("expected dst");
        let src = self.op1.expect("expected src");

        let instr = match (dst, src) {
            (X86Operand::Reg(ls), X86Operand::Reg(_)) => match ls.size {
                X86RegSize::Byte => Instruction::with2::<iced_x86::Register, iced_x86::Register>(Code::Sar_rm8_CL, ls.into(), iced_x86::Register::CL),
                X86RegSize::Word => Instruction::with2::<iced_x86::Register, iced_x86::Register>(Code::Sar_rm16_CL, ls.into(), iced_x86::Register::CL),
                X86RegSize::Dword => Instruction::with2::<iced_x86::Register, iced_x86::Register>(Code::Sar_rm32_CL, ls.into(), iced_x86::Register::CL),
                X86RegSize::Qword => Instruction::with2::<iced_x86::Register, iced_x86::Register>(Code::Sar_rm64_CL, ls.into(), iced_x86::Register::CL),
                X86RegSize::SimdVec => panic!("sar doesn't support simd vectors"),
            },
            (X86Operand::Reg(ls), X86Operand::Const(rs)) => match ls.size {
                X86RegSize::Byte => Instruction::with2::<iced_x86::Register, i32>(Code::Sar_rm8_CL, ls.into(), rs as i32),
                X86RegSize::Word => Instruction::with2::<iced_x86::Register, i32>(Code::Sar_rm16_CL, ls.into(), rs as i32),
                X86RegSize::Dword => Instruction::with2::<iced_x86::Register, i32>(Code::Sar_rm32_CL, ls.into(), rs as i32),
                X86RegSize::Qword => Instruction::with2::<iced_x86::Register, i32>(Code::Sar_rm64_CL, ls.into(), rs as i32),
                X86RegSize::SimdVec => panic!("sar doesn't support simd vectors"),
            },
            (X86Operand::MemDispl(ls), X86Operand::Reg(_)) => match ls.size {
                X86RegSize::Byte => Instruction::with2::<iced_x86::MemoryOperand, iced_x86::Register>(Code::Sar_rm8_CL, ls.into(), iced_x86::Register::CL),
                X86RegSize::Word => Instruction::with2::<iced_x86::MemoryOperand, iced_x86::Register>(Code::Sar_rm16_CL, ls.into(), iced_x86::Register::CL),
                X86RegSize::Dword => Instruction::with2::<iced_x86::MemoryOperand, iced_x86::Register>(Code::Sar_rm32_CL, ls.into(), iced_x86::Register::CL),
                X86RegSize::Qword => Instruction::with2::<iced_x86::MemoryOperand, iced_x86::Register>(Code::Sar_rm64_CL, ls.into(), iced_x86::Register::CL),
                X86RegSize::SimdVec => panic!("sar doesn't support simd vectors"),
            },
            _ => panic!("invalid variant: {self}"),
        }.expect("invalid instruction");

        BlockEncoder::encode(
            64, 
            InstructionBlock::new(&[instr], 0), 
            BlockEncoderOptions::DONT_FIX_BRANCHES
        ).expect("encoding error").code_buffer
    }

    fn encode_shr(&self) -> Vec<u8> {
        let dst = self.op1.expect("expected dst");
        let src = self.op1.expect("expected src");

        let instr = match (dst, src) {
            (X86Operand::Reg(ls), X86Operand::Reg(_)) => match ls.size {
                X86RegSize::Byte => Instruction::with2::<iced_x86::Register, iced_x86::Register>(Code::Shr_rm8_CL, ls.into(), iced_x86::Register::CL),
                X86RegSize::Word => Instruction::with2::<iced_x86::Register, iced_x86::Register>(Code::Shr_rm16_CL, ls.into(), iced_x86::Register::CL),
                X86RegSize::Dword => Instruction::with2::<iced_x86::Register, iced_x86::Register>(Code::Shr_rm32_CL, ls.into(), iced_x86::Register::CL),
                X86RegSize::Qword => Instruction::with2::<iced_x86::Register, iced_x86::Register>(Code::Shr_rm64_CL, ls.into(), iced_x86::Register::CL),
                X86RegSize::SimdVec => panic!("shr doesn't support simd vectors"),
            },
            (X86Operand::Reg(ls), X86Operand::Const(rs)) => match ls.size {
                X86RegSize::Byte => Instruction::with2::<iced_x86::Register, i32>(Code::Shr_rm8_CL, ls.into(), rs as i32),
                X86RegSize::Word => Instruction::with2::<iced_x86::Register, i32>(Code::Shr_rm16_CL, ls.into(), rs as i32),
                X86RegSize::Dword => Instruction::with2::<iced_x86::Register, i32>(Code::Shr_rm32_CL, ls.into(), rs as i32),
                X86RegSize::Qword => Instruction::with2::<iced_x86::Register, i32>(Code::Shr_rm64_CL, ls.into(), rs as i32),
                X86RegSize::SimdVec => panic!("shr doesn't support simd vectors"),
            },
            (X86Operand::MemDispl(ls), X86Operand::Reg(_)) => match ls.size {
                X86RegSize::Byte => Instruction::with2::<iced_x86::MemoryOperand, iced_x86::Register>(Code::Shr_rm8_CL, ls.into(), iced_x86::Register::CL),
                X86RegSize::Word => Instruction::with2::<iced_x86::MemoryOperand, iced_x86::Register>(Code::Shr_rm16_CL, ls.into(), iced_x86::Register::CL),
                X86RegSize::Dword => Instruction::with2::<iced_x86::MemoryOperand, iced_x86::Register>(Code::Shr_rm32_CL, ls.into(), iced_x86::Register::CL),
                X86RegSize::Qword => Instruction::with2::<iced_x86::MemoryOperand, iced_x86::Register>(Code::Shr_rm64_CL, ls.into(), iced_x86::Register::CL),
                X86RegSize::SimdVec => panic!("shr doesn't support simd vectors"),
            },
            _ => panic!("invalid variant: {self}"),
        }.expect("invalid instruction");

        BlockEncoder::encode(
            64, 
            InstructionBlock::new(&[instr], 0), 
            BlockEncoderOptions::DONT_FIX_BRANCHES
        ).expect("encoding error").code_buffer
    }

    fn encode_shl(&self) -> Vec<u8> {
        let dst = self.op1.expect("expected dst");
        let src = self.op1.expect("expected src");

        let instr = match (dst, src) {
            (X86Operand::Reg(ls), X86Operand::Reg(_)) => match ls.size {
                X86RegSize::Byte => Instruction::with2::<iced_x86::Register, iced_x86::Register>(Code::Shl_rm8_CL, ls.into(), iced_x86::Register::CL),
                X86RegSize::Word => Instruction::with2::<iced_x86::Register, iced_x86::Register>(Code::Shl_rm16_CL, ls.into(), iced_x86::Register::CL),
                X86RegSize::Dword => Instruction::with2::<iced_x86::Register, iced_x86::Register>(Code::Shl_rm32_CL, ls.into(), iced_x86::Register::CL),
                X86RegSize::Qword => Instruction::with2::<iced_x86::Register, iced_x86::Register>(Code::Shl_rm64_CL, ls.into(), iced_x86::Register::CL),
                X86RegSize::SimdVec => panic!("shl doesn't support simd vectors"),
            },
            (X86Operand::Reg(ls), X86Operand::Const(rs)) => match ls.size {
                X86RegSize::Byte => Instruction::with2::<iced_x86::Register, i32>(Code::Shl_rm8_CL, ls.into(), rs as i32),
                X86RegSize::Word => Instruction::with2::<iced_x86::Register, i32>(Code::Shl_rm16_CL, ls.into(), rs as i32),
                X86RegSize::Dword => Instruction::with2::<iced_x86::Register, i32>(Code::Shl_rm32_CL, ls.into(), rs as i32),
                X86RegSize::Qword => Instruction::with2::<iced_x86::Register, i32>(Code::Shl_rm64_CL, ls.into(), rs as i32),
                X86RegSize::SimdVec => panic!("shl doesn't support simd vectors"),
            },
            (X86Operand::MemDispl(ls), X86Operand::Reg(_)) => match ls.size {
                X86RegSize::Byte => Instruction::with2::<iced_x86::MemoryOperand, iced_x86::Register>(Code::Shl_rm8_CL, ls.into(), iced_x86::Register::CL),
                X86RegSize::Word => Instruction::with2::<iced_x86::MemoryOperand, iced_x86::Register>(Code::Shl_rm16_CL, ls.into(), iced_x86::Register::CL),
                X86RegSize::Dword => Instruction::with2::<iced_x86::MemoryOperand, iced_x86::Register>(Code::Shl_rm32_CL, ls.into(), iced_x86::Register::CL),
                X86RegSize::Qword => Instruction::with2::<iced_x86::MemoryOperand, iced_x86::Register>(Code::Shl_rm64_CL, ls.into(), iced_x86::Register::CL),
                X86RegSize::SimdVec => panic!("shl doesn't support simd vectors"),
            },
            _ => panic!("invalid variant: {self}"),
        }.expect("invalid instruction");

        BlockEncoder::encode(
            64, 
            InstructionBlock::new(&[instr], 0), 
            BlockEncoderOptions::DONT_FIX_BRANCHES
        ).expect("encoding error").code_buffer
    }

    fn encode_sal(&self) -> Vec<u8> {
        let dst = self.op1.expect("expected dst");
        let src = self.op1.expect("expected src");

        let instr = match (dst, src) {
            (X86Operand::Reg(ls), X86Operand::Reg(_)) => match ls.size {
                X86RegSize::Byte => Instruction::with2::<iced_x86::Register, iced_x86::Register>(Code::Sal_rm8_CL, ls.into(), iced_x86::Register::CL),
                X86RegSize::Word => Instruction::with2::<iced_x86::Register, iced_x86::Register>(Code::Sal_rm16_CL, ls.into(), iced_x86::Register::CL),
                X86RegSize::Dword => Instruction::with2::<iced_x86::Register, iced_x86::Register>(Code::Sal_rm32_CL, ls.into(), iced_x86::Register::CL),
                X86RegSize::Qword => Instruction::with2::<iced_x86::Register, iced_x86::Register>(Code::Sal_rm64_CL, ls.into(), iced_x86::Register::CL),
                X86RegSize::SimdVec => panic!("sal doesn't support simd vectors"),
            },
            (X86Operand::Reg(ls), X86Operand::Const(rs)) => match ls.size {
                X86RegSize::Byte => Instruction::with2::<iced_x86::Register, i32>(Code::Sal_rm8_CL, ls.into(), rs as i32),
                X86RegSize::Word => Instruction::with2::<iced_x86::Register, i32>(Code::Sal_rm16_CL, ls.into(), rs as i32),
                X86RegSize::Dword => Instruction::with2::<iced_x86::Register, i32>(Code::Sal_rm32_CL, ls.into(), rs as i32),
                X86RegSize::Qword => Instruction::with2::<iced_x86::Register, i32>(Code::Sal_rm64_CL, ls.into(), rs as i32),
                X86RegSize::SimdVec => panic!("sal doesn't support simd vectors"),
            },
            (X86Operand::MemDispl(ls), X86Operand::Reg(_)) => match ls.size {
                X86RegSize::Byte => Instruction::with2::<iced_x86::MemoryOperand, iced_x86::Register>(Code::Sal_rm8_CL, ls.into(), iced_x86::Register::CL),
                X86RegSize::Word => Instruction::with2::<iced_x86::MemoryOperand, iced_x86::Register>(Code::Sal_rm16_CL, ls.into(), iced_x86::Register::CL),
                X86RegSize::Dword => Instruction::with2::<iced_x86::MemoryOperand, iced_x86::Register>(Code::Sal_rm32_CL, ls.into(), iced_x86::Register::CL),
                X86RegSize::Qword => Instruction::with2::<iced_x86::MemoryOperand, iced_x86::Register>(Code::Sal_rm64_CL, ls.into(), iced_x86::Register::CL),
                X86RegSize::SimdVec => panic!("sal doesn't support simd vectors"),
            },
            _ => panic!("invalid variant: {self}"),
        }.expect("invalid instruction");

        BlockEncoder::encode(
            64, 
            InstructionBlock::new(&[instr], 0), 
            BlockEncoderOptions::DONT_FIX_BRANCHES
        ).expect("encoding error").code_buffer
    }

    fn encode_neg(&self) -> Vec<u8> {
        let dst = self.op1.expect("expected dst");
        
        let instr = match dst {
            X86Operand::Reg(dst) => match dst.size {
                X86RegSize::Byte => Instruction::with1::<iced_x86::Register>(Code::Neg_rm8, dst.into()),
                X86RegSize::Word => Instruction::with1::<iced_x86::Register>(Code::Neg_rm16, dst.into()),
                X86RegSize::Dword => Instruction::with1::<iced_x86::Register>(Code::Neg_rm32, dst.into()),
                X86RegSize::Qword => Instruction::with1::<iced_x86::Register>(Code::Neg_rm64, dst.into()),
                X86RegSize::SimdVec => panic!("neg doesn't support simd vec sized operands"),
            },
            X86Operand::MemDispl(dst) => match dst.size {
                X86RegSize::Byte => Instruction::with1::<iced_x86::MemoryOperand>(Code::Neg_rm8, dst.into()),
                X86RegSize::Word => Instruction::with1::<iced_x86::MemoryOperand>(Code::Neg_rm16, dst.into()),
                X86RegSize::Dword => Instruction::with1::<iced_x86::MemoryOperand>(Code::Neg_rm32, dst.into()),
                X86RegSize::Qword => Instruction::with1::<iced_x86::MemoryOperand>(Code::Neg_rm64, dst.into()),
                X86RegSize::SimdVec => panic!("neg doesn't support simd vec sized operands"),
            },
            _ => panic!("invalid variant: {self}"),
        }.expect("invalid instruction");

        BlockEncoder::encode(
            64, 
            InstructionBlock::new(&[instr], 0), 
            BlockEncoderOptions::DONT_FIX_BRANCHES
        ).expect("encoding error").code_buffer
    }

    fn encode_movsx(&self) -> Vec<u8> {
        todo!()
    }
    
    fn encode_movsxd(&self) -> Vec<u8> {
        todo!()
    }

    fn encode_cvtsi2sd(&self) -> Vec<u8> {
        todo!()
    }

    fn encode_cvtsi2ss(&self) -> Vec<u8> {
        todo!()
    }

    fn encode_cvtss2si(&self) -> Vec<u8> {
        todo!()
    }

    fn encode_cvtsd2si(&self) -> Vec<u8> {
        todo!()
    }

    fn encode_cvtss2sd(&self) -> Vec<u8> {
        todo!()
    }

    fn encode_cvtsd2ss(&self) -> Vec<u8> {
        todo!()
    }

    fn encode_idiv(&self) -> Vec<u8> {
        let Some(op1) = self.op1 else { panic!("expected op1"); };
        
        let instr = match op1 {
            X86Operand::Reg(reg) => match reg.size {
                X86RegSize::Byte => Instruction::with1::<iced_x86::Register>(Code::Idiv_rm8, reg.into()),
                X86RegSize::Word => Instruction::with1::<iced_x86::Register>(Code::Idiv_rm16, reg.into()),
                X86RegSize::Dword => Instruction::with1::<iced_x86::Register>(Code::Idiv_rm32, reg.into()),
                X86RegSize::Qword => Instruction::with1::<iced_x86::Register>(Code::Idiv_rm64, reg.into()),
                _ => panic!("invalid size"),
            },
            X86Operand::MemDispl(mem) => match mem.size {
                X86RegSize::Byte => Instruction::with1::<iced_x86::MemoryOperand>(Code::Idiv_rm8, mem.into()),
                X86RegSize::Word => Instruction::with1::<iced_x86::MemoryOperand>(Code::Idiv_rm16, mem.into()),
                X86RegSize::Dword => Instruction::with1::<iced_x86::MemoryOperand>(Code::Idiv_rm32, mem.into()),
                X86RegSize::Qword => Instruction::with1::<iced_x86::MemoryOperand>(Code::Idiv_rm64, mem.into()),
                _ => panic!("invalid size"),
            },
            _ => panic!("invalid variant: {self}"),
        }.expect("invalid instruction");

        BlockEncoder::encode(
            64, 
            InstructionBlock::new(&[instr], 0), 
            BlockEncoderOptions::DONT_FIX_BRANCHES
        ).expect("encoding error").code_buffer
    }

    fn encode_div(&self) -> Vec<u8> {
        let Some(op1) = self.op1 else { panic!("expected op1"); };
        
        let instr = match op1 {
            X86Operand::Reg(reg) => match reg.size {
                X86RegSize::Byte => Instruction::with1::<iced_x86::Register>(Code::Div_rm8, reg.into()),
                X86RegSize::Word => Instruction::with1::<iced_x86::Register>(Code::Div_rm16, reg.into()),
                X86RegSize::Dword => Instruction::with1::<iced_x86::Register>(Code::Div_rm32, reg.into()),
                X86RegSize::Qword => Instruction::with1::<iced_x86::Register>(Code::Div_rm64, reg.into()),
                _ => panic!("invalid size"),
            },
            X86Operand::MemDispl(mem) => match mem.size {
                X86RegSize::Byte => Instruction::with1::<iced_x86::MemoryOperand>(Code::Div_rm8, mem.into()),
                X86RegSize::Word => Instruction::with1::<iced_x86::MemoryOperand>(Code::Div_rm16, mem.into()),
                X86RegSize::Dword => Instruction::with1::<iced_x86::MemoryOperand>(Code::Div_rm32, mem.into()),
                X86RegSize::Qword => Instruction::with1::<iced_x86::MemoryOperand>(Code::Div_rm64, mem.into()),
                _ => panic!("invalid size"),
            },
            _ => panic!("invalid variant: {self}"),
        }.expect("invalid instruction");

        BlockEncoder::encode(
            64, 
            InstructionBlock::new(&[instr], 0), 
            BlockEncoderOptions::DONT_FIX_BRANCHES
        ).expect("encoding error").code_buffer
    }

    fn encode_cbw(&self) -> Vec<u8> {
        vec![0x98]
    }

    fn encode_cwd(&self) -> Vec<u8> {
        vec![0x99]
    }

    fn encode_cdq(&self) -> Vec<u8> {
        vec![0x99]
    }

    fn encode_cqo(&self) -> Vec<u8> {
        vec![0x48, 0x99]
    }
    fn encode_call(&self) -> Vec<u8> {
        let target = self.op1.expect("expected call target");

        let instr = match target {
            X86Operand::Reg(reg) => Instruction::with1::<iced_x86::Register>(Code::Call_rm64, reg.into()),
            X86Operand::Const(imm) => Instruction::with1(Code::Call_rel32_64, imm as i32),
            X86Operand::MemDispl(mem) => Instruction::with1::<iced_x86::MemoryOperand>(Code::Call_rm64, mem.into()),
            X86Operand::Rel(..) => Instruction::with_branch(Code::Call_rel32_64, 0), // branch will be resolved later
            _ => panic!("invalid variant: {self}"),
        }.expect("invalid instruction");

        BlockEncoder::encode(
            64, 
            InstructionBlock::new(&[instr], 0), 
            BlockEncoderOptions::DONT_FIX_BRANCHES
        ).expect("encoding error").code_buffer
    }

    fn encode_movq(&self) -> Vec<u8> {
        todo!()
    }

    fn encode_push(&self) -> Vec<u8> {
        let operand = self.op1.expect("expected op1");

        let instr = match operand {
            X86Operand::Reg(reg) => match reg.size {
                X86RegSize::Word => Instruction::with1::<iced_x86::Register>(Code::Push_r16, reg.into()),
                X86RegSize::Dword => Instruction::with1::<iced_x86::Register>(Code::Push_r32, reg.into()),
                X86RegSize::Qword => Instruction::with1::<iced_x86::Register>(Code::Push_r64, reg.into()),
                X86RegSize::SimdVec | X86RegSize::Byte => panic!("invalid size for push"),
            },
            X86Operand::Const(constant) => Instruction::with1(Code::Push_imm16, constant as i32),
            X86Operand::MemDispl(mem) => match mem.size {
                X86RegSize::Word => Instruction::with1::<iced_x86::MemoryOperand>(Code::Push_rm16, mem.into()),
                X86RegSize::Dword => Instruction::with1::<iced_x86::MemoryOperand>(Code::Push_rm32, mem.into()),
                X86RegSize::Qword => Instruction::with1::<iced_x86::MemoryOperand>(Code::Push_rm64, mem.into()),
                X86RegSize::SimdVec | X86RegSize::Byte => panic!("invalid size for push"),
            },
            _ => panic!("invalid variant: {self}"),
        }.expect("invalid instruction");

        BlockEncoder::encode(
            64, 
            InstructionBlock::new(&[instr], 0), 
            BlockEncoderOptions::DONT_FIX_BRANCHES
        ).expect("encoding error").code_buffer
    }
    
    fn encode_pop(&self) -> Vec<u8> {
        let operand = self.op1.expect("expected op1");

        let instr = match operand {
            X86Operand::Reg(reg) => match reg.size {
                X86RegSize::Word => Instruction::with1::<iced_x86::Register>(Code::Pop_r16, reg.into()),
                X86RegSize::Dword => Instruction::with1::<iced_x86::Register>(Code::Pop_r32, reg.into()),
                X86RegSize::Qword => Instruction::with1::<iced_x86::Register>(Code::Pop_r64, reg.into()),
                X86RegSize::SimdVec | X86RegSize::Byte => panic!("invalid size for pop"),
            },
            X86Operand::MemDispl(mem) => match mem.size {
                X86RegSize::Word => Instruction::with1::<iced_x86::MemoryOperand>(Code::Pop_rm16, mem.into()),
                X86RegSize::Dword => Instruction::with1::<iced_x86::MemoryOperand>(Code::Pop_rm32, mem.into()),
                X86RegSize::Qword => Instruction::with1::<iced_x86::MemoryOperand>(Code::Pop_rm64, mem.into()),
                X86RegSize::SimdVec | X86RegSize::Byte => panic!("invalid size for pop"),
            },
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

        // mem.scale = self.size.into();
        mem.scale = 1;

        if let Some(scale) = self.scale {
            if scale != 0 {
                mem.scale = scale as u32;
            }
        }

        if let Some(_) = self.rip_rel {
            mem.base = iced_x86::Register::RIP;
        }

        mem
    }
}