use std::{any::Any, fmt::Debug, hash::Hash};
use super::{FunctionType, IRBuilder, Type, TypeMetadata, Var, VerifyError};
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
IrTypeWith2!(ConstAssign, T, U);
IrTypeWith3!(Add, T, U, Z);
IrTypeWith3!(Sub, T, U, Z);
IrTypeWith3!(Xor, T, U, Z);

use crate::Support::Colorize;

impl Ir for Return<Type> {
    fn clone_box(&self) -> Box<dyn Ir> {
        Box::new(self.clone())
    }

    fn name(&self) -> String {
        "RetType".into()
    }

    fn dump(&self) -> String {
        let metadata: TypeMetadata = self.inner1.into();
        format!("ret {} {}", metadata, self.inner1.val())
    }

    fn dumpColored(&self) -> String {
        let metadata: TypeMetadata = self.inner1.into();
        format!("{} {} {}", "ret".blue(), metadata.to_string().cyan(), self.inner1.val().to_string().blue())
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
        registry.getCompileFuncRetType()(self, registry)
    }
}

impl Ir for Return<Var> {
    fn clone_box(&self) -> Box<dyn Ir> {
        Box::new(self.clone())
    }

    fn name(&self) -> String {
        "RetVar".into()
    }

    fn dump(&self) -> String {
        format!("ret {} {}", self.inner1.ty, self.inner1.name)
    }

