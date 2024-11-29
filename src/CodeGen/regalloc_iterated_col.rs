use std::collections::HashMap;

use crate::{ydbg, IR::{Function, TypeMetadata, Var}};

use super::{dag::{self, DagOp, DagOpTarget}, memory::Memory, reg::Reg};

/// Performes register allocation using iterated register coalescing
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItRegCoalAlloc<'a> {
    /// The free registers which are possibiy usable
    pub regs: Vec<Reg>,
    /// The current function
    pub curr_func: Option<&'a Function>,
    /// The target specific argument processor
    pub arg_processor: Option<fn(&mut ItRegCoalAlloc)>,
    /// The target specific memory processor
    pub mem_resolver: Option<fn(&mut ItRegCoalAlloc, &dag::DagNode, TypeMetadata) -> Memory>,
    /// Returns target specfic register overwrites for the node
    pub overwrite_proc: Option<fn(&dag::DagNode) -> Vec<Reg>>,
    /// The allocated vars
    pub vars: HashMap<String, DagOpTarget>,
    /// The current stack off
    pub stack: i32,
    /// Real types for allocated variables
    pub alloced_vars: HashMap::<String, TypeMetadata>
}

impl<'a> ItRegCoalAlloc<'a> {
    /// Creates a new iterated register coalescing register allocator
    pub fn new(regs: Vec<Reg>, arg_proc: fn(&mut ItRegCoalAlloc), mem_proc: fn(&mut ItRegCoalAlloc, &dag::DagNode, TypeMetadata) -> Memory, ov_proc: fn(&dag::DagNode) -> Vec<Reg>) -> Self {
        Self {
            regs: regs,
            curr_func: None,
            arg_processor: Some(arg_proc),
            mem_resolver: Some(mem_proc),
            overwrite_proc: Some(ov_proc),
            vars: HashMap::new(),
            alloced_vars: HashMap::new(),
            stack: 0,
        }
    }

    /// Runs the ircy allocator on the given input function
    pub fn init(&mut self, func: &'a Function) {
        ydbg!("[INIT] Register Allocator for {}", func.name);

        self.curr_func = Some(func);

        let Some(arg_proc) = self.arg_processor else {
            panic!("no current arg processor");
        };

        for block in &func.blocks {
            for node in &block.nodes {
                if let Some(alloca) = node.as_any().downcast_ref::<crate::prelude::Alloca>() {
                    self.alloced_vars.insert(alloca.inner1.name.to_owned(), alloca.inner2);
                }
            }
        }

        arg_proc(self);
    }

    /// Applys the var locations to the dag
    pub fn apply(&mut self, dag: &mut dag::DagNode) {
        self.sort();
        self.alloc_for_node(dag);

        if let Some(out) = &mut dag.out {
            if let DagOpTarget::UnallocatedVar(var) = &out.target {
                if let Some(target) = self.vars.get(&var.name) {
                    out.allocated = true;
                    out.target = target.to_owned();
                }
            }
        }

        for operand in &mut dag.ops {
            if let DagOpTarget::UnallocatedVar(var) = &operand.target {
                if let Some(target) = self.vars.get(&var.name) {
                    operand.allocated = true;
                    operand.target = target.to_owned();
                }
            }
        }
    }

    /// Sorts the free register list based on their score
    pub fn sort(&mut self) {
        self.regs.sort_by(|a, b| {
            use std::cmp::Ordering;

            if a.score() > b.score() { Ordering::Less }
            else if a.score() < b.score() { Ordering::Greater }
            else if a.score() == b.score() { Ordering::Equal }
            else {
                panic!("if something isn't bigger, smaller or equal what can it be?");
            }
        });
    }

    fn alloc_for_node(&mut self, dag: &mut dag::DagNode) {
        if !dag.out.is_some() { return; }

        let out_var: Var;

        let mut ty = if let Some(out) = &dag.out {
            if out.allocated { 
                if let DagOpTarget::Reg(reg) = out.target {
                    self.regs = self.regs.iter().filter(|r| **r != reg).map(|r| *r).collect::<Vec<Reg>>();
                }
                return; 
            }
        
            if let DagOpTarget::UnallocatedVar(v) = &out.target {
                out_var = v.to_owned();
                v.ty
            } else {
                todo!()
            }
        } else { unreachable!() };

        if let Some(real_ty) = self.alloced_vars.get(&out_var.name) {
            ty = *real_ty;
        }

        if let Some(mut free) = self.get_fitting_reg(dag, ty) {
            free.set_size(ty.byteSize());
    
            dag.out = Some(DagOp::reg(free));
            self.vars.insert(out_var.name, DagOpTarget::Reg(free));
        } else {
            let mem = self.alloc_stack(dag, ty);
        
            dag.out = Some(DagOp::mem(mem));
            self.vars.insert(out_var.name, DagOpTarget::Mem(mem));
        };
    }

