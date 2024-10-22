use crate::CodeGen::{MachineInstr, MachineMnemonic, MachineOperand};
use crate::Target::CallConv;

mod adr;
mod br;
mod call;
mod cmp;
mod downcast;
mod mov;
mod math;
mod prolog;
mod push;
mod ret;
mod stack;
mod switch;
mod zext;
mod cmov;

mod fmath;
mod fcmp;
mod fmove;
mod fcast;

use super::reg_alloc::*;


pub(crate) fn x64_lower_instr(conv: CallConv, sink: &mut Vec<X64RegAllocInstr>, instr: MachineInstr) {
    match &instr.mnemonic {
        MachineMnemonic::Move =>                                         mov::x64_lower_move(sink, &instr),
        MachineMnemonic::Add =>                                          math::x64_lower_add(sink, &instr),
        MachineMnemonic::And =>                                          math::x64_lower_and(sink, &instr),
        MachineMnemonic::Div =>                                          math::x64_lower_div(sink, &instr),
        MachineMnemonic::Rem =>                                          math::x64_lower_rem(sink, &instr),
        MachineMnemonic::Mul =>                                          math::x64_lower_mul(sink, &instr),
        MachineMnemonic::Or =>                                           math::x64_lower_or(sink, &instr),
        MachineMnemonic::Sub =>                                          math::x64_lower_sub(sink, &instr),
        MachineMnemonic::Xor =>                                          math::x64_lower_xor(sink, &instr),
        MachineMnemonic::Shl =>                                          math::x64_lower_shl(sink, &instr),
        MachineMnemonic::Shr =>                                          math::x64_lower_shr(sink, &instr),
        MachineMnemonic::Zext(start) =>                   zext::x64_lower_zext(sink, &instr, start),
        MachineMnemonic::Downcast(start) =>               downcast::x64_lower_downcast(sink, &instr, start),
        MachineMnemonic::Call(to) =>                            call::x64_lower_call(conv, sink, &instr, to),
        MachineMnemonic::Return =>                                       ret::x64_lower_return(sink, &instr),
        MachineMnemonic::AdressLoad(to) =>                      adr::x64_lower_adr_load(sink, &instr, to),
        MachineMnemonic::Br(to) =>                              br::x64_lower_br(sink, &instr, to),
        MachineMnemonic::BrCond(iftrue, iffalse) =>    br::x64_lower_cond_br(sink, &instr, iftrue, iffalse),
        MachineMnemonic::Compare(mode) =>                      cmp::x64_lower_cmp(sink, &instr, mode),
        MachineMnemonic::Prolog =>                                       prolog::x64_lower_prolog(sink, &instr),
        MachineMnemonic::Epilog =>                                       prolog::x64_lower_epilog(sink, &instr),
        MachineMnemonic::StackAlloc =>                                   stack::x64_lower_salloc(sink, &instr),
        MachineMnemonic::Store =>                                        stack::x64_lower_store(sink, &instr),
        MachineMnemonic::Load =>                                         stack::x64_lower_load(sink, &instr),
        MachineMnemonic::AdrMove =>                                      adr::x64_lower_adrm(sink, &instr),
        MachineMnemonic::Switch(cases) =>         switch::x64_lower_switch(sink, &instr, cases),
        MachineMnemonic::Neg =>                                          math::x64_lower_neg(sink, &instr),
        MachineMnemonic::MovIfZero =>                                    cmov::x64_lower_cmov_zero(sink, &instr),
        MachineMnemonic::MovIfNotZero =>                                 cmov::x64_lower_cmov_not_zero(sink, &instr),
        MachineMnemonic::FMove =>                                        fmove::x64_lower_fmove(sink, &instr),
        MachineMnemonic::FAdd =>                                         fmath::x64_lower_fadd(sink, &instr),
        MachineMnemonic::FDiv =>                                         fmath::x64_lower_fdiv(sink, &instr),
        MachineMnemonic::FMul =>                                         fmath::x64_lower_fmul(sink, &instr),
        MachineMnemonic::FSub =>                                         fmath::x64_lower_fsub(sink, &instr),
        MachineMnemonic::FCompare(mode) =>                     fcmp::x64_lower_fcmp(sink, &instr, mode),
        MachineMnemonic::FCast(input_type) =>             fcast::X64_lower_fcast(sink, &instr, *input_type),
        MachineMnemonic::ArgMove(index) =>                         mov::X64_lower_arg_move(sink, &instr, *index),
        _ => todo!("TDOD: {}", instr.mnemonic),
    }
}

/// The function used for lowering general `MachineInstr` into `MCInstr`
pub(crate) fn x64_lower(conv: CallConv, instrs: Vec<MachineInstr>) -> Vec<X64RegAllocInstr> {
    let mut out = Vec::new();

    for instr in instrs {
        x64_lower_instr(conv, &mut out, instr.clone());
    }

    let mut mc_instrs = vec![];

    for instr in out {
        mc_instrs.push( instr.into() );
    }

    mc_instrs
}

impl From<MachineOperand> for RegAllocOperand {
    fn from(value: MachineOperand) -> Self {
        match value {
            MachineOperand::Imm(imm) => RegAllocOperand::Imm(imm as i64),
            MachineOperand::Reg(vreg) => RegAllocOperand::Reg(vreg),
        }
    }
}