//! The wasm target
mod lower;
/// Wasm instruction encoding
pub mod asm;

mod reg_alloc;

use std::collections::HashMap;

use crate::CodeGen::{Allocator, CompilationHelper, ConstImmRules, MachineCallingConvention};

use super::{Arch, CallConv, TargetBackendDescr, WhiteList};

/// Wasm assembly printing
pub mod printer;

/// Initializes the wasm target
pub fn initializeWasmTarget(_: CallConv) -> TargetBackendDescr {
    let mut target = TargetBackendDescr::new();

    target.call = CallConv::WasmBasicCAbi;
    target.init = Some(initializeWasmTarget);

    target.lexer = Some(Box::new( asm::lexer::wasmLexer {} ));
    target.compile = Some(Box::new( asm::parser::wasmParser::new(Vec::new())));
    target.printer = Some(printer::WasmAsmPrinter::new());

    let alloc = Allocator {
        alloc: Some(reg_alloc::wasm_alloc),
        alloc_rv: Some(reg_alloc::wasm_alloc_var),
        alloc_stack: Some(reg_alloc::wasm_alloc_var),
        free: Some(reg_alloc::wasm_free),
        after_alloc: None,
        vars: HashMap::new(),
        var_types: HashMap::new(),
        allocated_vars: Vec::new(),
        epilog: false,
        scopes: HashMap::new(),
        phi_vars: HashMap::new(),
        stack_off: 0,
        fregs: Vec::new(),
        ffpregs: Vec::new(),
        call: MachineCallingConvention { call_conv: CallConv::WasmBasicCAbi },
    };

    let mut compiler = CompilationHelper::new(
         Arch::Wasm64, 
        MachineCallingConvention { call_conv: CallConv::WasmBasicCAbi }, 
        alloc, 
        crate::CodeGen::Reg::x64(crate::Target::x64::X64Reg::Al) // unnedded won't be used, so anything can go here
    );

    compiler.fp_imm = ConstImmRules::InstrOp;

    compiler.lower = Some(lower::wasm_lower);

    let whitelist = WhiteList::new();

    target.helper = Some(compiler);
    target.whitelist = whitelist;

    target
}