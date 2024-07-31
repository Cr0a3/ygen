use std::{collections::{HashMap, VecDeque}, fmt::Display};
use core::fmt::Debug;

use crate::prelude::{ir::*, Block, Function, Type, TypeMetadata, Var};

use super::{x64Reg, CallConv, Compiler, Instr, Lexer, MemOp, Reg};

#[derive(Debug)]
pub(crate) struct BackendInfos {
    pub(crate) varsStorage: HashMap<Var, VarStorage>,
    pub(crate) currStackOffsetForLocalVars: isize,
    pub(crate) openUsableRegisters64: VecDeque<Box<dyn Reg>>,
    pub(crate) openUsableRegisters32: VecDeque<Box<dyn Reg>>,
    pub(crate) openUsableRegisters16: VecDeque<Box<dyn Reg>>,
    pub(crate) openUsableRegisters8: VecDeque<Box<dyn Reg>>,
    pub(crate) tmpReg: Box<dyn Reg>,
    pub(crate) saveRegister: Vec<Box<dyn Reg>>,
    pub(crate) savedRegisters: Vec<Box<dyn Reg>>,
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

            tmpReg: x64Reg::Rax.boxed(),

            saveRegister: vec![],
            savedRegisters: vec![],
        }
    }

    /// Delets the variable of the varsStorage (giving out it's resources)
    pub(crate) fn drop(&mut self, var: &Var) {
        if let Some(loc) = self.varsStorage.get(var) {
            if let VarStorage::Register(reg) = loc {
                match var.ty {
                    TypeMetadata::u16 | TypeMetadata::i16 => {
                        self.openUsableRegisters16.push_front(reg.boxed());
                        self.openUsableRegisters64.push_front(reg.from(reg.sub64()));
                        self.openUsableRegisters32.push_front(reg.from(reg.sub32()));
                        self.openUsableRegisters8.push_front(reg.from(reg.sub8()));
                    },
                    TypeMetadata::u32 | TypeMetadata::i32 => {
                        self.openUsableRegisters32.push_front(reg.boxed());
                        self.openUsableRegisters64.push_front(reg.from(reg.sub64()));
                        self.openUsableRegisters16.push_front(reg.from(reg.sub16()));
                        self.openUsableRegisters8.push_front(reg.from(reg.sub8()));
                    },
                    TypeMetadata::u64 | TypeMetadata::i64 => {
                        self.openUsableRegisters64.push_front(reg.boxed());
                        self.openUsableRegisters32.push_front(reg.from(reg.sub32()));
                        self.openUsableRegisters16.push_front(reg.from(reg.sub16()));
                        self.openUsableRegisters8.push_front(reg.from(reg.sub8()));
                    },
                    TypeMetadata::Void => todo!(),
                }
            }
        }
    }

    /// Adds the register back to the usable registers in the front
    pub(crate) fn dropReg(&mut self, reg: Box<dyn Reg>) {
        self.openUsableRegisters64.push_front(reg.from(reg.sub64()));
        self.openUsableRegisters32.push_front(reg.from(reg.sub32()));
        self.openUsableRegisters16.push_front(reg.from(reg.sub16()));
        self.openUsableRegisters8.push_front(reg.from(reg.sub8()));
    }

    pub(crate) fn insertVar(&mut self, var: Var, store: VarStorage) {
        self.varsStorage.insert(var, store);
    }

    pub(crate) fn getOpenReg64(&mut self) -> Option<Box<dyn Reg>> {
        self.openUsableRegisters32.pop_front(); // update all other members
        self.openUsableRegisters16.pop_front();
        self.openUsableRegisters8.pop_front();
        let reg = self.openUsableRegisters64.pop_front()?;

        if self.savedRegisters.contains(&reg) && !self.saveRegister.contains(&reg) {
            self.savedRegisters.push(reg.boxed());
        }


        Some(reg)
    }

    pub(crate) fn getOpenReg32(&mut self) -> Option<Box<dyn Reg>> {
        self.openUsableRegisters64.pop_front(); // update all other members
        self.openUsableRegisters16.pop_front();
        self.openUsableRegisters8.pop_front();
        let reg = self.openUsableRegisters32.pop_front()?;

        if self.savedRegisters.contains(&reg) && !self.saveRegister.contains(&reg) {
            self.savedRegisters.push(reg.boxed());
        }

        Some(reg)
    }

    pub(crate) fn getOpenReg16(&mut self) -> Option<Box<dyn Reg>> {
        self.openUsableRegisters64.pop_front(); // update all other members
        self.openUsableRegisters32.pop_front();
        self.openUsableRegisters8.pop_front();
        let reg = self.openUsableRegisters16.pop_front()?;

        if self.savedRegisters.contains(&reg) && !self.saveRegister.contains(&reg) {
            self.savedRegisters.push(reg.boxed());
        }


        Some(reg)
    }

    #[allow(dead_code)]
    pub(crate) fn getOpenReg8(&mut self) -> Option<Box<dyn Reg>> {
        self.openUsableRegisters64.pop_front(); // update all other members
        self.openUsableRegisters32.pop_front();
        self.openUsableRegisters16.pop_front();
        let reg = self.openUsableRegisters8.pop_front()?;

        if self.savedRegisters.contains(&reg) && !self.saveRegister.contains(&reg) {
            self.savedRegisters.push(reg.boxed());
        }


        Some(reg)
    }

    pub(crate) fn getOpenRegBasedOnTy(&mut self, ty: TypeMetadata) -> Option<Box<dyn Reg>> {
        match ty {
            TypeMetadata::u16 | TypeMetadata::i16 => self.getOpenReg16(),
            TypeMetadata::u32 | TypeMetadata::i32 => self.getOpenReg32(),
            TypeMetadata::u64 | TypeMetadata::i64 => self.getOpenReg64(),
            TypeMetadata::Void => todo!("cannot use void as a register variable type. consider removing it"),
        }
    }

    pub(crate) fn getTmpReg16(&mut self) -> Box<dyn Reg> {
        self.tmpReg.from( self.tmpReg.sub16() )
    }

    pub(crate) fn getTmpReg32(&mut self) -> Box<dyn Reg> {
        self.tmpReg.from( self.tmpReg.sub32() )
    }

    pub(crate) fn getTmpReg64(&mut self) -> Box<dyn Reg> {
        self.tmpReg.from( self.tmpReg.sub64() )
    }

    pub(crate) fn getTmpBasedOnTy(&mut self, ty: TypeMetadata) -> Box<dyn Reg> {
        match ty {
            TypeMetadata::u16 | TypeMetadata::i16 => self.getTmpReg16(),
            TypeMetadata::u32 | TypeMetadata::i32 => self.getTmpReg32(),
            TypeMetadata::u64 | TypeMetadata::i64 => self.getTmpReg64(),
            TypeMetadata::Void => todo!("cannot use void as a register variable type. consider removing it"),
        }
    }
}

