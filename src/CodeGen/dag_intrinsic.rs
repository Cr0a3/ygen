use crate::IR::{instrincs::Intrinsic, Var};

use super::dag::{self, DagNode};

/// Loweres the `get_frame` intrinsic
pub fn lower_get_frame_ptr(_: &Intrinsic, out: &Var, dag: &mut Vec<dag::DagNode>) {
    let out = out.to_owned();
    dag.push(DagNode::new_with_out(dag::DagOpCode::GetFramePtr, dag::DagOp::var(out), Vec::new()));
}

/// Loweres the `get_stack` intrinsic
pub fn lower_get_stack_ptr(_: &Intrinsic, out: &Var, dag: &mut Vec<dag::DagNode>) {
    let out = out.to_owned();
    dag.push(DagNode::new_with_out(dag::DagOpCode::GetStackPtr, dag::DagOp::var(out), Vec::new()));
}