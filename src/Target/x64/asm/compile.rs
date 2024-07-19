use std::collections::VecDeque;
use std::error::Error;
use crate::Target::Reg;

use super::Token;

pub(crate) fn compile_btr(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::btr) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr32()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr32()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(15);
				result.push(179);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::btr) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr64()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr64()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(72);
				result.push(15);
				result.push(179);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::btr) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr32()) && matches!(tokens.clone().pop_back(), Some(Token::Num(_))) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Num(op1)) = tokens.pop_front() {
				result.push(15);
				result.push(186);
				result.push(240 + op0.enc());
				let bytes = op1.to_le_bytes();
				result.push(bytes[0]);
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::btr) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr64()) && matches!(tokens.clone().pop_back(), Some(Token::Num(_))) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Num(op1)) = tokens.pop_front() {
				result.push(72);
				result.push(15);
				result.push(186);
				result.push(240 + op0.enc());
				let bytes = op1.to_le_bytes();
				result.push(bytes[0]);
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_add(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::add) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr16()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr16()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(102);
				result.push(129);
				result.push(192);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::add) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr32()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr32()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(3);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::add) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr64()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr64()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(72);
				result.push(129);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::add) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr16()) && matches!(tokens.clone().pop_back(), Some(Token::Num(_))) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Num(op1)) = tokens.pop_front() {
				result.push(102);
				result.push(131);
				result.push(192 + op0.enc());
				let bytes = op1.to_le_bytes();
				result.push(bytes[0]);
				result.push(bytes[1]);
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::add) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr32()) && matches!(tokens.clone().pop_back(), Some(Token::Num(_))) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Num(op1)) = tokens.pop_front() {
				result.push(129);
				result.push(192 + op0.enc());
				let bytes = op1.to_le_bytes();
				result.push(bytes[0]);
				result.push(bytes[1]);
				result.push(bytes[2]);
				result.push(bytes[3]);
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::add) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr64()) && matches!(tokens.clone().pop_back(), Some(Token::Num(_))) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Num(op1)) = tokens.pop_front() {
				result.push(72);
				result.push(129);
				result.push(192 + op0.enc());
				let bytes = op1.to_le_bytes();
				result.push(bytes[0]);
				result.push(bytes[1]);
				result.push(bytes[2]);
				result.push(bytes[3]);
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_EEMS(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::eems) && tokens.len() == 0 {
		result.push(15);
		result.push(119);
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_cmovbe(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::cmovbe) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr16()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr16()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(102);
				result.push(15);
				result.push(70);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::cmovbe) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr32()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr32()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(15);
				result.push(70);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::cmovbe) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr64()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr64()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(72);
				result.push(15);
				result.push(70);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_cmove(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::cmove) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr16()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr16()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(102);
				result.push(15);
				result.push(68);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::cmove) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr32()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr32()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(15);
				result.push(68);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::cmove) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr64()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr64()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(72);
				result.push(15);
				result.push(68);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_cmovl(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::cmovl) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr16()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr16()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(102);
				result.push(15);
				result.push(76);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::cmovl) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr32()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr32()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(15);
				result.push(76);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::cmovl) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr64()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr64()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(72);
				result.push(15);
				result.push(76);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_mov(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::mov) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr16()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr16()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(102);
				result.push(137);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::mov) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr32()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr32()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(139);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::mov) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr64()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr64()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(72);
				result.push(139);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::mov) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr16()) && matches!(tokens.clone().pop_back(), Some(Token::Num(_))) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Num(op1)) = tokens.pop_front() {
				result.push(102);
				result.push(184 + op0.enc());
				let bytes = op1.to_le_bytes();
				result.push(bytes[0]);
				result.push(bytes[1]);
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::mov) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr32()) && matches!(tokens.clone().pop_back(), Some(Token::Num(_))) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Num(op1)) = tokens.pop_front() {
				result.push(184 + op0.enc());
				let bytes = op1.to_le_bytes();
				result.push(bytes[0]);
				result.push(bytes[1]);
				result.push(bytes[2]);
				result.push(bytes[3]);
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_bts(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::bts) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr32()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr32()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(15);
				result.push(171);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::bts) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr64()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr64()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(72);
				result.push(15);
				result.push(171);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::bts) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr32()) && matches!(tokens.clone().pop_back(), Some(Token::Num(_))) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Num(op1)) = tokens.pop_front() {
				result.push(15);
				result.push(186);
				result.push(232 + op0.enc());
				let bytes = op1.to_le_bytes();
				result.push(bytes[0]);
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::bts) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr64()) && matches!(tokens.clone().pop_back(), Some(Token::Num(_))) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Num(op1)) = tokens.pop_front() {
				result.push(72);
				result.push(15);
				result.push(186);
				result.push(232 + op0.enc());
				let bytes = op1.to_le_bytes();
				result.push(bytes[0]);
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_invd(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::invd) && tokens.len() == 0 {
		result.push(15);
		result.push(8);
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_and(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::and) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr16()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr16()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(102);
				result.push(33);
				result.push(224);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::and) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr32()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr32()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(33);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::and) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr64()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr64()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(72);
				result.push(33);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::and) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr16()) && matches!(tokens.clone().pop_back(), Some(Token::Num(_))) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Num(op1)) = tokens.pop_front() {
				result.push(102);
				result.push(129);
				result.push(224 + op0.enc());
				let bytes = op1.to_le_bytes();
				result.push(bytes[0]);
				result.push(bytes[1]);
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::and) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr32()) && matches!(tokens.clone().pop_back(), Some(Token::Num(_))) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Num(op1)) = tokens.pop_front() {
				result.push(129);
				result.push(224 + op0.enc());
				let bytes = op1.to_le_bytes();
				result.push(bytes[0]);
				result.push(bytes[1]);
				result.push(bytes[2]);
				result.push(bytes[3]);
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::and) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr64()) && matches!(tokens.clone().pop_back(), Some(Token::Num(_))) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Num(op1)) = tokens.pop_front() {
				result.push(72);
				result.push(129);
				result.push(224 + op0.enc());
				let bytes = op1.to_le_bytes();
				result.push(bytes[0]);
				result.push(bytes[1]);
				result.push(bytes[2]);
				result.push(bytes[3]);
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_clc(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::clc) && tokens.len() == 0 {
		result.push(248);
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_dec(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::dec) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr16()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
				result.push(102);
				result.push(255);
				result.push(200);
				result.push(0b11000000 | op0.enc());
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::dec) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr32()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
				result.push(255);
				result.push(200);
				result.push(0b11000000 | op0.enc());
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::dec) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr16()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
				result.push(72);
				result.push(255);
				result.push(200);
				result.push(0b11000000 | op0.enc());
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_cmp(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::cmp) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr16()) && matches!(tokens.clone().pop_back(), Some(Token::Num(_))) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Num(op1)) = tokens.pop_front() {
				result.push(102);
				result.push(129);
				result.push(248 + op0.enc());
				let bytes = op1.to_le_bytes();
				result.push(bytes[0]);
				result.push(bytes[1]);
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::cmp) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr32()) && matches!(tokens.clone().pop_back(), Some(Token::Num(_))) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Num(op1)) = tokens.pop_front() {
				result.push(129);
				result.push(248 + op0.enc());
				let bytes = op1.to_le_bytes();
				result.push(bytes[0]);
				result.push(bytes[1]);
				result.push(bytes[2]);
				result.push(bytes[3]);
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::cmp) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr64()) && matches!(tokens.clone().pop_back(), Some(Token::Num(_))) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Num(op1)) = tokens.pop_front() {
				result.push(72);
				result.push(129);
				result.push(248 + op0.enc());
				let bytes = op1.to_le_bytes();
				result.push(bytes[0]);
				result.push(bytes[1]);
				result.push(bytes[2]);
				result.push(bytes[3]);
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::cmp) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr16()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr16()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(102);
				result.push(57);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::cmp) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr32()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr32()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(57);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::cmp) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr64()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr64()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(72);
				result.push(57);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_aad(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::aad) && tokens.len() == 0 {
		result.push(213);
		result.push(160);
	}
	else if matches!(instr, Token::aad) && matches!(tokens.front(), Some(Token::Num(_))) {
		if let Some(Token::Num(op0)) = tokens.pop_front() {
				result.push(213);
				let bytes = op0.to_le_bytes();
				result.push(bytes[0]);
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_into(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::into) && tokens.len() == 0 {
		result.push(206);
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_incsspd(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::incsspd) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr32()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
				result.push(243);
				result.push(15);
				result.push(174);
				result.push(232 + op0.enc());
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_inc(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::inc) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr16()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
				result.push(102);
				result.push(255);
				result.push(192);
				result.push(0b11000000 | op0.enc());
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::inc) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr32()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
				result.push(255);
				result.push(192 + op0.enc());
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::inc) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr64()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
				result.push(72);
				result.push(255);
				result.push(192 + op0.enc());
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_bndcn(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::bndcn) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr32()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
				result.push(242);
				result.push(15);
				result.push(27);
				result.push(0b11000000 | op0.enc());
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::bndcn) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr64()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
				result.push(242);
				result.push(15);
				result.push(27);
				result.push(0b11000000 | op0.enc());
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_cdqe(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::cdqe) && tokens.len() == 0 {
		result.push(72);
		result.push(152);
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_cmovge(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::cmovge) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr16()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr16()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(102);
				result.push(15);
				result.push(77);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::cmovge) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr32()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr32()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(15);
				result.push(77);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::cmovge) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr64()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr64()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(72);
				result.push(15);
				result.push(77);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_int3(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::int3) && tokens.len() == 0 {
		result.push(204);
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_cmova(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::cmova) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr16()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr16()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(102);
				result.push(15);
				result.push(71);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::cmova) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr32()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr32()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(15);
				result.push(71);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::cmova) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr64()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr64()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(72);
				result.push(15);
				result.push(71);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_idiv(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::idiv) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr16()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
				result.push(102);
				result.push(247);
				result.push(248 + op0.enc());
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::idiv) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr32()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
				result.push(247);
				result.push(248 + op0.enc());
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::idiv) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr64()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
				result.push(72);
				result.push(247);
				result.push(248 + op0.enc());
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_ret(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::ret) && tokens.len() == 0 {
		result.push(195);
	}
	else if matches!(instr, Token::retf) && tokens.len() == 0 {
		result.push(203);
	}
	else if matches!(instr, Token::ret) && matches!(tokens.front(), Some(Token::Num(_))) {
		if let Some(Token::Num(op0)) = tokens.pop_front() {
				result.push(194);
				let bytes = op0.to_le_bytes();
				result.push(bytes[0]);
				result.push(bytes[1]);
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::retf) && matches!(tokens.front(), Some(Token::Num(_))) {
		if let Some(Token::Num(op0)) = tokens.pop_front() {
				result.push(202);
				let bytes = op0.to_le_bytes();
				result.push(bytes[0]);
				result.push(bytes[1]);
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_adc(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::adc) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr16()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr16()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(17);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::adc) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr32()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr32()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(19);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::adc) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr64()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr64()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(72);
				result.push(19);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::adc) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr16()) && matches!(tokens.clone().pop_back(), Some(Token::Num(_))) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Num(op1)) = tokens.pop_front() {
				result.push(129 + op0.enc());
				let bytes = op1.to_le_bytes();
				result.push(bytes[0]);
				result.push(bytes[1]);
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::adc) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr32()) && matches!(tokens.clone().pop_back(), Some(Token::Num(_))) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Num(op1)) = tokens.pop_front() {
				result.push(129);
				result.push(208 + op0.enc());
				let bytes = op1.to_le_bytes();
				result.push(bytes[0]);
				result.push(bytes[1]);
				result.push(bytes[2]);
				result.push(bytes[3]);
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::adc) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr64()) && matches!(tokens.clone().pop_back(), Some(Token::Num(_))) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Num(op1)) = tokens.pop_front() {
				result.push(72);
				result.push(129);
				result.push(208 + op0.enc());
				let bytes = op1.to_le_bytes();
				result.push(bytes[0]);
				result.push(bytes[1]);
				result.push(bytes[2]);
				result.push(bytes[3]);
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_bndcu(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::bndcu) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr32()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
				result.push(242);
				result.push(15);
				result.push(26);
				result.push(0b11000000 | op0.enc());
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::bndcu) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr64()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
				result.push(242);
				result.push(15);
				result.push(26);
				result.push(0b11000000 | op0.enc());
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_clac(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::clac) && tokens.len() == 0 {
		result.push(15);
		result.push(1);
		result.push(202);
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_clts(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::clts) && tokens.len() == 0 {
		result.push(15);
		result.push(6);
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_adox(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::adox) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr32()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr32()) {
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
				result.push(102);
				result.push(243);
				result.push(15);
				result.push(56);
				result.push(246);
				result.push(0b11000000 | (op1.enc()) << 3 | (op0.enc()));
			} else {
				return Err("invalid op0 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::adox) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr64()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr64()) {
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
				result.push(243);
				result.push(72);
				result.push(15);
				result.push(56);
				result.push(246);
				result.push(0b11000000 | (op1.enc()) << 3 | (op0.enc()));
			} else {
				return Err("invalid op0 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_lar(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::lar) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr16()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr16()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(15);
				result.push(2);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::lar) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr32()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr32()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(15);
				result.push(2);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_cqo(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::cqo) && tokens.len() == 0 {
		result.push(72);
		result.push(153);
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_cmpxchg(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::cmpxchg) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr16()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr16()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(102);
				result.push(15);
				result.push(176);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::cmpxchg) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr32()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr32()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(15);
				result.push(176);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::cmpxchg) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr64()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr64()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(72);
				result.push(15);
				result.push(176);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_das(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::das) && tokens.len() == 0 {
		result.push(47);
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_aas(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::aas) && tokens.len() == 0 {
		result.push(63);
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_int(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::int) && matches!(tokens.front(), Some(Token::Num(_))) {
		if let Some(Token::Num(op0)) = tokens.pop_front() {
				result.push(205);
				let bytes = op0.to_le_bytes();
				result.push(bytes[0]);
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_bswap(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::bswap) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr32()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
				result.push(15);
				result.push(200 + op0.enc());
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::bswap) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr64()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
				result.push(72);
				result.push(15);
				result.push(200 + op0.enc());
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_bt(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::bt) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr32()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr32()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(15);
				result.push(163);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::bt) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr64()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr64()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(72);
				result.push(15);
				result.push(163);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::bt) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr32()) && matches!(tokens.clone().pop_back(), Some(Token::Num(_))) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Num(op1)) = tokens.pop_front() {
				result.push(15);
				result.push(163);
				result.push(224 + op0.enc());
				let bytes = op1.to_le_bytes();
				result.push(bytes[0]);
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::bt) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr64()) && matches!(tokens.clone().pop_back(), Some(Token::Num(_))) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Num(op1)) = tokens.pop_front() {
				result.push(72);
				result.push(15);
				result.push(163);
				result.push(224 + op0.enc());
				let bytes = op1.to_le_bytes();
				result.push(bytes[0]);
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_cmovg(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::cmovg) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr16()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr16()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(102);
				result.push(15);
				result.push(79);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::cmovg) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr32()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr32()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(15);
				result.push(79);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::cmovg) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr64()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr64()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(72);
				result.push(15);
				result.push(79);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_aam(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::aam) && tokens.len() == 0 {
		result.push(212);
		result.push(160);
	}
	else if matches!(instr, Token::aam) && matches!(tokens.front(), Some(Token::Num(_))) {
		if let Some(Token::Num(op0)) = tokens.pop_front() {
				result.push(212);
				let bytes = op0.to_le_bytes();
				result.push(bytes[0]);
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_jmp(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::jmp) && matches!(tokens.front(), Some(Token::Num(_))) {
		if let Some(Token::Num(op0)) = tokens.pop_front() {
				result.push(235);
				let bytes = op0.to_le_bytes();
				result.push(bytes[0]);
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::jmp) && matches!(tokens.front(), Some(Token::Num(_))) {
		if let Some(Token::Num(op0)) = tokens.pop_front() {
				result.push(233);
				let bytes = op0.to_le_bytes();
				result.push(bytes[0]);
				result.push(bytes[1]);
				result.push(bytes[2]);
				result.push(bytes[3]);
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::jmp) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr64()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
				result.push(255);
				result.push(224 + op0.enc());
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_aaa(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::aaa) && tokens.len() == 0 {
		result.push(55);
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_adcx(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::adcx) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr32()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr32()) {
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
				result.push(102);
				result.push(15);
				result.push(56);
				result.push(246);
				result.push(0b11000000 | (op1.enc()) << 3 | (op0.enc()));
			} else {
				return Err("invalid op0 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::adcx) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr64()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr64()) {
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
				result.push(102);
				result.push(72);
				result.push(15);
				result.push(56);
				result.push(246);
				result.push(0b11000000 | (op1.enc()) << 3 | (op0.enc()));
			} else {
				return Err("invalid op0 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_clui(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::clui) && tokens.len() == 0 {
		result.push(243);
		result.push(15);
		result.push(1);
		result.push(238);
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_arpl(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::arpl) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr16()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr16()) {
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
				result.push(102);
				result.push(99);
				result.push(0b11000000 | (op1.enc()) << 3 | (op0.enc()));
			} else {
				return Err("invalid op0 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_cmc(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::cmc) && tokens.len() == 0 {
		result.push(245);
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_bsr(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::bsr) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr32()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr32()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(15);
				result.push(189);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::bsr) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr64()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr64()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(72);
				result.push(15);
				result.push(189);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_imul(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::imul) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr16()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr16()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(102);
				result.push(15);
				result.push(175);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::imul) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr32()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr32()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(15);
				result.push(175);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::imul) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr64()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr64()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(72);
				result.push(15);
				result.push(175);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_int1(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::int1) && tokens.len() == 0 {
		result.push(241);
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_btc(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::btc) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr32()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr32()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(15);
				result.push(187);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::btc) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr64()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr64()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(72);
				result.push(15);
				result.push(187);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::btc) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr32()) && matches!(tokens.clone().pop_back(), Some(Token::Num(_))) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Num(op1)) = tokens.pop_front() {
				result.push(15);
				result.push(186);
				result.push(248 + op0.enc());
				let bytes = op1.to_le_bytes();
				result.push(bytes[0]);
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::btc) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr64()) && matches!(tokens.clone().pop_back(), Some(Token::Num(_))) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Num(op1)) = tokens.pop_front() {
				result.push(72);
				result.push(15);
				result.push(186);
				result.push(248 + op0.enc());
				let bytes = op1.to_le_bytes();
				result.push(bytes[0]);
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_cpuid(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::cpuid) && tokens.len() == 0 {
		result.push(15);
		result.push(162);
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_hlt(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::hlt) && tokens.len() == 0 {
		result.push(244);
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_cmovb(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::cmovb) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr16()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr16()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(102);
				result.push(15);
				result.push(66);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::cmovb) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr32()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr32()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(15);
				result.push(66);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::cmovb) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr64()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr64()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(72);
				result.push(15);
				result.push(66);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_bndcl(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::bndcl) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr32()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
				result.push(243);
				result.push(15);
				result.push(26);
				result.push(0b11000000 | op0.enc());
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::bndcl) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr64()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
				result.push(243);
				result.push(15);
				result.push(26);
				result.push(0b11000000 | op0.enc());
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_cbw(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::cbw) && tokens.len() == 0 {
		result.push(102);
		result.push(152);
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_daa(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::daa) && tokens.len() == 0 {
		result.push(39);
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_cli(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::cli) && tokens.len() == 0 {
		result.push(250);
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_ENDBR64(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::endbr64) && tokens.len() == 0 {
		result.push(243);
		result.push(15);
		result.push(30);
		result.push(250);
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_cmovae(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::cmovae) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr16()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr16()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(102);
				result.push(15);
				result.push(67);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::cmovae) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr32()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr32()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(15);
				result.push(67);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::cmovae) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr64()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr64()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(72);
				result.push(15);
				result.push(67);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_crc32(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::crc32) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr32()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr32()) {
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
				result.push(242);
				result.push(15);
				result.push(56);
				result.push(241);
				result.push(0b11000000 | (op1.enc()) << 3 | (op0.enc()));
			} else {
				return Err("invalid op0 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::crc32) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr64()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr64()) {
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
				result.push(242);
				result.push(72);
				result.push(15);
				result.push(56);
				result.push(241);
				result.push(0b11000000 | (op1.enc()) << 3 | (op0.enc()));
			} else {
				return Err("invalid op0 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_iret(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::iret) && tokens.len() == 0 {
		result.push(102);
		result.push(207);
	}
	else if matches!(instr, Token::iretd) && tokens.len() == 0 {
		result.push(207);
	}
	else if matches!(instr, Token::iretQ) && tokens.len() == 0 {
		result.push(72);
		result.push(207);
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_lea(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::lea) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr16()) && matches!(tokens.clone().pop_back(), Some(Token::Mem(_))) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Mem(mut op1)) = tokens.pop_front() {
				result.push(102);
				result.push(141);
				result.extend_from_slice(&op1.enc(op0));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::lea) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr32()) && matches!(tokens.clone().pop_back(), Some(Token::Mem(_))) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Mem(mut op1)) = tokens.pop_front() {
				result.push(103);
				result.push(141);
				result.extend_from_slice(&op1.enc(op0));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::lea) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr64()) && matches!(tokens.clone().pop_back(), Some(Token::Mem(_))) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Mem(mut op1)) = tokens.pop_front() {
				result.push(72);
				result.push(141);
				result.extend_from_slice(&op1.enc(op0));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_cwd(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::cwd) && tokens.len() == 0 {
		result.push(153);
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_cmovle(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::cmovle) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr16()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr16()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(102);
				result.push(15);
				result.push(78);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::cmovle) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr32()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr32()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(15);
				result.push(78);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::cmovle) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr64()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr64()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(72);
				result.push(15);
				result.push(78);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_call(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::call) && matches!(tokens.front(), Some(Token::Num(_))) {
		if let Some(Token::Num(op0)) = tokens.pop_front() {
				result.push(232);
				let bytes = op0.to_le_bytes();
				result.push(bytes[0]);
				result.push(bytes[1]);
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::call) && matches!(tokens.front(), Some(Token::Num(_))) {
		if let Some(Token::Num(op0)) = tokens.pop_front() {
				result.push(232);
				let bytes = op0.to_le_bytes();
				result.push(bytes[0]);
				result.push(bytes[1]);
				result.push(bytes[2]);
				result.push(bytes[3]);
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::call) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr16()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
				result.push(102);
				result.push(255);
				result.push(208 + op0.enc());
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::call) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr32()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
				result.push(255);
				result.push(208 + op0.enc());
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::call) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr64()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
				result.push(255);
				result.push(208 + op0.enc());
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_cdq(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::cdq) && tokens.len() == 0 {
		result.push(153);
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_div(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::div) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr16()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
				result.push(102);
				result.push(247);
				result.push(240 + op0.enc());
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::div) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr32()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
				result.push(247);
				result.push(240 + op0.enc());
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::div) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr64()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
				result.push(56);
				result.push(247);
				result.push(240 + op0.enc());
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_bsf(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::bsf) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr32()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr32()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(15);
				result.push(188);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::bsf) && matches!(tokens.front(), Some(Token::Reg(reg)) if reg.is_gr64()) && matches!(tokens.clone().pop_back(), Some(Token::Reg(reg)) if reg.is_gr64()) {
		if let Some(Token::Reg(op0)) = tokens.pop_front() {
			if Some(&Token::Comma) != tokens.front() {
				Err("expected a comma (,) after first operand")?
			} else {
				tokens.pop_front(); // skip ,
			}
		if let Some(Token::Reg(op1)) = tokens.pop_front() {
				result.push(72);
				result.push(15);
				result.push(188);
				result.push(0b11000000 | (op0.enc()) << 3 | (op1.enc()));
			} else {
				return Err("invalid op1 operand".into());
			}
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_cld(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::cld) && tokens.len() == 0 {
		result.push(252);
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_cwde(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::cwde) && tokens.len() == 0 {
		result.push(152);
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_jcc(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::ja) && matches!(tokens.front(), Some(Token::Num(_))) {
		if let Some(Token::Num(op0)) = tokens.pop_front() {
				result.push(15);
				result.push(135);
				let bytes = op0.to_le_bytes();
				result.push(bytes[0]);
				result.push(bytes[1]);
				result.push(bytes[2]);
				result.push(bytes[3]);
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::jae) && matches!(tokens.front(), Some(Token::Num(_))) {
		if let Some(Token::Num(op0)) = tokens.pop_front() {
				result.push(15);
				result.push(131);
				let bytes = op0.to_le_bytes();
				result.push(bytes[0]);
				result.push(bytes[1]);
				result.push(bytes[2]);
				result.push(bytes[3]);
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::jb) && matches!(tokens.front(), Some(Token::Num(_))) {
		if let Some(Token::Num(op0)) = tokens.pop_front() {
				result.push(15);
				result.push(130);
				let bytes = op0.to_le_bytes();
				result.push(bytes[0]);
				result.push(bytes[1]);
				result.push(bytes[2]);
				result.push(bytes[3]);
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::jb) && matches!(tokens.front(), Some(Token::Num(_))) {
		if let Some(Token::Num(op0)) = tokens.pop_front() {
				result.push(15);
				result.push(134);
				let bytes = op0.to_le_bytes();
				result.push(bytes[0]);
				result.push(bytes[1]);
				result.push(bytes[2]);
				result.push(bytes[3]);
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::jbe) && matches!(tokens.front(), Some(Token::Num(_))) {
		if let Some(Token::Num(op0)) = tokens.pop_front() {
				result.push(15);
				result.push(134);
				let bytes = op0.to_le_bytes();
				result.push(bytes[0]);
				result.push(bytes[1]);
				result.push(bytes[2]);
				result.push(bytes[3]);
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::jc) && matches!(tokens.front(), Some(Token::Num(_))) {
		if let Some(Token::Num(op0)) = tokens.pop_front() {
				result.push(15);
				result.push(130);
				let bytes = op0.to_le_bytes();
				result.push(bytes[0]);
				result.push(bytes[1]);
				result.push(bytes[2]);
				result.push(bytes[3]);
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::je) && matches!(tokens.front(), Some(Token::Num(_))) {
		if let Some(Token::Num(op0)) = tokens.pop_front() {
				result.push(15);
				result.push(132);
				let bytes = op0.to_le_bytes();
				result.push(bytes[0]);
				result.push(bytes[1]);
				result.push(bytes[2]);
				result.push(bytes[3]);
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::jz) && matches!(tokens.front(), Some(Token::Num(_))) {
		if let Some(Token::Num(op0)) = tokens.pop_front() {
				result.push(15);
				result.push(132);
				let bytes = op0.to_le_bytes();
				result.push(bytes[0]);
				result.push(bytes[1]);
				result.push(bytes[2]);
				result.push(bytes[3]);
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::ja) && matches!(tokens.front(), Some(Token::Num(_))) {
		if let Some(Token::Num(op0)) = tokens.pop_front() {
				result.push(15);
				result.push(143);
				let bytes = op0.to_le_bytes();
				result.push(bytes[0]);
				result.push(bytes[1]);
				result.push(bytes[2]);
				result.push(bytes[3]);
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::jge) && matches!(tokens.front(), Some(Token::Num(_))) {
		if let Some(Token::Num(op0)) = tokens.pop_front() {
				result.push(15);
				result.push(141);
				let bytes = op0.to_le_bytes();
				result.push(bytes[0]);
				result.push(bytes[1]);
				result.push(bytes[2]);
				result.push(bytes[3]);
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::jl) && matches!(tokens.front(), Some(Token::Num(_))) {
		if let Some(Token::Num(op0)) = tokens.pop_front() {
				result.push(15);
				result.push(140);
				let bytes = op0.to_le_bytes();
				result.push(bytes[0]);
				result.push(bytes[1]);
				result.push(bytes[2]);
				result.push(bytes[3]);
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::jle) && matches!(tokens.front(), Some(Token::Num(_))) {
		if let Some(Token::Num(op0)) = tokens.pop_front() {
				result.push(15);
				result.push(142);
				let bytes = op0.to_le_bytes();
				result.push(bytes[0]);
				result.push(bytes[1]);
				result.push(bytes[2]);
				result.push(bytes[3]);
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::jna) && matches!(tokens.front(), Some(Token::Num(_))) {
		if let Some(Token::Num(op0)) = tokens.pop_front() {
				result.push(15);
				result.push(134);
				let bytes = op0.to_le_bytes();
				result.push(bytes[0]);
				result.push(bytes[1]);
				result.push(bytes[2]);
				result.push(bytes[3]);
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::jnae) && matches!(tokens.front(), Some(Token::Num(_))) {
		if let Some(Token::Num(op0)) = tokens.pop_front() {
				result.push(15);
				result.push(130);
				let bytes = op0.to_le_bytes();
				result.push(bytes[0]);
				result.push(bytes[1]);
				result.push(bytes[2]);
				result.push(bytes[3]);
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::jnb) && matches!(tokens.front(), Some(Token::Num(_))) {
		if let Some(Token::Num(op0)) = tokens.pop_front() {
				result.push(15);
				result.push(131);
				let bytes = op0.to_le_bytes();
				result.push(bytes[0]);
				result.push(bytes[1]);
				result.push(bytes[2]);
				result.push(bytes[3]);
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::jnbe) && matches!(tokens.front(), Some(Token::Num(_))) {
		if let Some(Token::Num(op0)) = tokens.pop_front() {
				result.push(15);
				result.push(135);
				let bytes = op0.to_le_bytes();
				result.push(bytes[0]);
				result.push(bytes[1]);
				result.push(bytes[2]);
				result.push(bytes[3]);
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::jnc) && matches!(tokens.front(), Some(Token::Num(_))) {
		if let Some(Token::Num(op0)) = tokens.pop_front() {
				result.push(15);
				result.push(131);
				let bytes = op0.to_le_bytes();
				result.push(bytes[0]);
				result.push(bytes[1]);
				result.push(bytes[2]);
				result.push(bytes[3]);
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::jne) && matches!(tokens.front(), Some(Token::Num(_))) {
		if let Some(Token::Num(op0)) = tokens.pop_front() {
				result.push(15);
				result.push(133);
				let bytes = op0.to_le_bytes();
				result.push(bytes[0]);
				result.push(bytes[1]);
				result.push(bytes[2]);
				result.push(bytes[3]);
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::jng) && matches!(tokens.front(), Some(Token::Num(_))) {
		if let Some(Token::Num(op0)) = tokens.pop_front() {
				result.push(15);
				result.push(142);
				let bytes = op0.to_le_bytes();
				result.push(bytes[0]);
				result.push(bytes[1]);
				result.push(bytes[2]);
				result.push(bytes[3]);
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::jnge) && matches!(tokens.front(), Some(Token::Num(_))) {
		if let Some(Token::Num(op0)) = tokens.pop_front() {
				result.push(15);
				result.push(140);
				let bytes = op0.to_le_bytes();
				result.push(bytes[0]);
				result.push(bytes[1]);
				result.push(bytes[2]);
				result.push(bytes[3]);
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::jnl) && matches!(tokens.front(), Some(Token::Num(_))) {
		if let Some(Token::Num(op0)) = tokens.pop_front() {
				result.push(15);
				result.push(141);
				let bytes = op0.to_le_bytes();
				result.push(bytes[0]);
				result.push(bytes[1]);
				result.push(bytes[2]);
				result.push(bytes[3]);
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::jnle) && matches!(tokens.front(), Some(Token::Num(_))) {
		if let Some(Token::Num(op0)) = tokens.pop_front() {
				result.push(15);
				result.push(143);
				let bytes = op0.to_le_bytes();
				result.push(bytes[0]);
				result.push(bytes[1]);
				result.push(bytes[2]);
				result.push(bytes[3]);
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::jno) && matches!(tokens.front(), Some(Token::Num(_))) {
		if let Some(Token::Num(op0)) = tokens.pop_front() {
				result.push(15);
				result.push(129);
				let bytes = op0.to_le_bytes();
				result.push(bytes[0]);
				result.push(bytes[1]);
				result.push(bytes[2]);
				result.push(bytes[3]);
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::jnp) && matches!(tokens.front(), Some(Token::Num(_))) {
		if let Some(Token::Num(op0)) = tokens.pop_front() {
				result.push(15);
				result.push(139);
				let bytes = op0.to_le_bytes();
				result.push(bytes[0]);
				result.push(bytes[1]);
				result.push(bytes[2]);
				result.push(bytes[3]);
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::jns) && matches!(tokens.front(), Some(Token::Num(_))) {
		if let Some(Token::Num(op0)) = tokens.pop_front() {
				result.push(15);
				result.push(137);
				let bytes = op0.to_le_bytes();
				result.push(bytes[0]);
				result.push(bytes[1]);
				result.push(bytes[2]);
				result.push(bytes[3]);
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::jnz) && matches!(tokens.front(), Some(Token::Num(_))) {
		if let Some(Token::Num(op0)) = tokens.pop_front() {
				result.push(15);
				result.push(133);
				let bytes = op0.to_le_bytes();
				result.push(bytes[0]);
				result.push(bytes[1]);
				result.push(bytes[2]);
				result.push(bytes[3]);
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::jo) && matches!(tokens.front(), Some(Token::Num(_))) {
		if let Some(Token::Num(op0)) = tokens.pop_front() {
				result.push(15);
				result.push(128);
				let bytes = op0.to_le_bytes();
				result.push(bytes[0]);
				result.push(bytes[1]);
				result.push(bytes[2]);
				result.push(bytes[3]);
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::jp) && matches!(tokens.front(), Some(Token::Num(_))) {
		if let Some(Token::Num(op0)) = tokens.pop_front() {
				result.push(15);
				result.push(138);
				let bytes = op0.to_le_bytes();
				result.push(bytes[0]);
				result.push(bytes[1]);
				result.push(bytes[2]);
				result.push(bytes[3]);
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::jpe) && matches!(tokens.front(), Some(Token::Num(_))) {
		if let Some(Token::Num(op0)) = tokens.pop_front() {
				result.push(15);
				result.push(138);
				let bytes = op0.to_le_bytes();
				result.push(bytes[0]);
				result.push(bytes[1]);
				result.push(bytes[2]);
				result.push(bytes[3]);
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::jpo) && matches!(tokens.front(), Some(Token::Num(_))) {
		if let Some(Token::Num(op0)) = tokens.pop_front() {
				result.push(15);
				result.push(139);
				let bytes = op0.to_le_bytes();
				result.push(bytes[0]);
				result.push(bytes[1]);
				result.push(bytes[2]);
				result.push(bytes[3]);
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::js) && matches!(tokens.front(), Some(Token::Num(_))) {
		if let Some(Token::Num(op0)) = tokens.pop_front() {
				result.push(15);
				result.push(136);
				let bytes = op0.to_le_bytes();
				result.push(bytes[0]);
				result.push(bytes[1]);
				result.push(bytes[2]);
				result.push(bytes[3]);
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else if matches!(instr, Token::jz) && matches!(tokens.front(), Some(Token::Num(_))) {
		if let Some(Token::Num(op0)) = tokens.pop_front() {
				result.push(15);
				result.push(132);
				let bytes = op0.to_le_bytes();
				result.push(bytes[0]);
				result.push(bytes[1]);
				result.push(bytes[2]);
				result.push(bytes[3]);
		} else {
			return Err("invalid op0 operand".into());
		}
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}
pub(crate) fn compile_ENDBR32(tokens: &mut VecDeque<Token>) -> Result<Vec<u8>, Box<dyn Error>> {
	let instr = tokens.pop_front().unwrap(); // safe bacuse this function is only called if this is some (look in compile_single function)
	let mut result = vec![];
	if matches!(instr, Token::endbr32) && tokens.len() == 0 {
		result.push(243);
		result.push(15);
		result.push(30);
		result.push(251);
	}
	else {
		Err(format!("invalid variant {:?}", tokens))?
	}
	Ok(result)
}