use std::{error::Error, fs::OpenOptions, path::Path};
use Ygen::{prelude::*, Support::ColorProfile, Target::initializeAllTargets};


pub fn main() -> Result<(), Box<dyn Error>> {
    let mut module = Module();

    let mut builder = IRBuilder();

    let other = module.add("printf", &FnTy(vec![TypeMetadata::u64], TypeMetadata::Void));
    other.import();
    let other = other.clone();
    
    let string = module.addConst("str");
    string.set("Hello World!\0".as_bytes().to_vec());
    let string = string.clone();

    let ty = FnTy(vec![TypeMetadata::i64], TypeMetadata::i32);
    
    let func = module.add(
        "main", &ty
    );

    func.extrn();

    let entry = func.addBlock("entry");
    builder.positionAtEnd(entry); 

    let string = builder.BuildAssign(&string);

    builder.BuildCall( &other, vec![string] );

    builder.BuildRet( Type::i32(0) );

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