    fn dumpColored(&self) -> String {
        format!("{} {} {}", "ret".blue(), self.inner1.ty.to_string().cyan(), self.inner1.name.to_string().magenta())
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

impl Ir for Add<Type, Type, Var> {
    fn clone_box(&self) -> Box<dyn Ir> {
        Box::new(self.clone())
    }

    fn name(&self) -> String {
        "AddTypeType".into()
    }

    fn dump(&self) -> String {
        format!("{} = add {} {}, {}", self.inner3.name, self.inner3.ty, self.inner1.val(), self.inner2.val())
    }

    fn dumpColored(&self) -> String {
        format!("{} = {} {} {}, {}", 
                self.inner3.name.magenta(), 
                "add".blue(), 
                self.inner3.ty.to_string().cyan(), 
                self.inner1.val().to_string().magenta(), 
                self.inner2.val().to_string().magenta()
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
        registry.getCompileFuncForAddTypeType()(self, registry)
    }

    fn uses(&self, var: &Var) -> bool {
        if *var == self.inner3 { true }
        else { false }
    }
}

impl Ir for Sub<Type, Type, Var> {
    fn clone_box(&self) -> Box<dyn Ir> {
        Box::new(self.clone())
    }

    fn name(&self) -> String {
        "SubTypeType".into()
    }

    fn dump(&self) -> String {
        format!("{} = sub {} {}, {}", self.inner3.name, self.inner3.ty, self.inner1.val(), self.inner2.val())
    }

    fn dumpColored(&self) -> String {
        format!("{} = {} {} {}, {}", 
                self.inner3.name.magenta(), 
                "sub".blue(), 
                self.inner3.ty.to_string().cyan(), 
                self.inner1.val().to_string().magenta(), 
                self.inner2.val().to_string().magenta()
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
        registry.getCompileFuncForSubTypeType()(self, registry)
    }

    fn uses(&self, var: &Var) -> bool {
        if *var == self.inner3 { true }
        else { false }
    }
}

impl Ir for Add<Var, Var, Var> {
    fn clone_box(&self) -> Box<dyn Ir> {
        Box::new(self.clone())
    }

    fn name(&self) -> String {
        "AddVarVar".into()
    }

    fn dump(&self) -> String {
        format!("{} = add {} {}, {}", self.inner3.name, self.inner3.ty, self.inner1.name, self.inner2.name)
    }

    fn dumpColored(&self) -> String {
        format!("{} = {} {} {}, {}", 
                self.inner3.name.magenta(), 
                "add".blue(), 
                self.inner3.ty.to_string().cyan(), 
                self.inner1.name.to_string().magenta(), 
                self.inner2.name.to_string().magenta()
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
        registry.getCompileFuncForAddVarVar()(self, registry)
    }

    fn uses(&self, var: &Var) -> bool {
        if *var == self.inner1 || *var == self.inner2 || *var == self.inner3 { true }
        else { false }
    }
}

impl Ir for Sub<Var, Var, Var> {
    fn clone_box(&self) -> Box<dyn Ir> {
        Box::new(self.clone())
    }

    fn name(&self) -> String {
        "SubVarVar".into()
    }

    fn dump(&self) -> String {
        format!("{} = sub {} {}, {}", self.inner3.name, self.inner3.ty, self.inner1.name, self.inner2.name)
    }

    fn dumpColored(&self) -> String {
        format!("{} = {} {} {}, {}", 
                self.inner3.name.magenta(), 
                "sub".blue(), 
                self.inner3.ty.to_string().cyan(), 
                self.inner1.name.to_string().magenta(), 
                self.inner2.name.to_string().magenta()
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
        registry.getCompileFuncForSubVarVar()(self, registry)
    }

    fn uses(&self, var: &Var) -> bool {
        if *var == self.inner1 || *var == self.inner2 || *var == self.inner3 { true }
        else { false }
    }
}

impl Ir for ConstAssign<Var, Type> {
    fn dump(&self) -> String {
        let meta: TypeMetadata = self.inner2.into();
        format!("{} = {} {}", self.inner1.name, meta, self.inner2.val())
    }

    fn dumpColored(&self) -> String {
        let meta: TypeMetadata = self.inner2.into();
        format!("{} = {} {}", 
            self.inner1.name.magenta(), 
            meta.to_string().cyan(), 
            self.inner2.val().to_string().blue()
        )
    }

    fn name(&self) -> String {
        "AssignVarType".into()
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

impl Ir for Xor<Type, Type, Var> {
    fn clone_box(&self) -> Box<dyn Ir> {
        Box::new(self.clone())
    }

    fn name(&self) -> String {
        "XorTypeType".into()
    }

    fn dump(&self) -> String {
        format!("{} = xor {} {}, {}", self.inner3.name, self.inner3.ty, self.inner1.val(), self.inner2.val())
    }

    fn dumpColored(&self) -> String {
        format!("{} = {} {} {}, {}", 
                self.inner3.name.magenta(), 
                "xor".blue(), 
                self.inner3.ty.to_string().cyan(), 
                self.inner1.val().to_string().magenta(), 
                self.inner2.val().to_string().magenta()
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
        registry.getCompileFuncForXorTypeType()(self, registry)
    }

    fn uses(&self, var: &Var) -> bool {
        if *var == self.inner3 { true }
        else { false }
    }
}

impl Ir for Xor<Var, Var, Var> {
    fn clone_box(&self) -> Box<dyn Ir> {
        Box::new(self.clone())
    }

    fn name(&self) -> String {
        "XorVarVar".into()
    }

    fn dump(&self) -> String {
        format!("{} = xor {} {}, {}", self.inner3.name, self.inner3.ty, self.inner1.name, self.inner2.name)
    }

    fn dumpColored(&self) -> String {
        format!("{} = {} {} {}, {}", 
                self.inner3.name.magenta(), 
                "xor".blue(), 
                self.inner3.ty.to_string().cyan(), 
                self.inner1.name.to_string().magenta(), 
                self.inner2.name.to_string().magenta()
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
        registry.getCompileFuncForXorVarVar()(self, registry)
    }

    fn uses(&self, var: &Var) -> bool {
        if *var == self.inner1 || *var == self.inner2 || *var == self.inner3 { true }
        else { false }
    }
}


/// Trait for the return instruction
/// Used for overloading the CreateRet function
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


/// Trait for the add function
/// Used for overloading the BuildAdd function
pub trait BuildAdd<T, U> {
    /// Adds the values
    fn BuildAdd(&mut self, op0: T, op1: U) -> Var;
}

impl BuildAdd<Type, Type> for IRBuilder<'_> {
    fn BuildAdd(&mut self, op0: Type, op1: Type)  -> Var {
        let block = self.blocks.get_mut(self.curr).expect("the IRBuilder needs to have an current block\nConsider creating one");
        
        let op0Ty: TypeMetadata = op0.into();

        let ty = op0Ty; // now both types need to be the same
        let var = Var::new(block, ty);

        block.push_ir(Add::new(op0, op1, var.clone()));

        var
    }
}

impl BuildAdd<Var, Var> for IRBuilder<'_> {
    fn BuildAdd(&mut self, op0: Var, op1: Var)  -> Var {
        let block = self.blocks.get_mut(self.curr).expect("the IRBuilder needs to have an current block\nConsider creating one");
        
        let op0Ty: TypeMetadata = op0.ty.into();

        let ty = op0Ty;
        let var = Var::new(block, ty);

        block.push_ir(Add::new(op0, op1, var.clone()));

        var
    }
}

/// Trait for the sub function
/// Used for overloading the BuildSub function
pub trait BuildSub<T, U> {
    /// Subs the values
    fn BuildSub(&mut self, op0: T, op1: U) -> Var;
}

impl BuildSub<Type, Type> for IRBuilder<'_> {
    fn BuildSub(&mut self, op0: Type, op1: Type)  -> Var {
        let block = self.blocks.get_mut(self.curr).expect("the IRBuilder needs to have an current block\nConsider creating one");
        
        let op0Ty: TypeMetadata = op0.into();

        let ty = op0Ty; // now both types need to be the same
        let var = Var::new(block, ty);

        block.push_ir(Sub::new(op0, op1, var.clone()));

        var
    }
}

impl BuildSub<Var, Var> for IRBuilder<'_> {
    fn BuildSub(&mut self, op0: Var, op1: Var)  -> Var {
        let block = self.blocks.get_mut(self.curr).expect("the IRBuilder needs to have an current block\nConsider creating one");
        
        let op0Ty: TypeMetadata = op0.ty.into();

        let ty = op0Ty;
        let var = Var::new(block, ty);

        block.push_ir(Sub::new(op0, op1, var.clone()));

        var
    }
}
/// Trait for the xor function
/// Used for overloading the BuildXor function
pub trait BuildXor<T, U> {
    /// Xors values
    fn BuildXor(&mut self, op0: T, op1: U) -> Var;
}

impl BuildXor<Type, Type> for IRBuilder<'_> {
    fn BuildXor(&mut self, op0: Type, op1: Type)  -> Var {
        let block = self.blocks.get_mut(self.curr).expect("the IRBuilder needs to have an current block\nConsider creating one");
        
        let op0Ty: TypeMetadata = op0.into();

        let ty = op0Ty; // now both types need to be the same
        let var = Var::new(block, ty);

        block.push_ir(Xor::new(op0, op1, var.clone()));

        var
    }
}

impl BuildXor<Var, Var> for IRBuilder<'_> {
    fn BuildXor(&mut self, op0: Var, op1: Var)  -> Var {
        let block = self.blocks.get_mut(self.curr).expect("the IRBuilder needs to have an current block\nConsider creating one");
        
        let op0Ty: TypeMetadata = op0.ty.into();

        let ty = op0Ty;
        let var = Var::new(block, ty);

        block.push_ir(Xor::new(op0, op1, var.clone()));

        var
    }
}

/// The ir trait
pub(crate) trait Ir: Debug + Any {
    /// Returns the ir node as his textual representation
    fn dump(&self) -> String;
    /// Returns the ir node as his textual representation with colors
    fn dumpColored(&self) -> String;

    /// Returns the name of the ir expr
    fn name(&self) -> String;

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