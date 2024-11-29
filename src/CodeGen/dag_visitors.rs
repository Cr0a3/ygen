use crate::Target::Arch;
use crate::IR::instrincs::{DefinedIntrinsic, Intrinsic};
use crate::IR::{Const, TypeMetadata};
use crate::IR::{ir::*, Type, Var};
use super::reg::Reg;
use super::DagVisitor;
use super::dag::{self, DagNode, DagOp};

pub(crate) static mut DAG_ARCH: Arch = Arch::Unknown;

fn dag_arch() -> Arch {
    unsafe { DAG_ARCH }
}

impl DagVisitor for Alloca {
    fn dag_visitor(&self, _dag: &mut Vec<dag::DagNode>) {
        // Alloca only influences the register allocation and 
        // operand selection for variables
    }
}

impl DagVisitor for Br {
    fn dag_visitor(&self, dag: &mut Vec<dag::DagNode>) {
        dag.push( DagNode::new(
            dag::DagOpCode::Br(self.inner1.name.to_owned()),
            TypeMetadata::ptr, // doesn't matter
        ));
    }
}

impl DagVisitor for Assign<Var, Var> {
    fn dag_visitor(&self, dag: &mut Vec<dag::DagNode>) {
        dag.push(dag::DagNode::copy(
            DagOp::var(self.inner2.to_owned()), 
            DagOp::var(self.inner1.to_owned()), 
            self.ty().unwrap()
        ))
    }
}

impl DagVisitor for Assign<Var, Type> {
    fn dag_visitor(&self, dag: &mut Vec<dag::DagNode>) {
        dag.push(dag::DagNode::copy(
            DagOp::imm(self.inner2), 
            DagOp::var(self.inner1.to_owned()), 
            self.ty().unwrap()
        ))
    }
}

impl DagVisitor for Assign<Var, Const> {
    fn dag_visitor(&self, _dag: &mut Vec<dag::DagNode>) {
        todo!()
    }
}

impl DagVisitor for BrCond {
    fn dag_visitor(&self, dag: &mut Vec<dag::DagNode>) {
        dag.push(DagNode::new_with_ops(
            dag::DagOpCode::BrIfEq(self.inner2.name.to_owned()), 
            vec![
                DagOp::var(self.inner1.clone()),
                DagOp::imm(Type::i32(1)),
            ],
            TypeMetadata::ptr
        ));
        dag.push(DagNode::new(dag::DagOpCode::Br(self.inner3.name.to_owned()), TypeMetadata::ptr));
    }
}

impl DagVisitor for Call {
    fn dag_visitor(&self, dag: &mut Vec<dag::DagNode>) {
        if let Some(intrinsic) = &self.instric {
            intrinsic.dag_visitor(&self.out, dag);
            return;
        }

        todo!()
    }
}

impl DagVisitor for Cast {
    fn dag_visitor(&self, _dag: &mut Vec<dag::DagNode>) {
        todo!()
    }
}

impl DagVisitor for Cmp {
    fn dag_visitor(&self, dag: &mut Vec<dag::DagNode>) {
        let opcode = match self.getCmpMode() {
            CmpMode::Eqal => dag::DagOpCode::CmpEq,
            CmpMode::NotEqal => dag::DagOpCode::CmpNe,
            CmpMode::GreaterThan => dag::DagOpCode::CmpGt,
            CmpMode::LessThan => dag::DagOpCode::CmpLt,
            CmpMode::GreaterThanOrEqual => dag::DagOpCode::CmpGte,
            CmpMode::LessThanOrEqual => dag::DagOpCode::CmpLte,
        };

        dag.push( DagNode::new_with_out(
            opcode, 
            DagOp::var(self.getOutput()), 
            vec![
                DagOp::from(&self.ls), 
                DagOp::from(&self.rs)
            ],
            self.getType()
        ));
    }
}

impl DagVisitor for DebugNode {
    fn dag_visitor(&self, _dag: &mut Vec<dag::DagNode>) {
        todo!()
    }
}

impl DagVisitor for GetElemPtr {
    fn dag_visitor(&self, _dag: &mut Vec<dag::DagNode>) {
        todo!()
    }
}

impl DagVisitor for Load {
    fn dag_visitor(&self, dag: &mut Vec<dag::DagNode>) {
        // We just do loading using a copy dag node
        // - copys the value of the ptr to the out var
        dag.push(DagNode::copy(
            DagOp::from(self.inner3.clone()), 
            DagOp::var(self.inner1.clone()), 
            self.inner2
        ));
    }
}

impl DagVisitor for Add {
    fn dag_visitor(&self, dag: &mut Vec<dag::DagNode>) {
        dag.push(DagNode::add(
            DagOp::from(&self.inner1), 
            DagOp::from(&self.inner2), 
            DagOp::var(self.inner3.to_owned()),
            self.inner3.ty,
        ));
    }
}

impl DagVisitor for Sub {
    fn dag_visitor(&self, dag: &mut Vec<dag::DagNode>) {
        dag.push(DagNode::sub(
            DagOp::from(&self.inner1), 
            DagOp::from(&self.inner2), 
            DagOp::var(self.inner3.to_owned()),
            self.inner3.ty,
        ));
    }
}

