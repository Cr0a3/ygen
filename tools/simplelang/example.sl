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

import func printf (fmt: string, ...) // printf from libc

func add (a: u32, b: u32) -> u32 {
    return a + b;
}

extern func main () -> void {
    var x: string = "Hello World!\n";

    printf(x);

    var a: u32 = 1;
    var b: u32 = 0xd; // 0xd -> 13
    var expected: u32 = a + b; 

    printf("add(%d, %d) = %d # expected: %d\n", a, b, add(a, b), expected);
}

extern func aaa () -> i32 {
    return 5;
}