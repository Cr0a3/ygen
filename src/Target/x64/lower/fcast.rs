use crate::{CodeGen::MachineInstr, Target::{x64::instr::*, x64Reg}, IR::TypeMetadata};

pub(crate) fn X64_lower_fcast(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr, input_type: TypeMetadata) {
    let out = instr.out.expect("fcast expects output");
    let out = out.into();

    let input = instr.operands.get(0).expect("fcast expects input operand");
    let input = (*input).into();

    let mut output_reg = x64Reg::Rax.sub_ty(instr.meta);

    if let Operand::Reg(reg) = &out {
        output_reg = *reg;
    }

    if input_type == TypeMetadata::f32 {
        match instr.meta {
            TypeMetadata::i32 => sink.push(X64MCInstr::with2(Mnemonic::Cvtss2si, Operand::Reg(output_reg), input)),
            TypeMetadata::i64 => sink.push(X64MCInstr::with2(Mnemonic::Cvtss2si, Operand::Reg(output_reg), input)),
            
            TypeMetadata::f32 => sink.push(X64MCInstr::with2(Mnemonic::Movss, Operand::Reg(output_reg), input)),
            TypeMetadata::f64 => sink.push(X64MCInstr::with2(Mnemonic::Cvtss2sd, Operand::Reg(output_reg), input)),
            _ => panic!("fcast can only cast from f32 to i32/i64/f32/f64")
        }
    } else if input_type == TypeMetadata::f64 {
        match instr.meta {
            TypeMetadata::i32 => sink.push(X64MCInstr::with2(Mnemonic::Cvtsd2si, Operand::Reg(output_reg), input)),
            TypeMetadata::i64 => sink.push(X64MCInstr::with2(Mnemonic::Cvtsd2si, Operand::Reg(output_reg), input)),
            
            TypeMetadata::f32 => sink.push(X64MCInstr::with2(Mnemonic::Cvtsd2ss, Operand::Reg(output_reg), input)),
            TypeMetadata::f64 => sink.push(X64MCInstr::with2(Mnemonic::Cvtsd2ss, Operand::Reg(output_reg), input)),
            _ => panic!("fcast can only cast from f64 to i32/i64/f32/f64")
        }
    } else if input_type == TypeMetadata::i32 || input_type ==  TypeMetadata::i64 {
        match instr.meta {
            TypeMetadata::f32 => sink.push(X64MCInstr::with2(Mnemonic::Cvtsi2ss, Operand::Reg(output_reg), input)),
            TypeMetadata::f64 => sink.push(X64MCInstr::with2(Mnemonic::Cvtsi2sd, Operand::Reg(output_reg), input)),
            _ => panic!("fcast can only cast from i32 to f32/f64")
        }
    } else {
        panic!("fcast expects the input type to be either f32/f64/i32/i64")
    }

    if let Operand::Mem(out) = &out {
        sink.push(if instr.meta == TypeMetadata::f32 {
            X64MCInstr::with2(Mnemonic::Movd, Operand::Mem(out.to_owned()), Operand::Reg(output_reg))
        } else if instr.meta == TypeMetadata::f64 {
            X64MCInstr::with2(Mnemonic::Movq, Operand::Mem(out.to_owned()), Operand::Reg(output_reg))
        } else {
            X64MCInstr::with2(Mnemonic::Mov, Operand::Mem(out.to_owned()), Operand::Reg(output_reg))
        });
    }
}