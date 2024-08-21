# Ygen - Yet another Code Generator
Ygen is a libary to build backends for compilers.

Its primary focus is having an really easy to use API like LLVMSwift (ygen is also implemented easily so everybody can add their own ir nodes and the compilation backends).

It has some advantages over llvm like being more memory safe cuz it's written in rust.
But it also lacks many ir nodes, usable optimization techniques, tools and contributours.

The IR doesn't differ to much from LLVMs.

### Contributions

![Contribution activity](https://repobeats.axiom.co/api/embed/70cb0d167ed0a296468773b0bf8d569f74d1b33a.svg "Repobeats analytics image")

### Simple example
Here is a simple example on how to use Ygen to build a simple add function:
```rust
use std::error::Error;
use Ygen::prelude::*;

pub fn main() -> Result<(), Box<dyn Error>> {
    let mut module = Module();

    let mut builder = IRBuilder();

    let ty = FnTy(vec![TypeMetadata::i32, TypeMetadata::i32], TypeMetadata::i32);
    let func = module.add(
        "add", &ty
    );

    func.extrn(); // make function externally visible

    let entry = func.addBlock("entry");
    builder.positionAtEnd(entry); 

    let val = builder.BuildAdd(ty.arg(0), ty.arg(1));
    builder.BuildRet( val );

    module.verify().print();

    eprintln!("{}",
        module.dumpColored()
    );

    Ok(())
}
```
When executed this simple program builds an add function and dumps it's ir:
```LLVM
define i32 @add( i32 %0,  i32 %1 ) {
 entry:
    %2 = add i32 %0, %1
    ret i32 %2
}
```

You can add following lines (you need to include `std::fs::Path`) to compile the IR down to assembly:
```Rust
module.emitToAsmFile(
    Triple::host(),
    &mut initializeAllTargets(),
    Path::new("out.asm")
)?;
```

### Support
Ygen currently supports following architectures
|Name    |Full ir |Full isa|
|--------|--------|--------|
|   x64  |         <b style="color:green">X</b>              | <b style="color:red">X</b>|

### Copyright
This project is owned by Cr0a3 and licensed under the Apache2 License
