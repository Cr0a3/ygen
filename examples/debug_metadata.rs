use std::{error::Error, fs::OpenOptions, path::{Path, PathBuf}};
use gimli::DW_LANG_C;
use ygen::{prelude::*, Support::ColorProfile, Target::initializeAllTargets};


// hello_world.c:
// 1 #include <stdout.h>
// 2 
// 3 int main() {
// 4     printf("Hello World!");
// 5     return 0;
// 6 }

pub fn main() -> Result<(), Box<dyn Error>> {
    let mut module = Module();
    module.init_dbg("ygen example".to_owned(), DW_LANG_C, &PathBuf::from("hello_world.c"));

    let mut builder = IRBuilder();

    let other = module.add("printf", &FnTy(vec![TypeMetadata::ptr], TypeMetadata::Void));
    other.import();
    let other = other.clone();
    
    let string = module.addConst("str");
    string.set("Hello World!\n\0".as_bytes().to_vec());
    let string = string.clone();

    let ty = FnTy(vec![], TypeMetadata::Void);
    
    let func = module.add(
        "main", &ty
    );

    func.extrn();

    let entry = func.addBlock("entry");
    builder.positionAtEnd(entry); 

    builder.BuildDebug(4, 0, PathBuf::from("hello_world.c"));
    let string = builder.BuildAssign(&string);
    builder.BuildCall( &other, vec![string] );

    builder.BuildDebug(5, 0, PathBuf::from("hello_world.c"));
    builder.BuildRet( Type::Void );

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

    let (mut object, debug) = module
        .emitMachineCode(
            triple, 
            &mut initializeAllTargets(triple)?,
            true // turns on debugging information
        )?;

    object.debug = true; // additionaly we need to turn debugging information in the object file on

    object.emit(
            OpenOptions::new().write(true).create(true).open("out.o")?, 
            debug
    )?;

    Ok(())
}