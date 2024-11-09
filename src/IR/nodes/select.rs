use super::{Assign, EvalOptVisitor, IROperand, Ir, IsNode, Select};
use crate::{prelude::{Type, TypeMetadata, Var}, Support::{AsAny, ColorClass}, IR::Function};

impl Ir for Select<IROperand, IROperand> {
    fn dump(&self) -> String {
        let yes_meta: TypeMetadata = self.yes.get_ty();
        let no_meta: TypeMetadata = self.no.get_ty();

        format!("{} = select {} {}, {} {}, {} {}", 
            self.out.name, 
            self.cond.ty,
            self.cond.name, 
            yes_meta, self.yes, 
            no_meta, self.no
        )
    }

    fn dumpColored(&self, profile: crate::Support::ColorProfile) -> String {
        let yes_meta: TypeMetadata = self.yes.get_ty();
        let no_meta: TypeMetadata = self.no.get_ty();
        format!("{} = {} {} {}, {} {}, {} {}", 
            profile.markup(&self.out.name, ColorClass::Var),
            profile.markup("select", ColorClass::Instr),
            profile.markup(&self.cond.ty.to_string(), ColorClass::Ty),
            profile.markup(&self.cond.name, ColorClass::Var),
            profile.markup(&yes_meta.to_string(), ColorClass::Ty), 
            profile.markup(&self.yes.to_string(), ColorClass::Value),
            profile.markup(&no_meta.to_string(), ColorClass::Ty),
            profile.markup(&self.no.to_string(), ColorClass::Value),
        )
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn verify(&self, _: crate::prelude::FunctionType) -> Result<(), crate::prelude::VerifyError> {
        Ok(())
    }

    fn clone_box(&self) -> Box<dyn Ir> {
        Box::from( self.clone() )
    }

    fn compile(&self, registry: &mut crate::Target::TargetBackendDescr, module: &mut crate::prelude::Module) {
        registry.compile_select(self, module)
    }

    fn compile_dir(&self, compiler: &mut crate::CodeGen::IrCodeGenHelper, block: &crate::prelude::Block, module: &mut crate::prelude::Module) {
        compiler.compile_select(self, block, module)
    }

    fn inputs(&self) -> Vec<Var> {
        let mut inputs = vec![self.cond.to_owned()];

        if let IROperand::Var(var) = &self.yes { inputs.push(var.to_owned()); } 
        if let IROperand::Var(var) = &self.no { inputs.push(var.to_owned()); } 

        inputs
    }
    
    fn inputs_mut(&mut self) -> Vec<&mut Var> {
        vec![&mut self.cond]
    }

    fn output(&self) -> Option<Var> {
        Some(self.out.clone())
    }
}

impl EvalOptVisitor for Select<IROperand, IROperand> {
    fn maybe_inline(&self, const_values: &std::collections::HashMap<String, Type>) -> Option<Box<dyn Ir>> {
        match (&self.yes, &self.no) {
            (IROperand::Type(yes), IROperand::Type(no)) => {
                if let Some(cond) = const_values.get(&self.cond.name) {
                    if cond.val() == 0.0 {
                        Some(Assign::new(self.out.clone(), *yes))
                    } else {
                        Some(Assign::new(self.out.clone(), *no))
                    }
                } else { None}
            },

            (IROperand::Var(yes), IROperand::Type(no)) => {
                if let Some(yes) = const_values.get(&yes.name) {
                    return Some(Box::new(Select {
                        out: self.out.clone(),
                        cond: self.cond.clone(),
                        yes: IROperand::Type(*yes),
                        no: IROperand::Type(*no),
                    }));
                } else { None }
            },

            (IROperand::Type(yes), IROperand::Var(no)) => {
                if let Some(no) = const_values.get(&no.name) {
                    return Some(Box::new(Select {
                        out: self.out.clone(),
                        cond: self.cond.clone(),
                        yes: IROperand::Type(*yes),
                        no: IROperand::Type(*no),
                    }));
                } else { None }
            },

            _ => None,
        }
    }

