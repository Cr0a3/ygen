use std::{error::Error, fs::OpenOptions, path::Path};
use ygen::{prelude::*, Support::ColorProfile, Target::initializeAllTargets};


pub fn main() -> Result<(), Box<dyn Error>> {
    let mut module = Module();

    let ty = FnTy(vec![TypeMetadata::i32, TypeMetadata::i32], TypeMetadata::i32);
    
    let func = module.add(
        "add", &ty
    );

    func.extrn();

    func.addBlock("entry");

    let val = func.BuildAdd(ty.arg(0), ty.arg(1));
    
    func.BuildRet( val );

    module.verify()?;

    eprintln!(
        "{}", module.dumpColored(ColorProfile::default())
    );

    let triple = Triple::host();

    module.emitToAsmFile(
        triple,
        &mut initializeAllTargets(triple)?,
        Path::new("out.asm")
    )?;

    module
        .emitMachineCode(
            triple, 
            &mut initializeAllTargets(triple)?,
            false
        )?.0.emit(
            OpenOptions::new().write(true).create(true).truncate(true).open("out.o")?, None
    )?;

    Ok(())
}