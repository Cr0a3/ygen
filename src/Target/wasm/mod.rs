//! The wasm target
mod lower;
/// Wasm instruction encoding
pub mod asm;

use crate::CodeGen::{reg_alloc::RegAlloc, CompilationHelper, MachineCallingConvention};

use super::{Arch, CallConv, TargetBackendDescr, WhiteList};

/// Initializes the wasm target
pub fn initializeWasmTarget(_: CallConv) -> TargetBackendDescr {
    let mut target = TargetBackendDescr::new();

    target.call = CallConv::WasmBasicCAbi;
    target.init = Some(initializeWasmTarget);

    target.lexer = Some(Box::new( asm::lexer::wasmLexer {} ));
    target.compile = Some(Box::new( asm::parser::wasmParser::new(Vec::new())));
    
    let mut compiler = CompilationHelper::new(
            Arch::Wasm64, 
            MachineCallingConvention {
            call_conv: CallConv::WasmBasicCAbi
        }, 
        RegAlloc::new(
            Arch::Wasm64, 
            CallConv::WasmBasicCAbi, 
            true
        ), 
        crate::CodeGen::Reg::x64(super::x64Reg::Al) // unnedded won't be used, so anything can go here
    );


    compiler.lower = Some(lower::wasm_lower);

    let whitelist = WhiteList::new();

    target.helper = Some(compiler);
    target.whitelist = whitelist;

    target
}