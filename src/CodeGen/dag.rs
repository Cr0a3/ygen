use crate::{prelude::IROperand, IR::{BlockId, Type, TypeMetadata, Var}};

use super::{memory::Memory, reg::Reg};

/// A dag function is just a wrapper around a vec for the blocks 
/// and its dag nodes
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DagFunction {
    /// A block with its dag nodes
    pub blocks: Vec<(BlockId, Vec<DagNode>)>
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
    /// the type of the dag node
    pub ty: TypeMetadata,
}

/// A dag opcode
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum DagOpCode {
    Copy,
    Ret,

    // Math
    Add,
    Sub,

    // Branches
    Br(String),

    // Intrisnics
    GetFramePtr,
    GetStackPtr,
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
    pub fn new(opcode: DagOpCode, ty: TypeMetadata) -> Self {
        Self {
            opcode: opcode,
            out: None,
            ops: Vec::new(),
            ty: ty,
        }
    }
    /// Creates an new dag node with an output
    pub fn new_with_out(opcode: DagOpCode, out: DagOp, ops: Vec<DagOp>, ty: TypeMetadata) -> Self {
        Self {
            opcode: opcode,
            out: Some(out),
            ops: ops,            
            ty: ty,
        }
    }

    /// Creates a new ret dag node
    #[inline]
    pub fn ret(ty: TypeMetadata) -> Self { DagNode::new(DagOpCode::Ret, ty) }

    /// Creates a new copy to reg dag node
    #[inline]
    pub fn copy(from: DagOp, to: DagOp, ty: TypeMetadata) -> Self { DagNode::new_with_out(DagOpCode::Copy, to, vec![from], ty) }
    
    /// Creates a new add dag node 
    #[inline]
    pub fn add(ls: DagOp, rs: DagOp, out: DagOp, ty:TypeMetadata) -> Self { DagNode::new_with_out(DagOpCode::Add, out, vec![ls, rs], ty) }
    
    /// Creates a new sub dag node 
    #[inline]
    pub fn sub(ls: DagOp, rs: DagOp, out: DagOp, ty:TypeMetadata) -> Self { DagNode::new_with_out(DagOpCode::Sub, out, vec![ls, rs], ty) }

}

impl DagOp {
    /// Creates the dag operand as a variable
    #[inline]
    pub fn var(var: Var) -> Self {
        Self {
            allocated: false,
            target: DagOpTarget::UnallocatedVar(var)
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
    /// Returns if the nth operand is a fp register
    pub fn is_op_fp(&self, op: usize) -> bool {
        if let Some(op) = self.ops.get(op) {
            if let DagOpTarget::Reg(reg) = op.target {
                return reg.is_fp();
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

    /// Returns if the output is a fp register
    pub fn is_out_fp(&self) -> bool {
        if let Some(out) = &self.out {
            if let DagOpTarget::Reg(reg) = out.target {
                return reg.is_fp();
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

    /// Is the nodes type the given type
    pub fn is_ty(&self, ty: TypeMetadata) -> bool {
        self.ty == ty
    }

    /// Returns the type metadata of the node
    pub fn get_ty(&self) -> TypeMetadata {
        self.ty
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
    /// Does the tmporary require a fp reg
    pub requires_fp: bool,
    /// Does the location even matter?
    pub shit_on_loc: bool,
    /// The type of the temporary
    pub ty: TypeMetadata,
}

impl DagTmpInfo {
    /// Creates a new dag temporary
    pub fn new(num: usize, ty: TypeMetadata) -> Self {
        Self {
            tmp_num: num,
            requires_gr: false,
            requires_fp: false,
            requires_mem: false,
            shit_on_loc: false,
            ty: ty,
        }
    }

    /// The temporary requires a gr register
    pub fn require_gr(&mut self) {
        self.requires_gr = true;
    }

    /// The temporary requires a fp register
    pub fn require_fp(&mut self) {
        self.requires_fp = true;
    }

    /// The temporary requires a memory displacment
    pub fn require_mem(&mut self) {
        self.requires_mem = true;
    }
}

impl From<IROperand> for DagOp {
    fn from(value: IROperand) -> Self {
        match value {
            IROperand::Type(ty) => DagOp {
                allocated: true,
                target: DagOpTarget::Constant(ty),
            },
            IROperand::Var(var) => DagOp::var(var),
        }
    }
}

impl From<&IROperand> for DagOp {
    fn from(value: &IROperand) -> Self {
        match value {
            IROperand::Type(ty) => DagOp {
                allocated: true,
                target: DagOpTarget::Constant(*ty),
            },
            IROperand::Var(var) => DagOp::var(var.to_owned()),
        }
    }
}