use super::*;

pub(crate) fn CompileConstAssign(assign: &ConstAssign<Var, Type>, registry: &mut TargetBackendDescr) -> Vec<Instr> {
    let infos = &mut registry.backend;
    let block = registry.block.unwrap();

    let ty = &assign.inner1.ty;

    let boxed: Box<dyn Ir> = Box::new(assign.clone());
    
    if !block.isVarUsedAfterNode(&boxed, &assign.inner1) {
        return vec![]; // all of these calculations don't need to be done: dead code removal
    }
    
    let store = {
        if let Some(reg) = infos.getOpenRegBasedOnTy(*ty) {
            VarStorage::Register(reg)
        } else {
            let addend = match ty {
                TypeMetadata::u16 | TypeMetadata::i16 => 2,
                TypeMetadata::u32 | TypeMetadata::i32 => 4,
                TypeMetadata::u64 | TypeMetadata::i64 | TypeMetadata::ptr => 8,
                TypeMetadata::Void => todo!("cant output an assing somthing to void"),
            };

            infos.currStackOffsetForLocalVars += addend;
            VarStorage::Memory(x64Reg::Rbp - (infos.currStackOffsetForLocalVars - addend) as u32)
        }
    };

    infos.insertVar(
        assign.inner1.clone(), 
        store.clone()
    );

    if let VarStorage::Register(reg) = &store {
        vec![ Instr::with2(Mnemonic::Mov, Operand::Reg(reg.boxed()), Operand::Imm(assign.inner2.val() as i64)) ]
    } else if let VarStorage::Memory(mem) = &store {
        vec![ Instr::with2(Mnemonic::Mov, Operand::Mem(mem.clone()), Operand::Imm(assign.inner2.val() as i64)) ]
    } else { todo!() }
}

pub(crate) fn CompileConstAssignVar(assign: &ConstAssign<Var, Var>, registry: &mut TargetBackendDescr) -> Vec<Instr> {
    let infos = &mut registry.backend;
    let block = registry.block.unwrap();

    let loc = if let Some(loc) = infos.varsStorage.get_key_value(&assign.inner2) {
        loc.1.clone()
    } else {
        panic!("unknown variable: {:?}", assign.inner2)
    };

    let ty = &assign.inner1.ty;

    let boxed: Box<dyn Ir> = Box::new(assign.clone());

    if !block.isVarUsedAfterNode(&boxed, &assign.inner2) {
        infos.drop(&assign.inner2);
    }
    if !block.isVarUsedAfterNode(&boxed, &assign.inner1) {
        return vec![]; // all of these calculations don't need to be done: dead code removal
    }
    
    let store = {
        if let Some(reg) = infos.getOpenRegBasedOnTy(*ty) {
            VarStorage::Register(reg)
        } else {
            let addend = match ty {
                TypeMetadata::u16 | TypeMetadata::i16 => 2,
                TypeMetadata::u32 | TypeMetadata::i32 => 4,
                TypeMetadata::u64 | TypeMetadata::i64 | TypeMetadata::ptr => 8,
                TypeMetadata::Void => todo!("cant output an assing somthing to void"),
            };

            infos.currStackOffsetForLocalVars += addend;
            VarStorage::Memory(x64Reg::Rbp - (infos.currStackOffsetForLocalVars - addend) as u32)
        }
    };

    infos.insertVar(
        assign.inner1.clone(), 
        store.clone()
    );

    if let VarStorage::Register(reg) = &store {
        if let VarStorage::Register(reg2) = &loc {
            vec![ Instr::with2(Mnemonic::Mov, Operand::Reg(reg.boxed()), Operand::Reg(reg2.boxed())) ]
        } else if let VarStorage::Memory(mem2) = &loc {
            vec![ Instr::with2(Mnemonic::Mov, Operand::Reg(reg.boxed()), Operand::Mem(mem2.clone())) ]
        } else { unreachable!() }
    } else if let VarStorage::Memory(mem) = &store {
        if let VarStorage::Register(reg2) = &loc {
            vec![ Instr::with2(Mnemonic::Mov, Operand::Mem(mem.clone()), Operand::Reg(reg2.boxed())) ]
        } else if let VarStorage::Memory(mem2) = &loc {
            vec![ 
                Instr::with2(Mnemonic::Mov, Operand::Reg(infos.getTmpBasedOnTy(*ty)), Operand::Mem(mem2.clone())),
                Instr::with2(Mnemonic::Mov, Operand::Mem(mem2.clone()), Operand::Reg(infos.getTmpBasedOnTy(*ty)))
            ]
        } else { unreachable!() }
    } else { todo!() }
}

pub(crate) fn CompileConstAssignConst(assign: &ConstAssign<Var, Const>, registry: &mut TargetBackendDescr) -> Vec<Instr> {
    let block = registry.block.unwrap();
    
    registry.backend.stackSafe = true;

    let infos = &mut registry.backend;

    let ty = &assign.inner1.ty;

    let boxed: Box<dyn Ir> = Box::new(assign.clone());

    if !block.isVarUsedAfterNode(&boxed, &assign.inner1) {
        return vec![]; // all of these calculations don't need to be done: dead code removal
    }
    
    let store = {
        if let Some(reg) = infos.getOpenRegBasedOnTy(*ty) {
            VarStorage::Register(reg)
        } else {
            let addend = match ty {
                TypeMetadata::u16 | TypeMetadata::i16 => 2,
                TypeMetadata::u32 | TypeMetadata::i32 => 4,
                TypeMetadata::u64 | TypeMetadata::i64 | TypeMetadata::ptr => 8,
                TypeMetadata::Void => todo!("cant output an assing somthing to void"),
            };

            infos.currStackOffsetForLocalVars += addend;
            VarStorage::Memory(x64Reg::Rbp - (infos.currStackOffsetForLocalVars - addend) as u32)
        }
    };

    infos.insertVar(
        assign.inner1.clone(), 
        store.clone()
    );

    if let VarStorage::Register(reg) = &store {
        vec![ 
            Instr::with2(Mnemonic::Lea, Operand::Reg(infos.getTmpBasedOnTy(*ty)), Operand::Mem(MemOp { base: None, index: None, scale: 0, displ: 1, rip: true })),
            Instr::with1(Mnemonic::Link, Operand::LinkDestination(assign.inner2.name.to_string(), -4)),
            Instr::with2(Mnemonic::Mov, Operand::Reg(reg.boxed()), Operand::Reg(infos.getTmpBasedOnTy(*ty)))
        ]
    } else if let VarStorage::Memory(mem) = &store {
        vec![ 
            Instr::with2(Mnemonic::Lea, Operand::Reg(infos.getTmpBasedOnTy(*ty)), Operand::Mem(MemOp { base: None, index: None, scale: 0, displ: 1, rip: true })),
            Instr::with1(Mnemonic::Link, Operand::LinkDestination(assign.inner2.name.to_string(), -4)),
            Instr::with2(Mnemonic::Mov, Operand::Mem(mem.clone()), Operand::Reg(infos.getTmpBasedOnTy(*ty)))
        ]
    } else { todo!() }
}
