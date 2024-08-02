use std::collections::VecDeque;

use crate::prelude::{Block, Function, Type, TypeMetadata, Var};
use crate::Target::target_descr::{TargetBackendDescr, VarStorage};
use crate::Target::Reg;
use crate::IR::ir::*;

use crate::Target::CallConv;

use super::{x64Reg, instr::*};

pub(crate) fn CompileAddVarVar(add: &Add<Var, Var, Var>, registry: &mut TargetBackendDescr) -> Vec<Instr> {
    let infos = &mut registry.backend;

    let loc1 = if let Some(loc1) = infos.varsStorage.get(&add.inner1) {
        loc1.clone()
    } else {
        panic!("unknown variable: {:?}", add.inner1)
    };
    
    let loc2 = if let Some(loc2) = infos.varsStorage.get(&add.inner2) {
        loc2.clone()
        
    } else {
        panic!("unknown variable: {:?}", add.inner1)
    };

    let boxed: Box<dyn Ir> = Box::new(add.clone());

    if !BlockX86FuncisVarUsedAfterNode(registry.block.unwrap(), &boxed, &add.inner1) {
        infos.drop(&add.inner1);
    }
    if !BlockX86FuncisVarUsedAfterNode(registry.block.unwrap(), &boxed, &add.inner2) {
        infos.drop(&add.inner2);
    }
    if !BlockX86FuncisVarUsedAfterNode(registry.block.unwrap(), &boxed, &add.inner3) {
        return vec![]; // all of these calculations don't need to be done: dead code removal
    }

    let ty = &add.inner1.ty;
    
    let ret = {
        if let Some(reg) = infos.getOpenRegBasedOnTy(*ty) {
            VarStorage::Register(reg)
        } else {
            let addend = match ty {
                TypeMetadata::u16 | TypeMetadata::i16=> 2,
                TypeMetadata::u32 | TypeMetadata::i32=> 4,
                TypeMetadata::u64 | TypeMetadata::i64=> 8,
                TypeMetadata::Void => todo!("cant output an addition into an void"),
            };

            infos.currStackOffsetForLocalVars += addend;
            VarStorage::Memory(x64Reg::Rbp - (infos.currStackOffsetForLocalVars - addend) as u32)
        }
    };

    infos.insertVar(
        add.inner3.clone(), 
        ret.clone()
    );
    let tmp = infos.getTmpBasedOnTy(*ty);

    if let VarStorage::Register(loc1Reg) = &loc1 {
        if let VarStorage::Register(loc2Reg) = &loc2 {
            if let VarStorage::Register(reg) = &ret {
                return vec![
                    Instr::with2(Mnemonic::Mov, Operand::Reg(tmp.boxed()), Operand::Reg(loc2Reg.boxed())),
                    Instr::with2(Mnemonic::Add, Operand::Reg(tmp.boxed()), Operand::Reg(loc1Reg.boxed())),
                    Instr::with2(Mnemonic::Mov, Operand::Reg(reg.boxed()), Operand::Reg(tmp.boxed())),
                ]
            } else if let VarStorage::Memory(mem) = &ret {
                return vec![
                    Instr::with2(Mnemonic::Mov, Operand::Reg(tmp.boxed()), Operand::Reg(loc2Reg.boxed())),
                    Instr::with2(Mnemonic::Add, Operand::Reg(tmp.boxed()), Operand::Reg(loc1Reg.boxed())),
                    Instr::with2(Mnemonic::Mov, Operand::Mem(mem.clone()), Operand::Reg(tmp.boxed())),
                    ];
            } else { todo!() }
        }
    }

    if let VarStorage::Memory(mem1) = loc1 {
        if let VarStorage::Memory(mem2) = loc2 {
            if let VarStorage::Register(reg) = &ret {
                return vec![
                    Instr::with2(Mnemonic::Mov, Operand::Reg(tmp.boxed()), Operand::Mem(mem1.clone())),
                    Instr::with2(Mnemonic::Add, Operand::Reg(tmp.boxed()), Operand::Mem(mem2.clone())),
                    Instr::with2(Mnemonic::Mov, Operand::Reg(reg.boxed()), Operand::Reg(tmp.boxed())),
                ];
            } else if let VarStorage::Memory(mem) = &ret {
                return vec![
                    Instr::with2(Mnemonic::Mov, Operand::Reg(tmp.boxed()), Operand::Mem(mem1.clone())),
                    Instr::with2(Mnemonic::Add, Operand::Reg(tmp.boxed()), Operand::Mem(mem2.clone())),
                    Instr::with2(Mnemonic::Mov, Operand::Mem(mem.clone()), Operand::Reg(tmp.boxed())),
                ];
            } else { todo!() }
        }
    }

    vec![]
}

