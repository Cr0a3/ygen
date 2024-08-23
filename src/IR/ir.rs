use std::{any::Any, fmt::Debug, hash::Hash};
use super::{Const, Function, FunctionType, IRBuilder, Type, TypeMetadata, Var, VerifyError};
use crate::Target::{instr::Instr, TargetBackendDescr};

macro_rules! IrTypeWith3 {
    ($name:tt, $param1:tt, $param2:tt, $param3:tt) => {
        /// An Ir node
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct $name<$param1, $param2, $param3> {
            /// first inner value
            pub(crate) inner1: $param1,
            /// second inner value
            pub(crate) inner2: $param2,
            /// third inner value
            pub(crate) inner3: $param3,
        }


        impl<$param1, $param2, $param3> $name<$param1, $param2, $param3> {
            /// Creates new instance
            #[allow(dead_code)]
            pub fn new(op0: $param1, op1: $param2, op2: $param3) -> Box<Self> {
                Box::from(
                    Self {
                        inner1: op0,
                        inner2: op1,
                        inner3: op2,
                    }
                )
            }
        }
    };
}
macro_rules! IrTypeWith2 {
    ($name:tt, $param1:tt, $param2:tt) => {
        /// An Ir node
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct $name<$param1, $param2> {
            /// first inner value
            pub inner1: $param1,
            /// second inner value
            pub inner2: $param2,
        }


        impl<$param1, $param2> $name<$param1, $param2> {
            /// Creates new instance
            #[allow(dead_code)]
            pub fn new(op0: $param1, op1: $param2) -> Box<Self> {
                Box::from(
                    Self {
                        inner1: op0,
                        inner2: op1,
                    }
                )
            }
        }
    };
}
macro_rules! IrTypeWith1 {
    ($name:tt, $param1:tt) => {
        /// An Ir node
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub(crate) struct $name<$param1> {
            /// inner value
            pub(crate) inner1: $param1,
        }


        impl<$param1> $name<$param1> {
            /// Creates new instance
            #[allow(dead_code)]
            pub(crate) fn new(op0: $param1) -> Box<Self> {
                Box::from(
                    Self {
                        inner1: op0,
                    }
                )
            }
        }
    };
}

IrTypeWith1!(Return, T);
IrTypeWith3!(Call, T, U, Z);
IrTypeWith2!(ConstAssign, T, U);
IrTypeWith3!(Cast, T, U, Z);
IrTypeWith3!(Add, T, U, Z);
IrTypeWith3!(Sub, T, U, Z);
IrTypeWith3!(Xor, T, U, Z);
IrTypeWith3!(Or, T, U, Z);
IrTypeWith3!(And, T, U, Z);

use crate::Support::{ColorClass, ColorProfile};