    fn eval(&self) -> Option<Box<dyn Ir>> {
        if self.no == self.yes {
            let yes: Box<dyn Ir> = match &self.yes {
                IROperand::Type(yes) => Assign::new(self.out.clone(), *yes),
                IROperand::Var(yes) => Assign::new(self.out.clone(), yes.to_owned()),
            };
            return Some(yes);
        }

        None
    }
}


impl<T, U> IsNode for Select<T, U> 
    where T: std::fmt::Debug + Clone + PartialEq + Eq + AsAny,
        U: std::fmt::Debug + Clone + PartialEq + Eq + AsAny,
{
    fn is_select(&self) -> bool {
        true
    }
}
impl<T, U> Select<T, U> 
    where T: std::fmt::Debug + Clone + PartialEq + Eq + AsAny + 'static,
        U: std::fmt::Debug + Clone + PartialEq + Eq + AsAny + 'static,
{
    /// Returns the condition
    pub fn getCondition(&self) -> Var {
        self.cond.to_owned()
    }

    /// Returns the output variable
    pub fn getOut(&self) -> Var {
        self.out.to_owned()
    }

    /// Returns the type
    pub fn getSelType(&self) -> TypeMetadata {
        self.out.ty
    }

    /// Returns if the true value is a variable
    pub fn isTrueVar(&self) -> bool {
        matches!(self.yes.as_any().downcast_ref::<IROperand>(), Some(IROperand::Var(_)))
    }

    /// Returns if the false value is a variable
    pub fn isFalseVar(&self) -> bool {
        matches!(self.no.as_any().downcast_ref::<IROperand>(), Some(IROperand::Var(_)))
    }

    /// Returns if the true value is a constant
    pub fn isTrueConst(&self) -> bool {
        matches!(self.yes.as_any().downcast_ref::<IROperand>(), Some(IROperand::Type(_)))
    }

    /// Returns if the false value is a constant
    pub fn isFalseConst(&self) -> bool {
        matches!(self.no.as_any().downcast_ref::<IROperand>(), Some(IROperand::Type(_)))
    }

    /// Returns the true value as a variable
    /// 
    /// ### Panics
    /// 
    /// panics if the true value is not a variable so first check
    pub fn getTrueVar(&self) -> Var {
        let Some(IROperand::Var(ret)) = self.yes.as_any().downcast_ref::<IROperand>() else {
            panic!();
        };
        ret.to_owned()
    }

    /// Returns the false value as a variable
    /// 
    /// ### Panics
    /// 
    /// panics if the false value is not a variable so first check
    pub fn getFalseVar(&self) -> Var {
        let Some(IROperand::Var(ret)) = self.no.as_any().downcast_ref::<IROperand>() else {
            panic!();
        };
        ret.to_owned()
    }

    /// Returns the true value as a constant
    /// 
    /// ### Panics
    /// 
    /// panics if the true value is not a constant so first check
    pub fn getTrueConst(&self) -> Type {
        let Some(IROperand::Type(ret)) = self.yes.as_any().downcast_ref::<IROperand>() else {
            panic!();
        };
        *ret
    }

    /// Returns the false value as a constant
    /// 
    /// ### Panics
    /// 
    /// panics if the false value is not a constant so first check
    pub fn getFalseConst(&self) -> Type {
        let Some(IROperand::Type(ret)) = self.no.as_any().downcast_ref::<IROperand>() else {
            panic!();
        };
        *ret
    }
}
/// This trait is used to build the select node
pub trait BuildSelect<T, U> {
    /// The select node.
    /// Let's say this example:
    /// ```
    /// fn test(a: i32) -> i32 {
    ///     if a == 0 {
    ///         5
    ///     } else { a}
    /// }
    /// 
    /// ```
    /// Which we could compile to:
    /// ```no-run
    /// define i32 @test(i32 %a) {
    ///   entry:
    ///     %if.cond = cmp eq i32, %a, 0
    ///     %ret = select i32 %if.cond, i32 5, i32 %a
    ///     ret i32 %ret
    /// }
    /// ```
    fn BuildSelect(&mut self, cond: Var, yes: T, no: U) -> Var;
}

impl BuildSelect<Type, Type> for Function {
    fn BuildSelect(&mut self, cond: Var, yes: Type, no: Type) -> Var {
        let block = self.blocks.back_mut().expect("expected valid current block");

        let out = Var::new(block, yes.into());

        block.push_ir(Box::new(Select {
            out: out.clone(),
            cond: cond,
            yes: IROperand::Type(yes),
            no: IROperand::Type(no),
        }));

        out
    }
}

impl BuildSelect<Type, Var> for Function {
    fn BuildSelect(&mut self, cond: Var, yes: Type, no: Var) -> Var {
        let block = self.blocks.back_mut().expect("expected valid current block");

        let out = Var::new(block, yes.into());

        block.push_ir(Box::new(Select {
            out: out.clone(),
            cond: cond,
            yes: IROperand::Type(yes),
            no: IROperand::Var(no),
        }));

        out
    }
}

impl BuildSelect<Var, Type> for Function {
    fn BuildSelect(&mut self, cond: Var, yes: Var, no: Type) -> Var {
        let block = self.blocks.back_mut().expect("expected valid current block");

        let out = Var::new(block, yes.ty);

        block.push_ir(Box::new(Select {
            out: out.clone(),
            cond: cond,
            yes: IROperand::Var(yes),
            no: IROperand::Type(no),
        }));

        out
    }
}

impl BuildSelect<Var, Var> for Function {
    fn BuildSelect(&mut self, cond: Var, yes: Var, no: Var) -> Var {
        let block = self.blocks.back_mut().expect("expected valid current block");

        let out = Var::new(block, yes.ty);

        block.push_ir(Box::new(Select {
            out: out.clone(),
            cond: cond,
            yes: IROperand::Var(yes),
            no: IROperand::Var(no),
        }));

        out
    }
}