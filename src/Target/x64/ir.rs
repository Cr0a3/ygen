use std::collections::VecDeque;

use crate::prelude::{Block, Function, Type, TypeMetadata, Var};
use crate::Target::target_descr::{TargetBackendDescr, VarStorage};
use crate::Target::Reg;
use crate::IR::ir::*;
use super::Optimize;

use crate::Target::CallConv;

use super::{x64Reg, instr::*};

macro_rules! CompileMathVarVar {
    ($name:ident, $node:ident, $mnemonic:expr) => {
        pub(crate) fn $name(node: &$node<Var, Var, Var>, registry: &mut TargetBackendDescr) -> Vec<Instr> {
            let infos = &mut registry.backend;

            let loc1 = if let Some(loc1) = infos.varsStorage.get(&node.inner1) {
                loc1.clone()
            } else {
                panic!("unknown variable: {:?}", node.inner1)
            };
            
            let loc2 = if let Some(loc2) = infos.varsStorage.get(&node.inner2) {
                loc2.clone()
                
            } else {
                panic!("unknown variable: {:?}", node.inner1)
            };

            let boxed: Box<dyn Ir> = Box::new(node.clone());

            if !BlockX86FuncisVarUsedAfterNode(registry.block.unwrap(), &boxed, &node.inner1) {
                infos.drop(&node.inner1);
            }
            if !BlockX86FuncisVarUsedAfterNode(registry.block.unwrap(), &boxed, &node.inner2) {
                infos.drop(&node.inner2);
            }
            if !BlockX86FuncisVarUsedAfterNode(registry.block.unwrap(), &boxed, &node.inner3) {
                return vec![]; // all of these calculations don't need to be done: dead code removal
            }

            let ty = &node.inner1.ty;
    
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
                node.inner3.clone(), 
                ret.clone()
            );
            let tmp = infos.getTmpBasedOnTy(*ty);

            if let VarStorage::Register(loc1Reg) = &loc1 {
                if let VarStorage::Register(loc2Reg) = &loc2 {
                    if let VarStorage::Register(reg) = &ret {
                        return vec![
                            Instr::with2(Mnemonic::Mov, Operand::Reg(tmp.boxed()), Operand::Reg(loc2Reg.boxed())),
                            Instr::with2($mnemonic, Operand::Reg(tmp.boxed()), Operand::Reg(loc1Reg.boxed())),
                            Instr::with2(Mnemonic::Mov, Operand::Reg(reg.boxed()), Operand::Reg(tmp.boxed())),
                        ]
                    } else if let VarStorage::Memory(mem) = &ret {
                        return vec![
                            Instr::with2(Mnemonic::Mov, Operand::Reg(tmp.boxed()), Operand::Reg(loc2Reg.boxed())),
                            Instr::with2($mnemonic, Operand::Reg(tmp.boxed()), Operand::Reg(loc1Reg.boxed())),
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
                            Instr::with2($mnemonic, Operand::Reg(tmp.boxed()), Operand::Mem(mem2.clone())),
                            Instr::with2(Mnemonic::Mov, Operand::Reg(reg.boxed()), Operand::Reg(tmp.boxed())),
                        ];
                    } else if let VarStorage::Memory(mem) = &ret {
                        return vec![
                            Instr::with2(Mnemonic::Mov, Operand::Reg(tmp.boxed()), Operand::Mem(mem1.clone())),
                            Instr::with2($mnemonic, Operand::Reg(tmp.boxed()), Operand::Mem(mem2.clone())),
                            Instr::with2(Mnemonic::Mov, Operand::Mem(mem.clone()), Operand::Reg(tmp.boxed())),
                        ];
                    } else { todo!() }
                }
            }

            todo!(); // nothing was compiled
        }
    };
}