macro_rules! MathIrNode {
    ($name:ident, $compileFuncVarVar:ident, $compileFuncVarTy:ident, $compileFuncTyTy:ident, $buildTraitName:ident, $buildFuncName:ident, $dump:expr) => {
        /// Used for overloading the build function
        pub trait $buildTraitName<T, U> {
            /// Xors values
            fn $buildFuncName(&mut self, op0: T, op1: U) -> Var;
        }

        impl $buildTraitName<Type, Type> for IRBuilder<'_> {
            fn $buildFuncName(&mut self, op0: Type, op1: Type)  -> Var {
                let block = self.blocks.get_mut(self.curr).expect("the IRBuilder needs to have an current block\nConsider creating one");
                
                let op0Ty: TypeMetadata = op0.into();

                let ty = op0Ty; // now both types need to be the same
                let var = Var::new(block, ty);

                block.push_ir($name::new(op0, op1, var.clone()));

                var
            }
        }

        impl $buildTraitName<Var, Var> for IRBuilder<'_> {
            fn $buildFuncName(&mut self, op0: Var, op1: Var)  -> Var {
                let block = self.blocks.get_mut(self.curr).expect("the IRBuilder needs to have an current block\nConsider creating one");
                
                let op0Ty: TypeMetadata = op0.ty.into();

                let ty = op0Ty;
                let var = Var::new(block, ty);

                block.push_ir($name::new(op0, op1, var.clone()));

                var
            }
        }

        impl $buildTraitName<Var, Type> for IRBuilder<'_> {
            fn $buildFuncName(&mut self, op0: Var, op1: Type)  -> Var {
                let block = self.blocks.get_mut(self.curr).expect("the IRBuilder needs to have an current block\nConsider creating one");
                
                let op0Ty: TypeMetadata = op0.ty.into();

                let ty = op0Ty;
                let var = Var::new(block, ty);

                block.push_ir($name::new(op0, op1, var.clone()));

                var
            }
        }

        impl Ir for $name<Type, Type, Var> {
            fn clone_box(&self) -> Box<dyn Ir> {
                Box::new(self.clone())
            }
        
            fn dump(&self) -> String {
                format!("{} = {} {} {}, {}", $dump, self.inner3.name, self.inner3.ty, self.inner1.val(), self.inner2.val())
            }
        
            fn dumpColored(&self, profile: ColorProfile) -> String {
                format!("{} = {} {} {}, {}", 
                    profile.markup(&self.inner3.name, ColorClass::Var), 
                    profile.markup($dump, ColorClass::Instr), 
                    profile.markup(&self.inner3.ty.to_string(), ColorClass::Ty), 
                    profile.markup(&self.inner1.val().to_string(), ColorClass::Value), 
                    profile.markup(&self.inner2.val().to_string(), ColorClass::Value)
                )
            }
        
            fn verify(&self, _: FunctionType) -> Result<(), VerifyError> {
                let op0Ty: TypeMetadata = self.inner1.into();
                let op1Ty: TypeMetadata = self.inner2.into();
                let op2Ty: TypeMetadata = self.inner3.ty.into();
        
                if !(op0Ty == op1Ty && op1Ty == op2Ty) {
                    if op0Ty != op1Ty {
                        Err(VerifyError::Op0Op1TyNoMatch(op0Ty, op1Ty))?
                    } else if op1Ty != op2Ty {
                        Err(VerifyError::Op0Op1TyNoMatch(op1Ty, op2Ty))?
                    } if op0Ty != op2Ty {
                        Err(VerifyError::Op0Op1TyNoMatch(op0Ty, op2Ty))?
                    } else { todo!("unknown error variant (debug: ty0 {} ty1 {} ty2 {})", op0Ty, op1Ty, op2Ty) }
                }
        
                Ok(())
            }
        
            fn as_any(&self) -> &dyn Any {
                self
            }
        
            fn compile(&self, registry: &mut TargetBackendDescr) -> Vec<Instr> {
                registry.$compileFuncTyTy()(self, registry)
            }
        
            fn uses(&self, var: &Var) -> bool {
                if *var == self.inner3 { true }
                else { false }
            }
        }
        
        impl Ir for $name<Var, Var, Var> {
            fn clone_box(&self) -> Box<dyn Ir> {
                Box::new(self.clone())
            }

            fn dump(&self) -> String {
                format!("{} = {} {} {}, {}", $dump, self.inner3.name, self.inner3.ty, self.inner1.name, self.inner2.name)
            }
        
            fn dumpColored(&self, profile: ColorProfile) -> String {
                format!("{} = {} {} {}, {}", 
                    profile.markup(&self.inner3.name, ColorClass::Var), 
                    profile.markup($dump, ColorClass::Instr), 
                    profile.markup(&self.inner3.ty.to_string(), ColorClass::Ty), 
                    profile.markup(&self.inner1.name.to_string(), ColorClass::Var), 
                    profile.markup(&self.inner2.name.to_string(), ColorClass::Var)
                )
            }
        
            fn verify(&self, _: FunctionType) -> Result<(), VerifyError> {
                let op0Ty: TypeMetadata = self.inner1.ty.into();
                let op1Ty: TypeMetadata = self.inner2.ty.into();
                let op2Ty: TypeMetadata = self.inner3.ty.into();
        
                if !(op0Ty == op1Ty && op1Ty == op2Ty) {
                    if op0Ty != op1Ty {
                        Err(VerifyError::Op0Op1TyNoMatch(op0Ty, op1Ty))?
                    } else if op1Ty != op2Ty {
                        Err(VerifyError::Op0Op1TyNoMatch(op1Ty, op2Ty))?
                    } if op0Ty != op2Ty {
                        Err(VerifyError::Op0Op1TyNoMatch(op0Ty, op2Ty))?
                    } else { todo!("unknown error variant (debug: ty0 {} ty1 {} ty2 {})", op0Ty, op1Ty, op2Ty) }
                }
        
                Ok(())
            }
        
            fn as_any(&self) -> &dyn Any {
                self
            }
        
            fn compile(&self, registry: &mut TargetBackendDescr) -> Vec<Instr> {
                registry.$compileFuncVarVar()(self, registry)
            }
        
            fn uses(&self, var: &Var) -> bool {
                if *var == self.inner1 || *var == self.inner2 || *var == self.inner3 { true }
                else { false }
            }
        }
        
        impl Ir for $name<Var, Type, Var> {
            fn clone_box(&self) -> Box<dyn Ir> {
                Box::new(self.clone())
            }
        
            fn dump(&self) -> String {
                format!("{} = {} {} {}, {}", $dump, self.inner3.name, self.inner1.ty, self.inner1.name, self.inner2.val())
            }
        
            fn dumpColored(&self, profile: ColorProfile) -> String {
                format!("{} = {} {} {}, {}", 
                    profile.markup(&self.inner3.name, ColorClass::Var), 
                    profile.markup($dump, ColorClass::Instr), 
                    profile.markup(&self.inner1.ty.to_string(), ColorClass::Ty), 
                    profile.markup(&self.inner1.name.to_string(), ColorClass::Var), 
                    profile.markup(&self.inner2.val().to_string(), ColorClass::Var)
                )
            }
        
            fn verify(&self, _: FunctionType) -> Result<(), VerifyError> {
                let op0Ty: TypeMetadata = self.inner1.ty.into();
                let op1Ty: TypeMetadata = self.inner3.ty.into();
                let op2Ty: TypeMetadata = self.inner2.into();
        
                if !(op0Ty == op1Ty && op1Ty == op2Ty) {
                    if op0Ty != op1Ty {
                        Err(VerifyError::Op0Op1TyNoMatch(op0Ty, op1Ty))?
                    } else if op1Ty != op2Ty {
                        Err(VerifyError::Op0Op1TyNoMatch(op1Ty, op2Ty))?
                    } if op0Ty != op2Ty {
                        Err(VerifyError::Op0Op1TyNoMatch(op0Ty, op2Ty))?
                    } else { todo!("unknown error variant (debug: ty0 {} ty1 {} ty2 {})", op0Ty, op1Ty, op2Ty) }
                }
        
                Ok(())
            }
        
            fn as_any(&self) -> &dyn Any {
                self
            }
        
            fn compile(&self, registry: &mut TargetBackendDescr) -> Vec<Instr> {
                registry.$compileFuncVarTy()(self, registry)
            }
        
            fn uses(&self, var: &Var) -> bool {
                if *var == self.inner1 || *var == self.inner3 { true }
                else { false }
            }
        }
        
    };
}

