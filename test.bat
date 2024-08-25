@echo off
cargo run -p simplelang -- -in="tools/simplelang/example.sl" -o="out.o" -asm > out.asm
llvm-objdump -d out.o --x86-asm-syntax=intel > dump.asm
gcc out.o