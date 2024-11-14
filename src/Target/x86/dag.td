def Pat<Copy gr -> gr> {
    asm -> mov $out, $1;
}

def Pat<Copy imm -> gr> {
    asm -> mov $out, $1;
}

def Pat<Ret> {
    asm -> ret;
}