MathIrNode!(Add, getCompileFuncForAddVarVar, getCompileFuncForAddVarType, getCompileFuncForAddTypeType, BuildAdd, BuildAdd, "add");
MathIrNode!(Sub, getCompileFuncForSubVarVar, getCompileFuncForSubVarType, getCompileFuncForSubTypeType, BuildSub, BuildSub, "sub");
MathIrNode!(Xor, getCompileFuncForXorVarVar, getCompileFuncForXorVarType, getCompileFuncForXorTypeType, BuildXor, BuildXor, "xor");
MathIrNode!(Or, getCompileFuncForOrVarVar, getCompileFuncForOrVarType, getCompileFuncForOrTypeType, BuildOr, BuildOr, "or");
MathIrNode!(And, getCompileFuncForAndVarVar, getCompileFuncForAndVarType, getCompileFuncForAndTypeType, BuildAnd, BuildAnd, "and");


impl Ir for Return<Type> {
    fn clone_box(&self) -> Box<dyn Ir> {
        Box::new(self.clone())
    }

    fn dump(&self) -> String {
        let metadata: TypeMetadata = self.inner1.into();
        format!("ret {} {}", metadata, self.inner1.val())
    }

    fn dumpColored(&self, profile: ColorProfile) -> String {
        let metadata: TypeMetadata = self.inner1.into();
        format!("{} {} {}", 
            profile.markup("ret", ColorClass::Instr),
            profile.markup(&metadata.to_string(), ColorClass::Ty), 
            profile.markup(&self.inner1.val().to_string(), ColorClass::Var),
        )
    }

    fn verify(&self, FuncTy: FunctionType) -> Result<(), VerifyError> {
        let ty: TypeMetadata = self.inner1.into();

        if ty != FuncTy.ret {
            Err(VerifyError::RetTyNotFnTy(ty, FuncTy.ret))?
        }

        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn compile(&self, registry: &mut TargetBackendDescr) -> Vec<Instr> {
        registry.getCompileFuncForRetType()(self, registry)
    }
}

impl Ir for Return<Var> {
    fn clone_box(&self) -> Box<dyn Ir> {
        Box::new(self.clone())
    }

