use super::*;

macro_rules! CompileMathVarVar {
    ($name:ident, $node:ident, $mnemonic:expr) => {
        pub(crate) fn $name(node: &$node<Var, Var, Var>, registry: &mut TargetBackendDescr) -> Vec<Instr> {
            let infos = &mut registry.backend;
            let block = registry.block.unwrap();

            let loc1 = if let Some(loc1) = infos.varsStorage.get(&node.inner1) {
                loc1.clone()
            } else {
                panic!("unknown variable: {:?}", node.inner1)
            };
            
            let loc2 = if let Some(loc2) = infos.varsStorage.get(&node.inner2) {
                loc2.clone()
                
            } else {
                panic!("unknown variable: {:?}", node.inner1)
            };

            let boxed: Box<dyn Ir> = Box::new(node.clone());

            if !block.isVarUsedAfterNode(&boxed, &node.inner1) {
                infos.drop(&node.inner1);
            }
            if !block.isVarUsedAfterNode(&boxed, &node.inner2) {
                infos.drop(&node.inner2);
            }
            if !block.isVarUsedAfterNode(&boxed, &node.inner3) {
                return vec![]; // all of these calculations don't need to be done: dead code removal
            }

            let ty = &node.inner1.ty;
    
            let ret = {
                if let Some(reg) = infos.getOpenRegBasedOnTy(*ty) {
                    VarStorage::Register(reg)
                } else {
                    let addend = match ty {
                        TypeMetadata::u16 | TypeMetadata::i16 => 2,
                        TypeMetadata::u32 | TypeMetadata::i32 => 4,
                        TypeMetadata::u64 | TypeMetadata::i64 | TypeMetadata::ptr => 8,
                        TypeMetadata::Void => todo!("cant output an addition into an void"),
                    };

                    infos.currStackOffsetForLocalVars += addend;
                    VarStorage::Memory(x64Reg::Rbp - (infos.currStackOffsetForLocalVars - addend) as u32)
                }
            };

            infos.insertVar(
                node.inner3.clone(), 
                ret.clone()
            );
            let tmp = infos.getTmpBasedOnTy(*ty);

            if let VarStorage::Register(loc1Reg) = &loc1 {
                if let VarStorage::Register(loc2Reg) = &loc2 {
                    if let VarStorage::Register(reg) = &ret {
                        return vec![
                            Instr::with2(Mnemonic::Mov, Operand::Reg(tmp.boxed()), Operand::Reg(loc1Reg.boxed())),
                            Instr::with2($mnemonic, Operand::Reg(tmp.boxed()), Operand::Reg(loc2Reg.boxed())),
                            Instr::with2(Mnemonic::Mov, Operand::Reg(reg.boxed()), Operand::Reg(tmp.boxed())),
                        ]
                    } else if let VarStorage::Memory(mem) = &ret {
                        return vec![
                            Instr::with2(Mnemonic::Mov, Operand::Reg(tmp.boxed()), Operand::Reg(loc1Reg.boxed())),
                            Instr::with2($mnemonic, Operand::Reg(tmp.boxed()), Operand::Reg(loc2Reg.boxed())),
                            Instr::with2(Mnemonic::Mov, Operand::Mem(mem.clone()), Operand::Reg(tmp.boxed())),
                            ];
                    } else { todo!() }
                }
            }

            if let VarStorage::Memory(mem1) = loc1 {
                if let VarStorage::Memory(mem2) = loc2 {
                    if let VarStorage::Register(reg) = &ret {
                        return vec![
                            Instr::with2(Mnemonic::Mov, Operand::Reg(tmp.boxed()), Operand::Mem(mem1.clone())),
                            Instr::with2($mnemonic, Operand::Reg(tmp.boxed()), Operand::Mem(mem2.clone())),
                            Instr::with2(Mnemonic::Mov, Operand::Reg(reg.boxed()), Operand::Reg(tmp.boxed())),
                        ];
                    } else if let VarStorage::Memory(mem) = &ret {
                        return vec![
                            Instr::with2(Mnemonic::Mov, Operand::Reg(tmp.boxed()), Operand::Mem(mem1.clone())),
                            Instr::with2($mnemonic, Operand::Reg(tmp.boxed()), Operand::Mem(mem2.clone())),
                            Instr::with2(Mnemonic::Mov, Operand::Mem(mem.clone()), Operand::Reg(tmp.boxed())),
                        ];
                    } else { todo!() }
                }
            }

            todo!(); // nothing was compiled
        }
    };
}

