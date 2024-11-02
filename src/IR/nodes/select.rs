use std::any::{Any, TypeId};

use super::{Assign, EvalOptVisitor, Ir, IsNode, Select};
use crate::{prelude::{Type, TypeMetadata, Var}, Support::{AsAny, ColorClass}, IR::Function};

impl Ir for Select<Type, Type> {
    fn dump(&self) -> String {
        let yes_meta: TypeMetadata = self.yes.into();
        let no_meta: TypeMetadata = self.no.into();
        format!("{} = select {} {}, {} {}, {} {}", 
            self.out.name, 
            self.cond.ty,
            self.cond.name, 
            yes_meta, self.yes.val(), 
            no_meta, self.no.val()
        )
    }

    fn dumpColored(&self, profile: crate::Support::ColorProfile) -> String {
        let yes_meta: TypeMetadata = self.yes.into();
        let no_meta: TypeMetadata = self.no.into();
        format!("{} = {} {} {}, {} {}, {} {}", 
            profile.markup(&self.out.name, ColorClass::Var),
            profile.markup("select", ColorClass::Instr),
            profile.markup(&self.cond.ty.to_string(), ColorClass::Ty),
            profile.markup(&self.cond.name, ColorClass::Var),
            profile.markup(&yes_meta.to_string(), ColorClass::Ty), 
            profile.markup(&self.yes.val().to_string(), ColorClass::Value),
            profile.markup(&no_meta.to_string(), ColorClass::Ty),
            profile.markup(&self.no.val().to_string(), ColorClass::Value),
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
        registry.compile_select_tt(self, module)
    }

    fn compile_dir(&self, compiler: &mut crate::CodeGen::IrCodeGenHelper, block: &crate::prelude::Block, module: &mut crate::prelude::Module) {
        compiler.compile_select_tt(self, block, module)
    }

    fn inputs(&self) -> Vec<Var> {
        vec![self.cond.to_owned()]
    }
    
    fn inputs_mut(&mut self) -> Vec<&mut Var> {
        vec![&mut self.cond]
    }

    fn output(&self) -> Option<Var> {
        Some(self.out.clone())
    }
}

impl EvalOptVisitor for Select<Type, Type> {
    fn maybe_inline(&self, const_values: &std::collections::HashMap<String, Type>) -> Option<Box<dyn Ir>> {
        if let Some(cond) = const_values.get(&self.cond.name) {
            if cond.val() == 0.0 {
                return Some(Assign::new(self.out.clone(), self.yes));
            } else {
                return Some(Assign::new(self.out.clone(), self.no));
            }
        }

        None
    }

    fn eval(&self) -> Option<Box<dyn Ir>> {
        if self.no == self.yes {
            return Some(Assign::new(self.out.clone(), self.yes));
        }

        None
    }
}

impl Ir for Select<Var, Type> {
    fn dump(&self) -> String {
        let no_meta: TypeMetadata = self.no.into();
        format!("{} = select {} {}, {} {}, {} {}", 
            self.out.name, 
            self.cond.ty,
            self.cond.name, 
            self.yes.ty, self.yes.name, 
            no_meta, self.no.val()
        )
    }

    fn dumpColored(&self, profile: crate::Support::ColorProfile) -> String {
        let no_meta: TypeMetadata = self.no.into();
        format!("{} = {} {} {}, {} {}, {} {}", 
            profile.markup(&self.out.name, ColorClass::Var),
            profile.markup("select", ColorClass::Instr),
            profile.markup(&self.cond.ty.to_string(), ColorClass::Ty),
            profile.markup(&self.cond.name, ColorClass::Var),
            profile.markup(&self.yes.ty.to_string(), ColorClass::Ty), 
            profile.markup(&self.yes.name, ColorClass::Var),
            profile.markup(&no_meta.to_string(), ColorClass::Ty),
            profile.markup(&self.no.val().to_string(), ColorClass::Value),
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
        registry.compile_select_vt(self, module)
    }

    fn compile_dir(&self, compiler: &mut crate::CodeGen::IrCodeGenHelper, block: &crate::prelude::Block, module: &mut crate::prelude::Module) {
        compiler.compile_select_vt(self, block, module)
    }

    fn inputs(&self) -> Vec<Var> {
        vec![self.cond.to_owned(), self.yes.clone()]
    }
    
    fn inputs_mut(&mut self) -> Vec<&mut Var> {
        vec![&mut self.cond, &mut self.yes]
    }

    fn output(&self) -> Option<Var> {
        Some(self.out.clone())
    }
}

impl EvalOptVisitor for Select<Var, Type> {
    fn maybe_inline(&self, const_values: &std::collections::HashMap<String, Type>) -> Option<Box<dyn Ir>> {
        if let Some(yes) = const_values.get(&self.yes.name) {
            return Some(Select {
                out: self.out.clone(),
                cond: self.cond.clone(),
                yes: *yes,
                no: self.no,
            }.clone_box());
        }
        
        None
    }

    fn eval(&self) -> Option<Box<dyn Ir>> {
        None
    }
}

impl Ir for Select<Type, Var> {
    fn dump(&self) -> String {
        let yes_meta: TypeMetadata = self.yes.into();
        format!("{} = select {} {}, {} {}, {} {}", 
            self.out.name, 
            self.cond.ty,
            self.cond.name, 
            yes_meta, self.yes.val(), 
            self.no.ty, self.no.name
        )
    }

