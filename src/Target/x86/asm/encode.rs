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
        todo!()
    }

    fn encode_movss(&self) -> Vec<u8> {
        todo!()
    }

    fn encode_movsd(&self) -> Vec<u8> {
        todo!()
    }

    fn encode_movdqa(&self) -> Vec<u8> {
        todo!()
    }

    fn encode_ret(&self) -> Vec<u8> {
        vec![0xC3]
    }

    fn encode_add(&self) -> Vec<u8> {
        todo!()
    }

    fn encode_paddq(&self) -> Vec<u8> {
        todo!()
    }

    fn encode_paddd(&self) -> Vec<u8> {
        todo!()
    }

    fn encode_sub(&self) -> Vec<u8> {
        todo!()
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
        todo!()
    }

    fn encode_jmp(&self) -> Vec<u8> {
        todo!()
    }

    fn encode_sete(&self) -> Vec<u8> {
        todo!()
    }

    fn encode_setne(&self) -> Vec<u8> {
        todo!()
    }

    fn encode_setl(&self) -> Vec<u8> {
        todo!()
    }

    fn encode_setle(&self) -> Vec<u8> {
        todo!()
    }

    fn encode_setg(&self) -> Vec<u8> {
        todo!()
    }

    fn encode_setge(&self) -> Vec<u8> {
        todo!()
    }

    fn encode_cmp(&self) -> Vec<u8> {
        todo!()
    }

    fn encode_pinsrb(&self) -> Vec<u8> {
        todo!()
    }

    fn encode_pinsrw(&self) -> Vec<u8> {
        todo!()
    }

    fn encode_pinsrd(&self) -> Vec<u8> {
        todo!()
    }

    fn encode_pinsrq(&self) -> Vec<u8> {
        todo!()
    }

    fn encode_insertps(&self) -> Vec<u8> {
        todo!()
    }
}