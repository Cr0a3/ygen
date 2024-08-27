use std::collections::HashMap;

use super::*;

pub(crate) fn CompileCall(call: &Call<Function, Vec<Var>, Var>, registry: &mut TargetBackendDescr) -> Vec<Instr> {
    let boxed: Box<dyn Ir> = Box::new(call.clone());
    let block = registry.block.unwrap();

    let mut asm = vec![
        Instr::with0(Mnemonic::EndOptimization),
    ];

    registry.backend.stackSafe = true;

    let mut saved_var_memory_locations = HashMap::new();

    let mut offset = registry.backend.currStackOffsetForLocalVars + 8;

    for reg in &registry.backend.mutable {
        if !registry.backend.openUsableRegisters64.contains(reg) { // is a variable in the reg
            let var = registry.backend.getVarByReg(reg.clone());

            if let Some(var) = var { // get the var
                asm.push( Instr::with2(Mnemonic::Mov, Operand::Mem(x64Reg::Rbp - (offset as u32)), Operand::Reg(reg.clone())) );
                saved_var_memory_locations.insert(var.clone(), offset);
                offset += 8;
            }
        }
    }

    let mut reg_args = 0;

    for arg in &call.inner2 {
        let loc = if let Some((x, y)) = registry.backend.varsStorage.get_key_value(arg) {
            (x, y)
        } else {
            panic!("unknown variable {}", arg);
        };

        match loc.1 {
            VarStorage::Register(reg) => {
                if reg_args < registry.call.regArgs() {

                    let args = match arg.ty {
                        TypeMetadata::i64 | TypeMetadata::u64 | TypeMetadata::ptr => 
                            registry.call.args64(),
                        TypeMetadata::i32 | TypeMetadata::u32 => 
                            registry.call.args32(),
                        TypeMetadata::i16 | TypeMetadata::u16 => 
                            registry.call.args16(),
                        TypeMetadata::Void => vec![],
                    };

                    if args.contains(reg.as_any().downcast_ref::<x64Reg>().unwrap()) {
                        if let Some(offset) = saved_var_memory_locations.get(arg) {
                            asm.push( Instr::with2(Mnemonic::Mov, Operand::Reg(args[reg_args].boxed()), Operand::Mem(x64Reg::Rbp - (*offset as u32))));
                        }
                    } else {
                        asm.push(Instr::with2(Mnemonic::Mov, Operand::Reg(args[reg_args].boxed()), Operand::Reg(reg.clone())));
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
    }

    asm.push( Instr::with1(Mnemonic::Call, Operand::Imm(0)) );
    asm.push( Instr::with1(Mnemonic::Link, Operand::LinkDestination(call.inner1.name.to_string(), -4)) );

    if call.inner1.ty.ret != TypeMetadata::Void && block.isVarUsedAfterNode(&boxed, &call.inner3){  
        let store = if let Some(reg) = registry.backend.getOpenRegBasedOnTy(call.inner3.ty) {
            match call.inner1.ty.ret {
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

            match call.inner1.ty.ret {
                TypeMetadata::u16 | TypeMetadata::i16 => asm.push( Instr::with2(Mnemonic::Mov, Operand::Mem(mem.clone()), Operand::Reg(registry.call.ret16().boxed())) ),
                TypeMetadata::u32 | TypeMetadata::i32 => asm.push( Instr::with2(Mnemonic::Mov, Operand::Mem(mem.clone()), Operand::Reg(registry.call.ret32().boxed())) ),
                TypeMetadata::u64 | TypeMetadata::i64 => asm.push( Instr::with2(Mnemonic::Mov, Operand::Mem(mem.clone()), Operand::Reg(registry.call.ret64().boxed())) ),
                _ => unreachable!(),
            };

            VarStorage::Memory(mem)
        };

        registry.backend.insertVar(call.inner3.clone(), store);
    }

    for (var, off) in saved_var_memory_locations {
        let reg = if let Some(VarStorage::Register(reg)) = registry.backend.varsStorage.get(&var) {
            reg.to_owned()
        } else { todo!() }; // shouldn't happen

        asm.push( Instr::with2(Mnemonic::Mov, Operand::Reg(reg), Operand::Mem(x64Reg::Rbp - (off as u32))) );
    }

    asm.push(Instr::with0(Mnemonic::StartOptimization));

    asm
}
