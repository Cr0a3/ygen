use crate::{prelude::{Block, Function, Type, TypeMetadata, Var}, Target::{registry::VarStorage, TARGETS}, IR::ir::*};

use crate::Target::CallConv;

pub(crate) fn CompileAddVarVar(add: &Add<Var, Var, Var>) -> Vec<String> {
    let infos = &mut TARGETS.lock().unwrap().backend;

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

    let ty = &add.inner1.ty;
    
    let ret = {
        if let Some(reg) = infos.getOpenReg() {
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
                return vec![format!("lea {}, [{} + {}", reg, op0, op1)];
            } else if let VarStorage::Memory(mem) = &ret {
                return vec![
                    format!("lea rax, [{} + {}", op0, op1),
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

pub(crate) fn CompileConstAssign(assign: &ConstAssign<Var, Type>) -> Vec<String> {
    let infos = &mut TARGETS.lock().unwrap().backend;

    let ty = &assign.inner1.ty;
    
    let store = {
        if let Some(reg) = infos.getOpenReg() {
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

pub(crate) fn CompileAddTyTy(add: &Add<Type, Type, Var>) -> Vec<String> {
    let val = add.inner1.val() + add.inner2.val();
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
    }))
}

pub(crate) fn CompileRetType(ret: &Return<Type>) -> Vec<String> {
    vec![format!("mov {}, {}", match ret.inner1 {
        Type::u16(_) | Type::i16(_) => TARGETS.lock().unwrap().call.ret16(),
        Type::u32(_) | Type::i32(_) => TARGETS.lock().unwrap().call.ret32(),
        Type::u64(_) | Type::i64(_) => TARGETS.lock().unwrap().call.ret64(),
        Type::Void => todo!(), 
    }, ret.inner1.val())]
}


pub(crate) fn CompileRetVar(ret: &Return<Var>) -> Vec<String> {
    let target = &TARGETS.lock().unwrap();
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
        let info = &mut TARGETS.lock().unwrap().backend;

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

        let mut out = vec![];

        for node in &self.nodes {
            out.extend_from_slice(
                &node.compile()
            );
        }

        out
    }
}