    fn dump(&self) -> String {
        format!("ret {} {}", self.inner1.ty, self.inner1.name)
    }

    fn dumpColored(&self, profile: ColorProfile) -> String {
        format!("{} {} {}", 
            profile.markup("ret", ColorClass::Instr), 
            profile.markup(&self.inner1.ty.to_string(), ColorClass::Ty), 
            profile.markup(&self.inner1.name.to_string(), ColorClass::Var),
        )
    }

    fn verify(&self, FuncTy: FunctionType) -> Result<(), VerifyError> {
        let ty: TypeMetadata = self.inner1.ty.into();

        if ty != FuncTy.ret {
            Err(VerifyError::RetTyNotFnTy(ty, FuncTy.ret))?
        }

        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn compile(&self, registry: &mut TargetBackendDescr) -> Vec<Instr> {
        registry.getCompileFuncForRetVar()(self, registry)
    }

    fn uses(&self, var: &Var) -> bool {
        if *var == self.inner1 { true }
        else { false }
    }
}

impl Ir for ConstAssign<Var, Type> {
    fn dump(&self) -> String {
        let meta: TypeMetadata = self.inner2.into();
        format!("{} = {} {}", self.inner1.name, meta, self.inner2.val())
    }

    fn dumpColored(&self, profile: ColorProfile) -> String {
        let meta: TypeMetadata = self.inner2.into();
        format!("{} = {} {}", 
            profile.markup(&self.inner1.name, ColorClass::Var), 
            profile.markup(&meta.to_string(), ColorClass::Instr), 
            profile.markup(&self.inner2.val().to_string(), ColorClass::Value),
        )
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn verify(&self, _: FunctionType) -> Result<(), VerifyError> {
        let op0Ty = self.inner1.ty;
        let op1Ty = self.inner2.into();
        if op0Ty != op1Ty {
            Err(VerifyError::Op0Op1TyNoMatch(op0Ty, op1Ty))?
        }

        Ok(())
    }

    fn clone_box(&self) -> Box<dyn Ir> {
        Box::new(self.clone())
    }

    fn compile(&self, registry: &mut TargetBackendDescr) -> Vec<Instr> {
        registry.getCompileFuncForConstAssign()(self, registry)
    }

    fn uses(&self, var: &Var) -> bool {
        if *var == self.inner1 { true }
        else { false }
    }
}

impl Ir for ConstAssign<Var, Var> {
    fn dump(&self) -> String {
        let meta: TypeMetadata = self.inner2.ty;
        format!("{} = {} {}", self.inner1.name, meta, self.inner2.name)
    }

    fn dumpColored(&self, profile: ColorProfile) -> String {
        let meta: TypeMetadata = self.inner2.ty;
        format!("{} = {} {}", 
            profile.markup(&self.inner1.name, ColorClass::Var), 
            profile.markup(&meta.to_string(), ColorClass::Instr), 
            profile.markup(&self.inner2.name.to_string(), ColorClass::Value),
        )
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn verify(&self, _: FunctionType) -> Result<(), VerifyError> {
        let op0Ty = self.inner1.ty;
        let op1Ty = self.inner2.ty;
        if op0Ty != op1Ty {
            Err(VerifyError::Op0Op1TyNoMatch(op0Ty, op1Ty))?
        }

        Ok(())
    }

    fn clone_box(&self) -> Box<dyn Ir> {
        Box::new(self.clone())
    }

    fn compile(&self, registry: &mut TargetBackendDescr) -> Vec<Instr> {
        registry.getCompileFuncForConstAssignVar()(self, registry)
    }

    fn uses(&self, var: &Var) -> bool {
        if *var == self.inner1 { true }
        else if *var == self.inner2 { true }
        else { false }
    }
}

impl Ir for ConstAssign<Var, Const> {
    fn dump(&self) -> String {
        format!("{} = ptr {}", self.inner1.name, self.inner2.name)
    }

