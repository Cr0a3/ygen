use std::collections::VecDeque;
use crate::IR::Builder;
use super::Pass;

/// The manager of all passes (PassManager)
pub struct PassManager<T> {
    passes: VecDeque<T>,
}

impl<T> PassManager<T> where T: Pass + Clone {
    /// Creates an new pass manager
    pub fn new() -> Self {
        Self {
            passes: VecDeque::new(),
        }
    }

    /// Adds a new pass to the back of the pass queue
    pub fn add(&mut self, pass: T)  {
        self.passes.push_back( pass.clone() );
    }

    /// Adds a new pass to the front of the pass queue
    pub fn add_front(&mut self, pass: T)  {
        self.passes.push_front( pass.clone() );
    }

    /// Executes the entire pass queue
    pub fn run(&self, builder: &mut Builder) {
        for pass in &self.passes {
            pass.run(builder);
        }
    }
}