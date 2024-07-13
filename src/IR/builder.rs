use std::collections::VecDeque;

use super::Block;

/// IRBuilder: used for building the ir of a function
pub struct IRBuilder<'a> {
    pub(crate)  blocks: VecDeque<&'a mut Block>,
    /// The current block as an index in the blocks list
    pub(crate)  curr: usize,
}

impl<'a> IRBuilder<'a> {
    /// Creates an new ir builder
    pub fn new() -> Self {
        Self {
            blocks: VecDeque::new(),
            curr: 0,
        }
    }

    /// Positions the block at the end of the blocks list
    pub fn positionAtEnd(&mut self, block: &'a mut Block) {
        self.blocks.push_back(block);
        self.curr = self.blocks.len() - 1; // Can cause an intenger underflow but shouldn't
    }

    /// Positions the block at the start of the blocks list
    pub fn positionAtStart(&mut self, block: &'a mut Block) {
        self.blocks.push_front(block);
        self.curr = 0; // Can cause an intenger underflow but shouldn't
    }

    /// Returns the last block of the builder
    pub fn getLastBlock(&mut self) -> Option<&Block> {
        Some(self.blocks.back()?.to_owned().to_owned())
    }
}

/// Creates an new IRBuilder
pub fn IRBuilder<'a>() -> IRBuilder<'a> {
    IRBuilder::new()
}