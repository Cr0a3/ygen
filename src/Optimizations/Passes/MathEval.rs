use crate::{prelude::*, Optimizations::Pass};

/// ## Pass InlineConstValue <br>
/// Inlines constant values
pub struct PreComputeValue {}

/// Creates a new PreComputeValue pass which is heap allocated
pub fn PreComputeValue() -> Box<PreComputeValue> {
    Box::from( PreComputeValue {} )
}

impl Pass for PreComputeValue {
    fn run(&self, block: &mut crate::prelude::Block) {
        for node in block.nodes.iter_mut() {
            if let Some(or) = node.as_any().downcast_ref::<Add<Type, Type, Var>>() {
                let res = or.inner1.val() + or.inner2.val();
                node.replace(ConstAssign::new(or.inner3.clone(), {
                    match or.inner3.ty {
                        TypeMetadata::u16 => Type::u16(res as u16),
                        TypeMetadata::u32 => Type::u32(res as u32),
                        TypeMetadata::u64 => Type::u64(res as u64),
                        TypeMetadata::i16 => Type::i16(res as i16),
                        TypeMetadata::i32 => Type::i32(res as i32),
                        TypeMetadata::i64 => Type::i64(res as i64),
                        TypeMetadata::ptr => Type::ptr(res as i64),
                        TypeMetadata::Void => Type::Void,
                    }
                }))
            }
            if let Some(or) = node.as_any().downcast_ref::<Sub<Type, Type, Var>>() {
                let res = or.inner1.val() - or.inner2.val();
                node.replace(ConstAssign::new(or.inner3.clone(), {
                    match or.inner3.ty {
                        TypeMetadata::u16 => Type::u16(res as u16),
                        TypeMetadata::u32 => Type::u32(res as u32),
                        TypeMetadata::u64 => Type::u64(res as u64),
                        TypeMetadata::i16 => Type::i16(res as i16),
                        TypeMetadata::i32 => Type::i32(res as i32),
                        TypeMetadata::i64 => Type::i64(res as i64),
                        TypeMetadata::ptr => Type::ptr(res as i64),
                        TypeMetadata::Void => Type::Void,
                    }
                }))
            }
            if let Some(or) = node.as_any().downcast_ref::<And<Type, Type, Var>>() {
                let res = or.inner1.val() & or.inner2.val();
                node.replace(ConstAssign::new(or.inner3.clone(), {
                    match or.inner3.ty {
                        TypeMetadata::u16 => Type::u16(res as u16),
                        TypeMetadata::u32 => Type::u32(res as u32),
                        TypeMetadata::u64 => Type::u64(res as u64),
                        TypeMetadata::i16 => Type::i16(res as i16),
                        TypeMetadata::i32 => Type::i32(res as i32),
                        TypeMetadata::i64 => Type::i64(res as i64),
                        TypeMetadata::ptr => Type::ptr(res as i64),
                        TypeMetadata::Void => Type::Void,
                    }
                }))
            }
            if let Some(or) = node.as_any().downcast_ref::<Xor<Type, Type, Var>>() {
                let res = or.inner1.val() ^ or.inner2.val();
                node.replace(ConstAssign::new(or.inner3.clone(), {
                    match or.inner3.ty {
                        TypeMetadata::u16 => Type::u16(res as u16),
                        TypeMetadata::u32 => Type::u32(res as u32),
                        TypeMetadata::u64 => Type::u64(res as u64),
                        TypeMetadata::i16 => Type::i16(res as i16),
                        TypeMetadata::i32 => Type::i32(res as i32),
                        TypeMetadata::i64 => Type::i64(res as i64),
                        TypeMetadata::ptr => Type::ptr(res as i64),
                        TypeMetadata::Void => Type::Void,
                    }
                }))
            }
            if let Some(or) = node.as_any().downcast_ref::<Or<Type, Type, Var>>() {
                let res = or.inner1.val() | or.inner2.val();
                node.replace(ConstAssign::new(or.inner3.clone(), {
                    match or.inner3.ty {
                        TypeMetadata::u16 => Type::u16(res as u16),
                        TypeMetadata::u32 => Type::u32(res as u32),
                        TypeMetadata::u64 => Type::u64(res as u64),
                        TypeMetadata::i16 => Type::i16(res as i16),
                        TypeMetadata::i32 => Type::i32(res as i32),
                        TypeMetadata::i64 => Type::i64(res as i64),
                        TypeMetadata::ptr => Type::ptr(res as i64),
                        TypeMetadata::Void => Type::Void,
                    }
                }))
            }
        }
    }
}