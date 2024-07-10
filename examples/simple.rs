use std::{error::Error, process::exit};

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

    match module.verify() {
        Ok(_) => {},
        Err(e) => {
            println!("{}", e);
            exit(0)
        },
    };

    println!("{}",
        module.dumpColored()
    );

    Ok(())
}