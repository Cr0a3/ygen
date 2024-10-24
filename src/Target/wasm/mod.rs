//! The wasm target
mod lower;
/// Wasm instruction encoding
pub mod asm;

use crate::CodeGen::{reg_alloc::RegAlloc, CompilationHelper, ConstImmRules, MachineCallingConvention};

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
        crate::CodeGen::Reg::x64(crate::Target::x64::X64Reg::Al) // unnedded won't be used, so anything can go here
    );

    compiler.fp_imm = ConstImmRules::InstrOp;

    compiler.lower = Some(lower::wasm_lower);

    let whitelist = WhiteList::new();

    target.helper = Some(compiler);
    target.whitelist = whitelist;

    target
}