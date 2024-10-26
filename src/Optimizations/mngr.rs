use std::collections::VecDeque;
use crate::IR::Block;
use super::Pass;

/// The manager of all passes (PassManager)
pub struct PassManager {
    pub(crate) passes: VecDeque<Box<dyn Pass>>,
}

impl PassManager {
    /// Creates an new pass manager
    pub fn new() -> Self {
        Self {
            passes: VecDeque::new(),
        }
    }

    /// Adds a new pass to the back of the pass queue
    pub fn add(&mut self, pass: Box<dyn Pass>)  {
        self.passes.push_back( pass );
    }

    /// Adds a new pass to the front of the pass queue
    pub fn addFront(&mut self, pass: Box<dyn Pass>)  {
        self.passes.push_front( pass );
    }

    /// Executes the entire pass queue
    pub fn run(&self, block: &mut Block) {
        for pass in &self.passes {
            pass.run(block);
        }
    }
}