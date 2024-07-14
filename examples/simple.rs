use std::{error::Error, path::Path};
use Ygen::{prelude::*, PassManager::Passes::PreComputeValue};

pub fn main() -> Result<(), Box<dyn Error>> {
    let mut module = Module();

    let mut builder = IRBuilder();

    let ty = FnTy(vec![TypeMetadata::i32, TypeMetadata::i32], TypeMetadata::i32);
    let func = module.add(
        "add", &ty
    );

    let entry = func.addBlock("entry");
    builder.positionAtEnd(entry); 

    let val = builder.BuildAdd(ty.arg(0), ty.arg(1));
    let add2 = builder.BuildAdd(Type::i32(5), Type::i32(5)); // If you wonder: this here tests the optimizer if it will be automaticly optimized out and inlined to 10
    let ret = builder.BuildAdd(val, add2);
    builder.BuildRet( ret );

    module.verify().print();

    let mut passes = PassManager::new();
    passes.add( PreComputeValue() );

    module.runPassMngr(passes);

    eprintln!("{}",
        module.dumpColored()
    );

    module.emitToAsmFile(Path::new("out.asm"))?;

    Ok(())
}