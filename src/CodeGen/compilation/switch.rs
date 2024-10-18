use crate::{prelude::Switch, CodeGen::{MachineInstr, MachineMnemonic}, IR::{Block, TypeMetadata}};

use super::CompilationHelper;

impl CompilationHelper {
    #[allow(missing_docs)]
    pub fn compile_switch(&mut self, node: &Switch, mc_sink: &mut Vec<MachineInstr>, _: &Block, _: &mut crate::prelude::Module) {
        let mut cases = Vec::new();

        for (case_type, case_node) in &node.cases {
            cases.push((*case_type, case_node.to_owned()));
        }

        let mut instr = MachineInstr::new(MachineMnemonic::Switch(
            cases.to_owned()
        ));

        instr.add_operand(
            self.vars.get(&node.to_switch.name)
            .expect("expected valid variable")
            .into()
        );

        instr.meta = if let Some((ty, _)) = &cases.get(0) {
            (*ty).into()
        } else { TypeMetadata::i32 };

        mc_sink.push(instr);

        let instr = MachineInstr::new(
            MachineMnemonic::Br(node.default.name.to_owned())
        );
        mc_sink.push(instr);
    }
}