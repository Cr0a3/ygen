use std::{error::Error, fs::OpenOptions, path::Path};
use ygen::{prelude::*, Support::ColorProfile, Target::initializeAllTargets};


pub fn main() -> Result<(), Box<dyn Error>> {
    let mut module = Module();

    let other = module.add("printf", &FnTy(vec![TypeMetadata::ptr], TypeMetadata::Void));
    other.import();
    let other = other.id();
    
    let string = module.addConst("str");
    string.set("Hello World!\n\0".as_bytes().to_vec());
    let string = string.clone();

    let ty = FnTy(vec![], TypeMetadata::Void);
    
    let func = module.add(
        "main", &ty
    );

    func.extrn();

    func.addBlock("entry");

    let string = func.BuildAssign(&string);
    func.BuildCall( &other, vec![string] );

    func.BuildRet( Type::Void );

    module.verify()?;

    eprintln!(
        "{}", module.dumpColored(ColorProfile::default())
    );

    let triple = Triple::host();

    module.emitToAsmFile(
        triple,
        &mut initializeAllTargets(triple)?,
        Path::new("out.o")
    )?;

    module
        .emitMachineCode(
            triple, 
            &mut initializeAllTargets(triple)?,
            false
        )?.0.emit(
            OpenOptions::new().write(true).create(true).open("out.o")?, None
    )?;

    Ok(())
}