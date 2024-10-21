use crate::Jit::AbsSymDealer;

/// handler for x64 abs syms
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct X64AbsSymDealer {}

/// The type of the abs sym (rep relative, call, jmp and so on)
enum X64AbsSymType {
    RipRel,
    Je,
    Jne,
    Jmp,
    Call,
}

impl AbsSymDealer for X64AbsSymDealer {
    fn handle(&self, code: &mut Vec<u8>, pos: usize, adr: usize) {
        // FIND OUT X64AbsSymType
        todo!();
        // Fix
        todo!();
        // Write
        todo!();
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