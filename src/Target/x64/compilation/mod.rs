mod assign;
mod call;
mod cast;
mod math;
mod prolog;
mod ret;

pub(crate) use assign::*;
pub(crate) use call::*;
pub(crate) use cast::*;
pub(crate) use math::*;
pub(crate) use prolog::*;
pub(crate) use ret::*;


use std::collections::VecDeque;

use crate::prelude::{Block, Function, Type, TypeMetadata, Var};
use crate::Optimizations::auto_max_optimize;
use crate::Target::target_descr::{TargetBackendDescr, VarStorage};
use crate::Target::Reg;
use crate::IR::{ir::*, Const};

use crate::Target::CallConv;

use super::{x64Reg, instr::*};

pub(crate) fn buildAsmX86<'a>(block: &'a Block, func: &Function, call: &CallConv, registry: &mut TargetBackendDescr<'a>) -> Vec<Instr> {
    registry.block = Some(&block);

    let info = &mut registry.backend;

    let mut reg_vars = 0;
    let mut stack_off = 8; // because in an call the return adress gets pushed which is 8 bytes long
    let mut var_index = 0;

    for (_, meta) in &func.ty.args {
        let mut var = Var(&mut block.clone(), meta.to_owned());
        var.name = format!("%{}", var_index);

        info.insertVar(var, {
            if reg_vars >= call.regArgs() {
                let addend = match meta {
                    TypeMetadata::u16 | TypeMetadata::i16 => 2,
                    TypeMetadata::u32 | TypeMetadata::i32 => 4,
                    TypeMetadata::u64 | TypeMetadata::i64 | TypeMetadata::ptr => 8,
                    TypeMetadata::Void => continue,
                };

                stack_off += addend;
                VarStorage::Memory(x64Reg::Rbp - (stack_off - addend))
            } else {
                reg_vars += 1;
                VarStorage::Register( match meta {
                    TypeMetadata::u16 | TypeMetadata::i16 => call.args16()[reg_vars - 1].boxed(),
                    TypeMetadata::u32 | TypeMetadata::i32 => call.args32()[reg_vars - 1].boxed(),
                    TypeMetadata::u64 | TypeMetadata::i64 | TypeMetadata::ptr => call.args64()[reg_vars - 1].boxed(),
                    TypeMetadata::Void => continue,
                })
            }
        });

        var_index += 1;
    }

    if reg_vars < call.regArgs() {
        info.dropReg(call.args64()[reg_vars].boxed());        
    }

    let mut out: VecDeque<Instr> = VecDeque::new();

    for node in &block.nodes {
        let compiled = node.compile(registry);
        out.extend(compiled);
        out.push_back(Instr::with1(Mnemonic::Debug, Operand::Debug(format!("{}", node.dump()))));
    }

    registry.block = None;

    out.extend(x64BuildEpilog(&block, registry));

    for epAsm in  x64BuildProlog(&block, registry) {
        out.push_front(epAsm);
    }

    let mut out = Vec::from(out);

    auto_max_optimize(&mut out);

    out

}