    /// Returns a register that fits the type
    pub fn get_fitting_reg(&mut self, node: &dag::DagNode, mut ty: TypeMetadata) -> Option<Reg> {
        // TODOD: register wishes
        
        // output of a cmp is not the same as the input type - it's a bool
        use dag::DagOpCode::*;
        if matches!(node.opcode, CmpEq | CmpNe | CmpLt | CmpGt | CmpLte | CmpGte) {
            ty = TypeMetadata::u8;
        }

        if let Some(out) = &node.out {
            if out.should_be_mem {
                return None; // now our allocator thinks: FUCK THE REGISTERS RAN OUT i need to now use the stack
            }
        }

        let mut index = 0;

        for mut reg in self.regs.to_vec() {
            // check for simd
            if ty.isVectorTy() && reg.is_simd(&ty.getVectorTy()) {
                self.regs.remove(index);
                reg.size = ty.bitSize();
                return Some(reg);
            }

            // normal registers here

            if ty.float() && reg.is_fp() {
                self.regs.remove(index);
                reg.size = ty.byteSize();
                return Some(reg);
            }

            
            if ty.intenger() && reg.is_gr() {
                self.regs.remove(index);
                reg.size = ty.byteSize();
                return Some(reg);
            }

            index += 1;
        }

        None
    }

    /// Allocates a stack position
    pub fn alloc_stack(&mut self, node: &dag::DagNode, mut ty: TypeMetadata) -> Memory {
        // output of a cmp is not the same as the input type - it's a bool
        use dag::DagOpCode::*;
        if matches!(node.opcode, CmpEq | CmpNe | CmpLt | CmpGt | CmpLte | CmpGte) {
            ty = TypeMetadata::u8;
        }

        let Some(mem_resolver) = self.mem_resolver else {
            panic!("no registered target mem_resolver")
        };

        mem_resolver(self, node, ty)
    }

    /// Returns a location for the given temporary 
    pub fn request_tmp(&mut self, _: &dag::DagNode, tmp: &dag::DagTmpInfo) -> dag::DagOpTarget {
        if tmp.requires_mem {
            let mem = self.alloc_stack(&&dag::DagNode::ret(TypeMetadata::u8), tmp.size);
            
            return DagOpTarget::Mem(mem);
        }
        
        
        // we pass in any node - it just doesn't matter which one
        let Some(reg) = self.get_fitting_reg(&&dag::DagNode::ret(TypeMetadata::u8), tmp.size) else {
            panic!("unable to get fitting register\nTODO: implement spills and recalls for tmps");
        };
        
        ydbg!("[REG ALLOC] resolved temporary ({:?}) -> {}", tmp, reg);

        DagOpTarget::Reg(reg)
    }
}

/// A base for performing register allocation using iterated register coalescing
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItRegCoalAllocBase {
    /// The free registers which are possibiy usable
    pub regs: Vec<Reg>,
    /// The target specific argument processor
    pub arg_processor: Option<fn(&mut ItRegCoalAlloc)>,
    /// The target specific memory processor
    pub mem_processor: Option<fn(&mut ItRegCoalAlloc, &dag::DagNode, TypeMetadata) -> Memory>,
    /// The target specific overwrite processor
    pub overwrite_proc: Option<fn(&dag::DagNode) -> Vec<Reg>>,
    /// stack offset
    pub stack: i32,
}

impl ItRegCoalAllocBase {
    /// Creates a new irc reg allocator with the given function
    /// and runs register allocation for the arguments
    pub fn fork<'a>(&self, func: &'a Function) -> ItRegCoalAlloc<'a> {
        let mut alloc = ItRegCoalAlloc::new(
            self.regs.to_owned(), 
            self.arg_processor.unwrap(), 
            self.mem_processor.unwrap(),
            self.overwrite_proc.unwrap(),
        );
        

        alloc.stack = self.stack;
        alloc.init(func);

        alloc
    }
}