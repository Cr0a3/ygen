/*

Example for simplelang

WHAT TO EXPECT:
 - this program should output Hello World!

HOW TO RUN:
 - go to the workspace main (where you can see the readme)
 - run: cargo run -p simplelang -- -in="tools/simplelang/helloworld.sl" -o="out.o"
 - then link it into a executable by using: gcc out.o 
 - which should create a.exe in windows and a.out in linux

*/

import func printf (fmt: string, ...) // printf from libc


extern func main () -> void {
    printf("Hello World!");
}