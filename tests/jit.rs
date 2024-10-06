use std::error::Error;
use ygen::{prelude::*, Jit::JitFunction, Target::initializeAllTargets};

type AddFunc = unsafe extern "C" fn(i32, i32) -> i32;

#[test]
pub fn basic() -> Result<(), Box<dyn Error>> {
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

        assert_eq!(ret, 9);
    }

    Ok(())
}

#[test]
pub fn blocks() -> Result<(), Box<dyn Error>> {
    let mut module = ygen::ir!(
        r#"
        define i32 @test(i32 %0) {
          entry:
             %ret_value = call i32 add i32 %0 i32 %0
             ret i32 %ret_value
        }

        define i32 @add(i32 %0, i32 %1) {
          entry:
            %2 = add i32 %0, %1
            ret i32 %2
        }
        "#
    );

    let mut funcs = module.jitMap(&mut initializeAllTargets(Triple::host())? )?;

    let mut add: JitFunction<unsafe extern "C" fn(i32) -> i32> = funcs.get_function("test").expect("hmm shouldn't happen");

    unsafe {
        let ret = add.call(5);

        assert_eq!(ret, 10);
    }

    Ok(())
}