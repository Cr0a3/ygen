extern with (a: i32, b: i32) cfunc: {}

with (a: i32, b: i32) func: {
    return (cfunc(a, b) + 2) * b;
}