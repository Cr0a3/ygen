use std::error::Error;
use ygen::{prelude::*, Jit::JitFunction, Target::initializeAllTargets};

type AddFunc = unsafe extern "C" fn(i32, i32) -> i32;

#[cfg(feature = "jit")]
pub fn main() -> Result<(), Box<dyn Error>> {
    let mut module = Module();

    let mut builder = IRBuilder();

    let ty = FnTy(vec![TypeMetadata::i32, TypeMetadata::i32], TypeMetadata::i32);
    
    let func = module.add(
        "add", &ty
    );

    func.extrn();

    let entry = func.addBlock("entry");
    builder.positionAtEnd(entry); 

    let val = builder.BuildAdd(ty.arg(0), ty.arg(1));
    
    builder.BuildRet( val );

    module.verify()?;

    let mut funcs = module.jitMap(&mut initializeAllTargets(Triple::host())? )?;

    let mut add: JitFunction<AddFunc> = funcs.get_function("add").expect("hmm shouldn't happen");

    unsafe {
        let ret = add.call(5, 4);

        println!("5 + 4 = {ret}");
    }

    Ok(())
}

#[cfg(not(feature = "jit"))]
pub fn main() {
    panic!("This example requires feature jit")
}