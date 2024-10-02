# Ygen - Yet another Code Generator
![GitHub branch check runs](https://img.shields.io/github/check-runs/Cr0a3/ygen/main?style=flat-square&label=build)
![Crates.io Version](https://img.shields.io/crates/v/Ygen?style=flat-square)
![GitHub Repo stars](https://img.shields.io/github/stars/cr0a3/ygen?style=flat-square)

Welcome to Ygen!
This repository contains the source code of the ygen project.

Ygen is a toolkit for building fast and clean compilers using a memory safe api.

## Why ygen?

You are probably wondering: why would I choose ygen and not llvm or cranelift??
Here are a few reasons:

- **Simplicity**: One of ygens main focus is simplicity which means to us that as much code as possible is readable and shared
- **Similare API**: Ygens API is very similar to LLVMs API for example i designed the `IRBuilder` to be very similar to the `Builder` from LLVM
- **Traits**: Ygen uses a lot of traits to overload functions. Great examples are the `Build...` functions from the `IRBuilder` to build ir nodes

> [!WARNING]
> This project is still early in its developement. Bugs and miscompilations are expected. <br>
> ONLY USE YGEN FOR TOY COMPILERS


### Contributions

![Contribution activity](https://repobeats.axiom.co/api/embed/70cb0d167ed0a296468773b0bf8d569f74d1b33a.svg "Repobeats analytics image")

### Simple example
Here is a simple example on how to use Ygen to build an add function:
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
    &mut initializeAllTargets(Triple::host())?,
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
