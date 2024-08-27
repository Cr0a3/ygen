use std::{collections::{HashMap, VecDeque}, fmt::Display};
use core::fmt::Debug;

use crate::{prelude::{ir::*, Block, Function, Type, TypeMetadata, Var}, IR::Const};

use super::{x64Reg, CallConv, Compiler, instr::Instr, Lexer, instr::MemOp, Reg};

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
    pub(crate) mutable: Vec<Box<dyn Reg>>,

    pub(crate) stackSafe: bool,

    pub(crate) shadow: i64,
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
            mutable: vec![],

            tmpReg: x64Reg::Rax.boxed(),

            saveRegister: vec![],
            savedRegisters: vec![],

            stackSafe: false,
            shadow: 0,
        }
    }

    /// Delets the variable of the varsStorage (giving out it's resources)
    pub(crate) fn drop(&mut self, var: &Var) {
        if let Some(loc) = &self.varsStorage.get(var) {
            if let VarStorage::Register(reg) = &loc {
               self.dropReg(reg.clone());
            } // don't decrease the stack offset because if it isn't at the bottom other variables will may be overriden
        }
    }

    /// Adds the register back to the usable registers in the front
    pub(crate) fn dropReg(&mut self, reg: Box<dyn Reg>) {
        self.openUsableRegisters64.push_front(reg.from(reg.sub64()));
        self.openUsableRegisters32.push_front(reg.from(reg.sub32()));
        self.openUsableRegisters16.push_front(reg.from(reg.sub16()));
        self.openUsableRegisters8.push_front(reg.from(reg.sub8()));
    }

    /// Returns a variable which uses the given reg
    pub(crate) fn getVarByReg(&self, reg: Box<dyn Reg>) -> Option<&Var> {
        let mut out = None;

        for (var, store) in &self.varsStorage {
            if let VarStorage::Register(var_reg) = store {
                if var_reg.sub64() == reg.sub64() {
                    out = Some(var);
                }
            }
        }

        out
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
            TypeMetadata::u64 | TypeMetadata::i64 | TypeMetadata::ptr => self.getOpenReg64(),
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
            TypeMetadata::u64 | TypeMetadata::i64 | TypeMetadata::ptr => self.getTmpReg64(),
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
    funcForConstAssignVar: Option<CompileFunc<ConstAssign<Var, Var>>>,
    funcForConstAssignConst: Option<CompileFunc<ConstAssign<Var, Const>>>,
    funcForCastTyVar: Option<CompileFunc<Cast<Var, TypeMetadata, Var>>>,

    funcForAddVarVar: Option<CompileFunc<Add<Var, Var, Var>>>,
    funcForAddVarType: Option<CompileFunc<Add<Var, Type, Var>>>,
    funcForAddTypeType: Option<CompileFunc<Add<Type, Type, Var>>>,
    
    funcForSubVarVar: Option<CompileFunc<Sub<Var, Var, Var>>>,
    funcForSubVarType: Option<CompileFunc<Sub<Var, Type, Var>>>,
    funcForSubTypeType: Option<CompileFunc<Sub<Type, Type, Var>>>,

    funcForXorVarVar: Option<CompileFunc<Xor<Var, Var, Var>>>,
    funcForXorVarType: Option<CompileFunc<Xor<Var, Type, Var>>>,
    funcForXorTypeType: Option<CompileFunc<Xor<Type, Type, Var>>>,

    funcForOrVarVar: Option<CompileFunc<Or<Var, Var, Var>>>,
    funcForOrVarType: Option<CompileFunc<Or<Var, Type, Var>>>,
    funcForOrTypeType: Option<CompileFunc<Or<Type, Type, Var>>>,

    funcForAndVarVar: Option<CompileFunc<And<Var, Var, Var>>>,
    funcForAndVarType: Option<CompileFunc<And<Var, Type, Var>>>,
    funcForAndTypeType: Option<CompileFunc<And<Type, Type, Var>>>,

    funcForCall: Option<CompileFunc<Call<Function, Vec<Var>, Var>>>,

    pub(crate) buildAsm: Option<for<'b> fn(&'b Block, &Function, &CallConv, &mut TargetBackendDescr<'b>) -> Vec<Instr>>,
    pub(crate) init: Option<fn(CallConv)->TargetBackendDescr<'a>>,

    pub(crate) lexer: Option<Box<dyn Lexer>>,
    pub(crate) compile: Option<Box<dyn Compiler>>,

    pub(crate) block: Option<&'a Block>,
    pub(crate) backend: BackendInfos,
    pub(crate) call: CallConv,
}

macro_rules! get_set_compile_func {
    ($get_name:ident, $set_name:ident, $var:ident, $($ty:tt)*) => {     
        /// sets the callback for compiling the ir node into asm
        pub(crate) fn $set_name(&mut self, callback: $($ty)*) {
            self.$var = Some(callback);
        }

        /// gets the callback for compiling the  ir node into asm
        pub(crate) fn $get_name(&self) -> $($ty)* {
            if let Some(func) = self.$var {
                func
            } else { todo!("an corresponding assembly handler needs to be registered in order to compile ir node")}
        }
    };
}

