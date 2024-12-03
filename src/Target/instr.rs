use std::fmt::Display;

use crate::Obj;

/// A target specific assembly instruction
pub trait McInstr {
    /// returns the assembly string of the instruction
    fn asm(&self) -> String;

    /// encodes the instruction
    fn encode(&self) -> Vec<u8>;

    /// Returns a option of a block branch
    fn branch_to_block(&self) -> Option<Obj::Link>;
    
    /// Returns a option of a relocation
    fn relocation(&self) -> Option<Obj::Link>;
}

impl Display for Box<dyn McInstr> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.asm())
    }
}

impl PartialEq for Box<dyn McInstr> {
    fn eq(&self, other: &Self) -> bool {
        self.asm() == other.asm()
    }
}