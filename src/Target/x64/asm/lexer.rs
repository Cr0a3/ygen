use std::{error::Error, fmt::Display, num::ParseIntError};

use logos::Logos;

use crate::Target::{x64Reg, Reg};

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
	#[regex("[a-zA-Z0-9_]+", priority = 3, callback = |lex| lex.slice().to_string())]
	Ident(String),

	#[regex("[0x0-9_]+", priority = 5, callback = |lex| {
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

    #[token("+")]
    Add,

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
	#[token("bl", |_| x64Reg::Bl)]
	#[token("bx", |_| x64Reg::Bx)]
	#[token("ebx", |_| x64Reg::Ebx)]
	#[token("rbx", |_| x64Reg::Rbx)]
	#[token("dil", |_| x64Reg::Dil)]
	#[token("di", |_| x64Reg::Di)]
	#[token("edi", |_| x64Reg::Dil)]
	#[token("rdi", |_| x64Reg::Rdi)]
	#[token("dl", |_| x64Reg::Dl)]
	#[token("dx", |_| x64Reg::Dx)]
	#[token("edx", |_| x64Reg::Edx)]
	#[token("rdx", |_| x64Reg::Rdx)]
	#[token("sil", |_| x64Reg::Sil)]
	#[token("si", |_| x64Reg::Si)]
	#[token("esi", |_| x64Reg::Esi)]
	#[token("rsi", |_| x64Reg::Rsi)]
	#[token("spl", |_| x64Reg::Spl)]
	#[token("sp", |_| x64Reg::Sp)]
	#[token("esp", |_| x64Reg::Esp)]
	#[token("rsp", |_| x64Reg::Rsp)]
	#[token("al", |_| x64Reg::Al)]
	#[token("ax", |_| x64Reg::Ax)]
	#[token("eax", |_| x64Reg::Eax)]
	#[token("rax", |_| x64Reg::Rax)]
	#[token("bpl", |_| x64Reg::Bpl)]
	#[token("bp", |_| x64Reg::Bp)]
	#[token("ebp", |_| x64Reg::Ebp)]
	#[token("rbp", |_| x64Reg::Rbp)]
	#[token("cl", |_| x64Reg::Cl)]
	#[token("cx", |_| x64Reg::Cx)]
	#[token("ecx", |_| x64Reg::Ecx)]
	#[token("rcx", |_| x64Reg::Rcx)]
	Reg(x64Reg),
    
	#[regex(r"[a-zA-Z0-9\[\]]+", priority = 2, callback = |lex| {
		let string = lex.slice().to_string();
		Mem::from(&string)
	})]
	Mem(Mem),
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
		let mut tokens = vec![];
		for tok in Token::lexer(&string) {
			tokens.push( tok?) 
		}
		if let Some(Token::Num(x)) = tokens.first() {
			let mut enc = vec![0b100, 0 | 0b100 << 3 | 0b101];
			enc.extend_from_slice(&(*x as u32).to_le_bytes().to_vec());
			Ok(Self {
				enc: enc,
			})
		} else { todo!() }
	}

    /// Returns the encoded
	pub fn enc(&mut self, op0: x64Reg) -> Vec<u8> {
		self.enc[0] = 0b100 | op0.enc() << 3;
		self.enc.clone()
	}
}

/// Lexes (Turns the source into tokens) the incoming assembly string and outputs tokens
pub fn lex(string: String) -> Result<Vec<Token>, Box<dyn Error>> {
    let mut tokens = vec![];

    for tok in Token::lexer(&string) {
        tokens.push( tok? );
    }

    Ok(tokens)
}