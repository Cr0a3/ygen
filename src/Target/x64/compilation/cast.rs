use super::*;

pub(crate) fn CompileCast(cast: &Cast<Var, TypeMetadata, Var>, registry: &mut TargetBackendDescr) -> Vec<Instr> {
    let boxed: Box<dyn Ir> = Box::new(cast.clone());
    let block = registry.block.unwrap();

    let loc = if let Some(loc) = registry.backend.varsStorage.get(&cast.inner1) {
        loc.clone()      
    } else {
        panic!("unknown variable: {:?}", cast.inner1)
    };

    if block.isVarUsedAfterNode(&boxed, &cast.inner1) {
        registry.backend.drop(&cast.inner1);
    } 
    if block.isVarUsedAfterNode(&boxed, &cast.inner3) {
        return vec![];
    } 

    let store = {
        if let Some(reg) = registry.backend.getOpenRegBasedOnTy(cast.inner2) {
            VarStorage::Register(reg)
        } else {
            let addend = match cast.inner2 {
                TypeMetadata::u16 | TypeMetadata::i16 => 2,
                TypeMetadata::u32 | TypeMetadata::i32 => 4,
                TypeMetadata::u64 | TypeMetadata::i64 => 8,
                TypeMetadata::Void => todo!("cant cast into void"),
                TypeMetadata::ptr => todo!("cant cast into ptr"),
            };

            registry.backend.currStackOffsetForLocalVars += addend;
            VarStorage::Memory(x64Reg::Rbp - (registry.backend.currStackOffsetForLocalVars - addend) as u32)
        }
    };

    registry.backend.insertVar(
        cast.inner3.clone(), 
        store.clone()
    );

    match loc {
        VarStorage::Register(inbound) => {
            if let VarStorage::Register(outboud) = store {
                if cast.inner1.ty.bitSize() > cast.inner3.ty.bitSize() {
                    if inbound.is_gr16() || inbound.is_gr8() { // zero extend
                        return vec![
                            Instr::with2(Mnemonic::Movzx, Operand::Reg(outboud), Operand::Reg(inbound)),
                        ];
                    } else {
                        return vec![
                            Instr::with2(Mnemonic::Mov, Operand::Reg(outboud), Operand::Reg(inbound))
                        ];
                    }
                } else {
                    return vec![{
                        if inbound.is_gr64() {
                            Instr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::parse(outboud.sub64()).unwrap().boxed()), Operand::Reg(x64Reg::parse(inbound.sub64()).unwrap().boxed()))
                        } else if inbound.is_gr32() {
                            Instr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::parse(outboud.sub32()).unwrap().boxed()), Operand::Reg(x64Reg::parse(inbound.sub32()).unwrap().boxed()))
                        } else if inbound.is_gr16() {
                            Instr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::parse(outboud.sub16()).unwrap().boxed()), Operand::Reg(x64Reg::parse(inbound.sub16()).unwrap().boxed()))
                        } else if inbound.is_gr8() {
                            Instr::with2(Mnemonic::Mov, Operand::Reg(x64Reg::parse(outboud.sub8()).unwrap().boxed()), Operand::Reg(x64Reg::parse(inbound.sub8()).unwrap().boxed()))
                        } else { panic!() }
                    }];
                }
            }
        },
        VarStorage::Memory(_) => todo!(),
    }

    vec![]
}
