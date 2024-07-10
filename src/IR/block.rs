use crate::Support::Colorize;

use super::{Function, ir::Ir};

#[derive(Debug, Clone)]
/// A basic block: stores ir of a specific area of a function
pub struct Block {
    //pub(crate) func: Function,
    pub(crate) name: String,
    pub(crate) ir: Vec<Box<dyn Ir>>,
}

impl Block {
    /// Creates a new block
    pub fn new(name: &str, _func: &Function) -> Self {
        Self {
            //func: func.clone(),
            name: name.to_string(),
            ir: vec![],
        }
    }

    /// Emits the ir of the block into one string
    pub fn dump(&self) -> String {
        let mut dump = String::new();

        dump += &format!("{}:\n", self.name);

        for node in &self.ir {
            dump += &format!("\t{}\n", node.dump());
        }

        dump
    }

    pub(crate) fn push_ir(&mut self, ir: Box<dyn Ir>) {
        self.ir.push( ir );
    }

    /// Emits the ir of the block into one colored string
    pub fn dumpColored(&self) -> String {
        let mut dump = String::new();

        dump += &format!("{}:\n", self.name.cyan());

        for node in &self.ir {
            dump += &format!("\t{}\n", node.dumpColored());
        }

        dump
    }
}