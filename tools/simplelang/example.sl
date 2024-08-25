/*

Example for simplelang

WHAT TO EXPECT:
 - this program should output Hello World!

HOW TO RUN:
 - go to the workspace main (where you can see the readme)
 - run: cargo run -p simplelang -- -in="tools/simplelang/example.sl" -o="out.o"
 - then link it into a executable by using: gcc out.o 
 - which should create a.exe in windows and a.out in linux

*/

import with (fmt: string, ...) printf // printf from libc

with (a: u64, b: u64) add: {
    return a + b;
}

extern with () main: {
    var x: string = "Hello World!\n";

    printf(x);

    var a: u64 = 1;
    var b: u64 = 0xd; // 0xd -> 13
    var expected: u64 = a + b; 

    printf("add(%d, %d) = %d # expected: %d\n", a, b, add(a, b), expected);

    return 0;
}