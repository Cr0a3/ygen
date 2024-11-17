def Pat<Copy gr -> gr> {
    asm -> mov $out, $1;
}

def Pat<Copy fp -> fp> f32 {
    asm -> movss $out, $1;
}

def Pat<Copy fp -> fp> f64 {
    asm -> movsd $out, $1;
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

def Pat<GetFramePtr -> gr> {
    asm -> mov $out, X64Operand::Reg(X64Reg::Rbp()); 
}

def Pat<GetStackPtr -> gr> {
    asm -> mov $out, X64Operand::Reg(X64Reg::Rsp()); 
}