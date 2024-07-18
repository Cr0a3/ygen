use std::collections::VecDeque;
use std::error::Error;

use crate::Support::Colorize;

use super::Token;
use super::compile::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Compiler {
    tokens: VecDeque<Token>,

    /// the generated machine code
    pub(crate) out: Vec<u8>,
    index: usize,
}

impl Compiler {
    /// Creates a new parser
    pub(crate) fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens.into(),
            out: vec![],
            index: 0,
        }
    }

    /// Parses the tokens into expressions
    pub(crate) fn parse(&mut self) -> Result<(), Box<dyn Error>> {
        let out = if let Some(tok) = self.tokens.front() {
            match &tok {
                Token::btr => Some(compile_btr(&mut self.tokens)),
                Token::add => Some(compile_add(&mut self.tokens)),
                Token::eems => Some(compile_EEMS(&mut self.tokens)),
                Token::cmovbe => Some(compile_cmovbe(&mut self.tokens)),
                Token::cmove => Some(compile_cmove(&mut self.tokens)),
                Token::cmovl => Some(compile_cmovl(&mut self.tokens)),
                Token::mov => Some(compile_mov(&mut self.tokens)),
                Token::bts => Some(compile_bts(&mut self.tokens)),
                Token::invd => Some(compile_invd(&mut self.tokens)),
                Token::and => Some(compile_and(&mut self.tokens)),
                Token::clc => Some(compile_clc(&mut self.tokens)),
                Token::dec => Some(compile_dec(&mut self.tokens)),
                Token::cmp => Some(compile_cmp(&mut self.tokens)),
                Token::aad => Some(compile_aad(&mut self.tokens)),
                Token::into => Some(compile_into(&mut self.tokens)),
                Token::incsspd => Some(compile_incsspd(&mut self.tokens)),
                Token::inc => Some(compile_inc(&mut self.tokens)),
                Token::bndcn => Some(compile_bndcn(&mut self.tokens)),
                Token::cdqe => Some(compile_cdqe(&mut self.tokens)),
                Token::cmovge => Some(compile_cmovge(&mut self.tokens)),
                Token::int3 => Some(compile_int3(&mut self.tokens)),
                Token::cmova => Some(compile_cmova(&mut self.tokens)),
                Token::idiv => Some(compile_idiv(&mut self.tokens)),
                Token::ret => Some(compile_ret(&mut self.tokens)),
                Token::retf => Some(compile_ret(&mut self.tokens)),
                Token::adc => Some(compile_adc(&mut self.tokens)),
                Token::bndcu => Some(compile_bndcu(&mut self.tokens)),
                Token::clac => Some(compile_clac(&mut self.tokens)),
                Token::clts => Some(compile_clts(&mut self.tokens)),
                Token::adox => Some(compile_adox(&mut self.tokens)),
                Token::lar => Some(compile_lar(&mut self.tokens)),
                Token::cqo => Some(compile_cqo(&mut self.tokens)),
                Token::cmpxchg => Some(compile_cmpxchg(&mut self.tokens)),
                Token::das => Some(compile_das(&mut self.tokens)),
                Token::aas => Some(compile_aas(&mut self.tokens)),
                Token::int => Some(compile_int(&mut self.tokens)),
                Token::bswap => Some(compile_bswap(&mut self.tokens)),
                Token::bt => Some(compile_bt(&mut self.tokens)),
                Token::cmovg => Some(compile_cmovg(&mut self.tokens)),
                Token::aam => Some(compile_aam(&mut self.tokens)),
                Token::jmp => Some(compile_jmp(&mut self.tokens)),
                Token::aaa => Some(compile_aaa(&mut self.tokens)),
                Token::adcx => Some(compile_adcx(&mut self.tokens)),
                Token::clui => Some(compile_clui(&mut self.tokens)),
                Token::arpl => Some(compile_arpl(&mut self.tokens)),
                Token::cmc => Some(compile_cmc(&mut self.tokens)),
                Token::bsr => Some(compile_bsr(&mut self.tokens)),
                Token::imul => Some(compile_imul(&mut self.tokens)),
                Token::int1 => Some(compile_int1(&mut self.tokens)),
                Token::btc => Some(compile_btc(&mut self.tokens)),
                Token::cpuid => Some(compile_cpuid(&mut self.tokens)),
                Token::hlt => Some(compile_hlt(&mut self.tokens)),
                Token::cmovb => Some(compile_cmovb(&mut self.tokens)),
                Token::bndcl => Some(compile_bndcl(&mut self.tokens)),
                Token::cbw => Some(compile_cbw(&mut self.tokens)),
                Token::daa => Some(compile_daa(&mut self.tokens)),
                Token::cli => Some(compile_cli(&mut self.tokens)),
                Token::endbr64 => Some(compile_ENDBR64(&mut self.tokens)),
                Token::cmovae => Some(compile_cmovae(&mut self.tokens)),
                Token::crc32 => Some(compile_crc32(&mut self.tokens)),
                Token::iret => Some(compile_iret(&mut self.tokens)),
                Token::iretd => Some(compile_iret(&mut self.tokens)),
                Token::iretQ => Some(compile_iret(&mut self.tokens)),
                Token::lea => Some(compile_lea(&mut self.tokens)),
                Token::cwd => Some(compile_cwd(&mut self.tokens)),
                Token::cmovle => Some(compile_cmovle(&mut self.tokens)),
                Token::call => Some(compile_call(&mut self.tokens)),
                Token::cdq => Some(compile_cdq(&mut self.tokens)),
                Token::div => Some(compile_div(&mut self.tokens)),
                Token::bsf => Some(compile_bsf(&mut self.tokens)),
                Token::cld => Some(compile_cld(&mut self.tokens)),
                Token::cwde => Some(compile_cwde(&mut self.tokens)),
                Token::ja => Some(compile_jcc(&mut self.tokens)),
                Token::jae => Some(compile_jcc(&mut self.tokens)),
                Token::jb => Some(compile_jcc(&mut self.tokens)),
                Token::jbe => Some(compile_jcc(&mut self.tokens)),
                Token::jc => Some(compile_jcc(&mut self.tokens)),
                Token::je => Some(compile_jcc(&mut self.tokens)),
                Token::jz => Some(compile_jcc(&mut self.tokens)),
                Token::jge => Some(compile_jcc(&mut self.tokens)),
                Token::jl => Some(compile_jcc(&mut self.tokens)),
                Token::jle => Some(compile_jcc(&mut self.tokens)),
                Token::jna => Some(compile_jcc(&mut self.tokens)),
                Token::jnae => Some(compile_jcc(&mut self.tokens)),
                Token::jnb => Some(compile_jcc(&mut self.tokens)),
                Token::jnbe => Some(compile_jcc(&mut self.tokens)),
                Token::jnc => Some(compile_jcc(&mut self.tokens)),
                Token::jne => Some(compile_jcc(&mut self.tokens)),
                Token::jng => Some(compile_jcc(&mut self.tokens)),
                Token::jnge => Some(compile_jcc(&mut self.tokens)),
                Token::jnl => Some(compile_jcc(&mut self.tokens)),
                Token::jnle => Some(compile_jcc(&mut self.tokens)),
                Token::jno => Some(compile_jcc(&mut self.tokens)),
                Token::jnp => Some(compile_jcc(&mut self.tokens)),
                Token::jns => Some(compile_jcc(&mut self.tokens)),
                Token::jnz => Some(compile_jcc(&mut self.tokens)),
                Token::jo => Some(compile_jcc(&mut self.tokens)),
                Token::jp => Some(compile_jcc(&mut self.tokens)),
                Token::jpe => Some(compile_jcc(&mut self.tokens)),
                Token::jpo => Some(compile_jcc(&mut self.tokens)),
                Token::js => Some(compile_jcc(&mut self.tokens)),
                Token::endbr32 => Some(compile_ENDBR32(&mut self.tokens)),

                tok => {
                    println!("{} {}: {:?}", "Parsing error:".red().bold(), "Unknown instruction".gray(), tok);
                    None
                },
            }
        } else { None };

        if let Some(out) = out {
            self.out = match out {
                Ok(val) => val,
                Err(e) => { println!("{}", e); Err(e)? },
            };
        }

        Ok(())
    }
}

/// The asm compiler for the x64 backend
/// 
/// # Don't use!! It is for internal pourpuses only
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct x64Compiler {
    pub(crate) inner: Option<Compiler>,
}

impl x64Compiler {
    /// Creates an new x64 backend
    /// # Don't use!! This structure is for internal pourpuses only
    pub fn new() -> Self {
        Self {
            inner: None,
        }
    }
}

impl crate::Target::Compiler for x64Compiler {
    fn parse(&mut self) -> Result<(), Box<dyn Error>> {
        let mut compiler = self.inner.clone().unwrap();
        compiler.parse()?;
        self.inner = Some(compiler);

        Ok(())
    }

    fn boxed(&self) -> Box<dyn crate::Target::Compiler> {
        Box::from( self.clone() )
    }
    
    fn new(&self, tokens: Vec<Token>) -> Box<dyn crate::Target::Compiler> {
        Self {
            inner: Some(Compiler::new(tokens)),
        }.boxed()
    }
    
    fn out(&self) -> Vec<u8> {
        let compiler = self.inner.clone().unwrap();
        compiler.out
    }
}