pub(crate) fn CompileSubVarVar(sub: &Sub<Var, Var, Var>, registry: &mut TargetBackendDescr) -> Vec<Instr> {
    let infos = &mut registry.backend;

    let loc1 = if let Some(loc1) = infos.varsStorage.get(&sub.inner1) {
        loc1.clone()
    } else {
        panic!("unknown variable: {:?}", sub.inner1)
    };
    
    let loc2 = if let Some(loc2) = infos.varsStorage.get(&sub.inner2) {
        loc2.clone()
        
    } else {
        panic!("unknown variable: {:?}", sub.inner1)
    };

    let boxed: Box<dyn Ir> = Box::new(sub.clone());

    if !BlockX86FuncisVarUsedAfterNode(registry.block.unwrap(), &boxed, &sub.inner1) {
        infos.drop(&sub.inner1);
    }
    if !BlockX86FuncisVarUsedAfterNode(registry.block.unwrap(), &boxed, &sub.inner2) {
        infos.drop(&sub.inner2);
    }
    if !BlockX86FuncisVarUsedAfterNode(registry.block.unwrap(), &boxed, &sub.inner3) {
        return vec![]; // all of these calculations don't need to be done: dead code removal
    }

    let ty = &sub.inner1.ty;
    
    let ret = {
        if let Some(reg) = infos.getOpenRegBasedOnTy(*ty) {
            VarStorage::Register(reg)
        } else {
            let addend = match ty {
                TypeMetadata::u16 | TypeMetadata::i16=> 2,
                TypeMetadata::u32 | TypeMetadata::i32=> 4,
                TypeMetadata::u64 | TypeMetadata::i64=> 8,
                TypeMetadata::Void => todo!("cant output an addition into an void"),
            };

            infos.currStackOffsetForLocalVars += addend;
            VarStorage::Memory(x64Reg::Rbp - (infos.currStackOffsetForLocalVars - addend) as u32)
        }
    };

    infos.insertVar(
        sub.inner3.clone(), 
        ret.clone()
    );
    let tmp = infos.getTmpBasedOnTy(*ty);

    if let VarStorage::Register(loc1Reg) = &loc1 {
        if let VarStorage::Register(loc2Reg) = &loc2 {
            if let VarStorage::Register(reg) = &ret {
                return vec![
                    Instr::with2(Mnemonic::Mov, Operand::Reg(tmp.boxed()), Operand::Reg(loc2Reg.boxed())),
                    Instr::with2(Mnemonic::Sub, Operand::Reg(tmp.boxed()), Operand::Reg(loc1Reg.boxed())),
                    Instr::with2(Mnemonic::Mov, Operand::Reg(reg.boxed()), Operand::Reg(tmp.boxed())),
                ]
            } else if let VarStorage::Memory(mem) = &ret {
                return vec![
                    Instr::with2(Mnemonic::Mov, Operand::Reg(tmp.boxed()), Operand::Reg(loc2Reg.boxed())),
                    Instr::with2(Mnemonic::Sub, Operand::Reg(tmp.boxed()), Operand::Reg(loc1Reg.boxed())),
                    Instr::with2(Mnemonic::Mov, Operand::Mem(mem.clone()), Operand::Reg(tmp.boxed())),
                    ];
            } else { todo!() }
        }
    }

    if let VarStorage::Memory(mem1) = loc1 {
        if let VarStorage::Memory(mem2) = loc2 {
            if let VarStorage::Register(reg) = &ret {
                return vec![
                    Instr::with2(Mnemonic::Mov, Operand::Reg(tmp.boxed()), Operand::Mem(mem1.clone())),
                    Instr::with2(Mnemonic::Sub, Operand::Reg(tmp.boxed()), Operand::Mem(mem2.clone())),
                    Instr::with2(Mnemonic::Mov, Operand::Reg(reg.boxed()), Operand::Reg(tmp.boxed())),
                ];
            } else if let VarStorage::Memory(mem) = &ret {
                return vec![
                    Instr::with2(Mnemonic::Mov, Operand::Reg(tmp.boxed()), Operand::Mem(mem1.clone())),
                    Instr::with2(Mnemonic::Sub, Operand::Reg(tmp.boxed()), Operand::Mem(mem2.clone())),
                    Instr::with2(Mnemonic::Mov, Operand::Mem(mem.clone()), Operand::Reg(tmp.boxed())),
                ];
            } else { todo!() }
        }
    }

    vec![]
}

