use crate::Support::{ColorProfile, Colorize};

use super::{ir::Ir, Function, Var, VerifyError};

/// A basic block: stores ir of a specific area of a function
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    //pub(crate) func: Function,
    pub(crate) name: String,
    pub(crate) nodes: Vec<Box<dyn Ir>>,
    pub(crate) varCount: usize,
}

impl Block {
    /// Creates a new block
    pub fn new(name: &str, func: &Function) -> Self {
        Self {
            //func: func.clone(),
            name: name.to_string(),
            nodes: vec![],
            varCount: func.ty.args.len(),
        }
    }

    /// Emits the ir of the block into one string
    pub fn dump(&self) -> String {
        let mut dump = String::new();

        dump += &format!("  {}:\n", self.name);

        for node in &self.nodes {
            dump += &format!("\t{}\n", node.dump());
        }

        dump
    }

    pub(crate) fn push_ir(&mut self, ir: Box<dyn Ir>) {
        self.nodes.push( ir );
    }

    /// Emits the ir of the block into one colored string
    pub fn dumpColored(&self, profile: ColorProfile) -> String {
        let mut dump = String::new();

        dump += &format!("{}:\n", self.name.cyan());

        for node in &self.nodes {
            dump += &format!("    {}\n", node.dumpColored(profile));
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

    /// Returns true if the variable is used after the ir node
    pub(crate) fn isVarUsedAfterNode(&self, start: &Box<dyn Ir>, var: &Var) -> bool {
        let mut used = false;
        let mut started = false;
    
        for node in &self.nodes {
            if node.uses(var) && started {
                used = true;
            }
    
            if node.is(start) {
                started = true;
            }
        }
    
        used
    }
}

/// Creates an new block
pub fn Block(name: &str, func: &Function) -> Block {
    Block::new(name, func)
}