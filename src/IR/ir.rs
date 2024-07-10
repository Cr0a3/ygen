use std::{error::Error, fmt::Debug};
use super::{FunctionType, IRBuilder, Type, TypeMetadata, Var, VerifyError};

macro_rules! IrTypeWith3 {
    ($name:tt, $param1:tt, $param2:tt, $param3:tt) => {
        /// An Ir node
        #[derive(Debug, Clone)]
        pub struct $name<$param1, $param2, $param3> {
            /// first inner value
            pub inner1: $param1,
            /// second inner value
            pub inner2: $param2,
            /// third inner value
            pub inner3: $param3,
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
/*macro_rules! IrTypeWith2 {
    ($name:tt, $param1:tt, $param2:tt) => {
        /// An Ir node
        #[derive(Debug, Clone)]
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
}*/
macro_rules! IrTypeWith1 {
    ($name:tt, $param1:tt) => {
        /// An Ir node
        #[derive(Debug, Clone)]
        pub(crate) struct $name<$param1> {
            /// inner value
            pub inner1: $param1,
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
IrTypeWith3!(Add, T, U, Z);

use crate::Support::Colorize;

impl Ir for Return<Type> {
    fn clone_box(&self) -> Box<dyn Ir> {
        Box::new(self.clone())
    }

    fn dump(&self) -> String {
        let metadata: TypeMetadata = self.inner1.into();
        format!("ret {} {}", metadata, self.inner1.val())
    }

    fn dumpColored(&self) -> String {
        let metadata: TypeMetadata = self.inner1.into();
        format!("{} {} {}", "ret".blue(), metadata.to_string().green(), self.inner1.val().to_string().blue())
    }

    fn verify(&self, FuncTy: FunctionType) -> Result<(), VerifyError> {
        let ty: TypeMetadata = self.inner1.into();

        if ty != FuncTy.ret {
            Err(VerifyError::RetTyNotFnTy(ty, FuncTy.ret))?
        }

        Ok(())
    }
}

impl Ir for Return<Var> {
    fn clone_box(&self) -> Box<dyn Ir> {
        Box::new(self.clone())
    }

    fn dump(&self) -> String {
        format!("ret {} {}", self.inner1.ty, self.inner1.name)
    }

    fn dumpColored(&self) -> String {
        format!("{} {} {}", "ret".blue(), self.inner1.ty.to_string().green(), self.inner1.name.to_string().blue())
    }

    fn verify(&self, FuncTy: FunctionType) -> Result<(), VerifyError> {
        let ty: TypeMetadata = self.inner1.ty.into();

        if ty != FuncTy.ret {
            Err(VerifyError::RetTyNotFnTy(ty, FuncTy.ret))?
        }

        Ok(())
    }
}

impl Ir for Add<Type, Type, Var> {
    fn clone_box(&self) -> Box<dyn Ir> {
        Box::new(self.clone())
    }

    fn dump(&self) -> String {
        format!("{} = add {} {}, {}", self.inner3.name, self.inner3.ty, self.inner1.val(), self.inner2.val())
    }

    fn dumpColored(&self) -> String {
        format!("{} = {} {} {}, {}", 
                self.inner3.name.magenta(), 
                "add".blue(), 
                self.inner3.ty.to_string().green(), 
                self.inner1.val().to_string().blue(), 
                self.inner2.val().to_string().blue()
            )
    }

    fn verify(&self, FuncTy: FunctionType) -> Result<(), VerifyError> {
        let ty: TypeMetadata = self.inner1.into();

        if ty != FuncTy.ret {
            Err(VerifyError::RetTyNotFnTy(ty, FuncTy.ret))?
        }

        Ok(())
    }
}

/// Trait for the return instruction
/// Used for overloading the CreateRet function
/// So you can return a TypeConstant or a variable
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

/// Trait for the return instruction
/// Used for overloading the CreateRet function
/// So you can return a TypeConstant or a variable
pub trait BuildAdd<T, U> {
    /// Adds to values
    fn BuildAdd(&mut self, op0: T, op1: U) -> Result<Var, Box<dyn Error>>;
}

impl BuildAdd<Type, Type> for IRBuilder<'_> {
    fn BuildAdd(&mut self, op0: Type, op1: Type)  -> Result<Var, Box<dyn Error>> {
        let block = self.blocks.get_mut(self.curr).expect("the IRBuilder needs to have an current block\nConsider creating one");
        
        let op0Ty: TypeMetadata = op0.into();
        let op1Ty: TypeMetadata = op1.into();

        if op0Ty != op1Ty {
            Err(VerifyError::Op0Op1TyNoMatch(op0Ty, op1Ty))?
        }

        let ty = op0Ty; // now both types need to be the same
        let var = Var::new(block, ty);

        block.push_ir(Add::new(op0, op1, var.clone()));

        Ok(var)
    }
}

/// The ir trait
pub(crate) trait Ir: Debug {
    /// Returns the ir node as his textual representation
    fn dump(&self) -> String;
    /// Returns the ir node as his textual representation with colors
    fn dumpColored(&self) -> String;

    fn verify(&self, FuncTy: FunctionType) -> Result<(), VerifyError>;

    /// Clones the node into a box of `Box<dyn Ir>`
    fn clone_box(&self) -> Box<dyn Ir>;
}

impl Clone for Box<dyn Ir> {
    fn clone(&self) -> Box<dyn Ir> {
        self.clone_box()
    }
}
