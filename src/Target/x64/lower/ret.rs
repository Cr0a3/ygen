use crate::CodeGen::{MachineInstr, MachineMnemonic, MachineOperand, Reg};
//use crate::Target::x64Reg;
use crate::Target::x64::asm::instr::*;
use crate::Target::x64Reg;

use super::fmove::x64_lower_fmove;

pub(crate) fn x64_lower_return(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr) {
    let op = instr.operands.get(0).expect("return expectes operand");

    if instr.meta.float() {
        let mut instr = MachineInstr::new(MachineMnemonic::FMove);
        instr.add_operand(*op);
        instr.set_out(MachineOperand::Reg(Reg::x64(x64Reg::Xmm0)));

        x64_lower_fmove(sink, &instr);
    } else {
        sink.push( X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rax.sub_ty(instr.meta)), (*op).into()));
    }

    sink.push( X64MCInstr::with0(Mnemonic::Ret).into() );
}
