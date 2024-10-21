use std::{error::Error, fs::OpenOptions, path::Path};

use ygen::debug::{DebugLocation, DebugRegistry};
use ygen::{Obj::*, Target::Triple};
use ygen::Target::x64::{instr::*, X64Reg};

fn main() -> Result<(), Box<dyn Error>> {
    // test.c:
    // 01 #include <stdio.h>
    // 02
    // 03 int main() {
    // 04    printf("Hello World!");
    // 05    return 0;
    // 06 }

    let mut obj = ObjectBuilder::new(
        Triple::parse("x86_64-pc-windows")?
    );

    obj.decls(vec![
        ("main", Decl::Function, Linkage::External),
        ("string", Decl::Constant, Linkage::Internal),
        ("printf", Decl::Function, Linkage::Extern),
    ]);

    let mut debug = DebugRegistry::new("ygen example".to_string(), gimli::DW_LANG_C, Path::new("test.c"));

    let mut data = vec![];
    data.extend_from_slice(&X64MCInstr::with1(Mnemonic::Push, Operand::Reg(X64Reg::Rbp)).compile()?);
    data.extend_from_slice(&X64MCInstr::with2(Mnemonic::Sub, Operand::Reg(X64Reg::Rsp), Operand::Imm(40)).compile()?);

    debug.add_location(&"main".to_string(), DebugLocation { line: 3, col: 0, epilog: false, prolog: true, adr: data.len() as u64 });

    let rip_relativ = Operand::Mem(MemOp { base: None, index: None, scale: 1, displ: 0, rip: true });
    
    data.extend_from_slice(&X64MCInstr::with2(Mnemonic::Lea, Operand::Reg(X64Reg::Rax), rip_relativ).compile()?);

    obj.link( Link { from: "main".into(), to: "string".into(), at: data.len(), addend: -4, special: false });
    
    if cfg!(target_os = "windows") {
        data.extend_from_slice(&X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(X64Reg::Rcx), Operand::Reg(X64Reg::Rax)).compile()?);
    } else {
        data.extend_from_slice(&X64MCInstr::with2(Mnemonic::Mov, Operand::Reg(X64Reg::Rsi), Operand::Reg(X64Reg::Rax)).compile()?);
    }
    

    debug.add_location(&"main".to_string(), DebugLocation { line: 4, col: 4, epilog: false, prolog: false, adr: data.len() as u64 });
    data.extend_from_slice(&X64MCInstr::with1(Mnemonic::Call, Operand::Imm(0)).compile()?); // call printf

    obj.link( Link { from: "main".into(), to: "printf".into(), at: data.len(), addend: -4, special: false });

    
    debug.add_location(&"main".to_string(), DebugLocation { line: 5, col: 4, epilog: false, prolog: false, adr: data.len() as u64 });
    data.extend_from_slice(&X64MCInstr::with2(Mnemonic::Xor, Operand::Reg(X64Reg::Eax), Operand::Reg(X64Reg::Eax)).compile()?);
    
    debug.add_location(&"main".to_string(), DebugLocation { line: 6, col: 0, epilog: true, prolog: false, adr: data.len() as u64 });
    data.extend_from_slice(&X64MCInstr::with2(Mnemonic::Add, Operand::Reg(X64Reg::Rsp), Operand::Imm(40)).compile()?);
    data.extend_from_slice(&X64MCInstr::with0(Mnemonic::Ret).compile()?); // ret

    obj.define("main", data);
    obj.define("string", b"Hello World!\n\0".into());

    obj.debug = true;

    obj.emit(
        OpenOptions::new().create(true).write(true).open("output.o")?, Some(debug)
    )?;

    Ok(())
}