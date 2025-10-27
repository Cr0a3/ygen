use std::collections::HashMap;

use lang_c::{ast::*, span::Node};
use ygen::{prelude::*, IR::{FunctionType, Module, TypeMetadata}};

/// Code generation using ygen
pub struct CodeGeneration {
    src: Vec<Node<ExternalDeclaration>>,
    ids: HashMap<String, FuncId>,
    pub module: Module,
    if_count: usize,

}

impl CodeGeneration {
    pub fn new(src: Vec<Node<ExternalDeclaration>>) -> Self {
        Self {
            if_count: 0,
            src: src,
            module: Module::new(),
            ids: HashMap::new(),
        }
    }

    pub fn codegen(&mut self) {
        self.gather_ids();

        for node in self.src.clone() {
            match node.node {
                ExternalDeclaration::Declaration(_) => {}, // nothing todo cuz it's either later defined or extern
                ExternalDeclaration::StaticAssert(_) => todo!(),
                ExternalDeclaration::FunctionDefinition(node) => self.gen_func(node),
            }
        }
    }

    fn get_fnid(&self, name: &str) -> &FuncId {
        self.ids.get(name).expect(&format!("Unknown function: {name}"))
    }

    fn gather_ids(&mut self) {
        for node in &self.src {
            if let ExternalDeclaration::Declaration(decl) = &node.node {
                let decl = &decl.node;

                let mut ret: Option<TypeMetadata> = None;
                for specifier in &decl.specifiers {
                    if let DeclarationSpecifier::TypeSpecifier(ty) = &specifier.node {
                        let ty = ty.node.to_owned();
                        let ty: TypeWrapper = ty.into();
                        ret = Some(ty.into());
                    }
                }
                let ret = ret.expect("expected return type");

                let name = match &decl.declarators[0].node.declarator.node.kind.node {
                    lang_c::ast::DeclaratorKind::Identifier(node) => node.node.name.to_owned(),
                    _ => panic!("expected function name"),
                };
                let parsed_args = decl.declarators[0].node.declarator.node.derived.to_vec();

                let mut fn_ty = FunctionType::new(Vec::new(), ret);

                if let Some(node) = parsed_args.get(0).cloned() {
                    if let DerivedDeclarator::Function(func) = node.node {
                        let func = func.node.to_owned();
            
                        if func.ellipsis == Ellipsis::Some {
                            fn_ty.activate_dynamic_arguments();
                        }
            
                        for arg in func.parameters {
                            if arg.node.declarator.is_none() {
                                let DeclarationSpecifier::TypeSpecifier(ty) = arg.node.specifiers.get(0).unwrap().to_owned().node else { panic!("exepected type")};
                                let ty: TypeWrapper = ty.node.into();
                                let ty: TypeMetadata = ty.into();
                                fn_ty.add_arg(ty);
                                continue;
                            }

                            let DeclaratorKind::Identifier(name) = arg.node.declarator.expect("expected argument name").node.kind.node else {panic!("expected ident")};
                            let name = name.node.name;
            
                            let DeclarationSpecifier::TypeSpecifier(ty) = arg.node.specifiers.get(0).unwrap().to_owned().node else { panic!("exepected type")};
                            let ty: TypeWrapper = ty.node.into();
                            let ty: TypeMetadata = ty.into();
            
                            fn_ty.args.push((format!("%{name}"), ty));
                        }
                    }
                }
            
                let id = FuncId::new(&name, fn_ty);
                self.ids.insert(name.to_owned(), id);
            }
        }
    }

    fn gen_func(&mut self, node: Node<FunctionDefinition>) {
        let mut extrn = false;
        let mut ret: Option<TypeMetadata> = None;

        for specifier in node.node.specifiers {
            let specifier = specifier.node;

            if let DeclarationSpecifier::StorageClass(storage_class) = &specifier {
                let storage_class = &storage_class.node;

                match storage_class {
                    StorageClassSpecifier::Typedef => todo!(),
                    StorageClassSpecifier::Extern => extrn = true,
                    StorageClassSpecifier::Static => todo!(),
                    StorageClassSpecifier::ThreadLocal => todo!(),
                    StorageClassSpecifier::Auto => todo!(),
                    StorageClassSpecifier::Register => todo!(),
                }
            }

            if let DeclarationSpecifier::TypeSpecifier(ty) = specifier {
                let ty = ty.node;
                let ty: TypeWrapper = ty.into();
                ret = Some(ty.into());
            }
        }

        let ret = ret.expect("expected return type");

        let name = match node.node.declarator.node.kind.node {
            lang_c::ast::DeclaratorKind::Identifier(node) => node.node.name,
            _ => panic!("expected function name"),
        };

        let parsed_args = node.node.declarator.node.derived;

        let mut fn_ty = FunctionType::new(Vec::new(), ret);

        if let Some(node) = parsed_args.get(0).cloned() {
            if let DerivedDeclarator::Function(func) = node.node {
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
            }
        }

        let mut func = Function::new(name, fn_ty);

        if extrn {
            func.extrn();
        }

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

        self.gen_compount(&mut func, &mut vars, body);
        
        self.module.add_raw(func);
    }

