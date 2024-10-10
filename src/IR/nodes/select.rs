use super::{Assign, Ir, Select};
use crate::{prelude::{Type, TypeMetadata, Var}, Support::ColorClass, IR::Function};

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

    fn compile(&self, registry: &mut crate::Target::TargetBackendDescr) {
        registry.compile_select_tt(self)
    }

    fn compile_dir(&self, compiler: &mut crate::CodeGen::IrCodeGenHelper, block: &crate::prelude::Block) {
        compiler.compile_select_tt(self, block)
    }

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

    fn inputs(&self) -> Vec<Var> {
        vec![self.cond.to_owned()]
    }

    fn output(&self) -> Option<Var> {
        Some(self.out.clone())
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

    fn compile(&self, registry: &mut crate::Target::TargetBackendDescr) {
        registry.compile_select_vt(self)
    }

    fn compile_dir(&self, compiler: &mut crate::CodeGen::IrCodeGenHelper, block: &crate::prelude::Block) {
        compiler.compile_select_vt(self, block)
    }

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

    fn inputs(&self) -> Vec<Var> {
        vec![self.cond.to_owned(), self.yes.clone()]
    }

    fn output(&self) -> Option<Var> {
        Some(self.out.clone())
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

    fn compile(&self, registry: &mut crate::Target::TargetBackendDescr) {
        registry.compile_select_tv(self)
    }

    fn compile_dir(&self, compiler: &mut crate::CodeGen::IrCodeGenHelper, block: &crate::prelude::Block) {
        compiler.compile_select_tv(self, block)
    }

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

    fn inputs(&self) -> Vec<Var> {
        vec![self.cond.to_owned()]
    }

    fn output(&self) -> Option<Var> {
        Some(self.out.clone())
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

    fn compile(&self, registry: &mut crate::Target::TargetBackendDescr) {
        registry.compile_select_vv(self)
    }

    fn compile_dir(&self, compiler: &mut crate::CodeGen::IrCodeGenHelper, block: &crate::prelude::Block) {
        compiler.compile_select_vv(self, block)
    }

    fn maybe_inline(&self, _: &std::collections::HashMap<String, Type>) -> Option<Box<dyn Ir>> {
        None
    }

    fn eval(&self) -> Option<Box<dyn Ir>> {
        if self.yes == self.no {
            return Some(Assign::new(self.out.clone(), self.yes.clone()));
        }

        None
    }

    fn inputs(&self) -> Vec<Var> {
        vec![self.cond.to_owned(), self.yes.clone()]
    }

    fn output(&self) -> Option<Var> {
        Some(self.out.clone())
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