use std::error::Error;
use Ygen::prelude::*;

pub fn main() -> Result<(), Box<dyn Error>> {
    let mut module = Module::new();
    let mut builder = IRBuilder::new();

    let ty = FunctionType::new(vec![TypeMetadata::i32, TypeMetadata::i32], TypeMetadata::i32);
    let func = module.add(
        "add", &ty
    );

    let entry = func.addBlock("entry");
    builder.positionAtEnd(entry); 

    let val = builder.BuildAdd(ty.arg(0), ty.arg(1));

    builder.BuildRet( val );

    module.verify().print();

    println!("{}",
        module.dumpColored()
    );

    Ok(())
}