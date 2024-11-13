use super::dag;
use crate::Target::instr::McInstr;

/// Loweres the dag to assembly
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DagLower {
    lower_func: Option<fn(dag::DagFunction) -> Vec<Box<dyn McInstr>>>
}

impl DagLower {
    /// Creates a new dag lower
    pub fn new(lower: fn(dag::DagFunction) -> Vec<Box<dyn McInstr>>) -> Self {
        Self {
            lower_func: Some(lower),
        }
    }

    /// Lowers the dag to assembly
    pub fn lower(&self, func: dag::DagFunction) -> Vec<Box<dyn McInstr>> {
        if let Some(lower_func) = self.lower_func {
            lower_func(func)
        } else {
            panic!("no registered dag lowering function")
        }
    }
}