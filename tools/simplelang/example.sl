import with (a: i32, b: i32) cfunc

extern with (a: i32, b: i32) func: {
    var tmp: i32 = cfunc(a, b);

    return tmp + 4;
}