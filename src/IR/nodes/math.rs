use super::*;

macro_rules! MathIrNode {
    ($name:ident, $compileFuncVarVar:ident, $compileFuncVarTy:ident, $compileFuncTyTy:ident, $buildTraitName:ident, $buildFuncName:ident, $dump:expr, $op:tt) => {
        /// Used for overloading the build function
        pub trait $buildTraitName<T, U> {
            /// Xors values
            fn $buildFuncName(&mut self, op0: T, op1: U) -> Var;
        }

        impl $buildTraitName<Type, Type> for Function {
            fn $buildFuncName(&mut self, op0: Type, op1: Type)  -> Var {
                let block = self.blocks.back_mut().expect("the IRBuilder needs to have an current block\nConsider creating one");
                
                let op0Ty: TypeMetadata = op0.into();

                let ty = op0Ty; // now both types need to be the same
                let var = Var::new(block, ty);

                block.push_ir($name::new(op0, op1, var.clone()));

                var
            }
        }

        impl $buildTraitName<Var, Var> for Function {
            fn $buildFuncName(&mut self, op0: Var, op1: Var)  -> Var {
                let block = self.blocks.back_mut().expect("the IRBuilder needs to have an current block\nConsider creating one");
                
                let op0Ty: TypeMetadata = op0.ty.into();

                let ty = op0Ty;
                let var = Var::new(block, ty);

                block.push_ir($name::new(op0, op1, var.clone()));

                var
            }
        }

        impl $buildTraitName<Var, Type> for Function {
            fn $buildFuncName(&mut self, op0: Var, op1: Type)  -> Var {
                let block = self.blocks.back_mut().expect("the IRBuilder needs to have an current block\nConsider creating one");
                
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
                format!("{} = {} {} {}, {}", self.inner3.name, $dump, self.inner3.ty, self.inner1.val(), self.inner2.val())
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
        
            fn compile(&self, registry: &mut TargetBackendDescr) {
                registry.$compileFuncTyTy(&self)
            }
        
            fn uses(&self, var: &Var) -> bool {
                if *var == self.inner3 { true }
                else { false }
            }
    
            fn compile_dir(&self, compiler: &mut crate::CodeGen::IrCodeGenHelper, block: &crate::prelude::Block) {
                compiler.$compileFuncTyTy(&self, &block)
            }
    
            fn maybe_inline(&self, _: &HashMap<String, Type>) -> Option<Box<dyn Ir>> {
                None
            }
            
            fn eval(&self) -> Option<Box<dyn Ir>> {
                Some(Assign::new(
                    self.inner3.to_owned(), Type::from_int
                    (self.inner1.into(), (self.inner1.val() $op self.inner2.val()) as i64
                )))
            }
    
            fn inputs(&self) -> Vec<Var> {
                vec![]
            }
            
            fn output(&self) -> Option<Var> {
                Some(self.inner3.to_owned())
            }
        }
        
        impl Ir for $name<Var, Var, Var> {
            fn clone_box(&self) -> Box<dyn Ir> {
                Box::new(self.clone())
            }

            fn dump(&self) -> String {
                format!("{} = {} {} {}, {}", self.inner3.name, $dump, self.inner3.ty, self.inner1.name, self.inner2.name)
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
        
            fn compile(&self, registry: &mut TargetBackendDescr) {
                registry.$compileFuncVarVar(&self)
            }
        
            fn uses(&self, var: &Var) -> bool {
                if *var == self.inner1 || *var == self.inner2 || *var == self.inner3 { true }
                else { false }
            }
    
            fn compile_dir(&self, compiler: &mut crate::CodeGen::IrCodeGenHelper, block: &crate::prelude::Block) {
                compiler.$compileFuncVarVar(&self, &block)
            }
    
            fn maybe_inline(&self, values: &HashMap<String, Type>) -> Option<Box<dyn Ir>> {
                if let Some(lhs) = values.get(&self.inner1.name) {
                    if let Some(rhs) = values.get(&self.inner2.name) {
                        Some(Assign::new(
                            self.inner3.to_owned(), Type::from_int
                            (self.inner1.ty, (lhs.val() $op rhs.val()) as i64
                        )))
                    } else {
                        Some($name::new(self.inner2.to_owned(), *lhs, self.inner3.to_owned()))
                    }
                } else if let Some(rhs) = values.get(&self.inner2.name) {
                    Some($name::new(self.inner2.to_owned(), *rhs, self.inner3.to_owned()))
                } else { None }
            }
            
            fn eval(&self) -> Option<Box<dyn Ir>> {
                None
            }
    
            fn inputs(&self) -> Vec<Var> {
                vec![self.inner1.to_owned(), self.inner2.to_owned()]
            }
            
            fn output(&self) -> Option<Var> {
                Some(self.inner3.to_owned())
            }
        }
        
        impl Ir for $name<Var, Type, Var> {
            fn clone_box(&self) -> Box<dyn Ir> {
                Box::new(self.clone())
            }
        
            fn dump(&self) -> String {
                format!("{} = {} {} {}, {}", self.inner3.name, $dump, self.inner1.ty, self.inner1.name, self.inner2.val())
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
        
            fn compile(&self, registry: &mut TargetBackendDescr) {
                registry.$compileFuncVarTy(&self)
            }
        
            fn uses(&self, var: &Var) -> bool {
                if *var == self.inner1 || *var == self.inner3 { true }
                else { false }
            }
    
            fn compile_dir(&self, compiler: &mut crate::CodeGen::IrCodeGenHelper, block: &crate::prelude::Block) {
                compiler.$compileFuncVarTy(&self, &block)
            }

            fn maybe_inline(&self, values: &HashMap<String, Type>) -> Option<Box<dyn Ir>> {
                if let Some(lhs) = values.get(&self.inner1.name) {
                    Some(Assign::new(
                        self.inner3.to_owned(), Type::from_int
                        (self.inner1.ty, (lhs.val() $op self.inner2.val()) as i64
                    )))
                } else { None }
            }
            
            fn eval(&self) -> Option<Box<dyn Ir>> {
                None
            }
    
            fn inputs(&self) -> Vec<Var> {
                vec![self.inner1.to_owned()]
            }
            
            fn output(&self) -> Option<Var> {
                Some(self.inner3.to_owned())
            }
        }
        
    };
}

MathIrNode!(Add,    compile_add_var_var,   compile_add_var_type, compile_add_type_type, BuildAdd, BuildAdd, "add", +);
MathIrNode!(Sub,    compile_sub_var_var,   compile_sub_var_type, compile_sub_type_type, BuildSub, BuildSub, "sub", -);
MathIrNode!(Xor,    compile_xor_var_var,   compile_xor_var_type, compile_xor_type_type, BuildXor, BuildXor, "xor", ^);
MathIrNode!(Or,     compile_or_var_var,    compile_or_var_type, compile_or_type_type, BuildOr, BuildOr, "or", |);
MathIrNode!(And,    compile_and_var_var,   compile_and_var_type, compile_and_type_type, BuildAnd, BuildAnd, "and", &);
MathIrNode!(Mul,    compile_mul_var_var,   compile_mul_var_type, compile_mul_type_type, BuildMul, BuildMul, "mul", *);
MathIrNode!(Div,    compile_div_var_var,   compile_div_var_type, compile_div_type_type, BuildDiv, BuildDiv, "div", /);
MathIrNode!(Rem,    compile_rem_var_var,   compile_rem_var_type, compile_rem_type_type, BuildRem, BuildRem, "rem", %);
MathIrNode!(Shl,    compile_shl_var_var,   compile_shl_var_type, compile_shl_type_type, BuildShl, BuildShl, "shl", <<);
MathIrNode!(Shr,    compile_shr_var_var,   compile_shr_var_type, compile_shr_type_type, BuildShr, BuildShr, "shr", >>);

