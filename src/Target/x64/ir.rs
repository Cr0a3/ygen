use std::{any::Any, collections::HashMap};

use crate::{prelude::{Block, Function, TypeMetadata, Var}, IR::ir::*};

use crate::Target::CallConv;

/// Stores compilation infos for ir node compilation
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct x64CompilationInfos {
    pub(crate) varsStorage: HashMap<Var, VarStorage>,
}

impl x64CompilationInfos {
    pub(crate) fn insertVar(&mut self, var: Var, store: VarStorage) {
        self.varsStorage.insert(var, store);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum VarStorage {
    Register(String),
    Memory(String),
}

/// A trait which is used to implement compilability for ir nodes
pub(crate) trait Compile: Ir {
    /// Compiles the node into an asm string
    fn compile(&self, infos: &mut x64CompilationInfos) -> Vec<String>;
}

impl Compile for Add<Var, Var, Var> {
    fn compile(&self, infos: &mut x64CompilationInfos) -> Vec<String> {
        let loc1 = if let Some(loc1) = infos.varsStorage.get(&self.inner1) {
            loc1.clone()
        } else {
            panic!("unknown variable: {:?}", self.inner1)
        };
        
        let loc2 = if let Some(loc2) = infos.varsStorage.get(&self.inner2) {
            loc2.clone()
            
        } else {
            panic!("unknown variable: {:?}", self.inner1)
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

        infos.insertVar(
            self.inner3.clone(), 
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
}

impl Block {
    /// Builds the block to x86 assembly intel syntax
    pub fn buildAsmX86(&self, func: &Function, call: &CallConv) -> Vec<String> {
        let mut info = x64CompilationInfos { varsStorage: HashMap::new() };

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
            let ty = (node.as_any()).downcast_ref::<Box<dyn Compile>>().unwrap();
            ty.compile(&mut info);
        }

        vec![]
    }
}