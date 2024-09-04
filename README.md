# Ygen - Yet another Code Generator
![GitHub branch check runs](https://img.shields.io/github/check-runs/Cr0a3/ygen/main?style=flat-square&label=build)
![Crates.io Version](https://img.shields.io/crates/v/Ygen?style=flat-square)
![GitHub Repo stars](https://img.shields.io/github/stars/cr0a3/ygen?style=flat-square)

Ygen is a libary for building compiler backends.

It provides easy to use apis for generating ygen-ir, which is also lowerable to machine code using easy to use class method.

The main focus is code generation but it also has support classes like for coloring.

> [!WARNING]
> This project is still early in its developement. Bugs and miscompilations are expected. DO NOT USE THE PROJECT FOR NOT TOY COMPILERS


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
|   x64  |         <b style="color:green">Yes</b>              | <b style="color:red">No</b>|

### Copyright
This project is owned by Cr0a3 and licensed under the Apache2 License
