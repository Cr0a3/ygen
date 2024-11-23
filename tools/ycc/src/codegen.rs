use std::collections::HashMap;

use lang_c::{ast::*, span::Node};
use ygen::{prelude::*, Target::Triple, IR::{FunctionType, Module, TypeMetadata}};

/// Code generation using ygen
pub struct CodeGeneration {
    src: Vec<Node<ExternalDeclaration>>,
    triple: Triple,
    pub module: Module,

}

impl CodeGeneration {
    pub fn new(src: Vec<Node<ExternalDeclaration>>, triple: Triple) -> Self {
        Self {
            src,
            triple,
            module: Module::new(),
        }
    }

    pub fn codegen(&mut self) {
        for node in self.src.clone() {
            match node.node {
                ExternalDeclaration::Declaration(_) => todo!(),
                ExternalDeclaration::StaticAssert(_) => todo!(),
                ExternalDeclaration::FunctionDefinition(node) => self.gen_func(node),
            }
        }
    }

    fn gen_func(&mut self, node: Node<FunctionDefinition>) {
        let ret = node.node.specifiers.get(0).expect("expected return type").node.clone();
        let ret: TypeWrapper = if let DeclarationSpecifier::TypeSpecifier(ty) = ret {
            ty.node.into()
        } else { panic!("{:?}", ret) };
        let ret: TypeMetadata = ret.into();

        let name = match node.node.declarator.node.kind.node {
            lang_c::ast::DeclaratorKind::Identifier(node) => node.node.name,
            _ => panic!("expected function name"),
        };

        let parsed_args = node.node.declarator.node.derived;

        let mut fn_ty = FunctionType::new(Vec::new(), ret);

        let DerivedDeclarator::Function(func) = &parsed_args.get(0).unwrap().node else { unreachable!() };
        let func = func.node.to_owned();

        if func.ellipsis == Ellipsis::Some {
            fn_ty.activate_dynamic_arguments();
        }

        for arg in func.parameters {
            let DeclaratorKind::Identifier(name) = arg.node.declarator.expect("expected argument name").node.kind.node else {panic!("expected ident")};
            let name = name.node.name;

            let DeclarationSpecifier::TypeSpecifier(ty) = arg.node.specifiers.get(0).unwrap().to_owned().node else { panic!("exepected type")};
            let ty: TypeWrapper = ty.node.into();
            let ty: TypeMetadata = ty.into();

            
            fn_ty.args.push((format!("%{name}"), ty));
        }
        
        let mut func = Function::new(name, fn_ty);

        func.addBlock("entry");

        let mut vars = HashMap::new();

        let mut n = 0;
        for (name, ty) in func.ty.args.clone().iter().rev() {
            let adr = func.BuildAlloca(*ty);
            func.BuildStore(adr.to_owned(), func.ty.arg(n));

            let name = name.chars().collect::<Vec<char>>();
            let mut name = name.iter();
            name.next();
            let name = name.collect::<String>();

            vars.insert(name.to_owned(), (adr, *ty));

            n += 1;
        }

        let Statement::Compound(body) = node.node.statement.node else { unreachable!() };

        for stmt in body {
            let stmt = stmt.node;

            match stmt {
                BlockItem::Statement(node) => self.gen_stmt(&mut func, &mut vars, node.node),
                BlockItem::Declaration(_node) => todo!("declerations"),
                BlockItem::StaticAssert(_) => todo!("static asserts"),
            }
        }
        
        self.module.add_raw(func);
    }

    fn gen_stmt(&mut self, func: &mut Function, vars: &mut HashMap<String, (Var, TypeMetadata)>, stmt: Statement) {
        match stmt {
            Statement::Labeled(node) => todo!(),
            Statement::Compound(vec) => todo!(),
            Statement::Expression(node) => todo!(),
            Statement::If(node) => todo!(),
            Statement::Switch(node) => todo!(),
            Statement::While(node) => todo!(),
            Statement::DoWhile(node) => todo!(),
            Statement::For(node) => todo!(),
            Statement::Goto(node) => todo!(),
            Statement::Continue => todo!(),
            Statement::Break => todo!(),
            Statement::Return(node) => self.gen_return(func, vars, node),
            Statement::Asm(node) => todo!(),
        }
    }

