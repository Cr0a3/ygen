import with (str: string) printf 

extern with () main: {
    var x: string = "Hallo Papa!\n"

    x += 5;

    printf(x);

    return 0;
}