pub(crate) fn CompileConstAssign(assign: &ConstAssign<Var, Type>, registry: &mut TargetBackendDescr) -> Vec<Instr> {
    let infos = &mut registry.backend;

    let ty = &assign.inner1.ty;
    

    let boxed: Box<dyn Ir> = Box::new(assign.clone());
    
    if !BlockX86FuncisVarUsedAfterNode(registry.block.unwrap(), &boxed, &assign.inner1) {
        return vec![]; // all of these calculations don't need to be done: dead code removal
    }
    
    let store = {
        if let Some(reg) = infos.getOpenRegBasedOnTy(*ty) {
            VarStorage::Register(reg)
        } else {
            let addend = match ty {
                TypeMetadata::u16 | TypeMetadata::i16=> 2,
                TypeMetadata::u32 | TypeMetadata::i32=> 4,
                TypeMetadata::u64 | TypeMetadata::i64=> 8,
                TypeMetadata::Void => todo!("cant output an addition into an void"),
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

pub(crate) fn CompileAddTyTy(add: &Add<Type, Type, Var>, registry: &mut TargetBackendDescr) -> Vec<Instr> {
    let val = add.inner1.val() + add.inner2.val();
    

    let boxed: Box<dyn Ir> = Box::new(add.clone());

    if !BlockX86FuncisVarUsedAfterNode(registry.block.unwrap(), &boxed, &add.inner3) {
        return vec![]; // all of these calculations don't need to be done: dead code removal
    }

    CompileConstAssign(&ConstAssign::new(add.inner3.clone(), {
        match add.inner3.ty {
            TypeMetadata::u16 => Type::u16(val as u16),
            TypeMetadata::u32 => Type::u32(val as u32),
            TypeMetadata::u64 => Type::u64(val as u64),
            TypeMetadata::i16 => Type::i16(val as i16),
            TypeMetadata::i32 => Type::i32(val as i32),
            TypeMetadata::i64 => Type::i64(val as i64),
            TypeMetadata::Void =>Type::Void,
        }
    }), registry)
}

pub(crate) fn CompileSubTyTy(sub: &Sub<Type, Type, Var>, registry: &mut TargetBackendDescr) -> Vec<Instr> {
    let val = sub.inner1.val() - sub.inner2.val();
    

    let boxed: Box<dyn Ir> = Box::new(sub.clone());

    if !BlockX86FuncisVarUsedAfterNode(registry.block.unwrap(), &boxed, &sub.inner3) {
        return vec![]; // all of these calculations don't need to be done: dead code removal
    }

    CompileConstAssign(&ConstAssign::new(sub.inner3.clone(), {
        match sub.inner3.ty {
            TypeMetadata::u16 => Type::u16(val as u16),
            TypeMetadata::u32 => Type::u32(val as u32),
            TypeMetadata::u64 => Type::u64(val as u64),
            TypeMetadata::i16 => Type::i16(val as i16),
            TypeMetadata::i32 => Type::i32(val as i32),
            TypeMetadata::i64 => Type::i64(val as i64),
            TypeMetadata::Void =>Type::Void,
        }
    }), registry)
}

pub(crate) fn CompileRetType(ret: &Return<Type>, registry: &mut TargetBackendDescr) -> Vec<Instr> {
    if ret.inner1 != Type::Void {
        vec![Instr::with2(Mnemonic::Mov, match ret.inner1.into() {
            TypeMetadata::u16 | TypeMetadata::i16 => Operand::Reg(registry.call.ret16().boxed()),
            TypeMetadata::u32 | TypeMetadata::i32 => Operand::Reg(registry.call.ret32().boxed()),
            TypeMetadata::u64 | TypeMetadata::i64=> Operand::Reg(registry.call.ret64().boxed()),
            _ => unreachable!(),
        }, Operand::Imm(ret.inner1.val() as i64))]
    } else {
        vec![]
    }
}


pub(crate) fn CompileRetVar(ret: &Return<Var>, registry: &mut TargetBackendDescr) -> Vec<Instr> {
    let (var, loc) = if let Some(loc) = registry.backend.varsStorage.get_key_value(&ret.inner1) {
        loc.clone()
    } else {
        panic!("unknown variable: {:?}", ret.inner1)
    };

    if var.ty == TypeMetadata::Void {
        return vec![];
    }

    vec![Instr::with2(Mnemonic::Mov, match var.ty {
        TypeMetadata::u16 | TypeMetadata::i16 => Operand::Reg(registry.call.ret16().boxed()),
        TypeMetadata::u32 | TypeMetadata::i32 => Operand::Reg(registry.call.ret32().boxed()),
        TypeMetadata::u64 | TypeMetadata::i64=> Operand::Reg(registry.call.ret64().boxed()),
        _ => unreachable!(),
    }, {
        if let VarStorage::Memory(mem) = loc { Operand::Mem(mem.clone()) }
        else if let VarStorage::Register(reg) = loc { Operand::Reg(reg.boxed()) }
        else { unreachable!() }
    })]
}

pub(crate) fn x64BuildProlog(_: &Block, registry: &mut TargetBackendDescr) -> Vec<Instr> {
    let mut res = vec![];

    if registry.backend.currStackOffsetForLocalVars != 0 {
        res.push( Instr::with1(Mnemonic::Push, Operand::Reg(x64Reg::Rbp.boxed())) );
        res.push( Instr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::Rbp.boxed()), Operand::Reg(x64Reg::Rsp.boxed())) );
        res.push( Instr::with2(Mnemonic::Sub, Operand::Reg(x64Reg::Rsp.boxed()), Operand::Imm(16)) );
    }

    for backuped in &registry.backend.saveRegister {
        res.push( Instr::with1(Mnemonic::Push, Operand::Reg(backuped.boxed())) )
    }

    res
}

pub(crate) fn x64BuildEpilog(_: &Block, registry: &mut TargetBackendDescr) -> Vec<Instr> {
    let mut res = vec![];

    for backuped in &registry.backend.saveRegister {
        res.push( Instr::with1(Mnemonic::Pop, Operand::Reg(backuped.boxed())) )
    }

    if registry.backend.currStackOffsetForLocalVars != 0 {
        res.push( Instr::with2(Mnemonic::Add, Operand::Reg(x64Reg::Rsp.boxed()), Operand::Imm(16)) );
        res.push( Instr::with1(Mnemonic::Pop, Operand::Reg(x64Reg::Rbp.boxed())) );
    }

    res.push( Instr::with0(Mnemonic::Ret));

    res
}

pub(crate) fn buildAsmX86<'a>(block: &'a Block, func: &Function, call: &CallConv, registry: &mut TargetBackendDescr<'a>) -> Vec<Instr> {
    registry.block = Some(&block);

    let info = &mut registry.backend;

    let mut reg_vars = 0;
    let mut stack_off = 0;
    let mut var_index = 0;

    for (_, meta) in &func.ty.args {
        let mut var = Var(&mut block.clone(), meta.to_owned());
        var.name = format!("%{}", var_index);

        info.insertVar(var, {
            if reg_vars >= call.regArgs() {
                let addend = match meta {
                    TypeMetadata::u16 | TypeMetadata::i16=> 2,
                    TypeMetadata::u32 | TypeMetadata::i32=> 4,
                    TypeMetadata::u64 | TypeMetadata::i64=> 8,
                    TypeMetadata::Void => continue,
                };

                stack_off += addend;
                VarStorage::Memory(x64Reg::Rbp - (stack_off - addend))
            } else {
                reg_vars += 1;
                VarStorage::Register( match meta {
                    TypeMetadata::u16 | TypeMetadata::i16 => call.args16()[reg_vars - 1].boxed(),
                    TypeMetadata::u32 | TypeMetadata::i32 => call.args32()[reg_vars - 1].boxed(),
                    TypeMetadata::u64 | TypeMetadata::i64 => call.args64()[reg_vars - 1].boxed(),
                    TypeMetadata::Void => continue,
                })
            }
        });

        var_index += 1;
    }

    if reg_vars <= call.regArgs() {
        info.dropReg(call.args64()[reg_vars].boxed());        
    }

    let mut out: VecDeque<Instr> = VecDeque::new();

    for node in &block.nodes {
        let compiled = node.compile(registry);

        out.extend(compiled);
    }



    registry.block = None;

    let mut prolog = x64BuildProlog(&block, registry);
    prolog.reverse(); // cuz: push_front

    for epAsm in prolog {
        out.push_front(epAsm);
    }

    out.extend(x64BuildEpilog(&block, registry));

    Vec::from(out)
}

pub(crate) fn BlockX86FuncisVarUsedAfterNode(block: &Block, startingNode: &Box<dyn Ir>, var: &Var) -> bool {
    let mut used = false;
    let mut started = false;

    for node in &block.nodes {
        if node.uses(var) && started {
            used = true;
        }

        if node.is(startingNode) {
            started = true;
        }
    }

    used
}