    fn dumpColored(&self, profile: ColorProfile) -> String {
        format!("{} = {} {}", 
            profile.markup(&self.inner1.name, ColorClass::Var), 
            profile.markup("ptr", ColorClass::Ty), 
            profile.markup(&self.inner2.name.to_string(), ColorClass::Value),
        )
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn verify(&self, _: FunctionType) -> Result<(), VerifyError> {
        Ok(())
    }

    fn clone_box(&self) -> Box<dyn Ir> {
        Box::new(self.clone())
    }

    fn compile(&self, registry: &mut TargetBackendDescr) -> Vec<Instr> {
        registry.getCompileFuncForConstAssignConst()(self, registry)
    }

    fn uses(&self, var: &Var) -> bool {
        if *var == self.inner1 { true }
        else { false }
    }
}

impl Ir for Cast<Var, TypeMetadata, Var> {
    fn dump(&self) -> String {
        format!("{} = cast {} to {}", self.inner3.name, self.inner1.name, self.inner2)
    }

    fn dumpColored(&self, profile: ColorProfile) -> String {
        format!("{} = {} {} {} {}", 
            profile.markup(&self.inner3.name, ColorClass::Var), 
            profile.markup(&"cast", ColorClass::Instr),
            profile.markup(&self.inner1.name, ColorClass::Var), 
            profile.markup(&"to", ColorClass::Instr),
            profile.markup(&self.inner2.to_string(), ColorClass::Ty),
        )
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn verify(&self, _: FunctionType) -> Result<(), VerifyError> {
        if self.inner3.ty != self.inner2 {
            Err(VerifyError::Op0Op1TyNoMatch(self.inner3.ty, self.inner2))?
        }
        Ok(())
    }

    fn uses(&self, var: &Var) -> bool {
        let var = var.to_owned();

        if var == self.inner1 || var == self.inner3 {
            true
        } else { false }
    }

    fn clone_box(&self) -> Box<dyn Ir> {
        Box::from( self.clone() )
    }

    fn compile(&self, registry: &mut TargetBackendDescr) -> Vec<Instr> {
        registry.getCompileFuncForCastTyVar()(self, registry)
    }
}

impl Ir for Call<Function, Vec<Var>, Var> {
    fn dump(&self) -> String {
        let mut fmt = String::new();
        
        for arg in &self.inner2 {
            fmt.push_str(&format!("{} ", arg))
        }

        format!("{} = call {} {} {}", self.inner3.name, self.inner1.ty.ret, self.inner1.name, fmt)
    }

