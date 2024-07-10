use std::error::Error;
use Ygen::prelude::*;

pub fn main() -> Result<(), Box<dyn Error>> {
    let mut module = Module::new();
    let mut builder = IRBuilder::new();

    let func = module.add(
        "func", FunctionType::new(vec![], TypeMetadata::i32)
    );

    let entry = func.addBlock("entry");
    builder.positionAtEnd(entry); 

let val = builder.BuildAdd(Type::i32(5), Type::i32(5))?;

    builder.BuildRet( val );

    module.verify().print();

    println!("{}",
        module.dumpColored()
    );

    Ok(())
}