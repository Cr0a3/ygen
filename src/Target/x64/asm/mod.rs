mod lexer;
mod parser;
/// x64 instruction encoding (compilation) and verifycation
pub mod instr;
/// x64 instruction set architecture specific stuff like rex prefix
pub mod isa;
mod optimizer;

pub use lexer::*;
pub use parser::*;
//pub(crate) use optimizer::*;