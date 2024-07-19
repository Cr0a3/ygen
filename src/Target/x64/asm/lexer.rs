use std::{collections::VecDeque, error::Error, fmt::Display, num::ParseIntError};

use logos::Logos;

use crate::Target::{x64Reg, Lexer, Reg};

/// An error which can occure during lexing
#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexingError {
    /// An invalid intenger
	InvalidInteger(String),
	#[default]
    /// A not supported character
	NonAsciiCharacter,
}

impl Display for LexingError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "error")
	}
}
impl From<Box<dyn Error>> for LexingError {
	fn from(_err: Box<dyn Error>) -> Self {
		LexingError::InvalidInteger(format!("error"))
	}
}

impl std::error::Error for LexingError {}

impl From<ParseIntError> for LexingError {
	fn from(err: ParseIntError) -> Self {
		LexingError::InvalidInteger(format!("{:?}", err.kind()))
	}
}

/// An assembly token
#[derive(Logos, Debug, Clone, PartialEq, Eq)]
#[logos(skip r"[ \t\n\f]+")]
#[logos(error = LexingError)]
#[doc(hidden)]
pub enum Token {
	#[regex("[a-zA-Z0-9_]+", priority = 5, callback = |lex| lex.slice().to_string())]
	Ident(String),

	#[regex("[0x0-9_]+", priority = 6, callback = |lex| {
		let string = lex.slice();
		if string.starts_with("0x") {
			u64::from_str_radix(&string.replace("0x", ""), 16)
		} else if string.starts_with("0b") {
			u64::from_str_radix(&string.replace("0b", ""), 2)
		} else {
			string.parse()
		}
	})]
	Num(u64),

    #[token(",")]
    Comma,

    #[token(":")]
    DoubleDot,

    /// x64 assembly instructions: Auto generated
	#[token("btr")]
	btr,
	#[token("add")]
	add,
	#[token("eems")]
	eems,
	#[token("cmovbe")]
	cmovbe,
	#[token("cmove")]
	cmove,
	#[token("cmovl")]
	cmovl,
	#[token("mov")]
	mov,
	#[token("bts")]
	bts,
	#[token("invd")]
	invd,
	#[token("and")]
	and,
	#[token("clc")]
	clc,
	#[token("dec")]
	dec,
	#[token("cmp")]
	cmp,
	#[token("aad")]
	aad,
	#[token("into")]
	into,
	#[token("incsspd")]
	incsspd,
	#[token("inc")]
	inc,
	#[token("bndcn")]
	bndcn,
	#[token("cdqe")]
	cdqe,
	#[token("cmovge")]
	cmovge,
	#[token("int3")]
	int3,
	#[token("cmova")]
	cmova,
	#[token("idiv")]
	idiv,
	#[token("ret")]
	ret,
	#[token("retf")]
	retf,
	#[token("adc")]
	adc,
	#[token("bndcu")]
	bndcu,
	#[token("clac")]
	clac,
	#[token("clts")]
	clts,
	#[token("adox")]
	adox,
	#[token("lar")]
	lar,
	#[token("cqo")]
	cqo,
	#[token("cmpxchg")]
	cmpxchg,
	#[token("das")]
	das,
	#[token("aas")]
	aas,
	#[token("int")]
	int,
	#[token("bswap")]
	bswap,
	#[token("bt")]
	bt,
	#[token("cmovg")]
	cmovg,
	#[token("aam")]
	aam,
	#[token("jmp")]
	jmp,
	#[token("aaa")]
	aaa,
	#[token("adcx")]
	adcx,
	#[token("clui")]
	clui,
	#[token("arpl")]
	arpl,
	#[token("cmc")]
	cmc,
	#[token("bsr")]
	bsr,
	#[token("imul")]
	imul,
	#[token("int1")]
	int1,
	#[token("btc")]
	btc,
	#[token("cpuid")]
	cpuid,
	#[token("hlt")]
	hlt,
	#[token("cmovb")]
	cmovb,
	#[token("bndcl")]
	bndcl,
	#[token("cbw")]
	cbw,
	#[token("daa")]
	daa,
	#[token("cli")]
	cli,
	#[token("endbr64")]
	endbr64,
	#[token("cmovae")]
	cmovae,
	#[token("crc32")]
	crc32,
	#[token("iret")]
	iret,
	#[token("iretd")]
	iretd,
	#[token("iretQ")]
	iretQ,
	#[token("lea")]
	lea,
	#[token("cwd")]
	cwd,
	#[token("cmovle")]
	cmovle,
	#[token("call")]
	call,
	#[token("cdq")]
	cdq,
	#[token("div")]
	div,
	#[token("bsf")]
	bsf,
	#[token("cld")]
	cld,
	#[token("cwde")]
	cwde,
	#[token("ja")]
	ja,
	#[token("jae")]
	jae,
	#[token("jb")]
	jb,
	#[token("jbe")]
	jbe,
	#[token("jc")]
	jc,
	#[token("je")]
	je,
	#[token("jz")]
	jz,
	#[token("jge")]
	jge,
	#[token("jl")]
	jl,
	#[token("jle")]
	jle,
	#[token("jna")]
	jna,
	#[token("jnae")]
	jnae,
	#[token("jnb")]
	jnb,
	#[token("jnbe")]
	jnbe,
	#[token("jnc")]
	jnc,
	#[token("jne")]
	jne,
	#[token("jng")]
	jng,
	#[token("jnge")]
	jnge,
	#[token("jnl")]
	jnl,
	#[token("jnle")]
	jnle,
	#[token("jno")]
	jno,
	#[token("jnp")]
	jnp,
	#[token("jns")]
	jns,
	#[token("jnz")]
	jnz,
	#[token("jo")]
	jo,
	#[token("jp")]
	jp,
	#[token("jpe")]
	jpe,
	#[token("jpo")]
	jpo,
	#[token("js")]
	js,
	#[token("endbr32")]
	endbr32,
	#[token("bl", |_| x64Reg::Bl, priority=6)]
	#[token("bx", |_| x64Reg::Bx, priority=6)]
	#[token("ebx", |_| x64Reg::Ebx, priority=6)]
	#[token("rbx", |_| x64Reg::Rbx, priority=6)]
	#[token("dil", |_| x64Reg::Dil, priority=6)]
	#[token("di", |_| x64Reg::Di, priority=6)]
	#[token("edi", |_| x64Reg::Edi, priority=6)]
	#[token("rdi", |_| x64Reg::Rdi, priority=6)]
	#[token("dl", |_| x64Reg::Dl, priority=6)]
	#[token("dx", |_| x64Reg::Dx, priority=6)]
	#[token("edx", |_| x64Reg::Edx, priority=6)]
	#[token("rdx", |_| x64Reg::Rdx, priority=6)]
	#[token("sil", |_| x64Reg::Sil, priority=6)]
	#[token("si", |_| x64Reg::Si, priority=6)]
	#[token("esi", |_| x64Reg::Esi, priority=6)]
	#[token("rsi", |_| x64Reg::Rsi, priority=6)]
	#[token("spl", |_| x64Reg::Spl, priority=6)]
	#[token("sp", |_| x64Reg::Sp, priority=6)]
	#[token("esp", |_| x64Reg::Esp, priority=6)]
	#[token("rsp", |_| x64Reg::Rsp, priority=6)]
	#[token("al", |_| x64Reg::Al, priority=6)]
	#[token("ax", |_| x64Reg::Ax, priority=6)]
	#[token("eax", |_| x64Reg::Eax, priority=6)]
	#[token("rax", |_| x64Reg::Rax, priority=6)]
	#[token("bpl", |_| x64Reg::Bpl, priority=6)]
	#[token("bp", |_| x64Reg::Bp, priority=6)]
	#[token("ebp", |_| x64Reg::Ebp, priority=6)]
	#[token("rbp", |_| x64Reg::Rbp, priority=6)]
	#[token("cl", |_| x64Reg::Cl, priority=6)]
	#[token("cx", |_| x64Reg::Cx, priority=6)]
	#[token("ecx", |_| x64Reg::Ecx, priority=6)]
	#[token("rcx", |_| x64Reg::Rcx, priority=6)]
	Reg(x64Reg),
    
	#[regex(r#"\[(\s*(\w+)\s*(\+\s*(\w+)\s*)?(\*\s*(1|2|4|8)\s*)?(\+\s*-?\d+|0x[0-9a-fA-F]+)?\s*)?\]"#, priority = 4, callback = |lex| {
		let string = lex.slice().to_string();
		Mem::from(&string)
	})]
	Mem(Mem),

	#[token("+", priority=5)]
	Add,
}