impl<'a> TargetBackendDescr<'a> {
    /// Creates a new instance
    pub fn new() -> Self {
        Self {
            funcForRetType: None,
            funcForRetVar: None,
            funcForConstAssign: None,
            funcForConstAssignVar: None,
            funcForConstAssignConst: None,
            funcForCastTyVar: None,

            funcForAddVarVar: None,
            funcForAddVarType: None,
            funcForAddTypeType: None,

            funcForSubVarVar: None,
            funcForSubVarType: None,
            funcForSubTypeType: None,

            funcForXorVarVar: None,
            funcForXorVarType: None,
            funcForXorTypeType: None,

            funcForOrVarVar: None,
            funcForOrVarType: None,
            funcForOrTypeType: None,

            funcForAndVarVar: None,
            funcForAndVarType: None,
            funcForAndTypeType: None,

            funcForCall: None,

            init: None,
            buildAsm: None,

            lexer: None,
            compile: None,

            block: None,

            call: CallConv::SystemV,
            backend: BackendInfos::new(),
        }
    }

    get_set_compile_func!(getCompileFuncForRetType, setCompileFuncForRetType, funcForRetType, CompileFunc<Return<Type>>);
    get_set_compile_func!(getCompileFuncForRetVar, setCompileFuncForRetVar, funcForRetVar, CompileFunc<Return<Var>>);
    get_set_compile_func!(getCompileFuncForConstAssign, setCompileFuncForConstAssign, funcForConstAssign, CompileFunc<ConstAssign<Var, Type>>);
    get_set_compile_func!(getCompileFuncForConstAssignVar, setCompileFuncForConstAssignVar, funcForConstAssignVar, CompileFunc<ConstAssign<Var, Var>>);
    get_set_compile_func!(getCompileFuncForConstAssignConst, setCompileFuncForConstAssignConst, funcForConstAssignConst, CompileFunc<ConstAssign<Var, Const>>);
    get_set_compile_func!(getCompileFuncForCastTyVar, setCompileFuncForCastTyVar,  funcForCastTyVar, CompileFunc<Cast<Var, TypeMetadata, Var>>);
    
    get_set_compile_func!(getCompileFuncForAddVarVar, setCompileFuncForAddVarVar,  funcForAddVarVar, CompileFunc<Add<Var, Var, Var>>);
    get_set_compile_func!(getCompileFuncForAddTypeType, setCompileFuncForAddTypeType,  funcForAddTypeType, CompileFunc<Add<Type, Type, Var>>);
    get_set_compile_func!(getCompileFuncForAddVarType, setCompileFuncForAddVarType,  funcForAddVarType, CompileFunc<Add<Var, Type, Var>>);

    get_set_compile_func!(getCompileFuncForSubVarVar,   setCompileFuncForSubVarVar,     funcForSubVarVar,   CompileFunc<Sub<Var, Var, Var>>);
    get_set_compile_func!(getCompileFuncForSubTypeType, setCompileFuncForSubTypeType,   funcForSubTypeType,   CompileFunc<Sub<Type, Type, Var>>);
    get_set_compile_func!(getCompileFuncForSubVarType,  setCompileFuncForSubVarType,    funcForSubVarType,   CompileFunc<Sub<Var, Type, Var>>);

    get_set_compile_func!(getCompileFuncForOrVarVar,   setCompileFuncForOrVarVar,     funcForOrVarVar,   CompileFunc<Or<Var, Var, Var>>);
    get_set_compile_func!(getCompileFuncForOrTypeType, setCompileFuncForOrTypeType,   funcForOrTypeType,   CompileFunc<Or<Type, Type, Var>>);
    get_set_compile_func!(getCompileFuncForOrVarType,  setCompileFuncForOrVarType,    funcForOrVarType,   CompileFunc<Or<Var, Type, Var>>);

    get_set_compile_func!(getCompileFuncForXorVarVar,   setCompileFuncForXorVarVar,     funcForXorVarVar,   CompileFunc<Xor<Var, Var, Var>>);
    get_set_compile_func!(getCompileFuncForXorTypeType, setCompileFuncForXorTypeType,   funcForXorTypeType,   CompileFunc<Xor<Type, Type, Var>>);
    get_set_compile_func!(getCompileFuncForXorVarType,  setCompileFuncForXorVarType,    funcForXorVarType,   CompileFunc<Xor<Var, Type, Var>>);

    get_set_compile_func!(getCompileFuncForAndVarVar,   setCompileFuncForAndVarVar,     funcForAndVarVar,   CompileFunc<And<Var, Var, Var>>);
    get_set_compile_func!(getCompileFuncForAndTypeType, setCompileFuncForAndTypeType,   funcForAndTypeType,   CompileFunc<And<Type, Type, Var>>);
    get_set_compile_func!(getCompileFuncForAndVarType,  setCompileFuncForAndVarType,    funcForAndVarType,   CompileFunc<And<Var, Type, Var>>);
    
    get_set_compile_func!(getCompileFuncForCall, setCompileFuncForCall, funcForCall, CompileFunc<Call<Function, Vec<Var>, Var>>);

    /// Returns the lexer to use with the TargetBackendDescr
    pub fn lexer(&self) -> Box<dyn Lexer> {
        self.lexer.clone().unwrap()
    }

    /// Returns the compiler to use with the TargetBackendDescr
    pub fn compiler(&self) -> Box<dyn Compiler> {
        self.compile.clone().unwrap()
    }
}