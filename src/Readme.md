# src

Ygens source code is splitted into (now) 7 subdirectorys:
|Name|Description|
|------|---------------|
|[CodeGen](https://github.com/Cr0a3/ygen/tree/main/src/CodeGen)|Shared code generation files (generates LIR)|
|[IR](https://github.com/Cr0a3/ygen/tree/main/src/IR)|Definition for all ir nodes and the ir lang|
|[Obj](https:://github.com/Cr0a3/ygen/tree/main/src/Obj)|Wrapper around the object and gimli libary to generate objects/excutables/shared libarys|
|[Optimizations](https:://github.com/Cr0a3/ygen/tree/main/src/Optimizations)|Contains optimizations for ygen-ir|
|[proc](https:://github.com/Cr0a3/ygen/tree/main/src/proc)|Ygens procedual macros|
|[Support](https:://github.com/Cr0a3/ygen/tree/main/src/Support)|Utility functions and classes|
|[Target](https:://github.com/Cr0a3/ygen/tree/main/src/Target)|Target depended code like instruction encoding, lir lowering, assembly|
|[debug.rs](https:://github.com/Cr0a3/ygen/tree/main/src/debug.rs)|Contains code for constructing debug metadata used by the `ObjectBuilder`|

For more information about each one refer to the ygen-dev guide