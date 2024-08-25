use ygen::prelude::*;

#[test]
pub fn ir_optimization() {
    let mut module = Module();

    let mut builder = IRBuilder();

    let other = module.add("cfunc", &FnTy(vec![TypeMetadata::i32, TypeMetadata::i32], TypeMetadata::i32));
    other.import();
    let other = other.clone();

    let ty = FnTy(vec![TypeMetadata::i32, TypeMetadata::i32], TypeMetadata::i32);
    
    let func = module.add(
        "add", &ty
    );

    func.extrn();

    let entry = func.addBlock("entry");
    builder.positionAtEnd(entry); 

    let val = builder.BuildCall( &other, vec![ty.arg(0), ty.arg(1)] );
    let val = builder.BuildAdd(val, ty.arg(0));
    
    builder.BuildRet( val );

    //assert_eq!(module.dump(), "define i32 @add(i32 %0, i32 %1) {\n entry:\n\t%2 = call i32 cfunc i32 %0 i32 %1 \n\tadd = %3 i32 %2, %0\n\tret i32 %3\n\n}\ndeclare i32 @cfunc(i32 %0, i32 %1)\n\n".to_string());
}