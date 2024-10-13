use super::CompilationHelper;
use crate::{prelude::*, CodeGen::*};

impl CompilationHelper {
    #[allow(missing_docs)]
    pub fn compile_getelemptr(&mut self, node: &GetElemPtr, mc_sink: &mut Vec<MachineInstr>, _: &Block, _: &mut crate::prelude::Module) {
        let vec = self.vars.get(&node.ptr.name).unwrap().into();
        let out = self.vars.get(&node.out.name).unwrap().into();
        let index = self.vars.get(&node.index.name).unwrap().into();

        let mut indexcalc_instr = MachineInstr::new( MachineMnemonic::Mul );
        indexcalc_instr.set_out(out);
        indexcalc_instr.add_operand(index);
        indexcalc_instr.add_operand(MachineOperand::Imm(node.ty.byteSize() as f64));

        indexcalc_instr.meta = TypeMetadata::ptr;

        mc_sink.push(indexcalc_instr);

        let mut add_instr = MachineInstr::new( MachineMnemonic::Add );
        add_instr.set_out(out);
        add_instr.add_operand(vec);
        add_instr.add_operand(out);

        add_instr.meta = TypeMetadata::ptr;

        mc_sink.push( add_instr );

        let mut mov_instr = MachineInstr::new( MachineMnemonic::Load );
        mov_instr.set_out(out);
        mov_instr.add_operand(out);

        mov_instr.meta = node.ty;

        mc_sink.push( mov_instr );
    }
}