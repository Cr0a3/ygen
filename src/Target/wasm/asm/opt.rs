use crate::Optimizations::Optimize;

use super::WasmMCInstr;

impl Optimize<WasmMCInstr> for Vec<WasmMCInstr> {
    fn optimize(&mut self) -> Self {
        // TODO: implement actuall optimizations
        self.to_owned()
    }
}