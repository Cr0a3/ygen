use std::{collections::{HashMap, VecDeque}, fmt::Display, sync::Mutex};

use lazy_static::lazy_static;

use crate::prelude::{ir::*, Type, Var};

use super::{Arch, CallConv};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BackendInfos {
    pub(crate) varsStorage: HashMap<Var, VarStorage>,
    pub(crate) currStackOffsetForLocalVars: isize,
    pub(crate) openUsableRegisters: VecDeque<String>,
}

impl BackendInfos {
    pub(crate) fn new() -> Self {
        Self {
            varsStorage: HashMap::new(),
            currStackOffsetForLocalVars: 0,
            openUsableRegisters: VecDeque::new(),
        }
    }

    pub(crate) fn insertVar(&mut self, var: Var, store: VarStorage) {
        self.varsStorage.insert(var, store);
    }

    pub(crate) fn getOpenReg(&mut self) -> Option<String> {
        self.openUsableRegisters.pop_front()
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum VarStorage {
    Register(String),
    Memory(String),
}

impl Display for VarStorage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            VarStorage::Memory(mem) => mem,
            VarStorage::Register(reg) => reg,
        })
    }
}

pub(crate) type CompileFunc<T> = fn(&T/*, &mut BackendInfos*/) -> Vec<String>;

/// The Target Registry: stores if a target was already initialized
#[derive(Debug, Clone)]
pub struct TargetRegistry {
    pub(crate) inited_targets: Vec<Arch>,
    funcForRetType: HashMap<Arch, CompileFunc<Return<Type>>>,
    funcForRetVar: HashMap<Arch, CompileFunc<Return<Var>>>,
    funcForConstAssign: HashMap<Arch, CompileFunc<ConstAssign<Var, Type>>>,
    funcForAddVarVar: HashMap<Arch, CompileFunc<Add<Var, Var, Var>>>,
    funcForAddTypeType: HashMap<Arch, CompileFunc<Add<Type, Type, Var>>>,
    pub(crate) backend: BackendInfos,
    pub(crate) call: CallConv,
}

lazy_static! {
    pub(crate) static ref TARGETS: Mutex<TargetRegistry> = Mutex::new( TargetRegistry::new() );
}

impl TargetRegistry {
    /// Creates a new instance
    pub fn new() -> Self {
        Self {
            inited_targets: vec![],
            funcForRetType: HashMap::new(),
            funcForRetVar: HashMap::new(),
            funcForConstAssign: HashMap::new(),
            funcForAddVarVar: HashMap::new(),
            funcForAddTypeType: HashMap::new(),
            call: CallConv::SystemV,
            backend: BackendInfos::new(),
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

    /// sets the callback for compiling the const assign node into asm
    pub(crate) fn setCompileFuncForConstAssign(&mut self, arch: Arch, callback: CompileFunc<ConstAssign<Var, Type>>) {
        if !self.funcForConstAssign.contains_key(&arch) {
            self.funcForConstAssign.insert(arch, callback);
        }
    }

    /// gets the callback for compiling the const assign into asm
    pub(crate) fn getCompileFuncForConstAssign(&self) -> CompileFunc<ConstAssign<Var, Type>> {
        if let Some(last_arch) = self.inited_targets.last() {
            if let Some(func) = self.funcForConstAssign.get(last_arch) {
                *func
            } else { todo!() }
        } else { todo!()}
    }

    /// sets the callback for compiling the add var var ir node into asm
    pub(crate) fn setCompileFuncForAddVarVar(&mut self, arch: Arch, callback: CompileFunc<Add<Var, Var, Var>>) {
        if !self.funcForAddVarVar.contains_key(&arch) {
            self.funcForAddVarVar.insert(arch, callback);
        }
    }

    /// gets the callback for compiling the add var var node into into asm
    pub(crate) fn getCompileFuncForAddVarVar(&self) -> CompileFunc<Add<Var, Var, Var>> {
        if let Some(last_arch) = self.inited_targets.last() {
            if let Some(func) = self.funcForAddVarVar.get(last_arch) {
                *func
            } else { todo!() }
        } else { todo!()}
    }

    /// sets the callback for compiling the add var var ir node into asm
    pub(crate) fn setCompileFuncForAddTypeType(&mut self, arch: Arch, callback: CompileFunc<Add<Type, Type, Var>>) {
        if !self.funcForAddTypeType.contains_key(&arch) {
            self.funcForAddTypeType.insert(arch, callback);
        }
    }

    /// gets the callback for compiling the add var var node into into asm
    pub(crate) fn getCompileFuncForAddTypeType(&self) -> CompileFunc<Add<Type, Type, Var>> {
        if let Some(last_arch) = self.inited_targets.last() {
            if let Some(func) = self.funcForAddTypeType.get(last_arch) {
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