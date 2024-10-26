use crate::Jit::AbsSymDealer;
use crate::Target::x64::instr::*;
use crate::Target::x64::X64Reg;

/// handler for x64 abs syms
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct X64AbsSymDealer {}

/// The type of the abs sym (rep relative, call, jmp and so on)
enum X64AbsSymType {
    RipRel,
    Jmp,
    Call,
}

impl AbsSymDealer for X64AbsSymDealer {
    fn handle(&self, code: &mut Vec<u8>, pos: usize, adr: usize) {
        // FIND OUT X64AbsSymType
        let variant = code.get(pos - 4).unwrap();

        let variant = match variant {
            0xE8 => X64AbsSymType::Call,
            0xE9 => X64AbsSymType::Jmp,
            0x05 => X64AbsSymType::RipRel,
            _ => panic!("find out variant for {}", variant),
        };

        let bytes_to_remove = match variant {
            X64AbsSymType::RipRel => 7,
            X64AbsSymType::Jmp => 5,
            X64AbsSymType::Call => 5,
        };

        
        for _ in 0..=bytes_to_remove {
            code.remove(pos - bytes_to_remove);
        }

        println!("{:02x?}", code);
        // Fix
        let mut fixup = Vec::new();

        fixup.push( X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(X64Reg::Rax), Operand::Imm(adr as i64)));

        fixup.push(match variant {
            X64AbsSymType::RipRel => X64MCInstr::with2(Mnemonic::Lea, Operand::Reg(X64Reg::Rax), Operand::Mem(X64Reg::Rax + 0)),
            X64AbsSymType::Jmp => X64MCInstr::with1(Mnemonic::Jmp, Operand::Reg(X64Reg::Rax)).make_far(),
            X64AbsSymType::Call => X64MCInstr::with1(Mnemonic::Jmp, Operand::Reg(X64Reg::Rax)).make_far(),
        });

        // Write
        let mut fixup = {
            let mut sink = Vec::new();
            for instr in fixup {
                sink.extend_from_slice( &instr.compile().expect("compilation failed") )
            }

            sink
        };

        fixup.reverse();

        for fix in fixup {
            code.insert(pos - bytes_to_remove, fix);
        }

        todo!("unsupported");
    }

    fn dbg(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("X64AbsSymDealer").finish()
    }

    fn cl(&self) -> Box<dyn AbsSymDealer> {
        Box::new( self.clone() )
    }

    fn eqal(&self, other: &Box<dyn AbsSymDealer>) -> bool {
        self.cl() == other.cl()
    }
}