    fn dumpColored(&self, profile: ColorProfile) -> String {
        let mut fmt = String::new();
        
        for arg in &self.inner2 {
            fmt.push_str(&arg.to_colored_string(profile));
            fmt.push(' ');
        }

        format!("{} = {} {} {} {}", 
            profile.markup(&self.inner3.name, ColorClass::Var),
            profile.markup("call", ColorClass::Instr),
            profile.markup(&self.inner1.ty.ret.to_string(), ColorClass::Ty),
            profile.markup(&self.inner1.name, ColorClass::Name),
            fmt
        )
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn verify(&self, _: FunctionType) -> Result<(), VerifyError> {
        if self.inner3.ty != self.inner1.ty.ret {
            Err(VerifyError::Op0Op1TyNoMatch(self.inner3.ty, self.inner1.ty.ret))?
        }

        let mut index = 0;
        let args = &self.inner1.ty.args;
        for arg in &self.inner2 {
            if index < args.len() {
                if matches!(args.get(index), Some((_, argty)) if *argty != (*arg).ty.into()) {
                    Err(VerifyError::InvalidArgumentTypeFound)?
                }
            } else {
                if !self.inner1.ty.any_args {
                    Err(VerifyError::ToManyArgumentsWereSupplyed)?
                }
            }
            
            index += 1;
        }

        Ok(())
    }

    fn clone_box(&self) -> Box<dyn Ir> {
        Box::from( self.clone() )
    }

    fn compile(&self, registry: &mut TargetBackendDescr) -> Vec<Instr> {
        registry.getCompileFuncForCall()(self, registry)
    }

    fn uses(&self, var: &Var) -> bool {
        let mut uses = false;

        if self.inner3 == *var {
            uses = true;
        }


        for arg in &self.inner2 {
            if *arg == *var {
                uses = true;
            }
        }

        uses
    }
}

/// Trait for the return instruction
/// Used for overloading the BuildRet function
pub trait BuildReturn<T> {
    /// Returns specified value
    fn BuildRet(&mut self, val: T);
}

impl BuildReturn<Type> for IRBuilder<'_> {
    fn BuildRet(&mut self, val: Type) {
        self.blocks.get_mut(self.curr).expect("the IRBuilder needs to have an current block\nConsider creating one")
            .push_ir(Return::new(val))
    }
}

impl BuildReturn<Var> for IRBuilder<'_> {
    fn BuildRet(&mut self, var: Var) {
        self.blocks.get_mut(self.curr).expect("the IRBuilder needs to have an current block\nConsider creating one")
            .push_ir(Return::new(var))
    }
}

/// Trait for the cast instruction
/// Used for overloading the BuildCast function
pub trait BuildCast<T, U> {
    /// builds an cast to form one variable into another type
    fn BuildCast(&mut self, var: T, ty: U) -> Var;
}

impl BuildCast<Var, TypeMetadata> for IRBuilder<'_> {
    fn BuildCast(&mut self, var: Var, ty: TypeMetadata) -> Var {
        let block = self.blocks.get_mut(self.curr).expect("the IRBuilder needs to have an current block\nConsider creating one");
        
        let out = Var::new(block, ty);

        block.push_ir(Cast::new(var, ty, out.clone()));

        out
    }
}

/// Trait for the call instruction
/// Used for overloading the BuildCall function
pub trait BuildCall<T, U> {
    /// builds a function call
    fn BuildCall(&mut self, func: T, args: U) -> Var;
}
impl BuildCall<&Function, Vec<Var>> for IRBuilder<'_> {
    fn BuildCall(&mut self, func: &Function, args: Vec<Var>) -> Var {
        let block = self.blocks.get_mut(self.curr).expect("the IRBuilder needs to have an current block\nConsider creating one");
        
        let out = Var::new(block, func.ty.ret);

        block.push_ir(Call::new(func.clone(), args, out.clone()));

        out 
    }
}

/// Trait used for overloading the BuildAssign function
pub trait BuildAssign<T> {
    /// builds an assignment
    fn BuildAssign(&mut self, value: T) -> Var;
}
impl BuildAssign<Type> for IRBuilder<'_> {
    fn BuildAssign(&mut self, value: Type) -> Var {
        let block = self.blocks.get_mut(self.curr).expect("the IRBuilder needs to have an current block\nConsider creating one");
        
        let out = Var::new(block, value.into());

        block.push_ir(ConstAssign::new(out.clone(), value));

        out
    }
}

impl BuildAssign<Var> for IRBuilder<'_> {
    fn BuildAssign(&mut self, value: Var) -> Var {
        let block = self.blocks.get_mut(self.curr).expect("the IRBuilder needs to have an current block\nConsider creating one");
        
        let out = Var::new(block, value.ty);

        block.push_ir(ConstAssign::new(out.clone(), value));

        out
    }
}

impl BuildAssign<&Const> for IRBuilder<'_> {
    fn BuildAssign(&mut self, value: &Const) -> Var {
        let block = self.blocks.get_mut(self.curr).expect("the IRBuilder needs to have an current block\nConsider creating one");
        
        let out = Var::new(block, TypeMetadata::ptr);

        block.push_ir(ConstAssign::new(out.clone(), value.clone()));

        out
    }
}

/// The ir trait
pub(crate) trait Ir: Debug + Any {
    /// Returns the ir node as his textual representation
    fn dump(&self) -> String;
    /// Returns the ir node as his textual representation with colors
    fn dumpColored(&self, profile: ColorProfile) -> String;

    /// Turns the ir node to an any
    fn as_any(&self) -> &dyn Any;

    fn verify(&self, FuncTy: FunctionType) -> Result<(), VerifyError>;

    /// Clones the node into a box of `Box<dyn Ir>`
    fn clone_box(&self) -> Box<dyn Ir>;

    /// Compiles the node based on the given target
    fn compile(&self, registry: &mut TargetBackendDescr) -> Vec<Instr>;

    /// Returns if the node uses the variable
    fn uses(&self, _: &Var) -> bool {
        false
    }

    fn is(&self, other: &Box<dyn Ir>) -> bool {
        other.dump() == self.dump()
    }
}

impl PartialEq for Box<dyn Ir> {
    fn eq(&self, other: &Self) -> bool {
        self.is(other)
    }
}

impl Eq for Box<dyn Ir> { }

impl Clone for Box<dyn Ir> {
    fn clone(&self) -> Box<dyn Ir> {
        self.clone_box()
    }
    
    fn clone_from(&mut self, source: &Self) {
        *self = source.clone()
    }
}

/// Used for sus workaround to replace current ir node
pub trait Replace<T> {
    /// Replaces current ir node
    fn replace(&mut self, other: T);
}

impl Replace<Box<dyn Ir>> for Box<dyn Ir> {
    fn replace(&mut self, other: Box<dyn Ir>) {
        *self = other
    }
}