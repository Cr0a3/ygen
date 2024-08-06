use super::instr::Instr;

/// used for optimizing
pub trait Optimize<T> {
    /// optimizes self
    fn optimize(&mut self) -> Self;
}

impl Optimize<Instr> for Vec<Instr> {
    fn optimize(&mut self) -> Vec<Instr> {
        let mut out = vec![];

        for instr in self.iter() {
            if let Some(last) = out.last() {
                if instr.invert_of(last) {
                    out.pop();
                } else { out.push(instr.to_owned()) }
            } else { out.push(instr.to_owned()) }
        }

        out
    }
}