    fn dumpColored(&self, profile: crate::Support::ColorProfile) -> String {
        let yes_meta: TypeMetadata = self.yes.into();
        format!("{} = {} {} {}, {} {}, {} {}", 
            profile.markup(&self.out.name, ColorClass::Var),
            profile.markup("select", ColorClass::Instr),
            profile.markup(&self.cond.ty.to_string(), ColorClass::Ty),
            profile.markup(&self.cond.name, ColorClass::Var),
            profile.markup(&yes_meta.to_string(), ColorClass::Ty), 
            profile.markup(&self.yes.val().to_string(), ColorClass::Value),
            profile.markup(&self.no.ty.to_string(), ColorClass::Ty),
            profile.markup(&self.no.name, ColorClass::Var),
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
        registry.compile_select_tv(self, module)
    }

    fn compile_dir(&self, compiler: &mut crate::CodeGen::IrCodeGenHelper, block: &crate::prelude::Block, module: &mut crate::prelude::Module) {
        compiler.compile_select_tv(self, block, module)
    }

    fn inputs(&self) -> Vec<Var> {
        vec![self.cond.to_owned(), self.no.to_owned()]
    }
    
    fn inputs_mut(&mut self) -> Vec<&mut Var> {
        vec![&mut self.cond, &mut self.no]
    }

    fn output(&self) -> Option<Var> {
        Some(self.out.clone())
    }
}

impl EvalOptVisitor for Select<Type, Var> {
    fn maybe_inline(&self, const_values: &std::collections::HashMap<String, Type>) -> Option<Box<dyn Ir>> {
        if let Some(no) = const_values.get(&self.no.name) {
            return Some(Select {
                out: self.out.clone(),
                cond: self.cond.clone(),
                yes: self.yes,
                no: *no,
            }.clone_box());
        }

        None
    }

    fn eval(&self) -> Option<Box<dyn Ir>> {
        None
    }
}

impl Ir for Select<Var, Var> {
    fn dump(&self) -> String {
        format!("{} = select {} {}, {} {}, {} {}", 
            self.out.name, 
            self.cond.ty,
            self.cond.name, 
            self.yes.ty, self.yes.name, 
            self.no.ty, self.no.name
        )
    }

    fn dumpColored(&self, profile: crate::Support::ColorProfile) -> String {
        format!("{} = {} {} {}, {} {}, {} {}", 
            profile.markup(&self.out.name, ColorClass::Var),
            profile.markup("select", ColorClass::Instr),
            profile.markup(&self.cond.ty.to_string(), ColorClass::Ty),
            profile.markup(&self.cond.name, ColorClass::Var),
            profile.markup(&self.yes.ty.to_string(), ColorClass::Ty), 
            profile.markup(&self.yes.name, ColorClass::Var),
            profile.markup(&self.no.ty.to_string(), ColorClass::Ty),
            profile.markup(&self.no.name, ColorClass::Var),
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
        registry.compile_select_vv(self, module)
    }

    fn compile_dir(&self, compiler: &mut crate::CodeGen::IrCodeGenHelper, block: &crate::prelude::Block, module: &mut crate::prelude::Module) {
        compiler.compile_select_vv(self, block, module)
    }

    fn inputs(&self) -> Vec<Var> {
        vec![self.cond.to_owned(), self.yes.to_owned(), self.no.to_owned()]
    }
    
    fn inputs_mut(&mut self) -> Vec<&mut Var> {
        vec![&mut self.cond, &mut self.yes, &mut self.no]
    }

    fn output(&self) -> Option<Var> {
        Some(self.out.clone())
    }
}

impl EvalOptVisitor for Select<Var, Var> {
    fn maybe_inline(&self, _: &std::collections::HashMap<String, Type>) -> Option<Box<dyn Ir>> {
        None
    }

    fn eval(&self) -> Option<Box<dyn Ir>> {
        if self.yes == self.no {
            return Some(Assign::new(self.out.clone(), self.yes.clone()));
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
        self.yes.type_id() == TypeId::of::<Var>()
    }

    /// Returns if the false value is a variable
    pub fn isFalseVar(&self) -> bool {
        self.no.type_id() == TypeId::of::<Var>()
    }

    /// Returns if the true value is a constant
    pub fn isTrueConst(&self) -> bool {
        self.yes.type_id() == TypeId::of::<Type>()
    }

    /// Returns if the false value is a constant
    pub fn isFalseConst(&self) -> bool {
        self.no.type_id() == TypeId::of::<Type>()
    }

    /// Returns the true value as a variable
    /// 
    /// ### Panics
    /// 
    /// panics if the true value is not a variable so first check
    pub fn getTrueVar(&self) -> Var {
        self.yes.as_any().downcast_ref::<Var>().unwrap().clone()
    }

    /// Returns if the false value is a variable
    /// 
    /// ### Panics
    /// 
    /// panics if the false value is not a variable so first check
    pub fn getFalseVar(&self) -> bool {
        self.no.type_id() == TypeId::of::<Var>()
    }

    /// Returns the true value as a constant
    /// 
    /// ### Panics
    /// 
    /// panics if the true value is not a constant so first check
    pub fn getTrueConst(&self) -> Type {
        self.yes.as_any().downcast_ref::<Type>().unwrap().clone()
    }

    /// Returns the false value as a constant
    /// 
    /// ### Panics
    /// 
    /// panics if the false value is not a constant so first check
    pub fn getFalseConst(&self) -> Type {
        self.no.as_any().downcast_ref::<Type>().unwrap().clone()
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
            yes: yes,
            no: no,
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
            yes: yes,
            no: no,
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
            yes: yes,
            no: no,
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
            yes: yes,
            no: no,
        }));

        out
    }
}