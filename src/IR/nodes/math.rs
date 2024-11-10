use super::*;

macro_rules! MathIrNode {
    ($name:ident, $compile_func:ident, $build_trait:ident, $build_func:ident, $dump:expr) => {
        /// Used for overloading the build function
        pub trait $build_trait<T, U> {
            /// does the math opeation on the values
            fn $build_func(&mut self, op0: T, op1: U) -> Var;
        }

        impl $build_trait<Type, Type> for Function {
            fn $build_func(&mut self, op0: Type, op1: Type)  -> Var {
                let block = self.blocks.back_mut().expect("the IRBuilder needs to have an current block\nConsider creating one");
                
                let op0Ty: TypeMetadata = op0.into();

                let ty = op0Ty; // now both types need to be the same
                let var = Var::new(block, ty);

                block.push_ir($name::new(IROperand::Type(op0), IROperand::Type(op1), var.clone()));

                var
            }
        }

        impl $build_trait<Var, Type> for Function {
            fn $build_func(&mut self, op0: Var, op1: Type)  -> Var {
                let block = self.blocks.back_mut().expect("the IRBuilder needs to have an current block\nConsider creating one");
                
                let op0Ty: TypeMetadata = op0.ty.into();

                let ty = op0Ty;
                let var = Var::new(block, ty);

                block.push_ir($name::new(IROperand::Var(op0), IROperand::Type(op1), var.clone()));

                var
            }
        }

        impl $build_trait<Type, Var> for Function {
            fn $build_func(&mut self, op0: Type, op1: Var)  -> Var {
                let block = self.blocks.back_mut().expect("the IRBuilder needs to have an current block\nConsider creating one");
                
                let op0Ty: TypeMetadata = op0.into();

                let ty = op0Ty;
                let var = Var::new(block, ty);

                block.push_ir($name::new(IROperand::Type(op0), IROperand::Var(op1), var.clone()));

                var
            }
        }

        impl $build_trait<Var, Var> for Function {
            fn $build_func(&mut self, op0: Var, op1: Var)  -> Var {
                let block = self.blocks.back_mut().expect("the IRBuilder needs to have an current block\nConsider creating one");
                
                let op0Ty: TypeMetadata = op0.ty.into();

                let ty = op0Ty;
                let var = Var::new(block, ty);

                block.push_ir($name::new(IROperand::Var(op0), IROperand::Var(op1), var.clone()));

                var
            }
        }

        impl Ir for $name {
            fn clone_box(&self) -> Box<dyn Ir> {
                Box::new(self.clone())
            }
        
            fn dump(&self) -> String {
                format!("{} = {} {} {}, {}", self.inner3.name, $dump, self.inner3.ty, self.inner1, self.inner2)
            }
        
            fn dumpColored(&self, profile: ColorProfile) -> String {
                format!("{} = {} {} {}, {}", 
                    profile.markup(&self.inner3.name, ColorClass::Var), 
                    profile.markup($dump, ColorClass::Instr), 
                    profile.markup(&self.inner3.ty.to_string(), ColorClass::Ty), 
                    profile.markup(&self.inner1.to_string(), ColorClass::Value), 
                    profile.markup(&self.inner2.to_string(), ColorClass::Value)
                )
            }
        
            fn verify(&self, _: FunctionType) -> Result<(), VerifyError> {
                let op0Ty: TypeMetadata = self.inner1.get_ty();
                let op1Ty: TypeMetadata = self.inner2.get_ty();
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
        
            fn compile(&self, registry: &mut TargetBackendDescr, module: &mut crate::prelude::Module) {
                registry.$compile_func(&self, module)
            }
        
            fn uses(&self, var: &Var) -> bool {
                if *var == self.inner3 { true }
                else { false }
            }
    
            fn compile_dir(&self, compiler: &mut crate::CodeGen::IrCodeGenHelper, block: &crate::prelude::Block, module: &mut crate::prelude::Module) {
                compiler.$compile_func(&self, &block, module)
            }
    
            fn inputs(&self) -> Vec<Var> {
                let mut inputs = Vec::new();

                if let IROperand::Var(ls) = &self.inner1 { inputs.push(ls.to_owned()); }
                if let IROperand::Var(rs) = &self.inner1 { inputs.push(rs.to_owned()); }
                inputs
            }
    
            fn inputs_mut(&mut self) -> Vec<&mut Var> {
                let mut inputs = Vec::new();

                if let IROperand::Var(ls) = &mut self.inner1 {
                    inputs.push(ls);
                }
                if let IROperand::Var(rs) = &mut self.inner2 {
                    inputs.push(rs);
                }
                inputs
            }
            
            fn output(&self) -> Option<Var> {
                Some(self.inner3.to_owned())
            }
        }
        
    };
}

MathIrNode!(Add,    compile_add,  BuildAdd, BuildAdd, "add");
MathIrNode!(Sub,    compile_sub,  BuildSub, BuildSub, "sub");
MathIrNode!(Xor,    compile_xor,  BuildXor, BuildXor, "xor");
MathIrNode!(Or,     compile_or, BuildOr, BuildOr, "or");
MathIrNode!(And,    compile_and,  BuildAnd, BuildAnd, "and");
MathIrNode!(Mul,    compile_mul,  BuildMul, BuildMul, "mul");
MathIrNode!(Div,    compile_div,  BuildDiv, BuildDiv, "div");
MathIrNode!(Rem,    compile_rem,  BuildRem, BuildRem, "rem");
MathIrNode!(Shl,    compile_shl,  BuildShl, BuildShl, "shl");
MathIrNode!(Shr,    compile_shr,  BuildShr, BuildShr, "shr");

impl EvalOptVisitor for Add {
    fn maybe_inline(&self, const_values: &HashMap<String, Type>) -> Option<Box<dyn Ir>> {
        todo!()
    }

    fn eval(&self) -> Option<Box<dyn Ir>> {
        todo!()
    }
}

impl EvalOptVisitor for Sub {
    fn maybe_inline(&self, const_values: &HashMap<String, Type>) -> Option<Box<dyn Ir>> {
        todo!()
    }

    fn eval(&self) -> Option<Box<dyn Ir>> {
        todo!()
    }
}

impl EvalOptVisitor for Xor {
    fn maybe_inline(&self, const_values: &HashMap<String, Type>) -> Option<Box<dyn Ir>> {
        todo!()
    }

    fn eval(&self) -> Option<Box<dyn Ir>> {
        todo!()
    }
}

impl EvalOptVisitor for Or {
    fn maybe_inline(&self, const_values: &HashMap<String, Type>) -> Option<Box<dyn Ir>> {
        todo!()
    }

    fn eval(&self) -> Option<Box<dyn Ir>> {
        todo!()
    }
}

impl EvalOptVisitor for And {
    fn maybe_inline(&self, const_values: &HashMap<String, Type>) -> Option<Box<dyn Ir>> {
        todo!()
    }

    fn eval(&self) -> Option<Box<dyn Ir>> {
        todo!()
    }
}

impl EvalOptVisitor for Mul {
    fn maybe_inline(&self, const_values: &HashMap<String, Type>) -> Option<Box<dyn Ir>> {
        todo!()
    }

    fn eval(&self) -> Option<Box<dyn Ir>> {
        todo!()
    }
}

impl EvalOptVisitor for Div {
    fn maybe_inline(&self, const_values: &HashMap<String, Type>) -> Option<Box<dyn Ir>> {
        todo!()
    }

    fn eval(&self) -> Option<Box<dyn Ir>> {
        todo!()
    }
}

impl EvalOptVisitor for Rem {
    fn maybe_inline(&self, const_values: &HashMap<String, Type>) -> Option<Box<dyn Ir>> {
        todo!()
    }

    fn eval(&self) -> Option<Box<dyn Ir>> {
        todo!()
    }
}

impl EvalOptVisitor for Shl {
    fn maybe_inline(&self, const_values: &HashMap<String, Type>) -> Option<Box<dyn Ir>> {
        todo!()
    }

    fn eval(&self) -> Option<Box<dyn Ir>> {
        todo!()
    }
}

impl EvalOptVisitor for Shr {
    fn maybe_inline(&self, const_values: &HashMap<String, Type>) -> Option<Box<dyn Ir>> {
        todo!()
    }

    fn eval(&self) -> Option<Box<dyn Ir>> {
        todo!()
    }
}