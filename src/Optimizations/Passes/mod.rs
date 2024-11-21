#[allow(hidden_glob_reexports)]
mod ConstantEvaluation;
#[allow(hidden_glob_reexports)]
mod DeadNodeElimination;
#[allow(hidden_glob_reexports)]
mod DeadBlockElimination;
#[allow(hidden_glob_reexports)]
mod InstrCombine;
#[allow(hidden_glob_reexports)]
mod UnusedCallRemovement;

pub use ConstantEvaluation::*;
pub use DeadNodeElimination::*;
pub use DeadBlockElimination::*;
pub use InstrCombine::*;
pub use UnusedCallRemovement::*;