use std::{error::Error, fs::OpenOptions, path::Path};
use Ygen::{prelude::*, Target::initializeAllTargets};


pub fn main() -> Result<(), Box<dyn Error>> {
    let mut module = Module();

    let mut builder = IRBuilder();

    let ty = FnTy(vec![TypeMetadata::i32, TypeMetadata::i32], TypeMetadata::i32);
    /*let func = module.add(
        "add", &ty
    );

    let entry = func.addBlock("entry");
    builder.positionAtEnd(entry); 

    let val = builder.BuildAdd(ty.arg(0), ty.arg(1));
    builder.BuildRet( val );*/

    
    let func = module.add(
        "sub", &ty
    );

    let entry = func.addBlock("entry");
    builder.positionAtEnd(entry); 

    let val = builder.BuildSub(ty.arg(0), ty.arg(1));
    builder.BuildRet( val );

    module.verify().print();

    eprintln!("{}",
        module.dumpColored()
    );

    module.emitToAsmFile(Path::new("out.asm"))?;

    module
        .emitMachineCode(
            Triple::host(), 
            &mut initializeAllTargets()
        )?.emit(
            OpenOptions::new().write(true).create(true).open("out.o")?
    )?;

    Ok(())
}