    fn gen_compount(&mut self, func: &mut Function, vars: &mut HashMap<String, (Var, TypeMetadata)>, comp: Vec<Node<BlockItem>>) {
        for block in comp {
            let block = block.node;

            match block {
                BlockItem::Statement(node) => self.gen_stmt(func, vars, node.node),
                BlockItem::Declaration(node) => self.gen_decl(func, vars, node.node),
                BlockItem::StaticAssert(_) => todo!("static asserts"),
            }
        }
    }

    fn gen_stmt(&mut self, func: &mut Function, vars: &mut HashMap<String, (Var, TypeMetadata)>, stmt: Statement) {
        match stmt {
            Statement::Labeled(_node) => todo!(),
            Statement::Compound(comp) => self.gen_compount(func, vars, comp),
            Statement::Expression(node) => {
                if let Some(expr) = node {
                    let expr = expr.node;
                    self.gen_stmt_expression(func, vars, expr)
                } else { panic!("empty expressions are currently not supported") }
            },
            Statement::If(node) => self.gen_if(func, vars, node.node),
            Statement::Switch(_node) => todo!(),
            Statement::While(_node) => todo!(),
            Statement::DoWhile(_node) => todo!(),
            Statement::For(_node) => todo!(),
            Statement::Goto(_node) => todo!(),
            Statement::Continue => todo!(),
            Statement::Break => todo!(),
            Statement::Return(node) => self.gen_return(func, vars, node),
            Statement::Asm(_node) => todo!(),
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
            Expression::Constant(node) => self.gen_const(func, node),
            Expression::StringLiteral(_node) => todo!(),
            Expression::GenericSelection(_node) => todo!(),
            Expression::Member(_node) => todo!(),
            Expression::Call(node) => self.gen_call(func, vars, node.node),
            Expression::CompoundLiteral(_node) => todo!(),
            Expression::SizeOfTy(_node) => todo!(),
            Expression::SizeOfVal(_node) => todo!(),
            Expression::AlignOf(_node) => todo!(),
            Expression::UnaryOperator(_node) => todo!(),
            Expression::Cast(_node) => todo!(),
            Expression::BinaryOperator(node) => self.gen_binary(func, vars, node.node),
            Expression::Conditional(_node) => todo!(),
            Expression::Comma(_vec) => todo!(),
            Expression::OffsetOf(_node) => todo!(),
            Expression::Statement(_node) => todo!(),
            Expression::VaArg(_node) => todo!("va_args are unsuported"),
        }
    }

    fn gen_stmt_expression(&mut self, func: &mut Function, vars: &mut HashMap<String, (Var, TypeMetadata)>, expr: Expression) {
        match expr {
            Expression::Call(node) => self.gen_call_wret(func, vars, node.node),
            Expression::Statement(node) => self.gen_stmt(func, vars, node.node),
            _ => todo!(),
        }
    }

    fn gen_load_var(&mut self, func: &mut Function, vars: &mut HashMap<String, (Var, TypeMetadata)>, node: Box<Node<Identifier>>) -> Var {
        let name = node.node.name;

        let (ptr, value_ty) = vars.get(&name).expect(&format!("unallocated variable: {name}"));

        func.BuildLoad(ptr.to_owned(), *value_ty)
    }

    fn gen_const(&mut self, func: &mut Function, node: Box<Node<Constant>>) -> Var {
        let val = node.node;
        
        match val {
            Constant::Integer(integer) => {
                let val = match integer.base {
                    IntegerBase::Decimal => (*integer.number).parse::<i64>().expect("number parsing error"),
                    IntegerBase::Octal => todo!(),
                    IntegerBase::Hexadecimal => todo!(),
                    IntegerBase::Binary => todo!(),
                };

                let ty = match integer.suffix.size {
                    IntegerSize::Int => if integer.suffix.unsigned { TypeMetadata::u32 } else { TypeMetadata::i32 },
                    IntegerSize::Long | IntegerSize::LongLong => if integer.suffix.unsigned { TypeMetadata::u64 } else { TypeMetadata::i64 },
                };

                func.BuildAssign(Type::from_int(ty, val as f64))
            },
            Constant::Float(_float) => todo!(),
            Constant::Character(_chr) => todo!(),
        }
    }

    fn gen_call_wret(&mut self, func: &mut Function, vars: &mut HashMap<String, (Var, TypeMetadata)>, node: CallExpression) {
        self.gen_call(func, vars, node);
    }

    fn gen_call(&mut self, func: &mut Function, vars: &mut HashMap<String, (Var, TypeMetadata)>, node: CallExpression) -> Var {
        let Expression::Identifier(func_name) = node.callee.node else { todo!("Only constant functions are currently supported") };
        let func_name = func_name.node.name;

        let mut args = Vec::new();

        for arg in node.arguments {
            let arg = arg.node;

            let arg = self.gen_expression(func, vars, arg);
            args.push(IROperand::Var(arg));
        }

        func.BuildCall(self.get_fnid(&func_name), args)
    }

