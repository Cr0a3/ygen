use crate::CodeGen::MachineInstr; 
use crate::Target::x64::instr::{Mnemonic, Operand, X64MCInstr};
use crate::Target::x64::X64Reg;
use crate::IR::TypeMetadata;

pub(crate) fn x64_lower_cmov_zero(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr) {
    if instr.meta.float() {
        return x64_lower_fcmov0(sink, instr);
    }

    let cond = instr.operands.get(0).expect("expected condition for valid cmov");
    let cond = (*cond).into();

    let value = instr.operands.get(1).expect("expected value for valid cmov");
    let value: Operand = (*value).into();

    let out = instr.out.expect("expected output for valid cmov");
    let out: Operand = out.into();

    if out.is_mem() || value.is_imm() {
        sink.push( X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(X64Reg::Rax.sub_ty(instr.meta)), value.clone()) );
    }

     if let Operand::Mem(_) = cond {
        sink.extend_from_slice(&[
            X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(X64Reg::R11), Operand::Imm(1)),
            X64MCInstr::with2(Mnemonic::Cmp, cond, Operand::Reg(X64Reg::R11))
            ]
        );
    } else {
        sink.push( X64MCInstr::with2(Mnemonic::Cmp, cond, Operand::Imm(1)));
    };

    match (&out, &value) {
        (Operand::Reg(_), Operand::Imm(_)) => {
            sink.push( X64MCInstr::with2(Mnemonic::Cmove, out, Operand::Reg(X64Reg::Rax.sub_ty(instr.meta))));
        },
        (Operand::Reg(_), Operand::Reg(_)) | (Operand::Reg(_), Operand::Mem(_)) => {
            sink.push( X64MCInstr::with2(Mnemonic::Cmove, out, value));
        },

        (Operand::Mem(_), Operand::Imm(_)) => {
            sink.push( X64MCInstr::with2(Mnemonic::Cmove, Operand::Reg(X64Reg::Rax.sub_ty(instr.meta)), Operand::Reg(X64Reg::Rax.sub_ty(instr.meta))));
            sink.push( X64MCInstr::with2(Mnemonic::Mov, out, Operand::Reg(X64Reg::Rax.sub_ty(instr.meta))));
        },

        (Operand::Mem(_), Operand::Reg(_)) | (Operand::Mem(_), Operand::Mem(_))=> {
            sink.push( X64MCInstr::with2(Mnemonic::Cmove, Operand::Reg(X64Reg::Rax.sub_ty(instr.meta)), value));
            sink.push( X64MCInstr::with2(Mnemonic::Mov, out, Operand::Reg(X64Reg::Rax.sub_ty(instr.meta))));
        },

        _ => todo!(),
    }
}  

pub(crate) fn x64_lower_cmov_not_zero(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr) {
    if instr.meta.float() {
        return x64_lower_fcmovne0(sink, instr);
    }

    let cond = instr.operands.get(0).expect("expected condition for valid cmov");
    let cond = (*cond).into();

    let value = instr.operands.get(1).expect("expected value for valid cmov");
    let value: Operand = (*value).into();

    let out = instr.out.expect("expected output for valid cmov");
    let out: Operand = out.into();

    if out.is_mem() || value.is_imm() {
        sink.push( X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(X64Reg::Rax.sub_ty(instr.meta)), value.clone()) );
    }

     if let Operand::Mem(_) = cond {
        sink.extend_from_slice(&[
            X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(X64Reg::R11), Operand::Imm(1)),
            X64MCInstr::with2(Mnemonic::Cmp, cond, Operand::Reg(X64Reg::R11))
            ]
        );
    } else {
        sink.push( X64MCInstr::with2(Mnemonic::Cmp, cond, Operand::Imm(1)));
    };

    match (&out, &value) {
        (Operand::Reg(_), Operand::Imm(_)) => {
            sink.push( X64MCInstr::with2(Mnemonic::Cmovne, out, Operand::Reg(X64Reg::Rax.sub_ty(instr.meta))));
        },
        (Operand::Reg(_), Operand::Reg(_)) | (Operand::Reg(_), Operand::Mem(_)) => {
            sink.push( X64MCInstr::with2(Mnemonic::Cmovne, out, value));
        },

        (Operand::Mem(_), Operand::Imm(_)) => {
            sink.push( X64MCInstr::with2(Mnemonic::Cmovne, Operand::Reg(X64Reg::Rax.sub_ty(instr.meta)), Operand::Reg(X64Reg::Rax.sub_ty(instr.meta))));
            sink.push( X64MCInstr::with2(Mnemonic::Mov, out, Operand::Reg(X64Reg::Rax.sub_ty(instr.meta))));
        },

        (Operand::Mem(_), Operand::Reg(_)) | (Operand::Mem(_), Operand::Mem(_))=> {
            sink.push( X64MCInstr::with2(Mnemonic::Cmovne, Operand::Reg(X64Reg::Rax.sub_ty(instr.meta)), value));
            sink.push( X64MCInstr::with2(Mnemonic::Mov, out, Operand::Reg(X64Reg::Rax.sub_ty(instr.meta))));
        },

        _ => todo!(),
    }
}  

