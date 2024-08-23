use super::*;

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