macro_rules! CompileMathVarType {
    ($name:ident, $node:ident, $mnemonic:expr) => {
        pub(crate) fn $name(node: &$node<Var, Type, Var>, registry: &mut TargetBackendDescr) -> Vec<Instr> {
            let infos = &mut registry.backend;

            let loc1 = if let Some(loc1) = infos.varsStorage.get(&node.inner1) {
                loc1.clone()
            } else {
                panic!("unknown variable: {:?}", node.inner1)
            };

            let boxed: Box<dyn Ir> = Box::new(node.clone());

            if !BlockX86FuncisVarUsedAfterNode(registry.block.unwrap(), &boxed, &node.inner1) {
                infos.drop(&node.inner1);
            }
            if !BlockX86FuncisVarUsedAfterNode(registry.block.unwrap(), &boxed, &node.inner3) {
                return vec![]; // all of these calculations don't need to be done: dead code removal
            }

            let ty = &node.inner1.ty;
    
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
                node.inner3.clone(), 
                ret.clone()
            );
            let tmp = infos.getTmpBasedOnTy(*ty);

            if let VarStorage::Register(loc1Reg) = &loc1 {
                if let VarStorage::Register(reg) = &ret {
                    return vec![
                        Instr::with2(Mnemonic::Mov, Operand::Reg(tmp.boxed()), Operand::Imm(node.inner2.val() as i64)),
                        Instr::with2($mnemonic, Operand::Reg(tmp.boxed()), Operand::Reg(loc1Reg.boxed())),
                        Instr::with2(Mnemonic::Mov, Operand::Reg(reg.boxed()), Operand::Reg(tmp.boxed())),
                    ]
                } else if let VarStorage::Memory(mem) = &ret {
                    return vec![
                        Instr::with2(Mnemonic::Mov, Operand::Reg(tmp.boxed()), Operand::Imm(node.inner2.val() as i64)),
                        Instr::with2($mnemonic, Operand::Reg(tmp.boxed()), Operand::Reg(loc1Reg.boxed())),
                        Instr::with2(Mnemonic::Mov, Operand::Mem(mem.clone()), Operand::Reg(tmp.boxed())),
                        ];
                } else { todo!() }
            }

            if let VarStorage::Memory(mem1) = loc1 {
                if let VarStorage::Register(reg) = &ret {
                    return vec![
                        Instr::with2(Mnemonic::Mov, Operand::Reg(tmp.boxed()), Operand::Mem(mem1.clone())),
                        Instr::with2($mnemonic, Operand::Reg(tmp.boxed()), Operand::Imm(node.inner2.val() as i64)),
                        Instr::with2(Mnemonic::Mov, Operand::Reg(reg.boxed()), Operand::Reg(tmp.boxed())),
                    ];
                } else if let VarStorage::Memory(mem) = &ret {
                    return vec![
                        Instr::with2(Mnemonic::Mov, Operand::Reg(tmp.boxed()), Operand::Mem(mem1.clone())),
                        Instr::with2($mnemonic, Operand::Reg(tmp.boxed()), Operand::Imm(node.inner2.val() as i64)),
                        Instr::with2(Mnemonic::Mov, Operand::Mem(mem.clone()), Operand::Reg(tmp.boxed())),
                    ];
                } else { todo!() }
            }

            todo!(); // nothing was compiled
        }
    };
}

