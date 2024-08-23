use super::*;

pub(crate) fn CompileCall(call: &Call<Function, Vec<Var>, Var>, registry: &mut TargetBackendDescr) -> Vec<Instr> {
    let boxed: Box<dyn Ir> = Box::new(call.clone());
    let block = registry.block.unwrap();

    let mut asm = vec![];

    for reg in vec![x64Reg::Rcx, x64Reg::Rdx, x64Reg::Rsi, x64Reg::Rdi, x64Reg::Rsi] { // save mutable registers
        if !registry.backend.openUsableRegisters64.contains(&reg.boxed()) {
            let var = registry.backend.getVarByReg(reg.boxed()).cloned();
            
            if let Some(var) = var {
                if block.isVarUsedAfterNode(&boxed, &var) {
                    asm.push(Instr::with1(Mnemonic::Push, Operand::Reg(reg.boxed())));
                }
            }
        }
    }

    let func = &call.inner1;

    let mut reg_args = 0;

    for arg in &call.inner2 {
        let loc = registry.backend.varsStorage.get_key_value(arg).expect("expected valid variable as arg input");

        match loc.1 {
            VarStorage::Register(reg) => {
                if reg_args < registry.call.regArgs() {
                    match arg.ty {
                        TypeMetadata::i64 | TypeMetadata::u64 | TypeMetadata::ptr => 
                            asm.push( Instr::with2(Mnemonic::Mov, Operand::Reg(registry.call.args64()[reg_args].boxed()), Operand::Reg(reg.clone()))),
                        TypeMetadata::i32 | TypeMetadata::u32 => 
                            asm.push( Instr::with2(Mnemonic::Mov, Operand::Reg(registry.call.args32()[reg_args].boxed()), Operand::Reg(reg.clone()))),
                        TypeMetadata::i16 | TypeMetadata::u16 => 
                            asm.push( Instr::with2(Mnemonic::Mov, Operand::Reg(registry.call.args16()[reg_args].boxed()), Operand::Reg(reg.clone()))),
                        TypeMetadata::Void => {},
                    }

                    reg_args += 1;
                } else {
                    asm.push( Instr::with1(Mnemonic::Push, Operand::Reg(reg.clone())));
                }
            },
            VarStorage::Memory(mem) => {
                if reg_args < registry.call.regArgs() {
                    if arg.ty == TypeMetadata::i64 || arg.ty == TypeMetadata::u64 {
                        asm.push( Instr::with2(Mnemonic::Mov, Operand::Reg(registry.call.args64()[reg_args].boxed()), Operand::Mem(mem.clone())));
                    } else if arg.ty == TypeMetadata::i32 || arg.ty == TypeMetadata::u32 {
                        asm.push( Instr::with2(Mnemonic::Mov, Operand::Reg(registry.call.args32()[reg_args].boxed()), Operand::Mem(mem.clone())));
                    } else if arg.ty == TypeMetadata::i16 || arg.ty == TypeMetadata::u16 {
                        asm.push( Instr::with2(Mnemonic::Mov, Operand::Reg(registry.call.args16()[reg_args].boxed()), Operand::Mem(mem.clone())));
                    }
                    reg_args += 1;
                } else {
                    asm.push( Instr::with1(Mnemonic::Push, Operand::Mem(mem.clone())));
                }
            },
        }
        if !block.isVarUsedAfterNode(&boxed, arg) {
            registry.backend.drop(arg);
        }
    }

    if registry.call.reset_eax() {
        asm.push(Instr::with2(Mnemonic::Xor, Operand::Reg(x64Reg::Eax.boxed()), Operand::Reg(x64Reg::Eax.boxed())));
    }

    asm.push( Instr::with1(Mnemonic::Call, Operand::Imm(0)));

    asm.push( Instr::with1(Mnemonic::Link, Operand::LinkDestination(call.inner1.name.to_string(), -4)));

    if func.ty.ret != TypeMetadata::Void {  
        let store = if let Some(reg) = registry.backend.getOpenRegBasedOnTy(call.inner3.ty) {
            match func.ty.ret {
                TypeMetadata::u16 | TypeMetadata::i16 => asm.push( Instr::with2(Mnemonic::Mov, Operand::Reg(reg.clone()), Operand::Reg(registry.call.ret16().boxed())) ),
                TypeMetadata::u32 | TypeMetadata::i32 => asm.push( Instr::with2(Mnemonic::Mov, Operand::Reg(reg.clone()), Operand::Reg(registry.call.ret32().boxed())) ),
                TypeMetadata::u64 | TypeMetadata::i64 => asm.push( Instr::with2(Mnemonic::Mov, Operand::Reg(reg.clone()), Operand::Reg(registry.call.ret64().boxed())) ),
                _ => unreachable!(),
            };
            VarStorage::Register(reg)
        } else {
            let addend = match call.inner3.ty {
                TypeMetadata::u16 | TypeMetadata::i16 => 2,
                TypeMetadata::u32 | TypeMetadata::i32 => 4,
                TypeMetadata::u64 | TypeMetadata::i64 | TypeMetadata::ptr => 8,
                TypeMetadata::Void => unreachable!(),
            };

            registry.backend.currStackOffsetForLocalVars += addend;
            let mem = x64Reg::Rbp - (registry.backend.currStackOffsetForLocalVars - addend) as u32;

            match func.ty.ret {
                TypeMetadata::u16 | TypeMetadata::i16 => asm.push( Instr::with2(Mnemonic::Mov, Operand::Mem(mem.clone()), Operand::Reg(registry.call.ret16().boxed())) ),
                TypeMetadata::u32 | TypeMetadata::i32 => asm.push( Instr::with2(Mnemonic::Mov, Operand::Mem(mem.clone()), Operand::Reg(registry.call.ret32().boxed())) ),
                TypeMetadata::u64 | TypeMetadata::i64 => asm.push( Instr::with2(Mnemonic::Mov, Operand::Mem(mem.clone()), Operand::Reg(registry.call.ret64().boxed())) ),
                _ => unreachable!(),
            };

            VarStorage::Memory(mem)
        };

        registry.backend.insertVar(call.inner3.clone(), store);
    }
    
    for reg in vec![x64Reg::Rcx, x64Reg::Rdx, x64Reg::Rsi, x64Reg::Rdi, x64Reg::Rsi] { // getback mutable registers
        if !registry.backend.openUsableRegisters64.contains(&reg.boxed()) {
            let var = registry.backend.getVarByReg(reg.boxed()).cloned();
            
            if let Some(var) = var {
                if block.isVarUsedAfterNode(&boxed, &var) {
                    asm.push(Instr::with1(Mnemonic::Pop, Operand::Reg(reg.boxed())));
                }
            }
        }
    }

    asm
}
