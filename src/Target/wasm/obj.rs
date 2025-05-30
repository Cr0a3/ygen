use std::{collections::{BTreeMap, HashMap}, error::Error};
use crate::{debug::DebugRegistry, Obj::ObjectBuilder, Optimizations::Optimize, Target::*, IR::TypeMetadata};
use wasm::{asm::{WasmMCInstr, WasmMnemonic, WasmOperand}, lower::wasm_construct_local_types};
use wasm_encoder::*;

pub(crate) fn wasm_emit_mccode(registry: &mut TargetRegistry, debug: bool, module: &mut crate::IR::Module) -> Result<(ObjectBuilder, Option<DebugRegistry>), Box<dyn Error>> {
    if debug {
        dbg!("wasm doesn't currently support debugging information so it will be left out");
    }
    
    let mut wasm = Module::new();

    // actuall code start

    let mut types = TypeSection::new();
    let mut functions = FunctionSection::new();
    let mut exports = ExportSection::new();
    let mut codes = CodeSection::new();

    let mut idx = 0;
    for (name, func) in module.funcs.clone() {
        // type
        let mut params = Vec::new();
        let mut ret = Vec::new();

        if func.ty.ret != TypeMetadata::Void {
            ret.push(func.ty.ret.into());
        }

        for (_, arg) in &func.ty.args {
            params.push((*arg).into());
        }

        types.ty().function(params, ret);

        
        // func
        
        functions.function(idx);
        
        // visibility

        exports.export(name.as_str(), ExportKind::Func, idx);

        // code

        let (instrs, locals) = wasm_build_instrs(&func, registry, module)?;

        let mut func = Function::new(locals);

        for instr in instrs {
            println!("instr: {}", instr);
            func.instruction(&instr.into());
        }

        codes.function(&func);

        idx += 1;
    }

    // actuall code end

    wasm.section(&types);
    wasm.section(&functions);
    wasm.section(&exports);
    wasm.section(&codes);

    let encoded = wasm.finish();

    let mut obj = ObjectBuilder::new(Triple { 
        arch: crate::Target::Arch::Wasm64, 
        vendor: crate::Target::Vendor::Unknown, 
        os: crate::Target::OS::Unknown, 
        env: crate::Target::Environment::Unknown,
        bin: crate::Target::ObjFormat::Wasm,
    }); // triple is unimportant here

    obj.just_write_bytes = Some(encoded);

    Ok((obj, None))
}

impl Into<ValType> for TypeMetadata {
    fn into(self) -> ValType {
        match self {
            TypeMetadata::u32 => ValType::I32,
            TypeMetadata::u64 => ValType::I64,
            TypeMetadata::i32 => ValType::I32,
            TypeMetadata::i64 => ValType::I64,
            TypeMetadata::ptr => ValType::Ref(RefType::ANYREF),
            TypeMetadata::f32 => ValType::F32,
            TypeMetadata::f64 => ValType::F64,
            _ => panic!("unsupported type for wasm: {}", self),
        }
    }
}

fn wasm_build_instrs(func: &crate::IR::Function, registry: &mut TargetRegistry, module: &mut crate::IR::Module) -> Result<(Vec<WasmMCInstr>, Vec<(u32, ValType)>), Box<dyn Error>> {
    let mut blocked_instrs: BTreeMap<&String, Vec<crate::CodeGen::MachineInstr>> = BTreeMap::new();

    for block in &func.blocks {
        let instrs = registry.buildMachineInstrsForTarget(Arch::Wasm64, block, func, module)?;

        blocked_instrs.insert(&block.name, instrs);
    }

    // bring the local types into the correct format

    let mut merged = Vec::new();

    for (_, instrs) in &blocked_instrs {
        merged.extend_from_slice(instrs);
    }

    let types = wasm_construct_local_types(&merged);

    let mut locals = Vec::new();

    for (num, ty) in types {
        locals.push((num as u32, ty.into()));
    }

    // lower the machine instr into mc instrs

    let mut lowered = Vec::new();
    
    let mut indexes = HashMap::new();

    let mut index = 0;

    let mut first = true;

    for (block, instrs) in blocked_instrs {
        if first {
            lowered.push(WasmMCInstr::with0(None, WasmMnemonic::Block));
        }
        for instr in instrs {
            super::lower::wasm_lower_instr(&mut lowered, instr);
        }
        lowered.push(WasmMCInstr::with0(None, WasmMnemonic::End));

        indexes.insert(block, index);
        
        index += 1;

        first = false;
    }

    // now we finally link

    for instr in &mut lowered {
        if let Some(WasmOperand::BlockLink(ref target)) = instr.op1 {
            let Some(index) = indexes.get(target) else { panic!("unknown block: {}", target) };

            instr.op1 = Some(WasmOperand::Const(*index as f64));
        }
    }

    lowered = lowered.optimize();

    Ok((lowered, locals))
}