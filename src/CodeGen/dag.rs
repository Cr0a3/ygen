use crate::{prelude::IROperand, IR::{BlockId, Const, Type, TypeMetadata, Var}};

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
    Mul,
    And,
    Or,
    Xor,
    Shr,
    Shl,
    Div,
    Rem,
    Neg,

    // Branches
    Br(String),

    // Intrisnics
    GetFramePtr,
    GetStackPtr,

    CmpEq,
    CmpNe,
    CmpLt,
    CmpGt,
    CmpLte,
    CmpGte,

    VecInsrt,

    BrIfEq(String),

    // Cast
    I8ToI16,
    I8ToI32,
    I8ToI64,
    I8ToF32,
    I8ToF64,

    I16ToI8,
    I16ToI32,
    I16ToI64,
    I16ToF32,
    I16ToF64,

    I32ToI8,
    I32ToI16,
    I32ToI64,
    I32ToF32,
    I32ToF64,

    I64ToI8,
    I64ToI16,
    I64ToI32,
    I64ToF32,
    I64ToF64,

    F32ToI8,
    F32ToI16,
    F32ToI32,
    F32ToI64,
    F32ToF64,

    F64ToI8,
    F64ToI16,
    F64ToI32,
    F64ToI64,
    F64ToF32,

    Call(String),
}

/// A operand in the dag
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DagOp {
    /// If it is an allocated operand
    pub allocated: bool,
    /// the actual location
    pub target: DagOpTarget,
    /// What to do with the operand
    pub operation: DagOperandOption,
    /// Should the operand be a memory location
    pub should_be_mem: bool,
    /// The type of the operand
    pub ty: TypeMetadata,
}

/// What to do with the operand target
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DagOperandOption {
    /// value gets loaded
    Load,
    /// a constant imm
    ConstantImm,
    /// a constant float
    ConstantFp,
    /// move the adress
    AdrMove(String),
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
    /// Creates a new dag node
    pub fn new(opcode: DagOpCode, ty: TypeMetadata) -> Self {
        Self {
            opcode: opcode,
            out: None,
            ops: Vec::new(),
            ty: ty,
        }
    }

    /// Creates a new dag node with operands
    pub fn new_with_ops(opcode: DagOpCode, ops: Vec<DagOp>, ty: TypeMetadata) -> Self {
        Self {
            opcode: opcode,
            out: None,
            ops: ops,
            ty: ty,
        }
    }

    /// Creates a new dag node with a output
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

    /// Creates a new mul dag node 
    #[inline]
    pub fn mul(ls: DagOp, rs: DagOp, out: DagOp, ty:TypeMetadata) -> Self { DagNode::new_with_out(DagOpCode::Mul, out, vec![ls, rs], ty) }

    /// Creates a new and dag node 
    #[inline]
    pub fn and(ls: DagOp, rs: DagOp, out: DagOp, ty:TypeMetadata) -> Self { DagNode::new_with_out(DagOpCode::And, out, vec![ls, rs], ty) }

    /// Creates a new or dag node 
    #[inline]
    pub fn or(ls: DagOp, rs: DagOp, out: DagOp, ty:TypeMetadata) -> Self { DagNode::new_with_out(DagOpCode::Or, out, vec![ls, rs], ty) }

    /// Creates a new xor dag node 
    #[inline]
    pub fn xor(ls: DagOp, rs: DagOp, out: DagOp, ty:TypeMetadata) -> Self { DagNode::new_with_out(DagOpCode::Xor, out, vec![ls, rs], ty) }

    /// Creates a new shr dag node 
    #[inline]
    pub fn shr(ls: DagOp, rs: DagOp, out: DagOp, ty:TypeMetadata) -> Self { DagNode::new_with_out(DagOpCode::Shr, out, vec![ls, rs], ty) }

    /// Creates a new shl dag node 
    #[inline]
    pub fn shl(ls: DagOp, rs: DagOp, out: DagOp, ty:TypeMetadata) -> Self { DagNode::new_with_out(DagOpCode::Shl, out, vec![ls, rs], ty) }

    /// Creates a new neg dag node 
    #[inline]
    pub fn neg(ls: DagOp, out: DagOp, ty:TypeMetadata) -> Self { DagNode::new_with_out(DagOpCode::Neg, out, vec![ls], ty) }

    /// Creates a new div dag node
    #[inline]
    pub fn div(ls: DagOp, rs: DagOp, out: DagOp, ty:TypeMetadata) -> Self { DagNode::new_with_out(DagOpCode::Div, out, vec![ls, rs], ty) }

    /// Creates a new rem dag node
    #[inline]
    pub fn rem(ls: DagOp, rs: DagOp, out: DagOp, ty:TypeMetadata) -> Self { DagNode::new_with_out(DagOpCode::Rem, out, vec![ls, rs], ty) }

    /// Creates a new call dag node
    #[inline]
    pub fn call(target: String, operands: Vec<DagOp>, out: DagOp, out_ty:TypeMetadata) -> Self { DagNode::new_with_out(DagOpCode::Call(target), out, operands, out_ty) }
}

impl DagOp {
    /// Creates the dag operand as a variable
    #[inline]
    pub fn var(var: Var) -> Self {
        Self {
            should_be_mem: var.ty == TypeMetadata::ptr,
            ty: var.ty,
            allocated: false,
            target: DagOpTarget::UnallocatedVar(var),
            operation: DagOperandOption::Load,
        }
    }