pub(crate) fn x64_lower_fcmov0(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr) {
    let cond = instr.operands.get(0).expect("expected condition for valid cmov");
    let cond: Operand = (*cond).into();

    let value = instr.operands.get(1).expect("expected value for valid cmov");
    let value: Operand = (*value).into();

    let out = instr.out.expect("expected output for valid cmov");
    let out: Operand = out.into();

    let tmp = if instr.meta == TypeMetadata::f32 { X64Reg::Eax } else { X64Reg::Rax };
    let tmp2 = if instr.meta == TypeMetadata::f32 { X64Reg::Ebx } else { X64Reg::R11 };
  
    let mnemonic =if instr.meta == TypeMetadata::f32 {
        Mnemonic::Movd
    } else {
        Mnemonic::Movq
    };

    sink.extend_from_slice(&if let Operand::Mem(_) = cond {
        vec![X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(X64Reg::R11), Operand::Imm(1)),
             X64MCInstr::with2(Mnemonic::Cmp, cond, Operand::Reg(X64Reg::R11))]
    } else {
        vec![X64MCInstr::with2(Mnemonic::Cmp, cond, Operand::Imm(1))]
    });

    if let Operand::Reg(_) = out {
        sink.extend_from_slice(&[
            X64MCInstr::with2(mnemonic, Operand::Reg(tmp), out.to_owned()),
            X64MCInstr::with2(mnemonic, Operand::Reg(tmp2), value),
            X64MCInstr::with2(Mnemonic::Cmove, Operand::Reg(tmp), Operand::Reg(tmp2)),
            X64MCInstr::with2(mnemonic, out, Operand::Reg(tmp)),
        ]);
    } else {
        sink.extend_from_slice(&[
            X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(tmp), out.to_owned()),
            X64MCInstr::with2(mnemonic, Operand::Reg(tmp2), value),
            X64MCInstr::with2(Mnemonic::Cmove, Operand::Reg(tmp), Operand::Reg(tmp2)),
            X64MCInstr::with2(Mnemonic::Mov, out, Operand::Reg(tmp)),
        ]);
    }
} 

pub(crate) fn x64_lower_fcmovne0(sink: &mut Vec<X64MCInstr>, instr: &MachineInstr) {
    let cond = instr.operands.get(0).expect("expected condition for valid cmov");
    let cond: Operand = (*cond).into();

    let value = instr.operands.get(1).expect("expected value for valid cmov");
    let value: Operand = (*value).into();

    let out = instr.out.expect("expected output for valid cmov");
    let out: Operand = out.into();

    let tmp = if instr.meta == TypeMetadata::f32 { X64Reg::Eax } else { X64Reg::Rax };
    let tmp2 = if instr.meta == TypeMetadata::f32 { X64Reg::Ebx } else { X64Reg::R11 };
  
    let mnemonic =if instr.meta == TypeMetadata::f32 {
        Mnemonic::Movd
    } else {
        Mnemonic::Movq
    };

    sink.extend_from_slice(&if let Operand::Mem(_) = cond {
        vec![X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(X64Reg::R11), Operand::Imm(1)),
             X64MCInstr::with2(Mnemonic::Cmp, cond, Operand::Reg(X64Reg::R11))]
    } else {
        vec![X64MCInstr::with2(Mnemonic::Cmp, cond, Operand::Imm(1))]
    });

    if let Operand::Reg(_) = out {
        sink.extend_from_slice(&[
            X64MCInstr::with2(mnemonic, Operand::Reg(tmp), out.to_owned()),
            X64MCInstr::with2(mnemonic, Operand::Reg(tmp2), value),
            X64MCInstr::with2(Mnemonic::Cmovne, Operand::Reg(tmp), Operand::Reg(tmp2)),
            X64MCInstr::with2(mnemonic, out, Operand::Reg(tmp)),
        ]);
    } else {
        sink.extend_from_slice(&[
            X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(tmp), out.to_owned()),
            X64MCInstr::with2(mnemonic, Operand::Reg(tmp2), value),
            X64MCInstr::with2(Mnemonic::Cmovne, Operand::Reg(tmp), Operand::Reg(tmp2)),
            X64MCInstr::with2(Mnemonic::Mov, out, Operand::Reg(tmp)),
        ]);
    }
} 
