use crate::CodeGen::{MCInstr, MachineCallingConvention, MachineInstr, MachineMnemonic, MachineOperand};
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

use super::optimizer::X64AsmOpt;
use super::{instr::{Mnemonic, Operand, X64MCInstr}, X64Reg};

//pub(crate) static mut USE_SP_FOR_STACK: bool = false;
//pub(crate) static mut SP_OFF: i32 = -4;

macro_rules! x64_stack {
    ($off:expr) => {
        //unsafe {
            //if !USE_SP_FOR_STACK {
                Operand::Mem(X64Reg::Rbp - $off as u32)
            /*} else {
                Operand::Mem(X64Reg::Rsp + ($off + SP_OFF) as u32)
            }*/
        //}
    };
}

pub(crate) fn x64_lower_instr(conv: CallConv, sink: &mut Vec<X64MCInstr>, instr: MachineInstr) {
    match &instr.mnemonic {        
    MachineMnemonic::CallStackPrepare => {
        sink.push(X64MCInstr::with2(
            Mnemonic::Sub, Operand::Reg(X64Reg::Rsp), 
            Operand::Imm(
                MachineCallingConvention {
                call_conv: conv
            }.shadow(crate::Target::Arch::X86_64) - 8
        )));
    },MachineMnemonic::CallStackRedo => {
        sink.push(X64MCInstr::with2(
            Mnemonic::Add, Operand::Reg(X64Reg::Rsp), 
            Operand::Imm(
                MachineCallingConvention {
                call_conv: conv
            }.shadow(crate::Target::Arch::X86_64) - 8
        )));
    },
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
        MachineMnemonic::Zext(_) =>                                      zext::x64_lower_zext(sink, &instr),
        MachineMnemonic::Downcast(_) =>                                  downcast::x64_lower_downcast(sink, &instr),
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
        MachineMnemonic::Push =>                                         push::x64_lower_push(sink, &instr),
        MachineMnemonic::PushCleanup =>                                  push::x64_lower_push_cleanup(sink, &instr),
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
        _ => todo!("TDOD: {}", instr.mnemonic),
    }
}

/// The function used for lowering general `MachineInstr` into `MCInstr`
pub(crate) fn x64_lower(conv: CallConv, instrs: Vec<MachineInstr>) -> Vec<Box<dyn MCInstr>> {
    let mut out = vec![
        X64MCInstr::with0(Mnemonic::StartOptimization)
    ];

    for instr in instrs {
        x64_lower_instr(conv, &mut out, instr.clone());
    }

    X64AsmOpt(&mut out);

    let mut mc_instrs = vec![];

    for instr in out {
        mc_instrs.push( instr.into() );
    }


    mc_instrs
}

impl From<MachineOperand> for Operand {
    fn from(value: MachineOperand) -> Self {
        match value {
            MachineOperand::Stack(stack, _) => x64_stack!(stack as i32),
            MachineOperand::Imm(imm) => Operand::Imm(imm as i64),
            MachineOperand::Reg(reg) => match reg {
                crate::CodeGen::Reg::x64(x64_reg) => Operand::Reg(x64_reg),
                _ => panic!("the x64 backend expects the register to be x64 registers and not any other")
            },
        }
    }
}