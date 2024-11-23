use crate::prelude::*;
use super::*;
use std::fmt::Debug;

/// extracts an element out of a vector
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetElemPtr {
    pub(crate) ptr: Var,
    pub(crate) ty: TypeMetadata,
    pub(crate) out: Var,

    pub(crate) index: Var,
}

impl Ir for GetElemPtr {
    fn dump(&self) -> String {
        format!("{} = getelemptr {} {}, {} {}, {}", 
            self.out.name, 

            self.ptr.ty, 
            self.ptr.name, 

            self.out.ty, 
            self.out.name,

            self.ty,
        )
    }

    fn dumpColored(&self, profile: ColorProfile) -> String {
        format!("{} = {} {} {}, {} {}, {}", 
            profile.markup(&self.out.name, ColorClass::Var),
            profile.markup("getelemptr", ColorClass::Instr),
            profile.markup(&self.ptr.ty.to_string(), ColorClass::Ty),
            profile.markup(&self.ptr.name, ColorClass::Var),
            profile.markup(&self.out.ty.to_string(), ColorClass::Ty),
            profile.markup(&self.out.name, ColorClass::Value),
            profile.markup(&self.ty.to_string(), ColorClass::Value),
        )
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn verify(&self, _: FunctionType) -> Result<(), VerifyError> {
        Ok(())
    }

    fn clone_box(&self) -> Box<dyn Ir> {
        Box::new( self.clone() )
    }

    fn inputs(&self) -> Vec<Var> {
        vec![self.ptr.to_owned(), self.index.to_owned()]
    }

    fn inputs_mut(&mut self) -> Vec<&mut Var> {
        vec![&mut self.ptr, &mut self.index]
    }

    fn output(&self) -> Option<Var> {
        Some(self.out.to_owned())
    }

    fn ty(&self) -> Option<TypeMetadata> {
        Some(self.ty)
    }
}

impl EvalOptVisitor for GetElemPtr {
    fn maybe_inline(&self, _: &HashMap<String, Type>) -> Option<Box<dyn Ir>> {
        None
    }

    fn eval(&self) -> Option<Box<dyn Ir>> {
        None
    }
}

impl IsNode for GetElemPtr {
    fn is_getelemptr(&self) -> bool {
        true
    }
}

impl Function {
    /// Builds the `getelemptr` node.
    /// It is used for example in this c code:
    /// ```no-run
    /// int test(int a[5]) {
    ///     return a[1];
    /// }
    /// ```
    /// 
    /// It would be compiled to this:
    /// ```no-run
    /// define i32 @test(ptr %0) {
    ///   entry:
    ///     %const0 = i32 1
    ///     %1 = getelemptr ptr %0, i32 %const0, i32
    ///     ret i32 %1
    /// }
    /// ```
    pub fn BuildGetelemptr(&mut self, var: Var, index: Var, ty: TypeMetadata) -> Var {
        let block = self.blocks.back_mut().expect("expected current block");

        let out = Var::new(block, ty);

        block.push_ir(Box::new(GetElemPtr {
            index: index,
            ptr: var,
            out: out.clone(),
            ty: ty,
        }));

        out.clone()
    }
}