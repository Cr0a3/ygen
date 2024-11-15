use std::collections::HashMap;

use crate::IR::{BlockId, Type, Var};

use super::{memory::Memory, reg::Reg};

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

    /// the operands
    pub ops: Vec<DagOp>,
}

/// A dag opcode
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum DagOpCode {
    Copy,
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
#[allow(missing_docs)]
pub enum DagOpTarget {
    Reg(Reg),
    UnallocatedVar(Var),
    Constant(Type),
    Mem(Memory),
}

impl DagNode {
    /// Creates an new dag node
    pub fn new(opcode: DagOpCode) -> Self {
        Self {
            opcode: opcode,
            out: None,
            ops: Vec::new(),
        }
    }
    /// Creates an new dag node with an output
    pub fn new_with_out(opcode: DagOpCode, out: DagOp, ops: Vec<DagOp>) -> Self {
        Self {
            opcode: opcode,
            out: Some(out),
            ops: ops,            
        }
    }

    /// Creates a new ret dag node
    #[inline]
    pub fn ret() -> Self { DagNode::new(DagOpCode::Ret) }

    /// Creates a new copy to reg dag node
    #[inline]
    pub fn copy(from: DagOp, to: DagOp) -> Self { DagNode::new_with_out(DagOpCode::Copy, to, vec![from]) }
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

impl DagNode {
    /// Returns if the nth operand is a gr register
    pub fn is_op_gr(&self, op: usize) -> bool {
        if let Some(op) = self.ops.get(op) {
            if let DagOpTarget::Reg(reg) = op.target {
                return reg.is_gr();
            } 
        }

        false
    }

    /// Returns if the nth operand is a stack variable
    pub fn is_op_mem(&self, op: usize) -> bool {
        if let Some(op) = self.ops.get(op) {
            if let DagOpTarget::Mem(_) = op.target {
                return true;
            } 
        }

        false
    }

    /// Returns if the nth operand is a const
    pub fn is_op_imm(&self, op: usize) -> bool {
        if let Some(op) = self.ops.get(op) {
            if let DagOpTarget::Constant(_) = op.target {
                return true;
            } 
        }

        false
    }

    /// Returns if the output is a gr register
    pub fn is_out_gr(&self) -> bool {
        if let Some(out) = &self.out {
            if let DagOpTarget::Reg(reg) = out.target {
                return reg.is_gr();
            } 
        }

        false
    }

    /// Returns if the output is a memory displacment
    pub fn is_out_mem(&self) -> bool {
        if let Some(out) = &self.out {
            if let DagOpTarget::Mem(_) = out.target {
                return true;
            } 
        }

        false
    }

    /// Returns the nth operand
    pub fn get_op(&self, op: usize) -> DagOp {
        self.ops.get(op).expect(&format!("the node {} does not have a {op}. operand", self)).to_owned()
    }

    /// Returns the opcode
    pub fn get_opcode(&self) -> DagOpCode {
        self.opcode.to_owned()
    }

    /// Returns the output
    pub fn get_out(&self) -> DagOp {
        self.out.as_ref().unwrap().to_owned()
    }
}

/// Returns information for the dag temporary
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DagTmpInfo {
    /// Returns the number of the temporary
    pub tmp_num: usize,
    /// Does the tmporary require a gr reg
    pub requires_gr: bool,
    /// Does the tmporary require a mem displacment
    pub requires_mem: bool,
    /// Does the location even matter?
    pub shit_on_loc: bool,
}

impl DagTmpInfo {
    /// Creates a new dag temporary
    pub fn new(num: usize) -> Self {
        Self {
            tmp_num: num,
            requires_gr: false,
            requires_mem: false,
            shit_on_loc: false,
        }
    }

    /// The temporary requires a register
    pub fn require_gr(&mut self) {
        self.requires_gr = true;
    }

    /// The temporary requires a memory displacment
    pub fn require_mem(&mut self) {
        self.requires_mem = true;
    }
}