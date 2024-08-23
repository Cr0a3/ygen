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

extern with () main: {
    printf("Hello World!\n");

    var x: u32 = 5;

    printf("%d = %d", x, 5);

    return 0;
}