use super::{dag, regalloc_iterated_col::ItRegCoalAlloc};
use crate::Target::instr::McInstr;

/// Loweres the dag to assembly
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DagLower {
    lower_func: Option<fn(&mut dag::DagFunction, &mut ItRegCoalAlloc) -> std::collections::HashMap<crate::IR::BlockId, Vec<Box<dyn McInstr>>>>,
    tmp_info: Option<fn(&dag::DagNode) -> Vec<dag::DagTmpInfo>>,
}

impl DagLower {
    /// Creates a new dag lower
    pub fn new(
        lower: fn(&mut dag::DagFunction, &mut ItRegCoalAlloc) -> std::collections::HashMap<crate::IR::BlockId, Vec<Box<dyn McInstr>>>, 
        tmp_info: fn(&dag::DagNode) -> Vec<dag::DagTmpInfo>
    ) -> Self {
        Self {
            lower_func: Some(lower),
            tmp_info: Some(tmp_info),
        }
    }

    /// Lowers the dag to assembly
    pub fn lower(&self, func: &mut dag::DagFunction, alloc: &mut ItRegCoalAlloc) -> std::collections::HashMap<crate::IR::BlockId, Vec<Box<dyn McInstr>>> {
        if let Some(lower_func) = self.lower_func {
            lower_func(func, alloc)
        } else {
            panic!("no registered dag lowering function")
        }
    }

    pub fn required_tmps(&self, node: &dag::DagNode) -> Vec<dag::DagTmpInfo> {
        if let Some(tmp) = self.tmp_info {
            tmp(node)
        } else {
            //panic!("no registered temp information system");
            Vec::new()
        }
    }
}