/// An assembly memory displacement
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mem {
    /// The encoded mod/rm + sib + displ
	pub enc: Vec<u8>,
}

impl Mem {
    /// Parses the string into an memory displacement
	pub fn from(string: &str) -> Result<Self, Box<dyn Error>> {
		let string = string.replace("[", "").replace("]", "");

		let mut tokens: VecDeque<Token> = VecDeque::new();

		for tok in Token::lexer(&string) {
			tokens.push_back( tok? ) 
		}

		if let Some(Token::Num(x)) = tokens.front() {
			let mut enc = vec![0b100, 0 | 0b100 << 3 | 0b101];
			enc.extend_from_slice(&(*x as u32).to_le_bytes().to_vec());
			Ok(Self {
				enc: enc,
			})
		} else if let Some(Token::Reg(op0)) = tokens.front() {
			if let Some(Token::Add) = tokens.get(1) {
				if let Some(Token::Reg(op1)) = tokens.get(2) {
					let enc = vec![0b00000100, 0 | op1.enc() << 3 | op0.enc()];
					Ok(Self {
						enc: enc,
					})
				} else if let Some(Token::Num(x)) = tokens.get(2) {
					todo!()
				} else { todo!("print error") }
			} else {
				let enc = vec![0 | op0.enc()];
				Ok(Self {
					enc: enc,
				})
			}

		} else { todo!() }
	}

    /// Returns the encoded
	pub fn enc(&mut self, op0: x64Reg) -> Vec<u8> {
		self.enc[0] |=  op0.enc() << 3;
		self.enc.clone()
	}
}


/// A temporary structure which implements the Lexer trait 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct x64Lexer {}

impl Lexer for x64Lexer {
	fn lex(&self, string: String) -> Result<Vec<Token>, Box<dyn Error>> {
		let mut tokens = vec![];
	
		for tok in Token::lexer(&string) {
			tokens.push( tok? );
		}
	
		Ok(tokens)
	}

	fn boxed(&self) -> Box<dyn Lexer> {
		Box::from( self.clone() )
	}
}