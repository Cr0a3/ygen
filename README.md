# Ygen - Yet another Code Generator
![GitHub branch check runs](https://img.shields.io/github/check-runs/Cr0a3/ygen/main?style=flat-square&label=build)
![Crates.io Version](https://img.shields.io/crates/v/Ygen?style=flat-square)
![GitHub Repo stars](https://img.shields.io/github/stars/cr0a3/ygen?style=flat-square)

Welcome to Ygen!
This repository contains the source code of the ygen project.

Ygen is a toolkit for building modern compilers, using a llvm like api.

## Why ygen?

You are probably wondering: Why would I choose ygen and not llvm?
Here are a few reasons:

- **Simplicity**: One of ygens main focus is simplicity which means to us that as much code as possible is readable and shared
- **Similare API**: Ygens API is very similar to LLVMs API. For example all function names for building ir are nearly the safe as llvms.
- **Simple start**: You can easily start with ygen. You do not need to install any dlls, or build it. Ygen also has many simple examples.

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

    let ty = FnTy(vec![TypeMetadata::i32, TypeMetadata::i32], TypeMetadata::i32);
    
    let func = module.add(
        "add", &ty
    );

    func.extrn();

    func.addBlock("entry");

    let val = func.BuildAdd(ty.arg(0), ty.arg(1));
    
    func.BuildRet( val );

    module.verify()?;

    // prints out the ir of the module
    println!("{}", module.dump());

    let triple = Triple::host();

    // compiles the module in the host assembly and saves it in the specified path
    module.emitToAsmFile(
        triple,
        &mut initializeAllTargets(triple)?,
        Path::new("out.asm")
    )?;

    // compiles the module to a host object file
    module
        .emitMachineCode(
            triple, 
            &mut initializeAllTargets(triple)?,
            false // is debugging metadata enabled
        )?.0.emit(
            OpenOptions::new().write(true).create(true).open("out.o")?, 
            None // if debugging metadata is enabled here is the outputed metadata
    )?;

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

### Support
Ygen currently supports following architectures:
 - `x86-64`
 - `wasm` [WIP]

### Copyright
This project is owned by Cr0a3 and licensed under the Apache2 License