    fn gen_binary(&mut self, func: &mut Function, vars: &mut HashMap<String, (Var, TypeMetadata)>, expr: BinaryOperatorExpression) -> Var {
        let ls = self.gen_expression(func, vars, expr.lhs.node);
        let rs = self.gen_expression(func, vars, expr.rhs.node);

        match expr.operator.node {
            BinaryOperator::Index => todo!(),
            BinaryOperator::Multiply => func.BuildMul(ls, rs),
            BinaryOperator::Divide => func.BuildDiv(ls, rs),
            BinaryOperator::Modulo => func.BuildRem(ls, rs),
            BinaryOperator::Plus => func.BuildAdd(ls, rs),
            BinaryOperator::Minus => func.BuildSub(ls, rs),
            BinaryOperator::ShiftLeft => func.BuildShl(ls, rs),
            BinaryOperator::ShiftRight => func.BuildShr(ls, rs),
            BinaryOperator::Less => func.BuildCmp(CmpMode::LessThan, ls, rs),
            BinaryOperator::Greater => func.BuildCmp(CmpMode::GreaterThan, ls, rs),
            BinaryOperator::LessOrEqual => func.BuildCmp(CmpMode::LessThanOrEqual, ls, rs),
            BinaryOperator::GreaterOrEqual => func.BuildCmp(CmpMode::GreaterThanOrEqual, ls, rs),
            BinaryOperator::Equals => func.BuildCmp(CmpMode::Equal, ls, rs),
            BinaryOperator::NotEquals => func.BuildCmp(CmpMode::NotEuqal, ls, rs),
            BinaryOperator::BitwiseAnd => func.BuildAnd(ls, rs),
            BinaryOperator::BitwiseXor => func.BuildXor(ls, rs),
            BinaryOperator::BitwiseOr => func.BuildOr(ls, rs),
            BinaryOperator::LogicalAnd => todo!(),
            BinaryOperator::LogicalOr => todo!(),
            BinaryOperator::Assign => todo!(),
            BinaryOperator::AssignMultiply => todo!(),
            BinaryOperator::AssignDivide => todo!(),
            BinaryOperator::AssignModulo => todo!(),
            BinaryOperator::AssignPlus => todo!(),
            BinaryOperator::AssignMinus => todo!(),
            BinaryOperator::AssignShiftLeft => todo!(),
            BinaryOperator::AssignShiftRight => todo!(),
            BinaryOperator::AssignBitwiseAnd => todo!(),
            BinaryOperator::AssignBitwiseXor => todo!(),
            BinaryOperator::AssignBitwiseOr => todo!(),
        }
    }

    fn gen_decl(&mut self, func: &mut Function, vars: &mut HashMap<String, (Var, TypeMetadata)>, decl: Declaration) {
        let mut var_ty = None;

        for specifier in decl.specifiers {
            if let DeclarationSpecifier::TypeSpecifier(ty) = &specifier.node {
                let ty = ty.node.to_owned();
                let ty: TypeWrapper = ty.into();

                var_ty = Some(ty.into());
            }
        }

        let var_ty: TypeMetadata = var_ty.expect("expected type");
    
        for decl in decl.declarators {
            let decl = decl.node;

            let name = match decl.declarator.node.kind.node {
                DeclaratorKind::Identifier(node) => node.node.name,
                DeclaratorKind::Abstract => todo!(),
                DeclaratorKind::Declarator(_) => todo!(),
            };

            let ptr = func.BuildAlloca(var_ty);

            if let Some(init) = decl.initializer {
                let init = init.node;

                if let Initializer::Expression(expr) = init {
                    let expr = expr.node;

                    let tmp= self.gen_expression(func, vars, expr);
                    
                    func.BuildStore(ptr.clone(), tmp);
                } else { todo!("list initializer") }
            };

            vars.insert(name, (ptr, var_ty));
        }
    }

    fn gen_if(&mut self, func: &mut Function, vars: &mut HashMap<String, (Var, TypeMetadata)>, if_stmt: IfStatement) {
        let cond = self.gen_expression(func, vars, if_stmt.condition.node);

        let curr = func.currentBlock();
        let then_block = func.addBlock(&format!("if{}.then", self.if_count));
        let else_block = func.addBlock(&format!("if{}.else", self.if_count));
        let after_block = func.addBlock(&format!("if{}.after", self.if_count));
        
        func.setCurrBlock(curr);
        self.if_count += 1;

        func.BuildBrCond(cond, &then_block, &else_block);

        func.setCurrBlock(then_block);
        self.gen_stmt(func, vars, if_stmt.then_statement.node);
        func.BuildBr(&after_block);

        func.setCurrBlock(else_block);
        
        if let Some(else_stmt) = if_stmt.else_statement {
            self.gen_stmt(func, vars, else_stmt.node);
        }

        func.BuildBr(&after_block);

        func.setCurrBlock(after_block);
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