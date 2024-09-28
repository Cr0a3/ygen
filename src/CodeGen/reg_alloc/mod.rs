use std::{any::TypeId, collections::HashMap};

use prep::RegAllocPrep;

use crate::{prelude::Ir, Support::TypeSwitch, Target::{Arch, CallConv}, IR::{Function, TypeMetadata}};

use super::{MachineCallingConvention, Reg, RegVec, VarLocation};

mod prep;

/// Ygens register allocator
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegAlloc {
    pub free_registers: RegVec,
    pub arch: Arch,
    pub call: MachineCallingConvention,
    pub stack_off: i64,

    pub(crate) vars: HashMap<String, VarLocation>,
    pub(crate) var_types: HashMap<String, TypeMetadata>,
}

impl RegAlloc {
    /// Creates an new register allocator
    pub fn new(arch: Arch, call: CallConv) -> Self {
        let call = MachineCallingConvention { 
            call_conv: call 
        };

        Self {
            free_registers: RegVec::new(),
            arch: arch,
            call: call,
            stack_off: call.shadow(arch),

            vars: HashMap::new(),
            var_types: HashMap::new(),
        }
    }

    fn arg_prep(&mut self, func: &Function) {
        let func = &func.ty;

        let mut num = 0;

        for ty in &func.args {
            let location = {
                if let Some(reg) = self.call.args(self.arch).get(num) {
                    VarLocation::Reg(match reg {
                        Reg::x64(x64) => Reg::x64(x64.sub_ty(*ty)),
                    })
                } else {
                    todo!("The new system currently doesn't support memory")
                }
            };

            let name = || func.arg(num).name;

            self.vars.insert(
                name(), 
                location
            );

            self.var_types.insert(name(), *ty);

            num += 1;
        }
    }

    /// runs all variable allocations for the function
    pub fn run_alloc(&mut self, func: &Function) {
        self.arg_prep(func);

        for block in &func.blocks {
            for node in &block.nodes {
                self.node_prep(node);
            }
        }
    }

