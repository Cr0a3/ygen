use crate::Target::Arch;
use crate::IR::Const;
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
        todo!()
    }
}

impl DagVisitor for Br {
    fn dag_visitor(&self, _dag: &mut Vec<dag::DagNode>) {
        todo!()
    }
}

impl DagVisitor for Assign<Var, Var> {
    fn dag_visitor(&self, _dag: &mut Vec<dag::DagNode>) {
        todo!()
    }
}

impl DagVisitor for Assign<Var, Type> {
    fn dag_visitor(&self, _dag: &mut Vec<dag::DagNode>) {
        todo!()
    }
}

impl DagVisitor for Assign<Var, Const> {
    fn dag_visitor(&self, _dag: &mut Vec<dag::DagNode>) {
        todo!()
    }
}

impl DagVisitor for BrCond {
    fn dag_visitor(&self, _dag: &mut Vec<dag::DagNode>) {
        todo!()
    }
}

impl DagVisitor for Call {
    fn dag_visitor(&self, _dag: &mut Vec<dag::DagNode>) {
        todo!()
    }
}

impl DagVisitor for Cast {
    fn dag_visitor(&self, _dag: &mut Vec<dag::DagNode>) {
        todo!()
    }
}

impl DagVisitor for Cmp {
    fn dag_visitor(&self, _dag: &mut Vec<dag::DagNode>) {
        todo!()
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
    fn dag_visitor(&self, _dag: &mut Vec<dag::DagNode>) {
        todo!()
    }
}

impl DagVisitor for Add {
    fn dag_visitor(&self, dag: &mut Vec<dag::DagNode>) {
        dag.push(DagNode::add(
            DagOp::from(&self.inner2), 
            DagOp::from(&self.inner2), 
            DagOp::var(self.inner3.to_owned()))
        );
    }
}

impl DagVisitor for Sub {
    fn dag_visitor(&self, _dag: &mut Vec<dag::DagNode>) {
        todo!()
    }
}

impl DagVisitor for Mul {
    fn dag_visitor(&self, _dag: &mut Vec<dag::DagNode>) {
        todo!()
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
    fn dag_visitor(&self, _dag: &mut Vec<dag::DagNode>) {
        todo!()
    }
}

impl DagVisitor for Or {
    fn dag_visitor(&self, _dag: &mut Vec<dag::DagNode>) {
        todo!()
    }
}

impl DagVisitor for Xor {
    fn dag_visitor(&self, _dag: &mut Vec<dag::DagNode>) {
        todo!()
    }
}

impl DagVisitor for Shl {
    fn dag_visitor(&self, _dag: &mut Vec<dag::DagNode>) {
        todo!()
    }
}

impl DagVisitor for Shr {
    fn dag_visitor(&self, _dag: &mut Vec<dag::DagNode>) {
        todo!()
    }
}

impl DagVisitor for Neg {
    fn dag_visitor(&self, _dag: &mut Vec<dag::DagNode>) {
        todo!()
    }
}

impl DagVisitor for Phi {
    fn dag_visitor(&self, _dag: &mut Vec<dag::DagNode>) {
        todo!()
    }
}

impl DagVisitor for Return {
    fn dag_visitor(&self, dag: &mut Vec<dag::DagNode>) {
        let ret_reg = Reg::ret(dag_arch(), self.inner1.get_ty()); 

        if self.isRetConst() {
            dag.push(DagNode::copy(
                self.getRetConst().into(), 
                DagOp::reg(ret_reg)
            ));
        } else {
            dag.push(DagNode::copy(
                DagOp::var(self.getRetVar()),
                DagOp::reg(ret_reg) 
            ));
        };

        dag.push( DagNode::ret() );
    }
}

impl DagVisitor for Select {
    fn dag_visitor(&self, _dag: &mut Vec<dag::DagNode>) {
        todo!()
    }
}

impl DagVisitor for Store {
    fn dag_visitor(&self, _dag: &mut Vec<dag::DagNode>) {
        todo!()
    }
}

impl DagVisitor for Switch {
    fn dag_visitor(&self, _dag: &mut Vec<dag::DagNode>) {
        todo!()
    }
}