macro_rules! CompileMathTyTy {
    ($name:ident, $node:ident, $op:tt) => {
        pub(crate) fn $name(node: &$node<Type, Type, Var>, registry: &mut TargetBackendDescr) -> Vec<Instr> {
            let val = node.inner1.val() $op node.inner2.val();
        
            let boxed: Box<dyn Ir> = Box::new(node.clone());
        
            if !BlockX86FuncisVarUsedAfterNode(registry.block.unwrap(), &boxed, &node.inner3) {
                return vec![]; // all of these calculations don't need to be done: dead code removal
            }
        
            CompileConstAssign(&ConstAssign::new(node.inner3.clone(), {
                match node.inner3.ty {
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
    }
}

CompileMathVarVar!(CompileAddVarVar, Add, Mnemonic::Add);
CompileMathVarVar!(CompileSubVarVar, Sub, Mnemonic::Sub);
CompileMathVarVar!(CompileXorVarVar, Xor, Mnemonic::Xor);
CompileMathVarVar!(CompileOrVarVar, Or, Mnemonic::Or);
CompileMathVarVar!(CompileAndVarVar, And, Mnemonic::And);

CompileMathVarType!(CompileAddVarTy, Add, Mnemonic::Add);
CompileMathVarType!(CompileSubVarTy, Sub, Mnemonic::Sub);
CompileMathVarType!(CompileXorVarTy, Xor, Mnemonic::Xor);
CompileMathVarType!(CompileOrVarTy, Or, Mnemonic::Or);
CompileMathVarType!(CompileAndVarTy, And, Mnemonic::And);

CompileMathTyTy!(CompileAddTyTy, Add, +);
CompileMathTyTy!(CompileSubTyTy, Sub, -);
CompileMathTyTy!(CompileXorTyTy, Xor, ^);
CompileMathTyTy!(CompileOrTyTy, Or, |);
CompileMathTyTy!(CompileAndTyTy, And, &);

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

pub(crate) fn CompileCast(cast: &Cast<Var, TypeMetadata, Var>, registry: &mut TargetBackendDescr) -> Vec<Instr> {
    let boxed: Box<dyn Ir> = Box::new(cast.clone());

    let loc = if let Some(loc) = registry.backend.varsStorage.get(&cast.inner1) {
        loc.clone()      
    } else {
        panic!("unknown variable: {:?}", cast.inner1)
    };

    if !BlockX86FuncisVarUsedAfterNode(registry.block.unwrap(), &boxed, &cast.inner1) {
        registry.backend.drop(&cast.inner1);
    } 
    if !BlockX86FuncisVarUsedAfterNode(registry.block.unwrap(), &boxed, &cast.inner3) {
        return vec![];
    } 

    let store = {
        if let Some(reg) = registry.backend.getOpenRegBasedOnTy(cast.inner2) {
            VarStorage::Register(reg)
        } else {
            let addend = match cast.inner2 {
                TypeMetadata::u16 | TypeMetadata::i16=> 2,
                TypeMetadata::u32 | TypeMetadata::i32=> 4,
                TypeMetadata::u64 | TypeMetadata::i64=> 8,
                TypeMetadata::Void => todo!("cant cast into void"),
            };

            registry.backend.currStackOffsetForLocalVars += addend;
            VarStorage::Memory(x64Reg::Rbp - (registry.backend.currStackOffsetForLocalVars - addend) as u32)
        }
    };

    registry.backend.insertVar(
        cast.inner3.clone(), 
        store.clone()
    );

    match loc {
        VarStorage::Register(inbound) => {
            if let VarStorage::Register(outboud) = store {
                if cast.inner1.ty.bitSize() > cast.inner3.ty.bitSize() {
                    if inbound.is_gr16() || inbound.is_gr8() { // zero extend
                        return vec![
                            Instr::with2(Mnemonic::Movzx, Operand::Reg(outboud), Operand::Reg(inbound)),
                        ];
                    } else {
                        return vec![
                            Instr::with2(Mnemonic::Mov, Operand::Reg(outboud), Operand::Reg(inbound))
                        ];
                    }
                } else {
                    return vec![{
                        if inbound.is_gr64() {
                            Instr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::parse(outboud.sub64()).unwrap().boxed()), Operand::Reg(x64Reg::parse(inbound.sub64()).unwrap().boxed()))
                        } else if inbound.is_gr32() {
                            Instr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::parse(outboud.sub32()).unwrap().boxed()), Operand::Reg(x64Reg::parse(inbound.sub32()).unwrap().boxed()))
                        } else if inbound.is_gr16() {
                            Instr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::parse(outboud.sub16()).unwrap().boxed()), Operand::Reg(x64Reg::parse(inbound.sub16()).unwrap().boxed()))
                        } else if inbound.is_gr8() {
                            Instr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::parse(outboud.sub8()).unwrap().boxed()), Operand::Reg(x64Reg::parse(inbound.sub8()).unwrap().boxed()))
                        } else { panic!() }
                    }];
                }
            }
        },
        VarStorage::Memory(_) => todo!(),
    }

    vec![]
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
        println!("{:?}", node);
        println!("{:?}", compiled);

        out.extend(compiled);
    }



    registry.block = None;

    let mut prolog = x64BuildProlog(&block, registry);
    prolog.reverse(); // cuz: push_front

    for epAsm in prolog {
        out.push_front(epAsm);
    }

    out.extend(x64BuildEpilog(&block, registry));

    Vec::from(out).optimize()
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