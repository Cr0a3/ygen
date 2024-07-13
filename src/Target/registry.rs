use std::{collections::HashMap, sync::Mutex};

use once_cell::sync::OnceCell;

use crate::prelude::{Return, Type, Var};

use super::{Arch, CallConv};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BackendInfos {
    pub(crate) varsStorage: HashMap<Var, VarStorage>,
}

impl BackendInfos {
    pub(crate) fn insertVar(&mut self, var: Var, store: VarStorage) {
        self.varsStorage.insert(var, store);
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum VarStorage {
    Register(String),
    Memory(String),
}
pub(crate) type CompileFunc<T> = fn(&T/*, &mut BackendInfos*/) -> Vec<String>;

/// The Target Registry: stores if a target was already initialized
#[derive(Debug, Clone)]
pub struct TargetRegistry {
    pub(crate) inited_targets: Vec<Arch>,
    funcForRetType: HashMap<Arch, CompileFunc<Return<Type>>>,
    funcForRetVar: HashMap<Arch, CompileFunc<Return<Var>>>,
    pub(crate) backend: BackendInfos,
    pub(crate) call: CallConv,
}

pub(crate) static TARGETS: OnceCell<Mutex<TargetRegistry>> = OnceCell::new();

impl TargetRegistry {
    /// Creates a new instance
    pub fn new() -> Self {
        Self {
            inited_targets: vec![],
            funcForRetType: HashMap::new(),
            funcForRetVar: HashMap::new(),
            call: CallConv::SystemV,
            backend: BackendInfos { varsStorage: HashMap::new() },
        }
    }

    /// sets the callback for compiling the return ir node into asm
    pub(crate) fn setCompileFuncForRetType(&mut self, arch: Arch, callback: CompileFunc<Return<Type>>) {
        if !self.funcForRetType.contains_key(&arch) {
            self.funcForRetType.insert(arch, callback);
        }
    }

    /// gets the callback for compiling the return ir node into asm
    pub(crate) fn getCompileFuncRetType(&self) -> CompileFunc<Return<Type>> {
        if let Some(last_arch) = self.inited_targets.last() {
            if let Some(func) = self.funcForRetType.get(last_arch) {
                *func
            } else { todo!() }
        } else { todo!()}
    }

    

    /// sets the callback for compiling the return ir node into asm
    pub(crate) fn setCompileFuncForRetVar(&mut self, arch: Arch, callback: CompileFunc<Return<Var>>) {
        if !self.funcForRetVar.contains_key(&arch) {
            self.funcForRetVar.insert(arch, callback);
        }
    }

    /// gets the callback for compiling the return ir node into asm
    pub(crate) fn getCompileFuncRetVar(&self) -> CompileFunc<Return<Var>> {
        if let Some(last_arch) = self.inited_targets.last() {
            if let Some(func) = self.funcForRetVar.get(last_arch) {
                *func
            } else { todo!() }
        } else { todo!()}
    }


    /// Sets an architecture as initialized
    pub fn set_inited(&mut self, arch: Arch) {
        if !self.inited_targets.contains(&arch) {
            self.inited_targets.push(arch);
        }
    }
}