#[derive(Debug)]
pub(crate) enum VarStorage {
    Register(Box<dyn Reg>),
    Memory(MemOp),
}

impl Clone for VarStorage {
    fn clone(&self) -> Self {
        match self {
            Self::Register(ref arg0) => Self::Register(arg0.boxed()),
            Self::Memory(ref arg0) => Self::Memory(arg0.clone()),
        }
    }
}

impl Display for VarStorage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            VarStorage::Memory(mem) => mem.to_string(),
            VarStorage::Register(reg) => format!("{}", reg),
        })
    }
}

pub(crate) type CompileFunc<T> = fn(&T, &mut TargetBackendDescr) -> Vec<Instr>;

/// The TargetBackendDescr is used to store all the functions/information to compile ir nodes into assembly
pub struct TargetBackendDescr<'a> {
    funcForRetType: Option<CompileFunc<Return<Type>>>,
    funcForRetVar: Option<CompileFunc<Return<Var>>>,
    funcForConstAssign: Option<CompileFunc<ConstAssign<Var, Type>>>,

    funcForAddVarVar: Option<CompileFunc<Add<Var, Var, Var>>>,
    funcForAddTypeType: Option<CompileFunc<Add<Type, Type, Var>>>,
    
    funcForSubVarVar: Option<CompileFunc<Sub<Var, Var, Var>>>,
    funcForSubTypeType: Option<CompileFunc<Sub<Type, Type, Var>>>,

    pub(crate) buildAsm: Option<for<'b> fn(&'b Block, &Function, &CallConv, &mut TargetBackendDescr<'b>) -> Vec<Instr>>,
    pub(crate) init: Option<fn(CallConv)->TargetBackendDescr<'a>>,

    pub(crate) lexer: Option<Box<dyn Lexer>>,
    pub(crate) compile: Option<Box<dyn Compiler>>,

    pub(crate) block: Option<&'a Block>,
    pub(crate) backend: BackendInfos,
    pub(crate) call: CallConv,
}

impl<'a> TargetBackendDescr<'a> {
    /// Creates a new instance
    pub fn new() -> Self {
        Self {
            funcForRetType: None,
            funcForRetVar: None,
            funcForConstAssign: None,
            funcForAddVarVar: None,
            funcForAddTypeType: None,
            funcForSubVarVar: None,
            funcForSubTypeType: None,
            init: None,
            buildAsm: None,

            lexer: None,
            compile: None,

            block: None,

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
    pub(crate) fn getCompileFuncForConstAssign(&self) ->  CompileFunc<ConstAssign<Var, Type>> {
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

    /// sets the callback for compiling the add var var ir node into asm
    pub(crate) fn setCompileFuncForSubVarVar(&mut self, callback: CompileFunc<Sub<Var, Var, Var>>) {
        self.funcForSubVarVar = Some(callback);
    }

    /// gets the callback for compiling the add var var node into into asm
    pub(crate) fn getCompileFuncForSubVarVar(&self) -> CompileFunc<Sub<Var, Var, Var>> {
        if let Some(func) = self.funcForSubVarVar {
            func
        } else { todo!("an corresponding assembly handler needs to be registered in order to compile an SubVarVar ir node")}
    }

    /// sets the callback for compiling the add var var ir node into asm
    pub(crate) fn setCompileFuncForSubTypeType(&mut self, callback: CompileFunc<Sub<Type, Type, Var>>) {
        self.funcForSubTypeType = Some(callback);
    }

    /// gets the callback for compiling the add var var node into into asm
    pub(crate) fn getCompileFuncForSubTypeType(&self) -> CompileFunc<Sub<Type, Type, Var>> {
        if let Some(func) = self.funcForSubTypeType {
            func
        } else { todo!("an corresponding assembly handler needs to be registered in order to compile an SubTypeType ir node")}
    }

    /// Returns the lexer to use with the TargetBackendDescr
    pub fn lexer(&self) -> Box<dyn Lexer> {
        self.lexer.clone().unwrap()
    }

    /// Returns the compiler to use with the TargetBackendDescr
    pub fn compiler(&self) -> Box<dyn Compiler> {
        self.compile.clone().unwrap()
    }
}