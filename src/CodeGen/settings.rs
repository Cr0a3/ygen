use super::instr::{MachineInstr, MCInstr};

/// Machine specific settings, like the lowering function
/// prolog, epilog functions, etc
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MCSettings {
    lowering: Option<fn(input: Vec<MachineInstr>) -> Vec<Box<dyn MCInstr>>>,
    prolog: Option<fn() -> Vec<Box<dyn MCInstr>>>,
    epilog: Option<fn() -> Vec<Box<dyn MCInstr>>>,
}

impl MCSettings {
    /// Creates new machine settings
    pub fn new() -> Self {
        Self {
            lowering: None,
            prolog: None,
            epilog: None,
        }
    }
}