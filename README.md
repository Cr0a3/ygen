# Ygen - Yet another Code Generator
Ygen is a libary to build backends for compilers.
It has an easy to use API like LLVMSwift. 

But one advantage: it is made in Rust. <br>
The dissavantages: everything.

It's primary focus is having an really easy to use API (which is also implemented easily so everybody can add their own ir nodes).

The IR doesn't differ to much from LLVMs.

### Simple example
Here is a simple example on how to use Ygen to build an simple add function:
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

    let entry = func.addBlock("entry");
    builder.positionAtEnd(entry); 

    let val = builder.BuildAdd(ty.arg(0), ty.arg(1));
    builder.BuildRet( val );

    module.verify().print();

    module.runPassMngr(passes);

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
module.emitToAsmFile(Path::new("out.asm"))?;
```

### Copyright
This project is owned by Cr0a3 and licensed under the MIT License
