mod mngr;
mod template;
/// all passes
pub mod Passes;
/// all analysis functions
pub mod Analysis;

/// Instruction combining
pub(crate) mod InstrCombine;

pub use mngr::PassManager;
pub use template::Pass;
/// used for optimizing
pub trait Optimize<T> {
    /// optimizes self
    fn optimize(&mut self) -> Self;
}

/// Automaticlly optimizes the input till it doesn't change
pub fn auto_max_optimize<T, U>(target: &mut T) where T: Optimize<U> + PartialEq + Clone {
    let mut last = target.clone();

    let mut tmp = target.clone();

    loop {
        tmp = tmp.optimize();

        if last == tmp {
            break;
        }

        last = tmp.clone();
    }

    *target = tmp;
}