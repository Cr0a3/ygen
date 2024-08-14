import with (a: i32, b: i32) cfunc

extern with (a: i32, b: i32) func: {
    return cfunc(a, b) + b;
}