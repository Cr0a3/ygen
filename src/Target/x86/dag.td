def Pat<Copy gr -> gr> {
    asm -> mov $out, $1;
}

def Pat<Copy imm -> gr> {
    asm -> mov $out, $1;
}

def Pat<Copy mem -> gr> {
    asm -> mov $out, $1;
}

def Pat<Add gr, gr -> gr> {
    asm -> lea $out, [$1 + $2];
}

def Pat<Ret> {
    asm -> ret;
}