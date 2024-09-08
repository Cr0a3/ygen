use std::collections::{HashMap, VecDeque};

use crate::{Obj::Linkage, IR::{Block, Const, Function, FunctionType, Module, TypeMetadata}};

use super::parser::{IrBlock, IrInstr, IrStmt};

/// Emits the ygen ir statements (emitted by the parser) into real definable ir statements
#[allow(unused)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IrGen {
    input: Vec<IrStmt>,

    funcs: Vec<Function>,
    consts: Vec<Const>,
}

impl IrGen {
    /// Creates a new ir generator
    pub fn new(input: Vec<IrStmt>) -> Self {
        Self {
            input: input,

            funcs: vec![],
            consts: vec![],
        }
    }

    fn gen_func(&mut self, name: String, ret: TypeMetadata, args:  (HashMap<String, TypeMetadata>, bool), body: HashMap<String, IrBlock>, scope: Linkage) {
        let mut ty = FunctionType::new(vec![], ret);

        for (_, arg) in &args.0 {
            ty.args.push( *arg );
        }

        let mut raw = Function {
            ty: ty,
            name: name,
            linkage: scope,
            blocks: VecDeque::new(),
        };

        for (name, block) in body {
            let mut raw_block = Block {
                name: name.to_owned(),
                nodes: vec![],
                varCount: 0,
            };

            for node in block.body {
                self.gen_node(node, &mut raw_block);
            }

            raw.blocks.push_back(raw_block);
        }

        self.funcs.push(raw);
    }

    fn gen_node(&self, node: IrInstr, block: &mut Block) {
        block.nodes.push( 
            node.inst
         );
    }

    /// lowers the incoming statemants into ygen ir
    pub fn gen(&mut self) {
        for stmt in self.input.clone() {
            match stmt {
                IrStmt::Func { name, ret, args, body, scope, location: _ } => self.gen_func(name, ret, args, body, scope),
                IrStmt::Const { name, data, location: _, scope } => self.gen_const(name, data, scope),
            }
        }
    }

    fn gen_const(&mut self, name: String, data: Vec<u8>, scope: Linkage) {
        let raw = Const {
            name: name,
            data: data,
            linkage: scope,
        };

        self.consts.push( raw );
    }

    /// emits the generated functions, constants, .. into a usable module
    pub fn module(&self) -> Module {
        let mut module = Module();

        for func in &self.funcs {
            module.add_raw( func.to_owned() );
        }

        for constant in &self.consts {
            module.add_raw_const( constant.to_owned() );
        }

        module
    }
}