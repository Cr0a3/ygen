use crate::{prelude::*, PassManager::Pass};

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
            if &node.name() == "AddTypeType" {
                let any = node.as_any();
                let add_node = any.downcast_ref::<Add<Type, Type, Var>>().unwrap().clone();
                let res = add_node.inner1.val() + add_node.inner2.val();
                node.replace(ConstAssign::new(add_node.inner3.clone(), {
                    match add_node.inner3.ty {
                        TypeMetadata::u16 => Type::u16(res as u16),
                        TypeMetadata::u32 => Type::u32(res as u32),
                        TypeMetadata::u64 => Type::u64(res as u64),
                        TypeMetadata::i16 => Type::i16(res as i16),
                        TypeMetadata::i32 => Type::i32(res as i32),
                        TypeMetadata::i64 => Type::i64(res as i64),
                        TypeMetadata::Void => Type::Void,
                    }
                }))
            } else if &node.name() == "SubTypeType" {
                let any = node.as_any();
                let sub_node = any.downcast_ref::<Sub<Type, Type, Var>>().unwrap().clone();
                let res = sub_node.inner1.val() - sub_node.inner2.val();
                node.replace(ConstAssign::new(sub_node.inner3.clone(), {
                    match sub_node.inner3.ty {
                        TypeMetadata::u16 => Type::u16(res as u16),
                        TypeMetadata::u32 => Type::u32(res as u32),
                        TypeMetadata::u64 => Type::u64(res as u64),
                        TypeMetadata::i16 => Type::i16(res as i16),
                        TypeMetadata::i32 => Type::i32(res as i32),
                        TypeMetadata::i64 => Type::i64(res as i64),
                        TypeMetadata::Void => Type::Void,
                    }
                }))
            } else if &node.name() == "XorTypeType" {
                let any = node.as_any();
                let sub_node = any.downcast_ref::<Xor<Type, Type, Var>>().unwrap().clone();
                let res = sub_node.inner1.val() ^ sub_node.inner2.val();
                node.replace(ConstAssign::new(sub_node.inner3.clone(), {
                    match sub_node.inner3.ty {
                        TypeMetadata::u16 => Type::u16(res as u16),
                        TypeMetadata::u32 => Type::u32(res as u32),
                        TypeMetadata::u64 => Type::u64(res as u64),
                        TypeMetadata::i16 => Type::i16(res as i16),
                        TypeMetadata::i32 => Type::i32(res as i32),
                        TypeMetadata::i64 => Type::i64(res as i64),
                        TypeMetadata::Void => Type::Void,
                    }
                }))
            }
        }
    }
}