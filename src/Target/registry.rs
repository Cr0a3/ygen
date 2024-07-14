use std::{collections::{HashMap, VecDeque}, fmt::Display};

use crate::prelude::{ir::*, Type, TypeMetadata, Var};

use super::{Arch, CallConv};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BackendInfos {
    pub(crate) varsStorage: HashMap<Var, VarStorage>,
    pub(crate) currStackOffsetForLocalVars: isize,
    pub(crate) openUsableRegisters64: VecDeque<String>,
    pub(crate) openUsableRegisters32: VecDeque<String>,
    pub(crate) openUsableRegisters16: VecDeque<String>,
    pub(crate) openUsableRegisters8: VecDeque<String>,
}

impl BackendInfos {
    pub(crate) fn new() -> Self {
        Self {
            varsStorage: HashMap::new(),
            currStackOffsetForLocalVars: 0,
            openUsableRegisters64: VecDeque::new(),
            openUsableRegisters32: VecDeque::new(),
            openUsableRegisters16: VecDeque::new(),
            openUsableRegisters8: VecDeque::new(),
        }
    }

    pub(crate) fn insertVar(&mut self, var: Var, store: VarStorage) {
        self.varsStorage.insert(var, store);
    }

    pub(crate) fn getOpenReg64(&mut self) -> Option<String> {
        self.openUsableRegisters32.pop_front(); // update all other members
        self.openUsableRegisters16.pop_front();
        self.openUsableRegisters8.pop_front();
        self.openUsableRegisters64.pop_front()
    }

    pub(crate) fn getOpenReg32(&mut self) -> Option<String> {
        self.openUsableRegisters64.pop_front(); // update all other members
        self.openUsableRegisters16.pop_front();
        self.openUsableRegisters8.pop_front();
        self.openUsableRegisters32.pop_front()
    }

    pub(crate) fn getOpenReg16(&mut self) -> Option<String> {
        self.openUsableRegisters64.pop_front(); // update all other members
        self.openUsableRegisters32.pop_front();
        self.openUsableRegisters8.pop_front();
        self.openUsableRegisters16.pop_front()
    }

    #[allow(dead_code)]
    pub(crate) fn getOpenReg8(&mut self) -> Option<String> {
        self.openUsableRegisters64.pop_front(); // update all other members
        self.openUsableRegisters32.pop_front();
        self.openUsableRegisters16.pop_front();
        self.openUsableRegisters8.pop_front()
    }

    pub(crate) fn getOpenRegBasedOnTy(&mut self, ty: TypeMetadata) -> Option<String> {
        match ty {
            TypeMetadata::u16 | TypeMetadata::i16 => self.getOpenReg16(),
            TypeMetadata::u32 | TypeMetadata::i32 => self.getOpenReg32(),
            TypeMetadata::u64 | TypeMetadata::i64 => self.getOpenReg64(),
            TypeMetadata::Void => todo!("cannot use void as a register variable type. consider removing it"),
        }
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

pub(crate) type CompileFunc<T> = fn(&T, &mut TargetBackendDescr) -> Vec<String>;

/// The TargetBackendDescr is used to store all the functions/information to compile ir nodes into assembly
#[derive(Debug, Clone)]
pub struct TargetBackendDescr {
    funcForRetType: Option<CompileFunc<Return<Type>>>,
    funcForRetVar: Option<CompileFunc<Return<Var>>>,
    funcForConstAssign: Option<CompileFunc<ConstAssign<Var, Type>>>,
    funcForAddVarVar: Option<CompileFunc<Add<Var, Var, Var>>>,
    funcForAddTypeType: Option<CompileFunc<Add<Type, Type, Var>>>,
    pub(crate) backend: BackendInfos,
    pub(crate) call: CallConv,
}

impl TargetBackendDescr {
    /// Creates a new instance
    pub fn new() -> Self {
        Self {
            funcForRetType: None,
            funcForRetVar: None,
            funcForConstAssign: None,
            funcForAddVarVar: None,
            funcForAddTypeType: None,

            call: CallConv::SystemV,
            backend: BackendInfos::new(),
        }
    }

    /// sets the callback for compiling the return ir node into asm
    pub(crate) fn setCompileFuncForRetType(&mut self, callback: CompileFunc<Return<Type>>) {
        self.funcForRetType = Some(callback);
    }

    /// gets the callback for compiling the return ir node into asm
    pub(crate) fn getCompileFuncRetType(&self) -> CompileFunc<Return<Type>> {
        if let Some(func) = self.funcForRetType {
            func
        } else { todo!("an corresponding assembly handler needs to be registered in order to compile an ReturnType ir node")}
    }

    /// sets the callback for compiling the return ir node into asm
    pub(crate) fn setCompileFuncForRetVar(&mut self, callback: CompileFunc<Return<Var>>) {
        self.funcForRetVar = Some(callback)
    }

    /// gets the callback for compiling the return ir node into asm
    pub(crate) fn getCompileFuncForRetVar(&self) -> CompileFunc<Return<Var>> {
        if let Some(func) = self.funcForRetVar {
            func
        } else { todo!("an corresponding assembly handler needs to be registered in order to compile an ReturnVar ir node")}
    }

    /// sets the callback for compiling the const assign node into asm
    pub(crate) fn setCompileFuncForConstAssign(&mut self, callback: CompileFunc<ConstAssign<Var, Type>>) {
        self.funcForConstAssign = Some(callback);
    }

    /// gets the callback for compiling the const assign into asm
    pub(crate) fn getCompileFuncForConstAssign(&self) -> CompileFunc<ConstAssign<Var, Type>> {
        if let Some(func) = self.funcForConstAssign {
            func
        } else { todo!("an corresponding assembly handler needs to be registered in order to compile an ConstAssign ir node")}
    }

    /// sets the callback for compiling the add var var ir node into asm
    pub(crate) fn setCompileFuncForAddVarVar(&mut self, callback: CompileFunc<Add<Var, Var, Var>>) {
        self.funcForAddVarVar = Some(callback);
    }

    /// gets the callback for compiling the add var var node into into asm
    pub(crate) fn getCompileFuncForAddVarVar(&self) -> CompileFunc<Add<Var, Var, Var>> {
        if let Some(func) = self.funcForAddVarVar {
            func
        } else { todo!("an corresponding assembly handler needs to be registered in order to compile an AddVarVar ir node")}
    }

    /// sets the callback for compiling the add var var ir node into asm
    pub(crate) fn setCompileFuncForAddTypeType(&mut self, callback: CompileFunc<Add<Type, Type, Var>>) {
        self.funcForAddTypeType = Some(callback);
    }

    /// gets the callback for compiling the add var var node into into asm
    pub(crate) fn getCompileFuncForAddTypeType(&self) -> CompileFunc<Add<Type, Type, Var>> {
        if let Some(func) = self.funcForAddTypeType {
            func
        } else { todo!("an corresponding assembly handler needs to be registered in order to compile an AddTypeType ir node")}
    }
}

/// The target registry: is just a big HashMap of `Arch` and `TargetBackendDescr`
#[derive(Debug, Clone)]
pub struct TargetRegistry {
    map: HashMap<Arch, TargetBackendDescr>
}

impl TargetRegistry {
    /// Creates an new target registry
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn setInited(&mut self, arch: Arch, descr: TargetBackendDescr) {
        self.map.insert(arch, descr);
    }

    #[allow(dead_code)]
    pub(crate) fn getDescr(&mut self, arch: Arch) -> Option<&TargetBackendDescr> {
        self.map.get(&arch)
    }
}