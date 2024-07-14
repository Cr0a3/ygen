use std::error::Error;
use Ygen::{prelude::*, PassManager::Passes::PreComputeValue, Target::{initializeX64Target, CallConv}};

pub fn main() -> Result<(), Box<dyn Error>> {
    
    initializeX64Target(CallConv::WindowsFastCall);

    let mut module = Module();

    let mut builder = IRBuilder();

    let ty = FnTy(vec![TypeMetadata::i32, TypeMetadata::i32], TypeMetadata::i32);
    let func = module.add(
        "add", &ty
    );

    let entry = func.addBlock("entry");
    builder.positionAtEnd(entry); 

    //let val = builder.BuildAdd(ty.arg(0), ty.arg(1));
    //let add2 = builder.BuildAdd(Type::i32(5), Type::i32(5));
    //let ret = builder.BuildAdd(val, add2);

    //builder.BuildRet( ret );

    builder.BuildRet( Type::i32(5) );

    let block = builder.getLastBlock().clone().unwrap().clone();
    let func = func.clone().to_owned().clone();

    module.verify().print();

    let mut passes = PassManager::new();
    passes.add( PreComputeValue() );

    module.runPassMngr(passes);

    eprintln!("{}",
        module.dumpColored()
    );

    eprintln!("{:#?}", block.buildAsmX86(&func, &CallConv::WindowsFastCall));

    Ok(())
}