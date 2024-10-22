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
        let mut fix = match variant {
            X64AbsSymType::RipRel => vec![0x48, 0x8D, 0x05, 0x07, 0x00, 0x00, 0x00, 0xFF, 0x25, 0x08, 0x00, 0x00, 0x00],
            X64AbsSymType::Jmp => vec![0xFF, 0x25, 0x00, 0x00, 0x00, 0x00] ,
            X64AbsSymType::Call => vec![0xFF, 0x15, 0x06, 0x00, 0x00, 0x00, 0xFF, 0x25, 0x08, 0x00, 0x00, 0x00],
        };

        // Write

        let bytes = adr.to_be_bytes();

        fix.push(bytes[0]);
        fix.push(bytes[1]);
        fix.push(bytes[2]);
        fix.push(bytes[3]);
        fix.push(bytes[4]);
        fix.push(bytes[5]);
        fix.push(bytes[6]);
        fix.push(bytes[7]);

        fix.reverse();

        for fix in fix {
            code.insert(pos - bytes_to_remove + 1, fix);
        }


        println!("{:02X?}", code);
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