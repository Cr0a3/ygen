use super::*;

pub(crate) fn CompileRetType(ret: &Return<Type>, registry: &mut TargetBackendDescr) -> Vec<Instr> {
    if ret.inner1 != Type::Void {
        vec![Instr::with2(Mnemonic::Mov, match ret.inner1.into() {
            TypeMetadata::u16 | TypeMetadata::i16 => Operand::Reg(registry.call.ret16().boxed()),
            TypeMetadata::u32 | TypeMetadata::i32 => Operand::Reg(registry.call.ret32().boxed()),
            TypeMetadata::u64 | TypeMetadata::i64 | TypeMetadata::ptr => Operand::Reg(registry.call.ret64().boxed()),
            _ => unreachable!(),
        }, Operand::Imm(ret.inner1.val() as i64))]
    } else {
        vec![]
    }
}

pub(crate) fn CompileRetVar(ret: &Return<Var>, registry: &mut TargetBackendDescr) -> Vec<Instr> {
    let (var, loc) = if let Some(loc) = registry.backend.varsStorage.get_key_value(&ret.inner1) {
        loc.clone()
    } else {
        panic!("unknown variable: {:?}", ret.inner1)
    };

    if var.ty == TypeMetadata::Void {
        return vec![];
    }

    vec![Instr::with2(Mnemonic::Mov, match var.ty {
        TypeMetadata::u16 | TypeMetadata::i16 => Operand::Reg(registry.call.ret16().boxed()),
        TypeMetadata::u32 | TypeMetadata::i32 => Operand::Reg(registry.call.ret32().boxed()),
        TypeMetadata::u64 | TypeMetadata::i64 | TypeMetadata::ptr => Operand::Reg(registry.call.ret64().boxed()),
        _ => unreachable!(),
    }, {
        if let VarStorage::Memory(mem) = loc { Operand::Mem(mem.clone()) }
        else if let VarStorage::Register(reg) = loc { Operand::Reg(reg.boxed()) }
        else { unreachable!() }
    })]
}