macro_rules! CompileMathVarType {
    ($name:ident, $node:ident, $mnemonic:expr) => {
        pub(crate) fn $name(node: &$node<Var, Type, Var>, registry: &mut TargetBackendDescr) -> Vec<Instr> {
            let infos = &mut registry.backend;
            let block = registry.block.unwrap();

            let loc1 = if let Some(loc1) = infos.varsStorage.get(&node.inner1) {
                loc1.clone()
            } else {
                panic!("unknown variable: {:?}", node.inner1)
            };

            let boxed: Box<dyn Ir> = Box::new(node.clone());

            if !block.isVarUsedAfterNode(&boxed, &node.inner1) {
                infos.drop(&node.inner1);
            }
            if !block.isVarUsedAfterNode(&boxed, &node.inner3) {
                return vec![]; // all of these calculations don't need to be done: dead code removal
            }

            let ty = &node.inner1.ty;
    
            let ret = {
                if let Some(reg) = infos.getOpenRegBasedOnTy(*ty) {
                    VarStorage::Register(reg)
                } else {
                    let addend = match ty {
                        TypeMetadata::u16 | TypeMetadata::i16=> 2,
                        TypeMetadata::u32 | TypeMetadata::i32=> 4,
                        TypeMetadata::u64 | TypeMetadata::i64 | TypeMetadata::ptr => 8,
                        TypeMetadata::Void => todo!("cant output an addition into an void"),
                    };

                    infos.currStackOffsetForLocalVars += addend;
                    VarStorage::Memory(x64Reg::Rbp - (infos.currStackOffsetForLocalVars - addend) as u32)
                }
            };

            infos.insertVar(
                node.inner3.clone(), 
                ret.clone()
            );
            let tmp = infos.getTmpBasedOnTy(*ty);

            if let VarStorage::Register(loc1Reg) = &loc1 {
                if let VarStorage::Register(reg) = &ret {
                    return vec![
                        Instr::with2(Mnemonic::Mov, Operand::Reg(tmp.boxed()), Operand::Imm(node.inner2.val() as i64)),
                        Instr::with2($mnemonic, Operand::Reg(tmp.boxed()), Operand::Reg(loc1Reg.boxed())),
                        Instr::with2(Mnemonic::Mov, Operand::Reg(reg.boxed()), Operand::Reg(tmp.boxed())),
                    ]
                } else if let VarStorage::Memory(mem) = &ret {
                    return vec![
                        Instr::with2(Mnemonic::Mov, Operand::Reg(tmp.boxed()), Operand::Imm(node.inner2.val() as i64)),
                        Instr::with2($mnemonic, Operand::Reg(tmp.boxed()), Operand::Reg(loc1Reg.boxed())),
                        Instr::with2(Mnemonic::Mov, Operand::Mem(mem.clone()), Operand::Reg(tmp.boxed())),
                        ];
                } else { todo!() }
            }

            if let VarStorage::Memory(mem1) = loc1 {
                if let VarStorage::Register(reg) = &ret {
                    return vec![
                        Instr::with2(Mnemonic::Mov, Operand::Reg(tmp.boxed()), Operand::Mem(mem1.clone())),
                        Instr::with2($mnemonic, Operand::Reg(tmp.boxed()), Operand::Imm(node.inner2.val() as i64)),
                        Instr::with2(Mnemonic::Mov, Operand::Reg(reg.boxed()), Operand::Reg(tmp.boxed())),
                    ];
                } else if let VarStorage::Memory(mem) = &ret {
                    return vec![
                        Instr::with2(Mnemonic::Mov, Operand::Reg(tmp.boxed()), Operand::Mem(mem1.clone())),
                        Instr::with2($mnemonic, Operand::Reg(tmp.boxed()), Operand::Imm(node.inner2.val() as i64)),
                        Instr::with2(Mnemonic::Mov, Operand::Mem(mem.clone()), Operand::Reg(tmp.boxed())),
                    ];
                } else { todo!() }
            }

            todo!(); // nothing was compiled
        }
    };
}

macro_rules! CompileMathTyTy {
    ($name:ident, $node:ident, $op:tt) => {
        pub(crate) fn $name(node: &$node<Type, Type, Var>, registry: &mut TargetBackendDescr) -> Vec<Instr> {
            let val = node.inner1.val() $op node.inner2.val();
            let block = registry.block.unwrap();
        
            let boxed: Box<dyn Ir> = Box::new(node.clone());
        
            if !block.isVarUsedAfterNode(&boxed, &node.inner3) {
                return vec![]; // all of these calculations don't need to be done: dead code removal
            }
        
            CompileConstAssign(&ConstAssign::new(node.inner3.clone(), {
                match node.inner3.ty {
                    TypeMetadata::u16 => Type::u16(val as u16),
                    TypeMetadata::u32 => Type::u32(val as u32),
                    TypeMetadata::u64 => Type::u64(val as u64),
                    TypeMetadata::i16 => Type::i16(val as i16),
                    TypeMetadata::i32 => Type::i32(val as i32),
                    TypeMetadata::i64 => Type::i64(val as i64),
                    TypeMetadata::ptr => Type::ptr(val as i64),
                    TypeMetadata::Void =>Type::Void,
                }
            }), registry)
        }
    }
}

CompileMathVarVar!(CompileAddVarVar, Add, Mnemonic::Add);
CompileMathVarVar!(CompileSubVarVar, Sub, Mnemonic::Sub);
CompileMathVarVar!(CompileXorVarVar, Xor, Mnemonic::Xor);
CompileMathVarVar!(CompileOrVarVar, Or, Mnemonic::Or);
CompileMathVarVar!(CompileAndVarVar, And, Mnemonic::And);

CompileMathVarType!(CompileAddVarTy, Add, Mnemonic::Add);
CompileMathVarType!(CompileSubVarTy, Sub, Mnemonic::Sub);
CompileMathVarType!(CompileXorVarTy, Xor, Mnemonic::Xor);
CompileMathVarType!(CompileOrVarTy, Or, Mnemonic::Or);
CompileMathVarType!(CompileAndVarTy, And, Mnemonic::And);

CompileMathTyTy!(CompileAddTyTy, Add, +);
CompileMathTyTy!(CompileSubTyTy, Sub, -);
CompileMathTyTy!(CompileXorTyTy, Xor, ^);
CompileMathTyTy!(CompileOrTyTy, Or, |);
CompileMathTyTy!(CompileAndTyTy, And, &);