impl DagVisitor for Mul {
    fn dag_visitor(&self, dag: &mut Vec<dag::DagNode>) {
        dag.push(DagNode::mul(
            DagOp::from(&self.inner1), 
            DagOp::from(&self.inner2), 
            DagOp::var(self.inner3.to_owned()),
            self.inner3.ty,
        ));
    }
}

impl DagVisitor for Div {
    fn dag_visitor(&self, _dag: &mut Vec<dag::DagNode>) {
        todo!()
    }
}

impl DagVisitor for Rem {
    fn dag_visitor(&self, _dag: &mut Vec<dag::DagNode>) {
        todo!()
    }
}

impl DagVisitor for And {
    fn dag_visitor(&self, dag: &mut Vec<dag::DagNode>) {
        dag.push(DagNode::and(
            DagOp::from(&self.inner1), 
            DagOp::from(&self.inner2), 
            DagOp::var(self.inner3.to_owned()),
            self.inner3.ty,
        ));
    }
}

impl DagVisitor for Or {
    fn dag_visitor(&self, dag: &mut Vec<dag::DagNode>) {
        dag.push(DagNode::or(
            DagOp::from(&self.inner1), 
            DagOp::from(&self.inner2), 
            DagOp::var(self.inner3.to_owned()),
            self.inner3.ty,
        ));
    }
}

impl DagVisitor for Xor {
    fn dag_visitor(&self, dag: &mut Vec<dag::DagNode>) {
        dag.push(DagNode::xor(
            DagOp::from(&self.inner1), 
            DagOp::from(&self.inner2), 
            DagOp::var(self.inner3.to_owned()),
            self.inner3.ty,
        ));
    }
}

impl DagVisitor for Shl {
    fn dag_visitor(&self, dag: &mut Vec<dag::DagNode>) {
        dag.push(DagNode::shl(
            DagOp::from(&self.inner1), 
            DagOp::from(&self.inner2), 
            DagOp::var(self.inner3.to_owned()),
            self.inner3.ty,
        ));
    }
}

impl DagVisitor for Shr {
    fn dag_visitor(&self, dag: &mut Vec<dag::DagNode>) {
        dag.push(DagNode::shr(
            DagOp::from(&self.inner1), 
            DagOp::from(&self.inner2), 
            DagOp::var(self.inner3.to_owned()),
            self.inner3.ty,
        ));
    }
}

impl DagVisitor for Neg {
    fn dag_visitor(&self, dag: &mut Vec<dag::DagNode>) {
        dag.push(DagNode::neg(
            DagOp::from(&self.inner1), 
            DagOp::var(self.inner2.to_owned()), 
            self.inner1.get_ty()    
        ));
    }
}

impl DagVisitor for Phi {
    fn dag_visitor(&self, _dag: &mut Vec<dag::DagNode>) {
        todo!()
    }
}

impl DagVisitor for Return {
    fn dag_visitor(&self, dag: &mut Vec<dag::DagNode>) {
        if self.inner1.get_ty() == TypeMetadata::Void {
            dag.push( DagNode::ret(TypeMetadata::Void) );
            return;
        }

        let ret_reg = Reg::ret(dag_arch(), self.inner1.get_ty()); 

        if self.isRetConst() {
            dag.push(DagNode::copy(
                self.getRetConst().into(), 
                DagOp::reg(ret_reg),
                self.inner1.get_ty(),
            ));
        } else {
            dag.push(DagNode::copy(
                DagOp::var(self.getRetVar()),
                DagOp::reg(ret_reg),
                self.inner1.get_ty(),
            ));
        };

        dag.push( DagNode::ret(self.inner1.get_ty()) );
    }
}

impl DagVisitor for Select {
    fn dag_visitor(&self, _dag: &mut Vec<dag::DagNode>) {
        todo!()
    }
}

impl DagVisitor for Store {
    fn dag_visitor(&self, dag: &mut Vec<dag::DagNode>) {
        // a store is also just basiclly a copy operation
        // we copy the value -> to the pointer
        let mut ptr = DagOp::var(self.inner1.to_owned());
        ptr.should_be_mem = true;

        dag.push(DagNode::copy(
            DagOp::from(self.inner2.to_owned()), 
            ptr,
            self.inner2.get_ty()
        ));
    }
}

impl DagVisitor for Switch {
    fn dag_visitor(&self, _dag: &mut Vec<dag::DagNode>) {
        todo!()
    }
}

impl DagVisitor for VecInsert {
    fn dag_visitor(&self, dag: &mut Vec<dag::DagNode>) {
        dag.push(DagNode::new_with_out(
            dag::DagOpCode::VecInsrt,
            DagOp::var(self.out.to_owned()), 
            vec![
                DagOp::var(self.vec.to_owned()),
                DagOp::from(self.elem.to_owned()),
                DagOp::imm(Type::i64(self.position as i64)),
            ], 
            self.vec.ty
        ));
    }
}

impl Intrinsic {
    fn dag_visitor(&self, out: &Var, dag: &mut Vec<dag::DagNode>) {
        match self.instrinc {
            DefinedIntrinsic::GetStackPtr => super::dag_intrinsic::lower_get_stack_ptr(self, out, dag),
            DefinedIntrinsic::GetFramePtr => super::dag_intrinsic::lower_get_frame_ptr(self, out, dag),
        }
    }
}