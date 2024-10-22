use crate::CodeGen::MachineInstr;
use crate::Target::x64::instr::*;
use crate::IR::TypeMetadata;
use super::{RegAllocOperand, X64RegAllocInstr};

pub(crate) fn X64_lower_fcast(sink: &mut Vec<X64RegAllocInstr>, instr: &MachineInstr, input_type: TypeMetadata) {
    let out = instr.out.expect("fcast expects output");
    let out = out.into();

    let input = instr.operands.get(0).expect("fcast expects input operand");
    let input = (*input).into();

    if input_type == TypeMetadata::f32 {
        match instr.meta {
            TypeMetadata::i32 => sink.push(X64RegAllocInstr::with2(Mnemonic::Cvtss2si, RegAllocOperand::Tmp0, input)),
            TypeMetadata::i64 => sink.push(X64RegAllocInstr::with2(Mnemonic::Cvtss2si, RegAllocOperand::Tmp0, input)),
            
            TypeMetadata::f32 => sink.push(X64RegAllocInstr::with2(Mnemonic::Movss, RegAllocOperand::Tmp0, input)),
            TypeMetadata::f64 => sink.push(X64RegAllocInstr::with2(Mnemonic::Cvtss2sd, RegAllocOperand::Tmp0, input)),
            _ => panic!("fcast can only cast from f32 to i32/i64/f32/f64")
        }
    } else if input_type == TypeMetadata::f64 {
        match instr.meta {
            TypeMetadata::i32 => sink.push(X64RegAllocInstr::with2(Mnemonic::Cvtsd2si, RegAllocOperand::Tmp0, input)),
            TypeMetadata::i64 => sink.push(X64RegAllocInstr::with2(Mnemonic::Cvtsd2si, RegAllocOperand::Tmp0, input)),
            
            TypeMetadata::f32 => sink.push(X64RegAllocInstr::with2(Mnemonic::Cvtsd2ss, RegAllocOperand::Tmp0, input)),
            TypeMetadata::f64 => sink.push(X64RegAllocInstr::with2(Mnemonic::Cvtsd2ss, RegAllocOperand::Tmp0, input)),
            _ => panic!("fcast can only cast from f64 to i32/i64/f32/f64")
        }
    } else if input_type == TypeMetadata::i32 || input_type ==  TypeMetadata::i64 {
        match instr.meta {
            TypeMetadata::f32 => sink.push(X64RegAllocInstr::with2(Mnemonic::Cvtsi2ss, RegAllocOperand::Tmp0, input)),
            TypeMetadata::f64 => sink.push(X64RegAllocInstr::with2(Mnemonic::Cvtsi2sd, RegAllocOperand::Tmp0, input)),
            _ => panic!("fcast can only cast from i32 to f32/f64")
        }
    } else {
        panic!("fcast expects the input type to be either f32/f64/i32/i64")
    }

    sink.push( X64RegAllocInstr::with2(Mnemonic::Mov, out, RegAllocOperand::Tmp0))
}