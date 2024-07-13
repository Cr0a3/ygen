use std::collections::HashMap;

use crate::{prelude::{Block, Function, Type, TypeMetadata, Var}, Target::{registry::{BackendInfos, VarStorage}, TARGETS}, IR::ir::*};

use crate::Target::CallConv;

pub(crate) fn CompileAddVarVar(add: &Add<Var, Var, Var>) -> Vec<String> {
    let infos = &mut TARGETS.get().unwrap().lock().unwrap().backend;

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

    let ret: String = "rax".into();
    todo!("implement actual variable storage information");

    infos.insertVar(
        add.inner3.clone(), 
        VarStorage::Register(ret.clone())
    );

    if let VarStorage::Register(_) = loc1 {
        if let VarStorage::Register(_) = loc2 {
            return vec![format!("lea {}, [{} + {}", ret, op0, op1)];
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

pub(crate) fn CompileRetType(ret: &Return<Type>) -> Vec<String> {
    vec![format!("mov {}, {}", match ret.inner1 {
        Type::u16(_) | Type::i16(_) => TARGETS.get().unwrap().lock().unwrap().call.ret16(),
        Type::u32(_) | Type::i32(_) => TARGETS.get().unwrap().lock().unwrap().call.ret32(),
        Type::u64(_) | Type::i64(_) => TARGETS.get().unwrap().lock().unwrap().call.ret64(),
        Type::Void => todo!(), 
    }, ret.inner1.val())]
}


pub(crate) fn CompileRetVar(ret: &Return<Var>) -> Vec<String> {
    let target = TARGETS.get().unwrap().lock().unwrap();
    let (var, loc) = if let Some(loc) = target.backend.varsStorage.get_key_value(&ret.inner1) {
        loc.clone()
    } else {
        panic!("unknown variable: {:?}", ret.inner1)
    };

    if var.ty == TypeMetadata::Void {
        return vec![];
    }

    vec![format!("mov {}, {}", match var.ty {
        TypeMetadata::u16 | TypeMetadata::i16 => target.call.ret16(),
        TypeMetadata::u32 | TypeMetadata::i32 => target.call.ret32(),
        TypeMetadata::u64 | TypeMetadata::i64=> target.call.ret64(),
        _ => unreachable!(),
    }, {
        if let VarStorage::Memory(mem) = loc { mem }
        else if let VarStorage::Register(reg) = loc { reg }
        else { unreachable!() }
    })]
}
impl Block {
    /// Builds the block to x86 assembly intel syntax
    pub fn buildAsmX86(&self, func: &Function, call: &CallConv) -> Vec<String> {
        let mut info = BackendInfos { varsStorage: HashMap::new() };

        let mut reg_vars = 0;
        let mut stack_off = 0;

        for (_, meta) in &func.ty.args {
            info.insertVar(Var(&mut self.clone(), *meta), {
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
                    VarStorage::Register(format!("{}", match meta {
                        TypeMetadata::u16 | TypeMetadata::i16 => call.args16()[reg_vars - 1].clone(),
                        TypeMetadata::u32 | TypeMetadata::i32 => call.args32()[reg_vars - 1].clone(),
                        TypeMetadata::u64 | TypeMetadata::i64 => call.args64()[reg_vars - 1].clone(),
                        TypeMetadata::Void => continue,
                    }))
                }
            });
        }

        for node in &self.nodes {
            node.compile();
        }

        vec![]
    }
}