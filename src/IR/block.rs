use crate::Support::Colorize;

use super::{ir::Ir, Function, VerifyError};

#[derive(Debug, Clone)]
/// A basic block: stores ir of a specific area of a function
pub struct Block {
    //pub(crate) func: Function,
    pub(crate) name: String,
    pub(crate) nodes: Vec<Box<dyn Ir>>,
    varCount: usize,
}

impl Block {
    /// Creates a new block
    pub fn new(name: &str, _func: &Function) -> Self {
        Self {
            //func: func.clone(),
            name: name.to_string(),
            nodes: vec![],
            varCount: 0,
        }
    }

    /// Emits the ir of the block into one string
    pub fn dump(&self) -> String {
        let mut dump = String::new();

        dump += &format!("{}:\n", self.name);

        for node in &self.nodes {
            dump += &format!("\t{}\n", node.dump());
        }

        dump
    }

    pub(crate) fn push_ir(&mut self, ir: Box<dyn Ir>) {
        self.nodes.push( ir );
    }

    /// Emits the ir of the block into one colored string
    pub fn dumpColored(&self) -> String {
        let mut dump = String::new();

        dump += &format!("{}:\n", self.name.cyan());

        for node in &self.nodes {
            dump += &format!("    {}\n", node.dumpColored());
        }

        dump
    }

    /// Requests an new variable name - which is the current var index
    /// Also counts up by one
    pub fn reqVarName(&mut self) -> String {
        let num = self.varCount;
        self.varCount += 1; 

        num.to_string()
    }

    /// Verifys if the block and all of its ir nodes are correct:
    ///  * Checks if the return type is the actual specified return type of the function
    pub fn verify(&self, func: &Function) -> Result<(), VerifyError> {
        for node in &self.nodes {
            node.verify(func.ty.clone())?;
        }

        Ok(())
    }
}