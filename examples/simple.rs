use Ygen::prelude::*;

pub fn main() {
    let mut module = Module::new();
    let mut builder = IRBuilder::new();

    let func = module.add(
        "func", FunctionType::new(vec![], TypeMetadata::i32)
    );

    let entry = func.addBlock("entry");
    builder.positionAtEnd(entry); 

    builder.BuildRet(
        Type::i32(5)
    );

    println!("{}",
        module.dump()
    );
}