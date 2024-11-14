/// Registers
pub mod reg;

/// Builds the dag
pub mod dag_builder;
/// Contains all dag nodes
pub mod dag;
mod dag_visitors;
/// Loweres the dag
pub mod dag_lower;

mod display;

/// A visitor for ir nodes for constructing the dag
pub trait DagVisitor {
    /// visits each ir node and builds a dag node out of it
    fn dag_visitor(&self, dag: &mut Vec<dag::DagNode>);
}