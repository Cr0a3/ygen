use std::collections::HashMap;

use crate::IR::{BlockId, Type, Var};

use super::reg::Reg;

/// A dag function is just a wrapper around a hashmap for the blocks 
/// and its dag nodes
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DagFunction {
    /// A block with its dag nodes
    pub blocks: HashMap<BlockId, Vec<DagNode>>
}

/// A dag node
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DagNode {
    /// The dag opcode
    pub opcode: DagOpCode,
    /// the output
    pub out: Option<DagOp>,
}

/// A dag opcode
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum DagOpCode {
    CopyToReg(DagOp),
    Ret,
}

/// A operand in the dag
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DagOp {
    /// If it is an allocated operand
    pub allocated: bool,
    /// the actual location
    pub target: DagOpTarget,
}

/// A target for an operand
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DagOpTarget {
    Reg(Reg),
    UnallocatedVar(Var),
    Constant(Type),
}

impl DagNode {
    /// Creates an new dag node
    pub fn new(opcode: DagOpCode) -> Self {
        Self {
            opcode: opcode,
            out: None,
        }
    }
    /// Creates an new dag node with an output
    pub fn new_with_out(opcode: DagOpCode, out: DagOp) -> Self {
        Self {
            opcode: opcode,
            out: Some(out),
        }
    }

    /// Creates a new ret dag node
    #[inline]
    pub fn ret() -> Self { DagNode::new(DagOpCode::Ret) }

    /// Creates a new copy to reg dag node
    #[inline]
    pub fn copy_to_reg(from: DagOp, to: DagOp) -> Self { DagNode::new(DagOpCode::Ret) }
}

impl DagOp {
    /// Creates the dag operand as a variable
    #[inline]
    pub fn var(var: Var) -> Self {
        Self {
            allocated: false,
            target: DagOpTarget::UnallocatedVar(var),
        }
    }

    /// Creates the dag operand as a variable
    #[inline]
    pub fn reg(reg: Reg) -> Self {
        Self { 
            allocated: true, 
            target: DagOpTarget::Reg(reg) 
        }
    }
}

impl Into<DagOp> for Type {
    fn into(self) -> DagOp {
        DagOp { 
            allocated: true, 
            target: DagOpTarget::Constant(self) 
        }
    }
}