    fn gen_return(&mut self, func: &mut Function, vars: &mut HashMap<String, (Var, TypeMetadata)>, value: Option<Box<Node<Expression>>>) {
        if let Some(value) = value {
            let value = value.node;
            let to_return = self.gen_expression(func, vars, value);

            func.BuildRet(to_return);
        } else {
            func.BuildRet(Type::Void);
        }
    }

    fn gen_expression(&mut self, func: &mut Function, vars: &mut HashMap<String, (Var, TypeMetadata)>, expr: Expression) -> Var {
        match expr {
            Expression::Identifier(node) => self.gen_load_var(func, vars, node),
            Expression::Constant(node) => todo!(),
            Expression::StringLiteral(node) => todo!(),
            Expression::GenericSelection(node) => todo!(),
            Expression::Member(node) => todo!(),
            Expression::Call(node) => todo!(),
            Expression::CompoundLiteral(node) => todo!(),
            Expression::SizeOfTy(node) => todo!(),
            Expression::SizeOfVal(node) => todo!(),
            Expression::AlignOf(node) => todo!(),
            Expression::UnaryOperator(node) => todo!(),
            Expression::Cast(node) => todo!(),
            Expression::BinaryOperator(node) => todo!(),
            Expression::Conditional(node) => todo!(),
            Expression::Comma(vec) => todo!(),
            Expression::OffsetOf(node) => todo!(),
            Expression::Statement(node) => todo!(),
            Expression::VaArg(node) => todo!("va_args are unsuported"),
        }
    }

    fn gen_load_var(&mut self, func: &mut Function, vars: &mut HashMap<String, (Var, TypeMetadata)>, node: Box<Node<Identifier>>) -> Var {
        let name = node.node.name;

        let (ptr, value_ty) = vars.get(&name).expect(&format!("unallocated variable: {name}"));

        func.BuildLoad(ptr.to_owned(), *value_ty)
    }
}

struct TypeWrapper(pub TypeMetadata);

impl From<TypeSpecifier> for TypeWrapper {
    fn from(value: TypeSpecifier) -> Self {
        match value {
            TypeSpecifier::Void => TypeWrapper(TypeMetadata::Void),
            TypeSpecifier::Char => TypeWrapper(TypeMetadata::u8),
            TypeSpecifier::Short => TypeWrapper(TypeMetadata::u16),
            TypeSpecifier::Int => TypeWrapper(TypeMetadata::u32),
            TypeSpecifier::Long => TypeWrapper(TypeMetadata::u64),
            TypeSpecifier::Float => TypeWrapper(TypeMetadata::f32),
            TypeSpecifier::Double => TypeWrapper(TypeMetadata::f64),
            TypeSpecifier::Signed => TypeWrapper(TypeMetadata::i32),
            TypeSpecifier::Unsigned => TypeWrapper(TypeMetadata::u32),
            TypeSpecifier::Bool => TypeWrapper(TypeMetadata::i8),
            TypeSpecifier::Complex => todo!(),
            TypeSpecifier::Atomic(_) => todo!(),
            TypeSpecifier::Struct(_) => todo!(),
            TypeSpecifier::Enum(_) => todo!(),
            TypeSpecifier::TypedefName(_) => todo!(),
            TypeSpecifier::TypeOf(_) => todo!(),
            TypeSpecifier::TS18661Float(_) => todo!(),
        }
    }
}

impl Into<TypeMetadata> for TypeWrapper {
    fn into(self) -> TypeMetadata {
        self.0
    }
}