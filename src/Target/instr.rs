use std::fmt::Display;

/// A target specific assembly instruction
pub trait McInstr {
    /// returns the assembly string of the instruction
    fn asm(&self) -> String;

    /// encodes the instruction
    fn encode(&self) -> Vec<u8>;
}

impl Display for Box<dyn McInstr> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.asm())
    }
}