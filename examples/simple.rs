use std::{error::Error, fs::OpenOptions, path::Path};
use ygen::{prelude::*, Support::ColorProfile, Target::initializeAllTargets};


pub fn main() -> Result<(), Box<dyn Error>> {
    let mut module = Module();

    let mut builder = IRBuilder();

    let other = module.add("cfunc", &FnTy(vec![TypeMetadata::i32, TypeMetadata::i32], TypeMetadata::i32));
    other.import();
    let other = other.clone();

    let ty = FnTy(vec![TypeMetadata::i32, TypeMetadata::i32], TypeMetadata::i32);
    
    let func = module.add(
        "add", &ty
    );

    func.extrn();

    let entry = func.addBlock("entry");
    builder.positionAtEnd(entry); 

    let val = builder.BuildCall( &other, vec![ty.arg(0), ty.arg(1)] );
    let val = builder.BuildAdd(val, ty.arg(0));
    
    builder.BuildRet( val );

    module.verify()?;

    eprintln!(
        "{}", module.dumpColored(ColorProfile::default())
    );

    module.emitToAsmFile(
        Triple::host(),
        &mut initializeAllTargets(),
        Path::new("out.asm")
    )?;

    module
        .emitMachineCode(
            Triple::host(), 
            &mut initializeAllTargets()
        )?.emit(
            OpenOptions::new().write(true).create(true).open("out.o")?
    )?;

    Ok(())
}