    fn node_prep(&mut self, node: &Box<dyn Ir>) {
        /*let out = node.output();

        if let Some(out) = out {
            let location = self.alloc_rv(out.ty);
            self.vars.insert(out.name.to_owned(), location);
        }*/

        let mut matcher = TypeSwitch::new();

        use crate::prelude::*;

        {
            matcher.case(TypeId::of::<Alloca<Var, TypeMetadata>>(), 1);
            matcher.case(TypeId::of::<Assign<Var, Type>>(), 2);
            matcher.case(TypeId::of::<Assign<Var, Var>>(), 3);
            matcher.case(TypeId::of::<Assign<Var, Const>>(), 4);
            matcher.case(TypeId::of::<Br<Box<Block>>>(), 5);
            matcher.case(TypeId::of::<BrCond<Var, Block, Block>>(), 6);
            matcher.case(TypeId::of::<Call<Function, Vec<Var>, Var>>(), 7);
            matcher.case(TypeId::of::<Cast<Var, TypeMetadata, Var>>(), 8);
            matcher.case(TypeId::of::<Cmp>(), 9);
            matcher.case(TypeId::of::<DebugNode>(), 10);
            matcher.case(TypeId::of::<Load<Var, Var, TypeMetadata>>(), 11);
            matcher.case(TypeId::of::<Return<Type>>(), 12);
            matcher.case(TypeId::of::<Return<Var>>(), 13);
            matcher.case(TypeId::of::<Store<Var, Var>>(), 14);
            matcher.case(TypeId::of::<Store<Var, Type>>(), 15);
            
            // math node
            matcher.case(TypeId::of::<Add<Var, Var, Var>>(), 16);
            matcher.case(TypeId::of::<Add<Var, Type, Var>>(), 17);
            matcher.case(TypeId::of::<Add<Type, Type, Var>>(), 18);
            matcher.case(TypeId::of::<Sub<Var, Var, Var>>(), 19);
            matcher.case(TypeId::of::<Sub<Var, Type, Var>>(), 20);
            matcher.case(TypeId::of::<Sub<Type, Type, Var>>(), 21);
            matcher.case(TypeId::of::<Xor<Var, Var, Var>>(), 22);
            matcher.case(TypeId::of::<Xor<Var, Type, Var>>(), 23);
            matcher.case(TypeId::of::<Xor<Type, Type, Var>>(), 24);
            matcher.case(TypeId::of::<Or<Var, Var, Var>>(), 25);
            matcher.case(TypeId::of::<Or<Var, Type, Var>>(), 26);
            matcher.case(TypeId::of::<Or<Type, Type, Var>>(), 27);
            matcher.case(TypeId::of::<And<Var, Var, Var>>(), 28);
            matcher.case(TypeId::of::<And<Var, Type, Var>>(), 29);
            matcher.case(TypeId::of::<And<Type, Type, Var>>(), 30);
            matcher.case(TypeId::of::<Mul<Var, Var, Var>>(), 31);
            matcher.case(TypeId::of::<Mul<Var, Type, Var>>(), 32);
            matcher.case(TypeId::of::<Mul<Type, Type, Var>>(), 33);
            matcher.case(TypeId::of::<Div<Var, Var, Var>>(), 34);
            matcher.case(TypeId::of::<Div<Var, Type, Var>>(), 35);
            matcher.case(TypeId::of::<Div<Type, Type, Var>>(), 36);
        }
        if let Some(switched) = matcher.switch(node.as_any().type_id()) {
            match *switched {
                1 => self.prep(node.as_any().downcast_ref::<Alloca<Var, TypeMetadata>>().unwrap()),
                2 => self.prep(node.as_any().downcast_ref::<Assign<Var, Type>>().unwrap()),
                3 => self.prep(node.as_any().downcast_ref::<Assign<Var, Var>>().unwrap()),
                4 => self.prep(node.as_any().downcast_ref::<Assign<Var, Const>>().unwrap()),
                5 => self.prep(node.as_any().downcast_ref::<Br<Box<Block>>>().unwrap()),
                6 => self.prep(node.as_any().downcast_ref::<BrCond<Var, Block, Block>>().unwrap()),
                7 => self.prep(node.as_any().downcast_ref::<Call<Function, Vec<Var>, Var>>().unwrap()),
                8 => self.prep(node.as_any().downcast_ref::<Cast<Var, TypeMetadata, Var>>().unwrap()),
                9 => self.prep(node.as_any().downcast_ref::<Cmp>().unwrap()),
                10 => {}, // doesn't add or use any variables
                11 => self.prep(node.as_any().downcast_ref::<Load<Var, Var, TypeMetadata>>().unwrap()),
                12 => self.prep(node.as_any().downcast_ref::<Return<Type>>().unwrap()),
                13 => self.prep(node.as_any().downcast_ref::<Return<Var>>().unwrap()),
                14 => self.prep(node.as_any().downcast_ref::<Store<Var, Var>>().unwrap()),
                15 => self.prep(node.as_any().downcast_ref::<Store<Var, Type>>().unwrap()),
                
                16 => self.prep(node.as_any().downcast_ref::<Add<Var, Var, Var>>().unwrap()),
                17 => self.prep(node.as_any().downcast_ref::<Add<Var, Type, Var>>().unwrap()),
                18 => self.prep(node.as_any().downcast_ref::<Add<Type, Type, Var>>().unwrap()),
                19 => self.prep(node.as_any().downcast_ref::<Sub<Var, Var, Var>>().unwrap()),
                20 => self.prep(node.as_any().downcast_ref::<Sub<Var, Type, Var>>().unwrap()),
                21 => self.prep(node.as_any().downcast_ref::<Sub<Type, Type, Var>>().unwrap()),
                22 => self.prep(node.as_any().downcast_ref::<Xor<Var, Var, Var>>().unwrap()),
                23 => self.prep(node.as_any().downcast_ref::<Xor<Var, Type, Var>>().unwrap()),
                24 => self.prep(node.as_any().downcast_ref::<Xor<Type, Type, Var>>().unwrap()),
                25 => self.prep(node.as_any().downcast_ref::<Or<Var, Var, Var>>().unwrap()),
                26 => self.prep(node.as_any().downcast_ref::<Or<Var, Type, Var>>().unwrap()),
                27 => self.prep(node.as_any().downcast_ref::<Or<Type, Type, Var>>().unwrap()),
                28 => self.prep(node.as_any().downcast_ref::<And<Var, Var, Var>>().unwrap()),
                29 => self.prep(node.as_any().downcast_ref::<And<Var, Type, Var>>().unwrap()),
                30 => self.prep(node.as_any().downcast_ref::<And<Type, Type, Var>>().unwrap()),
                31 => self.prep(node.as_any().downcast_ref::<Mul<Var, Var, Var>>().unwrap()),
                32 => self.prep(node.as_any().downcast_ref::<Mul<Var, Type, Var>>().unwrap()),
                33 => self.prep(node.as_any().downcast_ref::<Mul<Type, Type, Var>>().unwrap()),
                34 => self.prep(node.as_any().downcast_ref::<Div<Var, Var, Var>>().unwrap()),
                35 => self.prep(node.as_any().downcast_ref::<Div<Var, Type, Var>>().unwrap()),
                36 => self.prep(node.as_any().downcast_ref::<Div<Type, Type, Var>>().unwrap()),
                _ => todo!(),
            }
        } else {
            todo!("implement type for {}", node.dump())
        }

    }

    fn alloc_rv(&mut self, ty: TypeMetadata) -> VarLocation {
        if let Some(reg) = self.free_registers.pop(self.arch) {
            VarLocation::Reg( match reg {
                Reg::x64(x64_reg) => Reg::x64( x64_reg.sub_ty(ty) ),
            } )
        } else {
            self.alloc_stack(ty)
        }
    }

    pub(crate) fn alloc_stack(&mut self, _: TypeMetadata) -> VarLocation {
        let ret = VarLocation::Mem(self.stack_off);
            
        self.stack_off += self.call.align(self.arch);
        
        ret
    }
}