use std::error::Error;
#[cfg(feature = "jit")]
use ygen::{prelude::*, Jit::JitFunction, Target::initializeAllTargets};

type AddFunc = unsafe extern "C" fn(i32, i32) -> i32;

#[cfg(feature = "jit")]
pub fn main() -> Result<(), Box<dyn Error>> {
    let mut module = Module();

    let ty = FnTy(vec![TypeMetadata::i32, TypeMetadata::i32], TypeMetadata::i32);
    
    let func = module.add(
        "add", &ty
    );

    func.extrn();

    func.addBlock("entry");

    let val = func.BuildAdd(ty.arg(0), ty.arg(1));
    
    func.BuildRet( val );

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