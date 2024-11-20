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

def Pat<Add .., .. -> ..> int
    map %t1, gr
{
    asm -> mov %t1, $1;
    asm -> add %t1, $2;
    asm -> mov $out, %t1;
}

def Pat<Sub .., .. -> ..> int
    map %t1, gr
{
    asm -> mov %t1, $1;
    asm -> sub %t1, $2;
    asm -> mov $out, %t1;
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

def Pat<Add fp, fp -> fp> f32 
    map %t1, fp
{
    asm -> mov %t1, $1;
    asm -> add %t1, $2;
    asm -> mov $out, %t1;
}

def Pat<Add fp, fp -> fp> <2 x i64> 
    map %t1, fp
{
    asm -> movdqa %t1, $1;
    asm -> paddq %t1, $2;
    asm -> movdqa $out, %t1;
}

def Pat<Add fp, fp -> fp> <4 x i32> 
    map %t1, fp
{
    asm -> movdqa %t1, $1;
    asm -> paddd %t1, $2;
    asm -> movdqa $out, %t1;
}

def Pat<Sub fp, fp -> fp> <4 x i32> 
    map %t1, fp
{
    asm -> movdqa %t1, $1;
    asm -> psubd %t1, $2;
    asm -> movdqa $out, %t1;
}

def Pat<Sub fp, fp -> fp> <2 x i64> 
    map %t1, fp
{
    asm -> movdqa %t1, $1;
    asm -> psubq %t1, $2;
    asm -> movdqa $out, %t1;
}

def Pat<Sub fp, fp -> fp> <8 x i16> 
    map %t1, fp
{
    asm -> movdqa %t1, $1;
    asm -> psubw %t1, $2;
    asm -> movdqa $out, %t1;
}

def Pat<Sub fp, fp -> fp> <16 x i8> 
    map %t1, fp
{
    asm -> movdqa %t1, $1;
    asm -> psubb %t1, $2;
    asm -> movdqa $out, %t1;
}

def Pat<Copy fp -> fp> <4 x i32> {
    asm -> movdqa $out, $1;
}