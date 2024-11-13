use super::instr::McInstr;

/// Target specific assembly parsing
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AsmParser {

}

impl AsmParser {
    /// Parses the input assembly instruction into a mc instr
    pub fn parse(&self, input: &str) -> Result<Box<dyn McInstr>, Box<dyn std::error::Error>> {
        todo!()
    }
}