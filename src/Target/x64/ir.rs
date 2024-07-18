use std::collections::VecDeque;

use crate::prelude::{Block, Function, Type, TypeMetadata, Var};
use crate::Target::target_descr::{TargetBackendDescr, VarStorage};
use crate::Target::Reg;
use crate::IR::ir::*;

use crate::Target::CallConv;

pub(crate) fn CompileAddVarVar(add: &Add<Var, Var, Var>, registry: &mut TargetBackendDescr) -> Vec<String> {
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

    let op0 = if let VarStorage::Register(ref reg) = loc1 {
        reg.to_string()
    } else if let VarStorage::Memory(ref mem) = loc1 {
        mem.to_string()
    } else { panic!() };

    let op1 = if let VarStorage::Register(ref reg) = loc2 {
        reg.to_string()
    } else if let VarStorage::Memory(ref mem) = loc2 {
        mem.to_string()
    } else { panic!() };


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
            VarStorage::Memory(format!("[rbp - {}]", infos.currStackOffsetForLocalVars - addend))
        }
    };

    infos.insertVar(
        add.inner3.clone(), 
        ret.clone()
    );

    if let VarStorage::Register(_) = loc1 {
        if let VarStorage::Register(_) = loc2 {
            if let VarStorage::Register(reg) = &ret {
                return vec![format!("lea {}, [{} + {}]", reg, op0, op1)];
            } else if let VarStorage::Memory(mem) = &ret {
                return vec![
                    format!("lea rax, [{} + {}]", op0, op1),
                    format!("mov rax, {}", mem)
                    ];
            } else { todo!() }
        }
    }

    if let VarStorage::Memory(_) = loc1 {
        if let VarStorage::Memory(_) = loc2 {
            return vec![
                format!("mov rax, {}", op0),
                format!("mov rbx, {}", op1),
                format!("add rax, rbx"),
                format!("mov rax, {}", ret),
            ];
        }
    }

    vec![]
}

pub(crate) fn CompileConstAssign(assign: &ConstAssign<Var, Type>, registry: &mut TargetBackendDescr) -> Vec<String> {
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
            VarStorage::Memory(format!("[rbp - {}]", infos.currStackOffsetForLocalVars - addend))
        }
    };

    infos.insertVar(
        assign.inner1.clone(), 
        store.clone()
    );

    if let VarStorage::Register(reg) = &store {
        vec![format!("mov {}, {}", reg, assign.inner2.val())]
    } else if let VarStorage::Memory(mem) = &store {
        vec![format!("mov {}, {}", mem, assign.inner2.val())]
    } else { todo!() }
}

pub(crate) fn CompileAddTyTy(add: &Add<Type, Type, Var>, registry: &mut TargetBackendDescr) -> Vec<String> {
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

pub(crate) fn CompileRetType(ret: &Return<Type>, registry: &mut TargetBackendDescr) -> Vec<String> {
    if ret.inner1 != Type::Void {
        vec![format!("mov {}, {}", match ret.inner1 {
            Type::u16(_) | Type::i16(_) => registry.call.ret16(),
            Type::u32(_) | Type::i32(_) => registry.call.ret32(),
            Type::u64(_) | Type::i64(_) => registry.call.ret64(),
            Type::Void => todo!(), 
        }, ret.inner1.val())]
    } else {
        vec![]
    }
}


pub(crate) fn CompileRetVar(ret: &Return<Var>, registry: &mut TargetBackendDescr) -> Vec<String> {
    let (var, loc) = if let Some(loc) = registry.backend.varsStorage.get_key_value(&ret.inner1) {
        loc.clone()
    } else {
        panic!("unknown variable: {:?}", ret.inner1)
    };

    if var.ty == TypeMetadata::Void {
        return vec![];
    }

    vec![format!("mov {}, {}", match var.ty {
        TypeMetadata::u16 | TypeMetadata::i16 => registry.call.ret16(),
        TypeMetadata::u32 | TypeMetadata::i32 => registry.call.ret32(),
        TypeMetadata::u64 | TypeMetadata::i64=> registry.call.ret64(),
        _ => unreachable!(),
    }, {
        if let VarStorage::Memory(mem) = loc { mem.to_string() }
        else if let VarStorage::Register(reg) = loc { reg.to_string() }
        else { unreachable!() }
    })]
}

pub(crate) fn x64BuildProlog(_: &Block, registry: &mut TargetBackendDescr) -> Vec<String> {
    let mut res = vec![];

    if registry.backend.currStackOffsetForLocalVars != 0 {
        res.push(format!("push rbp"));
        res.push(format!("mov rbp, rsp"));
        res.push(format!("sub rsp, 16"));
    }

    for backuped in &registry.backend.saveRegister {
        res.push( format!("push {}", backuped) )
    }

    res
}

pub(crate) fn x64BuildEpilog(_: &Block, registry: &mut TargetBackendDescr) -> Vec<String> {
    let mut res = vec![];

    for backuped in &registry.backend.saveRegister {
        res.push( format!("pop {}", backuped) )
    }

    if registry.backend.currStackOffsetForLocalVars != 0 {
        res.push(format!("add rsp, 16"));
        res.push(format!("pop rbp"));
    }

    res.push(format!("ret"));

    res
}

pub(crate) fn buildAsmX86<'a>(block: &'a Block, func: &Function, call: &CallConv, registry: &mut TargetBackendDescr<'a>) -> Vec<String> {
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
                VarStorage::Memory(format!("[rbp - {}]", stack_off - addend))
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

    let mut out = VecDeque::new();

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