    /// Creates the dag operand as a register
    #[inline]
    pub fn reg(reg: Reg) -> Self {
        Self { 
            allocated: true, 
            target: DagOpTarget::Reg(reg),
            operation: DagOperandOption::Load,
            should_be_mem: false,
            ty: if reg.is_gr() { TypeMetadata::i64 } else { TypeMetadata::f64 }
        }
    }

    /// Creates the dag operand as a memory position
    #[inline]
    pub fn mem(mem: Memory) -> Self {
        Self { 
            allocated: true, 
            target: DagOpTarget::Mem(mem),
            operation: DagOperandOption::Load,
            should_be_mem: true,
            ty: TypeMetadata::ptr,
        }
    }


    /// Creates the dag operand as a imm
    #[inline]
    pub fn imm(imm: Type) -> Self {
        Self { 
            allocated: true, 
            target: DagOpTarget::Constant(imm),
            operation: DagOperandOption::ConstantImm,
            should_be_mem: false,
            ty: imm.into(),
        }
    }

    /// Returns the operation of the dag operand
    #[inline]
    pub fn get_operation(&self) -> DagOperandOption {
        self.operation.clone()
    }

    /// Returns if the operation is load
    #[inline]
    pub fn is_operation_load(&self) -> bool {
        self.get_operation() == DagOperandOption::Load
    }

    /// Returns if the operation is a constant imm
    #[inline]
    pub fn is_operation_cimm(&self) -> bool {
        self.get_operation() == DagOperandOption::ConstantImm
    }

    /// Returns if the operation is a constant fp
    #[inline]
    pub fn is_operation_cfp(&self) -> bool {
        self.get_operation() == DagOperandOption::ConstantFp
    }

    /// Returns if the operation is an adress move
    #[inline]
    pub fn is_operation_adrm(&self) -> bool {
        matches!(self.get_operation(), DagOperandOption::AdrMove(_))
    }

    /// Returns the adress move target
    pub fn get_adrm_target(&self) -> Option<String> {
        if !self.is_operation_adrm() { return None; }

        let DagOperandOption::AdrMove(adr) = self.get_operation() else { unreachable!(); };

        Some(adr)
    }
}

impl Into<DagOp> for Type {
    fn into(self) -> DagOp {
        DagOp { 
            allocated: true, 
            target: DagOpTarget::Constant(self),
            operation: {
                let ty: TypeMetadata = self.into();
                if ty.float() { DagOperandOption::ConstantFp }
                else { DagOperandOption::ConstantImm }
            },
            should_be_mem: false,
            ty: self.into(),
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
    /// The size of the temporary
    pub size: TypeMetadata,
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
            size: ty,
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
                operation: {
                    if let IROperand::Var(_) = value { DagOperandOption::Load }
                    else if let IROperand::Type(_) = value {
                        let ty: TypeMetadata = value.get_ty();
                        if ty.float() { DagOperandOption::ConstantFp }
                        else { DagOperandOption::ConstantImm }
                    } else { unreachable!() }
                },
                should_be_mem: false,
                ty: ty.into(),
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
                operation: {
                    if let IROperand::Var(_) = value { DagOperandOption::Load }
                    else if let IROperand::Type(_) = value {
                        let ty: TypeMetadata = value.get_ty();
                        if ty.float() { DagOperandOption::ConstantFp }
                        else { DagOperandOption::ConstantImm }
                    } else { unreachable!() }
                },
                should_be_mem: false,
                ty: (*ty).into(),
            },
            IROperand::Var(var) => DagOp::var(var.to_owned()),
        }
    }
}

static mut OP_CONSTS: usize = 0;

/// Operation compilation has two possible states which are returned by a function using a boolean
/// 1. Operation is just a instruction operand
/// 2. Operation inserts instructions
/// 3. Operation requires new constant (e.g: x86 constant fps)
pub trait OperationHandler {
    /// The type of the operand
    type Operand;
    /// The type of the instruction
    type Instr;

    /// Returns if the operation can just be representet as a operand
    fn just_op(&self, op: &DagOp) -> bool;

    /// Returns if the operation inserts instructions
    fn inserts_instrs(&self, op: &DagOp) -> bool;

    /// Returns if the operation requires a new constant (e.g: x86 constant fps)
    fn requires_new_const(&self, op: &DagOp) -> bool;

    /// Compiles the operation to a operand
    fn compile_op(&self, op: &DagOp, _constant: Option<&crate::IR::Const>) -> Option<Self::Operand>;

    /// Creates a constant 
    fn create_const(&self, module: &mut crate::IR::Module) -> Const {
        let num = unsafe { let t = OP_CONSTS; OP_CONSTS += 1; t };

        module.addConst(&format!("bc{num}")).to_owned()
    }

    /// Returns the temporary in a vector (either 0 or 1 element - for auto generation
    /// porupises)
    fn tmp(&self, op: &DagOp, num: usize) -> Vec<DagTmpInfo>;

    /// Compiles the operation to a vec of instrs
    fn compile_instrs(&self, op: &DagOp, _constant: Option<&crate::IR::Const>, tmp: DagTmpInfo) -> Option<Vec<Self::Instr>>;
}