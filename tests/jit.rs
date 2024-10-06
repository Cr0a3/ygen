use std::error::Error;
use ygen::{prelude::*, Jit::JitFunction, Target::initializeAllTargets};

type AddFunc = unsafe extern "C" fn(i32, i32) -> i32;

#[test]
pub fn basic() -> Result<(), Box<dyn Error>> {
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

        assert_eq!(ret, 9);
    }

    Ok(())
}

#[test]
pub fn call() -> Result<(), Box<dyn Error>> {
    let mut module = Module();

    let test_ty = FnTy(vec![TypeMetadata::i32], TypeMetadata::i32);
    let add_ty = FnTy(vec![TypeMetadata::i32, TypeMetadata::i32], TypeMetadata::i32);

    let add = module.add("add", &add_ty);
    add.addBlock("entry");

    let ret = add.BuildAdd(add_ty.arg(0), add_ty.arg(1));
    add.BuildRet(ret);

    let add = add.id();

    let test = module.add("test", &test_ty);
    test.addBlock("entry");

    let out = test.BuildCall(&add, vec![test_ty.arg(0), test_ty.arg(0)]);
    test.BuildRet(out);

    let mut funcs = module.jitMap(&mut initializeAllTargets(Triple::host())? )?;

    let mut add: JitFunction<unsafe extern "C" fn(i32) -> i32> = funcs.get_function("test").expect("hmm shouldn't happen");

    unsafe {
        let ret = add.call(5);

        assert_eq!(ret, 10);
    }

    Ok(())
}